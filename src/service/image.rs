use log::{error};
use std::io::{Error, ErrorKind};
use std::process::Command;
use std::fs::{File, OpenOptions};
use std::io::{Write};
use uuid::Uuid;

const ORIGIN_FILE: &'static str = "origin";
const EXT: &'static str = "jpg";
const AVATAR_SIDE_LENGTH: u32 = 384;

pub struct Image {
    f: Option<File>,
    temp: String
}

impl Image {
    pub fn new_filename(user_id: i32) -> String {
        format!("{}-{}.{}", user_id, Uuid::new_v4(), EXT)
    }
    pub fn delete(file: &str) -> Result<(), Error> {
        match Command::new("rm").arg(file).output() {
            Ok(output) => if output.status.success() {
                Ok(())
            }else{ 
                return Err(Error::new(ErrorKind::Other, String::from_utf8(output.stderr).unwrap_or("move file failed".to_string())))
            },
            Err(e) => Err(e)
        }
    }
    pub fn new() -> Result<Self, Error> {
        let temp = match Command::new("mktemp").arg("-d").output() {
            Ok(output) => if output.status.success() {
                String::from_utf8(output.stdout).unwrap().trim().to_string()
            }else{
                return Err(Error::new(ErrorKind::Other, String::from_utf8(output.stderr).unwrap_or("mktemp failed".to_string())))
            },
            Err(e) => return Err(e)
        };
        let f = match OpenOptions::new().create(true).write(true).open(format!("{}/{}", temp, ORIGIN_FILE)) {
            Ok(f) => f,
            Err(e) => return Err(e)
        };
        Ok(Self {temp: temp, f: Some(f)})
    }
    pub fn write(&mut self, bytes: &[u8]) -> Result<usize, Error> {
        self.f.as_ref().unwrap().write(bytes)
    }
    pub fn close(&mut self) -> &mut Self {
        self.f = None;
        self
    }
    pub fn convert(&mut self) -> Result<(), Error> {
        let mut c = Command::new("convert");
        let mut command = c.arg(&format!("{}/{}", self.temp, ORIGIN_FILE));
        
        let (w, h) = match self.get_size() { Ok(result) => result, Err(e) => return Err(e) };
        let mut size = w;
        if w != h {
            let (crop_w, crop_h) = if w > h { (h, h) }else{ (w, w) };
            command = command.arg("-gravity").arg("center").arg("-crop").arg(format!("{}x{}+0+0", crop_w, crop_h));
            size = crop_w;
        }
        if size > AVATAR_SIDE_LENGTH {
            command = command.arg("-resize").arg(format!("{}x{}", AVATAR_SIDE_LENGTH, AVATAR_SIDE_LENGTH));
        }
        
        match command.arg(&format!("{}/{}.{}", self.temp, ORIGIN_FILE, EXT)).output() {
            Ok(output) => if output.status.success() {
                Ok(())
            }else{
                return Err(Error::new(ErrorKind::Other, String::from_utf8(output.stderr).unwrap_or("mktemp failed".to_string())))
            },
            Err(e) => return Err(e)
        }
    }
    pub fn move_to(&mut self, target: &str) -> Result<(), Error> {
        match Command::new("cp").arg(format!("{}/{}.{}", self.temp, ORIGIN_FILE, EXT)).arg(target).output() {
            Ok(output) => if output.status.success() {
                Ok(())
            }else{
                return Err(Error::new(ErrorKind::Other, String::from_utf8(output.stderr).unwrap_or("move file failed".to_string())))
            },
            Err(e) => return Err(e)
        }
    }
    pub fn clear(&self) {
        match Command::new("rm").arg("-rf").arg(&self.temp).output() {
            Ok(output) => if !output.status.success() { error!("image clear failed at\"{}\".", &self.temp) },
            Err(_) => error!("image clear failed as \"{}\". ", &self.temp)
        }
    }
    fn get_size(&self) -> Result<(u32, u32), Error> {
        match Command::new("convert")
                .arg(&format!("{}/{}", self.temp, ORIGIN_FILE))
                .arg("-print").arg("%w*%h")
                .arg("/dev/null")
                .output() {
            Ok(output) => if output.status.success() {
                let s = String::from_utf8(output.stdout).unwrap();
                let v: Vec<&str> = s.split_terminator('*').collect();
                let w: u32 = v[0].parse().unwrap();
                let h: u32 = v[1].parse().unwrap();
                return Ok((w, h))
            }else{
                return Err(Error::new(ErrorKind::Other, String::from_utf8(output.stderr).unwrap_or("get size failed".to_string())))
            },
            Err(e) => return Err(e)
        }
    }
}