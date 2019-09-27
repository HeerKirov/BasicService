extern crate basic_service;

use std::env;
use basic_service::util::config::set_filepath;
use basic_service::util::parameters::Parameters;
use basic_service::server::run_server;
use basic_service::db::build_datasource;

fn main() {
    Parameters::new(env::args().collect())
        .side(|_| env_logger::init())
        .side(|p| set_filepath(p.param("config", "config.properties".to_string())))
        .execute(|command, _| match command {
            "runserver" => run_server(),
            "build-datasource" => build_datasource(),
            _ => panic!("no such command {}", command)
    });
}
