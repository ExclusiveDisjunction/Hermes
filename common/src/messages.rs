use serde::{Deserialize, de::DeserializeOwned, Serialize};
use serde_json::json;
use std::{collections::HashMap, fmt::Display, str::FromStr};
use std::iter::zip;

use crate::http_codes::HttpCodes;
use crate::file_io::FileType;
use crate::network_stats::TransferStats;

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub enum MessageType {
    Connect,
    Close,
    Ack,
    Upload,
    Download,
    Delete,
    Dir,
    Move,
    Subfolder,
    Stats
}
impl Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Connect => "connect",
            Self::Close => "close",
            Self::Ack => "ack",
            Self::Upload => "upload",
            Self::Download => "download",
            Self::Delete => "delete",
            Self::Dir => "dir",
            Self::Move => "move",
            Self::Subfolder => "subfolder",
            Self::Stats => "stats"
        };

        write!(f, "{}", str)
    }
}
impl FromStr for MessageType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "connect" => Ok(Self::Connect),
            "close" => Ok(Self::Close),
            "ack" => Ok(Self::Ack),
            "upload" => Ok(Self::Upload),
            "download" => Ok(Self::Download),
            "delete" => Ok(Self::Delete),
            "dir" => Ok(Self::Dir),
            "move" => Ok(Self::Move),
            "subfolder" => Ok(Self::Subfolder),
            "stats" => Ok(Self::Stats),
            _ => Err(format!("unable to parse literal '{}'", s))
        }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub enum MessageDirection {
    Request,
    Response
}
impl Display for MessageDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::Request => "request",
            Self::Response => "response"
        };

        write!(f, "{}", text)
    }
}
impl FromStr for MessageDirection {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "request" => Ok(Self::Request),
            "response" => Ok(Self::Response),
            _ => Err(Self::Err::from("Invalid direction"))
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum SubfolderAction {
    Add,
    Delete
}
impl Display for SubfolderAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Add => "add",
            Self::Delete => "delete"
        };

        write!(f, "{}", str)
    }
}
impl FromStr for SubfolderAction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "add" => Ok(Self::Add),
            "delete" => Ok(Self::Delete),
            _ => Err(format!("could not deduce SubfolderAction from '{}'", s))
        }
    }  
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    message_type: MessageType,
    direction: MessageDirection,
    data: HashMap<String, serde_json::Value>
}
impl Message {
    fn new(message_type: MessageType, direction: MessageDirection, data: HashMap<String, serde_json::Value>) -> Self {
        Self {
            message_type,
            direction,
            data
        }
    }

    pub fn message_type(&self) -> &MessageType {
        &self.message_type
    }
    pub fn direction(&self) -> &MessageDirection {
        &self.direction
    }

    pub fn extract(&self, property: &str) -> Option<&serde_json::Value> {
        self.data.get(property)
    }
    pub fn extract_mut(&mut self, property: &str) -> Option<&mut serde_json::Value> {
        self.data.get_mut(property)
    }
    pub fn extract_clone(&self, property: &str) -> Option<serde_json::Value> {
        Some( self.extract(property)?.clone() )
    }
    pub fn extract_as<T: DeserializeOwned>(&self, property: &str) -> Option<T> {
        let val = self.extract_clone(property)?;

        let result: Result<T, _> = serde_json::from_value(val);
        if let Ok(r) = result {
            Some(r)
        }
        else {
            None
        }
    }
}

fn make_message_data(properties: Vec<&str>, values: Vec<serde_json::Value>) -> HashMap<String, serde_json::Value> {
    assert_eq!(properties.len(), values.len());
    let property_strs = properties.iter().map(|x| x.to_string());

    let total_list = zip(property_strs, values);

    HashMap::<String, serde_json::Value>::from_iter(total_list)
}

pub fn connect_message(username: String, password: String) -> Message {
    Message::new(
        MessageType::Connect, 
        MessageDirection::Request, 
        make_message_data(
            vec!["username", "password"],
            vec![json!(username), json!(password)]
        )
    )
}
pub fn extract_connect_message(message: Message) -> Option<(String, String)> {
    if *message.message_type() != MessageType::Connect {
        return None
    }   

    let username: Option<String> = message.extract_as("username");
    let password: Option<String> = message.extract_as("password");
    
    match (username, password) {
        (Some(u), Some(p)) => Some( (u, p) ),
        (_, _) => None
    }
}

pub fn ack_messsage(direction: MessageDirection, code: HttpCodes, message: Option<String>) -> Message {
    let code_str = code.to_string();
    let data = make_message_data(
        vec!["code", "message"], 
        vec![
            json!(code), 
            json!(
                if let Some(msg) = message {
                    msg
                }
                else {
                    code_str
                }
            )
        ]
    );

    Message::new(MessageType::Ack, direction, data)
}
pub fn extract_ack_message(message: Message) -> Option<(HttpCodes, String)> {
    if *message.message_type() != MessageType::Ack {
        return None
    } 

    let code: Option<HttpCodes> = message.extract_as("code");
    let message: Option<String> = message.extract_as("message");

    match (code, message) {
        (Some(c), Some(m)) => Some((c, m)),
        _ => None
    }
}

pub fn close_message() -> Message {
    Message::new(MessageType::Close, MessageDirection::Request, HashMap::new())
}

