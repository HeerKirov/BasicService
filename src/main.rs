extern crate basic_service;

use std::env;
use basic_service::util::config::set_filepath;
use basic_service::util::parameters::Parameters;
use basic_service::server::run_server;
use basic_service::task::{initialize_datasource, clean_expired_token};

fn main() {
    Parameters::new(env::args().collect())
        .side(|_| env_logger::init())
        .side(|p| set_filepath(p.param("config", "config.properties".to_string())))
        .execute(|command, _| match command {
            "runserver" => run_server().unwrap(),
            "initialize-datasource" => initialize_datasource(),
            "clean-expired-token" => clean_expired_token(),
            _ => println!("no such command \"{}\"", command)
    });
}
