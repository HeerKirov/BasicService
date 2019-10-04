use log::*;
use super::model::global_setting::RegisterMode;
use super::model::user::CreatePath;
use super::service::transaction;
use super::service::user::{user_create, user_exists};
use super::service::global_setting::setting_create;
use super::service::token::token_clean_expired;
use super::util::config::*;

pub fn initialize_datasource() {
    let config = get_config();

    info!("Initialize database...");

    transaction(|trans| {
        //create admin user
        let username = config.get(BUILD_ADMIN_USERNAME);
        let password = config.get(BUILD_ADMIN_PASSWORD);
        let name = config.get(BUILD_ADMIN_NAME);
        if !user_exists(&trans, &username).unwrap() {
            if let Err(e) = user_create(&trans, &username, &password, &name, true, CreatePath::System) {
                error!("User creating failed: {}", e);
            }
        }else{
            warn!("User {} is already exists.", username);
        }
        //create global setting
        let register_mode = RegisterMode::from(&config.get(BUILD_SETTING_REGISTER_MODE)).expect("build.setting.register.mode is invalid");
        let effective_default = config.get(BUILD_SETTING_EFFECTIVE_DEFAULT).parse().unwrap();
        let effective_max = {
            let value = config.get(BUILD_SETTING_EFFECTIVE_MAX);
            if value.len() > 0 {
                value.parse().ok()
            }else{
                None
            }
        };
        if let Err(e) = setting_create(&trans, register_mode, effective_max, effective_default) {
            error!("Global setting create failed: {}", e);
        }
    });
    
    info!("Done.")
}

pub fn clean_expired_token() {
    info!("Clean expired token...");
    transaction(|trans| {
        match token_clean_expired(&trans) {
            Ok(size) => info!("{} record(s) is deleted.", size),
            Err(e) => error!("Token cleaning failed: {}", e)
        }
    });
    info!("Done.")
}