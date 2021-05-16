use std::collections::hash_map::DefaultHasher;
use std::env;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::time::SystemTime;
use structopt::StructOpt;
use users::get_current_username;

#[derive(StructOpt)]
struct Cli {
    command: String,
    message: Option<String>,
}

#[derive(Hash)]
struct Commit {
    author: String,
    message: String,
    time: SystemTime,
}

fn init(input_path: PathBuf) {
    let mut tags = input_path.clone();
    tags.push(".repo/refs/tags");
    fs::create_dir_all(tags).expect("Error while creating refs directory");

    let mut snapshots = input_path.clone();
    snapshots.push(".repo/snapshots");
    fs::create_dir_all(snapshots).expect("Error while creating refs directory");

    println!("Initialized repository in current directory")
}

fn commit(input_path: PathBuf, message: String) {
    let current_user = get_current_username().unwrap().into_string().unwrap();

    let commit = Commit {
        author: current_user,
        message: message,
        time: SystemTime::now(),
    };

    let mut hasher = DefaultHasher::new();
    commit.hash(&mut hasher);

    let hash = hasher.finish();

    println!("{0}", hash);
}

fn main() {
    let args = Cli::from_args();
    let input_path = env::current_dir().unwrap();

    match args.command.as_str() {
        "init" => init(input_path),
        "commit" => commit(input_path, args.message.unwrap()),
        _ => panic!("Invalid command"),
    }
}
