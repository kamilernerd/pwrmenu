extern crate cc;

fn main() {
    cc::Build::new().file("src/reboot.c").compile("reboot");
}

