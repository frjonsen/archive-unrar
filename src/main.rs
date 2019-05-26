use clap::{Arg, App, ArgMatches};
use std::env;

#[macro_use] extern crate log;

#[derive(PartialEq)]
enum ArchiveType {
    Movie,
    TV
}

fn build_help() -> App<'static, 'static> {
    App::new("Archive Unrar")
        .version("0.1")
        .about("Unrar achives")
        .arg(Arg::with_name("MOVIE").long("movie").short("m").help("Unpack archive as a movie"))
        .arg(Arg::with_name("DIRECTORY").long("dir").short("d").takes_value(true).help("Subdirectory to unpack to"))
}

fn get_path_from_env(archive_type: ArchiveType) -> Option<String> {
    let var = if archive_type == ArchiveType::TV { "TV" } else { "MOVIE" };
    let path = env::var(var);
    match path {
        Ok(p) => Some(p),
        Err(_) => { error!("Environment variable {} not set", var); None }
    }
}

fn main() {
    env_logger::init();
    let matches = build_help().get_matches();
    let archive_type = if matches.is_present("MOVIE") {ArchiveType::Movie } else { ArchiveType::TV };
    let path = match get_path_from_env(archive_type) {
        None => return,
        Some(p) => p
    };
    debug!("Unpacking to directory {}", path);
}
