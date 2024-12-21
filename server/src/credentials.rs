use serde::{Serialize, Deserialize};
use serde_json::json;
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::{Read, Write};

#[derive(PartialEq, Serialize, Deserialize)]
pub struct Credentials {
    username: String,
    password: String
}
impl Debug for Credentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.username)
    }
}
impl Display for Credentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Username: {}", &self.username)
    }
}
impl Credentials {
    pub fn new(username: String, password: String) -> Self{
        Self {
            username,
            password
        }
    }
    pub fn from(username: &str, password: &str) -> Self{
        Self {
            username: username.to_string(),
            password: password.to_string()
        }
    }
    // Returns the user that could be anyone
    pub fn any_user() -> Self {
        Self::from("any", "any")
    }

    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn password(&self) -> &str {
        &self.password
    }
}

pub struct UserDatabase {
    path: Option<String>,
    users: Vec<Credentials>
}
impl Debug for UserDatabase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, 
            "(Path: '{}', Users: {})", 
            match self.path.as_ref() {
                Some(s) => s,
                None => "Unopened"
            }, 
            self.users.len()
        )
    }
}
impl Display for UserDatabase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, 
            "Users: {}",
            match self.path.as_ref() {
                Some(s) => s,
                None => "Unopened"
            }
        )
    }
}
impl UserDatabase {
    pub const fn new() -> Self {
        Self {
            path: None,
            users: Vec::new()
        }
    }

    pub fn open(&mut self, path: String) -> Result<(), String> {
        if self.path.is_some() {
            return Err(format!("already open at path '{}'", self.path.as_ref().unwrap()));
        }

        let mut file = match File::open(&path) {
            Ok(f) => f,
            Err(e) => {
                match File::create(&path) {
                    Ok(f) => f,
                    Err(e2) => return Err(format!("unable to open because '{}' and unable to create because '{}'", e, e2))
                }
            }
        };

        let mut contents = String::new();
        if let Err(e) = file.read_to_string(&mut contents) {
            return Err(format!("could not read because '{}'", e))
        }

        if contents.is_empty() {
            contents = String::from("[ ]");
        }

        let json_contents: Vec<Credentials> = match serde_json::from_str(&contents) {
            Ok(j) => j,
            Err(e) => return Err(format!("parsing error '{e}'"))
        };

        self.users = json_contents;
        
        if self.validate() {
            Ok(())
        } else {
            Err(String::from("Duplicate or empty records found"))
        }
    }
    pub fn save(&self) -> Result<(), String> {
        if self.path.is_none() {
            return Err(String::from("no file opened"));
        }

        let mut file = match File::create(&self.path.as_ref().unwrap()) {
            Ok(f) => f,
            Err(e) => return Err(e.to_string())
        };

        let contents = json!(self.users).to_string();

        match file.write(contents.as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string())
        }
    }

    // Determines that every user has a password & that there are no duplicates
    fn validate(&self) -> bool {
        if self.path.is_none() {
            return false;
        }

        for (i, cred) in self.users.iter().enumerate() {
            for (j, cred2) in self.users.iter().enumerate() {
                if cred.username.is_empty() || cred.password.is_empty() || (i != j && cred == cred2) {
                    return false; //Something is empty or we have a duplicate
                }
            }
        }

        true
    }

    pub fn get_user(&self, username: &str) -> Option<&Credentials> {
        self.path.as_ref()?; //If we dont have a path then we return none
        self.users.iter().find(|x| x.username == username)
    }
    pub fn get_user_mut(&mut self, username: &str) -> Option<&mut Credentials> {
        self.path.as_ref(); //If we dont have a path then we return none
        self.users.iter_mut().find(|x| x.username == username)
    }
    // Determine if that user is in the database & if the passwords match. If the user is not in the database, it returns None. If it is, and the passwords match, it returns Some(true). Otherwise it returns Some(false)
    pub fn validate_user(&self, username: &str, password: &str) -> Option<bool> {
        let target = self.get_user(username)?;
        Some(target.password == password)
    }
}