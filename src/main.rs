use std::collections::hash_map::DefaultHasher;
use std::env;
use std::fs;
use std::hash::{Hash, Hasher};
use std::io;
use std::path::PathBuf;
use std::time::SystemTime;
use structopt::StructOpt;
use users::get_current_username;
extern crate fs_extra;

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
    let entries = fs::read_dir("./")
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .filter(|res| res.as_ref().unwrap().to_str().unwrap() != "./.repo")
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();
    let mut output_path = input_path.clone();
    output_path.push(".repo/snapshots");
    output_path.push(hash.to_string());
    let output_copy = output_path.clone();
    fs_extra::dir::create(output_copy, false).expect("Error while creating output directory");
    let options = fs_extra::dir::CopyOptions::new();
    fs_extra::copy_items(&entries, output_path, &options)
        .expect("Error while copying to snapshot directory");
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
