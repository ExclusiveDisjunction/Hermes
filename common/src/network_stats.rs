use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use std::sync::{Arc, Mutex};

use crate::file_io::JsonFile;

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct TransferStats {
    pub file_size: u32,
    pub transfer_time: f32,
    pub data_rate: f32,
    pub latency: f32,
    pub ip: String
}
impl Debug for TransferStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {} bytes, {} seconds, {} MB/s, {} s", &self.ip, self.file_size, self.transfer_time, self.data_rate, self.latency)
    }
}
impl Display for TransferStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IP: {}\nFile Size (bytes): {}\nTransfer Time (s):{}\nTransfer Rate (MB/s): {}\nLatency (s): {}", &self.ip, self.file_size, self.transfer_time, self.data_rate, self.latency)
    }
}

struct NetworkAnalyzerData {
    file: JsonFile,
    stats: Vec<TransferStats>
}
impl NetworkAnalyzerData {
    fn new() -> Self {
        Self { 
            file: JsonFile::new(),
            stats: Vec::new()
        }
    }

    fn open(&mut self, path: &str) -> Result<(), String> {
        let contents = self.file.open(path)?;

        let values: Result<Vec<TransferStats>, _> = serde_json::from_str(&contents);
        match values {
            Ok(l) => {
                self.stats = l;
                Ok(())
            },
            Err(e) => Err(e.to_string())
        }
    }
    fn save(&self) -> Result<(), String> {
        let contents = match serde_json::to_string(&self.stats) {
            Ok(s) => s,
            Err(e) => return Err(format!("{}", e))
        };

        self.file.save(&contents)
    }

    fn record_transfer(&mut self, file_size: u32, duration: f32, ip: &str) -> Result<(), String> {
        if !self.file.is_open() {
            return Err(String::from("no file is loaded"));
        }

        let rate = Self::calculate_data_rate(file_size, duration);
        if rate.is_none() {
            return Err(String::from("duration is less than or equal to zero"));
        }
        let latency = 1.0 / duration;

        let stat = TransferStats {
            file_size,
            transfer_time: duration,
            data_rate: rate.unwrap(),
            latency,
            ip: ip.to_string()
        };

        self.stats.push(stat);
        Ok(())
    }
    fn calculate_data_rate(file_size: u32, transfer_time: f32) -> Option<f32> {
        let conv: f32 = file_size as f32;

        if transfer_time > 0.0 {
            Some( (conv / transfer_time) / 1e6 )
        } else {
            None
        }

    }

    fn get_stats_by_ip(&self, ip: &str) -> Option<Vec<&TransferStats>> {
        if !self.file.is_open() {
            return None;
        }

        Some(
            self.stats.iter().filter(|x| x.ip == ip).collect()
        )
    }
    fn get_last_stat_by_ip(&self, ip: &str) -> Option<TransferStats> {
        if !self.file.is_open() {
            return None;
        }

        let list = self.get_stats_by_ip(ip)?;
        let item = list.last()?;
        Some((*item).clone())
    }
}

pub struct NetworkAnalyzer {
    data: Arc<Mutex<NetworkAnalyzerData>>
}
impl Default for NetworkAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
impl NetworkAnalyzer {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(NetworkAnalyzerData::new()))
        }
    }

    pub fn open(&self, path: &str) -> Result<(), String> {
        let mut data = self.data.lock().unwrap();
        data.open(path)
    }
    pub fn save(&self) -> Result<(), String> {
        let data = self.data.lock().unwrap();
        data.save()
    }

    pub fn record_transfer(&self, file_size: u32, duration: f32, ip: &str) -> Result<(), String> {
        let mut data = self.data.lock().unwrap();
        data.record_transfer(file_size, duration, ip)
    }

    pub fn get_last_stat_by_ip(&self, ip: &str) -> Option<TransferStats> {
        let data = self.data.lock().unwrap();
        data.get_last_stat_by_ip(ip)
    }
}