use crate::credentials::UserDatabase;
use hermes_common::network_stats::NetworkAnalyzer;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::fs;
use homedir::my_home;
use lazy_static::lazy_static;

pub fn host_directory() -> PathBuf {
    let home_r = my_home();
    let home = match home_r {
        Ok(p) => {
            match p {
                Some(s) => s,
                None => panic!("Unable to get home directory")
            }
        }
        Err(_) => panic!("Unable to get home directory")
    };

   home.join("cnt")
}
pub fn root_directory() -> PathBuf {
    host_directory().join("data")
}
pub fn user_database_path() -> PathBuf {
    host_directory().join("users.json")
}
pub fn file_owner_db_path() -> PathBuf {
    host_directory().join("files.json")
}
pub fn network_analyzer_path() -> PathBuf {
    host_directory().join("stats.json")
}

pub fn ensure_directories() -> bool {
    if fs::create_dir_all(host_directory()).is_err() || fs::create_dir_all(root_directory()).is_err() {
        return false;
    }

    let mut results: Vec<Result<std::fs::File, std::io::Error>> = Vec::new();
    if !user_database_path().exists() {
        results.push(
            fs::OpenOptions::new().create_new(true).truncate(false).open(user_database_path())
        );
    }
    if !file_owner_db_path().exists() {
        results.push(
            fs::OpenOptions::new().create_new(true).truncate(false).open(file_owner_db_path())
        );
    }
    if !network_analyzer_path().exists() {
        results.push( 
            fs::OpenOptions::new().create_new(true).truncate(false).open(network_analyzer_path())
        );
    }

    for result in results {
        if result.is_err() && result.err().unwrap().kind() != ErrorKind::AlreadyExists {
            return false;
        }
    }

    true
 }

lazy_static! {
    pub static ref NETWORK_ANALYZER: NetworkAnalyzer = NetworkAnalyzer::new();
    pub static ref USER_DB: UserDatabase = UserDatabase::new();
}  