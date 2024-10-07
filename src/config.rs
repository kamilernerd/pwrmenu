use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::Read,
};

extern crate core;

#[derive(serde::Deserialize, Clone)]
pub struct Config {
    use_system_theme: bool,
    lock_screen: String,
    buttons_layout: HashMap<String, String>,
    anchor: HashMap<String, bool>,
    size: HashMap<String, String>,
}

const DEFAULT_CONFIG: &str = r#"{
    "use_system_theme": true,
    "lock_screen": "",
    "buttons_layout": {
        "orientation": "horizontal",
        "vertical_align": "center",
        "horizontal_align": "center"
    },
    "anchor": {
        "left": true,
        "right": true,
        "top": false,
        "bottom": true
    },
    "size": {
        "width": "500",
        "height": "300"
    }
}
"#;

const CONFIG_PATH: &str = "/.config/pwrmenu/config.json";

impl Config {
    pub fn new() -> Config {
        let mut conf = Config {
            use_system_theme: true,
            lock_screen: "".to_string(),
            buttons_layout: HashMap::from([
                ("orientation".to_string(), "horizontal".to_owned()),
                ("vertical_align".to_string(), "center".to_owned()),
                ("horizontal_align".to_string(), "center".to_owned()),
            ]),
            anchor: HashMap::from([
                ("left".to_owned(), true),
                ("right".to_owned(), true),
                ("top".to_owned(), false),
                ("bottom".to_owned(), true),
            ]),
            size: HashMap::from([
                ("width".to_owned(), "400".to_owned()),
                ("height".to_owned(), "400".to_owned()),
            ]),
        };

        let tmp_conf = conf.read_config_file();

        conf.lock_screen = tmp_conf.lock_screen;
        conf.use_system_theme = tmp_conf.use_system_theme;
        conf.buttons_layout = tmp_conf.buttons_layout;
        conf.anchor = tmp_conf.anchor;
        conf.size = tmp_conf.size;
        conf
    }

    fn read_config_file(&mut self) -> Config {
        if !self.config_file_exists() {
            self.create_config_file()
        }

        let mut file = File::open(self.get_config_file_path()).unwrap();
        let mut buff = String::new();
        file.read_to_string(&mut buff).unwrap();
        let conf = serde_json::from_str(&buff).expect("Invalid configuration");
        conf
    }

    fn config_file_exists(&self) -> bool {
        fs::metadata(self.get_config_file_path()).is_ok()
    }

    fn get_config_file_path(&self) -> String {
        let home_dir = env::home_dir().unwrap().as_path().display().to_string();
        let path: String = home_dir + CONFIG_PATH;
        path
    }

    fn create_config_file(&mut self) {
        let config_file_path = self.get_config_file_path();
        let path = std::path::Path::new(&config_file_path);
        let prefix = path.parent().unwrap();

        std::fs::create_dir_all(prefix).unwrap();

        let _ = match File::create(self.get_config_file_path()) {
            Err(why) => panic!("couldn't create {}", why),
            Ok(file) => file,
        };
        let _ = fs::write(self.get_config_file_path(), DEFAULT_CONFIG);
    }

    pub fn use_system_theme(&self) -> bool {
        self.use_system_theme
    }

    pub fn get_lock_screen(&self) -> String {
        self.lock_screen.to_string()
    }

    pub fn get_anchor(&self) -> HashMap<String, bool> {
        self.anchor.clone()
    }

    pub fn get_size(&self) -> HashMap<String, String> {
        self.size.clone()
    }

    pub fn get_buttons_layout(&self) -> HashMap<String, String> {
        self.buttons_layout.clone()
    }

    pub fn get_buttons_orientation(&self) -> String {
        self.buttons_layout["orientation"].to_owned()
    }

    pub fn get_buttons_valign(&self) -> String {
        if !self.buttons_layout.contains_key("vertical_align") {
            return "center".to_owned();
        }
        self.buttons_layout["vertical_align"].to_owned()
    }

    pub fn get_buttons_halign(&self) -> String {
        if !self.buttons_layout.contains_key("horizontal_align") {
            return "center".to_owned();
        }
        self.buttons_layout["horizontal_align"].to_owned()
    }
}
