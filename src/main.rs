extern crate core;
use core::ffi::c_int;
use std::borrow::Borrow;
use std::ffi::CString;

mod config;
mod theme;

use config::Config;
use gdk::ffi::gdk_display_get_default;
use gdk::ffi::gdk_display_get_monitor_at_surface;
use gdk::ffi::gdk_monitor_get_geometry;
use gdk::ffi::gdk_surface_new_toplevel;
use gdk::ffi::GdkRectangle;
use gtk::{Application, ApplicationWindow, Box, Button, Orientation};
use gtk4 as gtk;
use gtk4::prelude::ApplicationExt;
use gtk4::prelude::ApplicationExtManual;
use gtk4::prelude::BoxExt;
use gtk4::prelude::ButtonExt;
use gtk4::prelude::GtkApplicationExt;
use gtk4::prelude::GtkWindowExt;
use gtk4::prelude::WidgetExt;
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use theme::Theme;

extern "C" {
    fn sys_reboot() -> c_int;
    fn sys_poweroff() -> c_int;
    fn sys_suspend() -> c_int;
    fn sys_logout() -> c_int;
    fn sys_lock(cmd: CString) -> c_int;
}

const OPTIONS: [&str; 5] = ["lock", "logout", "suspend", "reboot", "shutdown"];

fn main() -> glib::ExitCode {
    let config = Config::new();
    let application = Application::builder()
        .application_id("no.kamiloracz.pwrmenu")
        .build();

    let app = application.to_owned();
    application.connect_activate(move |a| {
        build_ui(a, config.borrow().clone());
        if !config.use_system_theme() {
            let disp = app.windows()[0].display();
            let mut theme = Theme::new(disp);
            theme.load_theme();
        }
    });

    application.run()
}

fn build_ui(app: &gtk::Application, config: Config) {
    let (width, height) = window_size_from_config(config.get_size());
    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(width)
        .default_height(height)
        .decorated(false)
        .resizable(false)
        .show_menubar(false)
        .name("top")
        .build();

    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.exclusive_zone();

    window.set_keyboard_mode(gtk4_layer_shell::KeyboardMode::Exclusive);

    let edge_map = [
        ("left", Edge::Left),
        ("right", Edge::Right),
        ("top", Edge::Top),
        ("bottom", Edge::Bottom),
    ];

    let anchors = config.get_anchor().to_owned();
    for (edge, edge_enum) in edge_map {
        window.set_anchor(edge_enum, anchors[edge])
    }

    let button_wrapper = Box::builder().orientation(Orientation::Horizontal).build();
    button_wrapper.set_halign(gtk::Align::Center);
    button_wrapper.set_valign(gtk::Align::Center);
    button_wrapper.set_widget_name("buttons-wrapper");

    let mut btns: Vec<Button> = vec![];
    for opt in OPTIONS {
        let btn = Button::builder()
            .label(opt.to_string())
            .name(opt.to_string())
            .build();

        let conf = config.to_owned();
        btn.connect_clicked(move |_ev| unsafe {
            match String::from(opt).as_str() {
                "suspend" => sys_suspend(),
                "reboot" => sys_reboot(),
                "shutdown" => sys_poweroff(),
                "logout" => sys_logout(),
                "lock" => sys_lock(CString::new(conf.get_lock_screen()).unwrap()),
                _ => 0,
            };
            std::process::exit(0);
        });
        btns.push(btn);
    }

    for btn in btns {
        button_wrapper.append(&btn);
    }

    window.set_child(Some(&button_wrapper));

    let event_controller = gtk::EventControllerKey::new();

    event_controller.connect_key_pressed(|_, key, _, _| {
        match key {
            gtk4::gdk::Key::Escape => {
                std::process::exit(0);
            }
            _ => (),
        }
        glib::Propagation::Proceed
    });

    window.add_controller(event_controller);
    window.present();
}

// Get current display's geometry
fn get_active_monitor_geometry() -> GdkRectangle {
    unsafe {
        let disp = gdk_display_get_default();
        let surf = gdk_surface_new_toplevel(disp);
        let monitor = gdk_display_get_monitor_at_surface(disp, surf);

        let geo: *mut GdkRectangle = &mut GdkRectangle {
            width: 0,
            height: 0,
            y: 0,
            x: 0,
        };

        gdk_monitor_get_geometry(monitor, geo as *mut GdkRectangle);
        return geo.as_ref().unwrap().to_owned();
    }
}

fn window_size_from_config(size: std::collections::HashMap<String, String>) -> (i32, i32) {
    let screen = get_active_monitor_geometry();
    let mut width = screen.width;
    let mut height = screen.height;

    if size.get("width").unwrap() == "screen" {
        width = screen.width;
    } else {
        width = size
            .get("width")
            .unwrap()
            .parse::<i32>()
            .expect("Invalid value width");
    }

    if size.get("height").unwrap() == "screen" {
        height = screen.height;
    } else {
        height = size
            .get("height")
            .unwrap()
            .parse::<i32>()
            .expect("Invalid value height");
    }

    return (width, height);
}
