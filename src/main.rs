use clap::{Arg, App, ArgMatches, ErrorKind};
use std::env;
use std::path::{PathBuf, Path};
use env_logger::Env;

#[macro_use] extern crate clap;
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
        .arg(Arg::with_name("MAX_EPS").long("count").short("c").takes_value(true).help("Max amount of episodes to unpack"))
}

fn get_path_from_env(archive_type: ArchiveType) -> Option<String> {
    let var = if archive_type == ArchiveType::TV { "TV" } else { "MOVIE" };
    let path = env::var(var);
    match path {
        Ok(p) => Some(p),
        Err(_) => { error!("Environment variable {} not set", var); None }
    }
}

fn get_cwd() -> PathBuf {
    env::current_dir().unwrap()
}

fn get_episode_count(args: &ArgMatches) -> Option<u32> {
    let count = match value_t!(args, "MAX_EPS", u32) {
        Err(e) => match e.kind {
            ErrorKind::ArgumentNotFound => None,
            _ => {
                error!("Value for episode count is invalid");
                std::process::exit(2);
            }
        },
        Ok(c) => Some(c)
    };
    count
}

fn get_arg_directory(args: &ArgMatches) -> Option<PathBuf> {
    let directory = match args.value_of("DIRECTORY") {
        None => return None,
        Some(d) => PathBuf::from(d)
    };

    let path = directory.as_path();
    if path.is_absolute() {
        error!("Destination path may not be absolute");
        std::process::exit(3);
    };

    Some(directory)
}

fn get_full_destination(args: &ArgMatches, archive_type: ArchiveType) -> PathBuf {
    let base_destination = match get_path_from_env(archive_type) {
        None => std::process::exit(1),
        Some(p) => PathBuf::from(p)
    };

    let destination = match get_arg_directory(args) {
        Some(d) => base_destination.join(d),
        None => base_destination
    };
    info!("Unpacking to directory {}", destination.display());

    destination
}

fn unpack_movie() {

}

fn main() {
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    let matches = build_help().get_matches();
    let archive_type = if matches.is_present("MOVIE") {ArchiveType::Movie } else { ArchiveType::TV };

    let count = get_episode_count(&matches);
    let destination= get_full_destination(&matches, archive_type);
}
