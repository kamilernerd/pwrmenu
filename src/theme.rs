use std::{
    env,
    fs::{self, File},
};

extern crate core;

use gtk4 as gtk;

pub struct Theme {
    theme_css: String,
    display: gtk4::gdk::Display,
}

const DEFAULT_STYTLE: &str = "#top {
    background-color: rgba(22,22,22,0.96);
}

button:focus {
    outline: none;
}

#buttons-wrapper {
    background-color: rgba(22,22,22,1);
}

#buttons-wrapper > button {
    border-radius: 1rem;
    border: 2px solid rgba(238, 83, 150, 0.5);
    background-color: rgba(36,36,36,0.5);
}

#buttons-wrapper > button:hover {
    background-color: rgba(36,36,36,1);
    border: 2px solid rgba(238, 83, 150, 1);
}
";

const THEME_PATH: &str = "/.config/pwrmenu/theme.css";

impl Theme {
    pub fn new(display: gtk4::gdk::Display) -> Theme {
        Theme {
            theme_css: DEFAULT_STYTLE.to_string(),
            display,
        }
    }

    pub fn load_theme(&mut self) {
        self.read_theme_file();

        let provider = gtk::CssProvider::new();
        provider.load_from_data(&self.theme_css.as_str());

        gtk::style_context_add_provider_for_display(
            &self.display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_USER,
        );
    }

    fn read_theme_file(&mut self) {
        if !self.theme_file_exists() {
            self.create_theme_file()
        }
        let css = fs::read_to_string(self.get_theme_file_path()).expect("Theme file is missing");
        self.theme_css = css.to_string();
    }

    fn theme_file_exists(&self) -> bool {
        fs::metadata(self.get_theme_file_path()).is_ok()
    }

    fn get_theme_file_path(&self) -> String {
        let home_dir = env::home_dir().unwrap().as_path().display().to_string();
        let path: String = home_dir + THEME_PATH;
        path
    }

    fn create_theme_file(&mut self) {
        let theme_file_path = self.get_theme_file_path();
        let path = std::path::Path::new(&theme_file_path);
        let prefix = path.parent().unwrap();

        std::fs::create_dir_all(prefix).unwrap();

        let _ = match File::create(self.get_theme_file_path()) {
            Err(why) => panic!("couldn't create {}", why),
            Ok(file) => file,
        };
        let _ = fs::write(self.get_theme_file_path(), &self.theme_css);
    }
}
