use std::path::{Path, PathBuf};
use std::fs::canonicalize;
use std::collections::HashMap;
use std::fmt::{Display, Debug};

use crate::credentials::Credentials;
use crate::io_loc::root_directory;
use hermes_common::file_io::{FileInfo, DirectoryContent, FileType, JsonFile};
use serde::{Deserialize, Serialize};

pub fn move_relative(raw_path: &str, curr_dir: &Path) -> Option<PathBuf> {
    let as_path = PathBuf::from(raw_path);
    if as_path.is_absolute() {
        None
    }
    else {
        Some(curr_dir.join(as_path))
    }
}
pub fn resolve_path(path: PathBuf) -> Option<PathBuf> {
    match canonicalize(path) {
        Ok(f) => Some(f),
        Err(_) => None
    }
}
pub fn make_relative(path: &Path) -> Option<PathBuf> {
    if !is_path_valid(path) {
        None
    }
    else if path.is_absolute() {
        Some(path.to_path_buf())
    }
    else {
        path.strip_prefix(root_directory()).map(|p| p.to_path_buf()).ok()
    }
}
pub fn is_path_valid(path: &Path) -> bool {
    let root_dir = root_directory();

    let our_path_len = path.iter().count();
    let target_size = root_dir.iter().count();
    if our_path_len == target_size && path == root_dir {
        true
    } else if our_path_len < target_size {
        false
    } else {
        //Compare each element up to our items

        let target_parts = PathBuf::from_iter(path.components().take(target_size));
        target_parts == root_dir
    }
}

#[test]
pub fn test_move_relative() {
    let curr_dir = root_directory();
    println!("{:?}", &curr_dir);

    assert_eq!( move_relative("thing", &curr_dir).unwrap(), PathBuf::from("/Users/exdisj/cnt/data/thing"));

    assert_eq!( move_relative("", &curr_dir).unwrap(), PathBuf::from("/Users/exdisj/cnt/data"));

    assert_eq!( move_relative(".", &curr_dir).unwrap(), PathBuf::from("/Users/exdisj/cnt/data"));

    assert_eq!( move_relative("..", &curr_dir).unwrap(), PathBuf::from("/Users/exdisj/cnt/data/.."));
}
#[test]
pub fn test_make_relative() {

}
#[test]
pub fn test_is_valid() {

}

#[derive(Serialize, Deserialize)]
pub struct ServerFile {
    id: u32,
    path: PathBuf,
    kind: FileType,
    owner: Option<Credentials>
}
impl Debug for ServerFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "'{:?}' ({:?}):({})",
            &self.path,
            &self.kind,
            if let Some(u) = self.owner.as_ref() {
                u.username()
            } else {
                "any"
            }
        )
    }
}
impl Display for ServerFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}'s {} file at '{:?}'",
            if let Some(u) = self.owner.as_ref() {
                u.username()
            } else {
                "any"
            },
            &self.kind,
            &self.path
        )
    }
}
impl PartialEq<Credentials> for ServerFile {
    fn eq(&self, other: &Credentials) -> bool {
        match self.owner.as_ref() {
            Some(u) => u == other,
            _ => false
        }
    }
}
impl PartialEq for ServerFile {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl ServerFile {
    fn new(path: PathBuf, owner: Option<Credentials>, kind: FileType, id: u32) -> Result<Self, std::io::Error> {
        if !path.exists() {
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, "path provided does not exist"))
        } else {
            Ok(
                Self {
                    id,
                    path,
                    owner,
                    kind
                }
            )
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn path(&self) -> &Path {
        &self.path
    }
    pub fn owner(&self) -> Option<&Credentials> {
        self.owner.as_ref()
    }
    pub fn set_owner(&mut self, cred: Option<Credentials>) {
        self.owner = cred
    }
    pub fn file_type(&self) -> FileType {
        self.kind
    }
}

pub struct FileDatabase {
    file: JsonFile,
    data: Vec<ServerFile>,
    curr_id: u32
}
impl Default for FileDatabase {
    fn default() -> Self {
        Self::new()
    }
}
impl FileDatabase {
    pub fn new() -> Self {
        Self {
            file: JsonFile::new(),
            data: vec![],
            curr_id: 0
        }
    }

    fn get_next_id(&mut self) -> u32 {
        self.curr_id += 1;

        self.curr_id
    }

    pub fn index(&mut self, host_dir: &Path) -> Result<(), String> {
        /*
            We need to:

            1. Review everything in the whole directory structure
            2. Load all contents into a HashMap<String, &ServerFile>
            3. Find all files that are in our directory that are *not* in the HashMap
            4. Add those files into the structure, under the Any user. 
         */

        if !self.file.is_open() {
            return Err(String::from("database is not currently open"));
        }

        let mut loaded_files: HashMap<String, &ServerFile> = HashMap::new();
        for file in &self.data {
            let path = match file.path.to_str() {
                Some(s) => String::from(s),
                None => return Err(String::from("could not convert path to string"))
            };
            
            if let Some(f) = loaded_files.insert(path, file) {
                return Err(format!("duplicate path determined at {:?}", f.path));
            }
        }

        todo!()
    }
    fn open(&mut self, path: &str) -> Result<(), String> {
        let contents = self.file.open(path)?;

        let list: Result<Vec<ServerFile>, _> = serde_json::from_str(&contents);
        match list {
            Ok(l) => {
                self.data = l;

                let max_id = self.data.iter().map(|x| x.id).max();
                self.curr_id = match max_id {
                    Some(x) => x,
                    None => 0
                };

                Ok(())
            },
            Err(e) => Err(e.to_string())
        }
    }
    pub fn save(&self) -> Result<(), String> {
        let contents_str = match serde_json::to_string(&self.data) {
            Ok(c) => c,
            Err(e) => return Err(e.to_string())
        };

        self.file.save(&contents_str)
    }

    pub fn close(&mut self) {
        self.data.clear();
        self.file.close();
    }

    pub fn get_file(&self, id: u32) -> Option<&ServerFile> {
        self.data.iter().find(|x| x.id == id)
    }
    pub fn get_file_mut(&mut self, id: u32) -> Option<&mut ServerFile> {
        self.data.iter_mut().find(|x| x.id == id)
    }
    pub fn get_file_id(&self, path: &Path) -> Option<u32> {
        Some( self.data.iter().find(|x| x.path == path)?.id )
    }

    pub fn set_file_owner(&mut self, id: u32, user: Credentials) -> Result<(), String> {
        let file = match self.get_file_mut(id) {
            Some(s) => s,
            None => return Err(format!("file not found with id {}", id))
        };

        file.set_owner(Some(user));
        Ok(())
    }

    pub fn register_file(&mut self, path: PathBuf, owner: Option<Credentials>, kind: FileType) -> Result<u32, String> {
        //First we determine if it is already contained

        {
            let prev_contained = self.data.iter().find(|x| x.path == path);
            if let Some(i) = prev_contained {
                return Err(
                    format!(
                        "path previously contained by owner '{}'",
                        if let Some(u) = i.owner() {
                            u.username()
                        } else {
                            "any"
                        }
                    )
                )
            }
        }

        let new_file = ServerFile::new(
            path,
            owner,
            kind,
            self.get_next_id()
        );

        match new_file {
            Ok(f) => {
                let id = f.id();
                self.data.push(f);

                Ok(id)
            },
            Err(e) => Err(e.to_string())
        }
    }

}