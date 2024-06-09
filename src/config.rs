use std::{
    env,
    fs::{self, File},
    io::Read,
};

extern crate core;

#[derive(serde::Deserialize, Clone)]
pub struct Config {
    use_system_theme: bool,
    lock_screen: String,
}

const DEFAULT_CONFIG: &str = r#"{
    "use_system_theme": true,
    "lock_screen": ""
}
"#;

const CONFIG_PATH: &str = "/.config/pwrmenu/config.json";

impl Config {
    pub fn new() -> Config {
        let mut conf = Config {
            use_system_theme: true,
            lock_screen: "".to_string(),
        };

        let tmp_conf = conf.read_config_file();

        conf.lock_screen = tmp_conf.lock_screen;
        conf.use_system_theme = tmp_conf.use_system_theme;
        conf
    }

    fn read_config_file(&mut self) -> Config {
        if !self.config_file_exists() {
            self.create_config_file()
        }

        let mut file = File::open(self.get_config_file_path()).unwrap();
        let mut buff = String::new();
        file.read_to_string(&mut buff).unwrap();
        let conf = serde_json::from_str(&buff).unwrap();
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

    pub fn lock_screen(&self) -> String {
        self.lock_screen.to_string()
    }
}

