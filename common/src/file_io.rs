use serde::{Serialize, Deserialize};
use std::fs::File;
use std::net::TcpStream;
use std::{fmt::{Debug, Display}, str::FromStr};
use std::path::Path;
use std::io::{Read, Write};

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum FileType {
    Text,
    Audio,
    Video,
    Binary,
    Archive
}
impl Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, 
            "{}", 
            match self {
                Self::Text => "text",
                Self::Audio => "audio",
                Self::Video => "video",
                Self::Archive => "archive",
                Self::Binary => "binary"
            }
        )
    }
}
impl FromStr for FileType{
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "text" => Ok(Self::Text),
            "audio" => Ok(Self::Audio),
            "video" => Ok(Self::Video),
            "binary" => Ok(Self::Binary),
            "archive" => Ok(Self::Archive),
            _ => Err(format!("could not deduce file type from '{s}'"))
        }
    }
}

pub fn get_file_type(path: &Path) -> Option<FileType> {
    let extr = path.extension()?.to_str()?;
    match extr {
        "mp4" | "mov" | "avi" | "wvm" => Some(FileType::Video),
        "mp3" | "wav" | "aac" | "flac" | "aiff" => Some(FileType::Audio),
        "pdf" | "docx" | "pptx" | "xlsx" => Some(FileType::Binary),
        "tar" | "gz" | "zip" => Some(FileType::Archive),
        "txt" | "rtf" | "md" => Some(FileType::Text),
        _ => None
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum DirectoryContent{
    File(FileInfo),
    Dir(DirectoryInfo)
}
impl Debug for DirectoryContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::File(fi) => write!(f, "File '{}'", fi.name()),
            Self::Dir(d) => write!(f, "Directory '{}' (Len {})", d.name(), d.contents().len())
        }
    }
}
impl Display for DirectoryContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::File(fi) => write!(f, "{}", fi),
            Self::Dir(d) => write!(f, "{}", d)
        }
    }
}
impl DirectoryContent {
    pub fn is_file(&self) -> bool {
        matches!(self, Self::File(_))
    }
    pub fn is_directory(&self) -> bool {
        matches!(self, Self::Dir(_))
    }

    pub fn as_file(self) -> Option<FileInfo> {
        match self {
            Self::File(f) => Some(f),
            _ => None
        }
    }
    pub fn as_directory(self) -> Option<DirectoryInfo> {
        match self {
            Self::Dir(d) => Some(d),
            _ => None
        }
    }
    pub fn as_file_ref(&self) -> Option<&FileInfo> {
        match self {
            Self::File(f) => Some(f),
            _ => None
        }
    }
    pub fn as_directory_ref(&self) -> Option<&DirectoryInfo> {
        match self {
            Self::Dir(d) => Some(d),
            _ => None
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct FileInfo {
    name: String,
    kind: FileType,
    owner: String,
    size: u32
}
impl Debug for FileInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", &self.name, &self.kind)
    }
}
impl Display for FileInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n\tSize: {}\n\tType: {}\n\tOwner: {}\n\t", &self.name, &self.size, &self.kind, &self.owner)
    }   
}
impl FileInfo {
    pub fn new(name: String, owner: String, kind: FileType, size: u32) -> Self {
        Self {
            name,
            owner, 
            kind,
            size
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn kind(&self) -> FileType {
        self.kind
    }
    pub fn owner(&self) -> &str {
        &self.owner
    }
    pub fn size(&self) -> u32 {
        self.size
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct DirectoryInfo {
    name: String,
    contents: Vec<DirectoryContent>
}
impl Debug for DirectoryInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (files, dirs) = self.spill_ref();
        write!(f, "Directory: {} files, {} directories", files.len(), dirs.len())
    }
}
impl Display for DirectoryInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Directory: {} contents", self.contents.len())
    }
}
impl DirectoryInfo {

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn contents(&self) -> &Vec<DirectoryContent> {
        &self.contents
    }
    pub fn get_files(&self) -> Vec<&FileInfo> {
        self.contents.iter().filter(|x| x.is_file()).map(|x| x.as_file_ref().unwrap()).collect()
    }
    pub fn get_directories(&self) -> Vec<&DirectoryInfo> {
        self.contents.iter().filter(|x| x.is_directory()).map(|x| x.as_directory_ref().unwrap()).collect()
    }
    pub fn spill(self) -> (Vec<FileInfo>, Vec<DirectoryInfo>) {
        let mut files = Vec::<FileInfo>::new();
        let mut dirs = Vec::<DirectoryInfo>::new();

        for item in self.contents {
            match item {
                DirectoryContent::File(f) => files.push(f),
                DirectoryContent::Dir(d) => dirs.push(d)
            }
        }

        (files, dirs)
    }
    pub fn spill_ref(&self) -> (Vec<&FileInfo>, Vec<&DirectoryInfo>) {
        let mut files: Vec<&FileInfo> = Vec::new();
        let mut dirs: Vec<&DirectoryInfo> = Vec::new();

        for item in &self.contents {
            match item {
                DirectoryContent::File(f) => files.push(f),
                DirectoryContent::Dir(d) => dirs.push(d)
            }
        }

        (files, dirs)
    }