pub fn upload_message(name: &str, f_type: FileType, frame_count: u32) -> Message {
    Message::new(
        MessageType::Upload,
        MessageDirection::Request,
        make_message_data(
            vec!["name", "type", "size"],
            vec![json!(name.to_string()), json!(f_type), json!(frame_count)]
        )
    )
}
pub fn extract_upload_message(message: Message) -> Option<(String, FileType, u32)> {
    if *message.message_type() != MessageType::Upload {
        return None
    } 

    let name: Option<String> = message.extract_as("name");
    let f_type: Option<FileType> = message.extract_as("type");
    let frame_count: Option<u32> = message.extract_as("size");

    match (name, f_type, frame_count) {
        (Some(n), Some(t), Some(f)) => Some((n, t, f)),
        _ => None
    }
}

pub fn download_message_request(path: &str) -> Message {
    Message::new(
        MessageType::Download,
        MessageDirection::Request,
        make_message_data(
            vec!["path"],
            vec![json!(path)]
        )
    )
}
pub fn download_message_response(status: HttpCodes, message: &str, kind: FileType, frame_count: u32) -> Message {
    Message::new(
        MessageType::Download, 
        MessageDirection::Response,
        make_message_data(
            vec!["status", "message", "kind", "size"],
            vec![json!(status), json!(message), json!(kind), json!(frame_count)]
        )
    )
}
pub fn extract_download_request_message(message: Message) -> Option<String> {
    if *message.message_type() != MessageType::Download {
        return None;
    }

    let path: Option<String> = message.extract_as("path");
    path
}
pub fn extract_download_response_message(message: Message) -> Option<(HttpCodes, String, FileType, u32)> {
    if *message.message_type() != MessageType::Download {
        return None;
    }

    let status: Option<HttpCodes> = message.extract_as("status");
    let msg: Option<String> = message.extract_as("message");
    let kind: Option<FileType> = message.extract_as("kind");
    let size: Option<u32> = message.extract_as("size");

    match (status, msg, kind, size) {
        (Some(c), Some(m), Some(t), Some(s)) => Some((c, m, t, s)),
        _ => None
    }
}

pub fn delete_message(path: &str) -> Message {
    Message::new(
        MessageType::Delete,
        MessageDirection::Request,
        make_message_data(
            vec!["path"],
            vec![json!(path)]
        )
    )
}
pub fn extract_delete_message(message: Message) -> Option<String> {
    if *message.message_type() != MessageType::Delete {
        return None;
    }

    let path: Option<String> = message.extract_as("path");
    path
}

pub fn dir_message_request() -> Message {
    Message::new(
        MessageType::Dir,
        MessageDirection::Request,
        HashMap::<String, serde_json::Value>::new()
    )
}
pub fn dir_message_response(status: HttpCodes, message: &str, curr_dir: &str, frame_count: u32) -> Message {
    Message::new(
        MessageType::Dir,
        MessageDirection::Response,
        make_message_data(
            vec!["status", "message", "curr_dir", "size"],
            vec![json!(status), json!(message), json!(curr_dir), json!(frame_count)]
        )
    )
}
pub fn extract_dir_response_message(message: Message) -> Option<(HttpCodes, String, String, u32)> {
    if *message.message_type() != MessageType::Dir {
        return None;
    }

    let status: Option<HttpCodes> = message.extract_as("status");
    let msg: Option<String> = message.extract_as("message");
    let curr_dir: Option<String> = message.extract_as("curr_dir");
    let size: Option<u32> = message.extract_as("size");

    match (status, msg, curr_dir, size) {
        (Some(s), Some(m), Some(c), Some(sz)) => Some((s, m, c, sz)),
        _ => None
    }
}

pub fn move_message(path: &str) -> Message {
    Message::new(
        MessageType::Move,
        MessageDirection::Request,
        make_message_data(
            vec!["path"],
            vec![json!(path)]
        )
    )
}
pub fn extract_move_message(message: Message) -> Option<String> {
    if *message.message_type() != MessageType::Move {
        return None;
    }

    let path: Option<String> = message.extract_as("path");
    path
}

pub fn subfolder_message(path: &str, action: SubfolderAction) -> Message {
    Message::new(
        MessageType::Subfolder,
        MessageDirection::Request,
        make_message_data(
            vec!["path", "action"],
            vec![json!(path), json!(action)]
        )
    )
}
pub fn extract_subfolder_message(message: Message) -> Option<(String, SubfolderAction)> {
    if *message.message_type() != MessageType::Subfolder {
        return None;
    }

    let path: Option<String> = message.extract_as("path");
    let action: Option<SubfolderAction> = message.extract_as("action");

    match (path, action) {
        (Some(p), Some(a)) => Some((p, a)),
        _ => None
    }
}

pub fn stats_request_message() -> Message {
    Message::new(
        MessageType::Stats,
        MessageDirection::Request,
        HashMap::<String, serde_json::Value>::new()
    )
}
pub fn stats_response_message(stats: TransferStats) -> Message {
    Message::new(
        MessageType::Stats,
        MessageDirection::Response,
        make_message_data(
            vec!["stats"], 
            vec![json!(stats)]
        )
    )
}
pub fn extract_stats_response_message(message: Message) -> Option<TransferStats> {
    if *message.message_type() != MessageType::Stats {
        return None;
    }

    let stats: Option<TransferStats> = message.extract_as("stats");
    stats
}