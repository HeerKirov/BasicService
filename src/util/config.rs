use std::sync::Mutex;
use std::collections::HashMap;
use std::fs;
use log::debug;

pub const SECRET_KEY: &'static str = "secret.key";
pub const WEB_API_PREFIX: &'static str = "web.api.prefix";
pub const WEB_PORT: &'static str = "web.port";
pub const DB_HOST: &'static str = "db.host";
pub const DB_PORT: &'static str = "db.port";
pub const DB_NAME: &'static str = "db.name";
pub const DB_USERNAME: &'static str = "db.username";
pub const DB_PASSWORD: &'static str = "db.password";
pub const BUILD_ADMIN_USERNAME: &'static str = "build.admin.username";
pub const BUILD_ADMIN_PASSWORD: &'static str = "build.admin.password";
pub const BUILD_ADMIN_NAME: &'static str = "build.admin.name";
pub const BUILD_SETTING_REGISTER_MODE: &'static str = "build.setting.register.mode";
pub const BUILD_SETTING_EFFECTIVE_MAX: &'static str = "build.setting.token.effective.max";
pub const BUILD_SETTING_EFFECTIVE_DEFAULT: &'static str = "build.setting.token.effective.default";
pub const STATIC_COVER_PREFIX: &'static str = "static.cover.prefix";
pub const STATIC_COVER_DIRECTORY: &'static str = "static.cover.directory";

lazy_static! {
    static ref FILEPATH: Mutex<String> = Mutex::new("config.properties".to_string());
    static ref CONF: Config = Config::load_from_file(FILEPATH.lock().unwrap().to_string());
}

pub struct Config {
    map: HashMap<String, String>
}

impl Config {
    fn load_from_file(filename: String) -> Self {
        debug!("Load configuration from file {}", filename);
        let content = fs::read_to_string(filename).expect("cannot open config file");
        let mut map: HashMap<String, String> = HashMap::new();

        for item in content.split("\n") {
            if let Some(i) = item.find("=") {
                let key = item[..i].to_string();
                let value = item[i+1..].to_string();
                map.insert(key, value);
            }
        }
        Self {
            map: map
        }
    }
    pub fn get(&self, key: &str) -> &String {
        self.map.get(key).expect(&format!("cannot find property {}", key))
    }
}

pub fn set_filepath(path: String) {
    *FILEPATH.lock().unwrap() = path;
}

pub fn get_config() -> &'static Config {
    &CONF
}