    pub fn append_content(&mut self, item: DirectoryContent) {
        self.contents.push(item);
    }
    pub fn append_many_content(&mut self, items: &mut Vec<DirectoryContent>) {
        self.contents.append(items);
    }
    pub fn set_content(&mut self, items: Vec<DirectoryContent>) {
        self.contents = items;
    }
}

const BUFF_SIZE: u32 = 4096;

pub fn read_file_for_network(path: &Path) -> Option<Vec<Vec<u8>>> {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return None
    };

    let mut buff = String::new();
    if file.read_to_string(&mut buff).is_err() {
        return None;
    }

    Some(split_binary_for_network(buff.into_bytes()))
}
pub fn split_binary_for_network(contents: Vec<u8>) -> Vec<Vec<u8>> {
    let windows = (contents.len() / 4096) + 1;
    if windows == 1 {
        vec![contents]
    }
    else {
        let mut result = Vec::<Vec<u8>>::new();
        let mut vals = contents.into_iter().peekable();

        while vals.peek().is_some() {
            result.push(vals.by_ref().take(10).collect());
        }

        result
    }
}

fn receive_network_data<P>(s: &mut TcpStream, frame_count: u32, p: &mut P) -> bool 
    where P: FnMut(&mut Vec<u8>) -> bool{
    if frame_count == 0 {
        return false;
    }

    let total_windows = frame_count as f32;
    let mut frame_size = frame_count * BUFF_SIZE;
    let mut windows_so_far: f32 = 0.0;

    while frame_size > 0 && windows_so_far < total_windows {
        let mut contents = vec![0; std::mem::size_of::<u32>() * BUFF_SIZE as usize];

        match s.read(&mut contents) {
            Ok(len) => {
                if !p(&mut contents) {
                    return false;
                }

                frame_size -= len as u32;
                windows_so_far += len as f32 / BUFF_SIZE as f32;
            }
            Err(_) => return false
        }
    }

    true
}
pub fn receive_network_file(path: &Path, s: &mut TcpStream, frame_count: u32) -> bool {
    let mut file = match File::create(path) {
        Ok(f) => f,
        Err(_) => return false
    };

    receive_network_data(s, frame_count, &mut |x| -> bool {
        file.write(x).is_ok()
    })
}
pub fn receive_network_binary(s: &mut TcpStream, frame_count: u32) -> Option<Vec<u8>> {
    let mut result = Vec::<u8>::new();

    let mut collect = |x: &mut Vec<u8>| -> bool {
        result.append(x);
        true
    };

    if !receive_network_data(s, frame_count, &mut collect) {
        None
    } else {
        Some(result)
    } 
}

pub struct JsonFile {
    path: Option<String>
}
impl Default for JsonFile {
    fn default() -> Self {
        Self::new()
    }
}
impl JsonFile {
    pub fn new() -> Self {
        Self {
            path: None
        }
    }

    pub fn is_open(&self) -> bool {
        self.path.is_some()
    }
    pub fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }

    pub fn open(&mut self, path: &str) -> Result<String, String> {
        if self.is_open() {
            return Err(format!("file already opened, at path '{}'", self.path().unwrap()));
        }

        let mut file = match File::open(path) {
            Err(e) => {
                //Try to open up as a new file
                match File::create(self.path.as_ref().unwrap()) {
                    Err(e2) => return Err(format!("failed to open because '{}' and failed to create because '{}'", e, e2)),
                    Ok(f) => f
                }
            },
            Ok(f) => f
        };

        let mut contents = String::new();
        match file.read_to_string(&mut contents)  {
            Err(e) => Err(e.to_string()),
            Ok(_) => {
                self.path = Some(path.to_string()); //Update path after all errors could occur
                Ok(contents)
            }
        }
    }
    pub fn save(&self, contents: &str) -> Result<(), String> {
        if !self.is_open() {
            return Ok(());
        }

        let mut file = match File::create(self.path.as_ref().unwrap()) {
            Ok(f) => f,
            Err(e) => return Err(format!("{}", e))
        };

        match file.write_all(contents.as_bytes()) {
            Err(e) => Err(e.to_string()),
            Ok(_) => Ok(())
        }
    }

    pub fn close(&mut self) {
        self.path = None;
    }
}