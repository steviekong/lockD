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
use std::fmt;
extern crate chrono;
use chrono::offset::Utc;
use chrono::DateTime;
use std::fs::OpenOptions;
use std::io::Write;

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

impl fmt::Display for Commit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let datetime: DateTime<Utc> = self.time.into();
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);

        let hash = hasher.finish();
        write!(
            f,
            "{0} {1} {2} {3}",
            hash,
            self.author,
            datetime.format("%d/%m/%Y %T"),
            self.message
        )
    }
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
        .filter(|res| {
            res.as_ref().unwrap().to_str().unwrap() != "./.repo"
                && res.as_ref().unwrap().to_str().unwrap() != "./.git"
        })
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

    let log_str = commit.to_string();
    let mut log_file_path = input_path.clone();
    log_file_path.push(".repo/snapshots/.commit");
    let mut log_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(log_file_path)
        .unwrap();

    if let Err(e) = writeln!(log_file, "{}", log_str) {
        eprintln!("Couldn't write to file: {}", e);
    }
    println!("Wrote commit to log file");
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
