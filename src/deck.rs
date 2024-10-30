use std::env::current_dir;
use std::path::Path;
use std::fs::{read_to_string, read_dir};
use std::io::{Result, ErrorKind, Error};
use rand::{seq::SliceRandom, thread_rng};

pub struct Card {
    pub front: String,
    pub back: String,
}

pub struct Deck {
    pub cards: Vec<Card>
}

impl Deck {

    pub fn new() -> Self{
        Deck {
            cards: Vec::new(),
        }
    }

    pub fn initialize(&mut self) -> Result<()> {
        self.visit_dirs(&current_dir().unwrap());
        if self.cards.len() == 0 {
            Err(Error::other("no md files found"))
        } else {
            Ok(())
        }
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    pub fn test_print(&self) {
        self.cards.iter().enumerate().for_each(|(i, c)| println!("card: {}, front: {}, back: {}", i, c.front, c.back));
    }

    fn visit_dirs(&mut self, path: &Path) {

        for entry in read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                self.visit_dirs(&path);
            } else if extension_check(&path, "md") {
                if let Ok(val) = file_to_card(&path) {
                    self.cards.push(val);
                };
            }
        }


    }
}


fn extension_check(path: &Path, pattern: &'static str) -> bool {
    let os_str = if let Some(s) = path.extension() {
        s
    } else {
        return false
    };
    
    match os_str.to_str() {
        Some(str) => {
            if str == pattern {
                true
            } else {
                false
            }
        }
        None => false,

    }
}

pub fn file_to_card(path: &Path) -> Result<Card> {
    
    let front = if let Some(v) = path.file_stem().unwrap().to_str() {
        if let Some(parent) = get_parent_dir(path) {
            parent.to_owned() + "/" + v
        } else {
            "".to_owned()
        }
    } else {
        return Err(Error::new(ErrorKind::Other, "Shit"))
    };
    let back = read_to_string(path)?;

    Ok(Card {
        front,
        back,
    })
}

fn get_parent_dir(path: &Path) -> Option<&str>  {
    if let Some(dir) = path.parent().unwrap().to_str() {
        // get the '/' at the end of the path and extract the parent directory
        for (i, c) in dir.chars().rev().enumerate() {
            println!("{}", c);
            if c == '/' {
                let (_, end) = dir.split_at(dir.len() - i);
                return Some(end)
            }
        }
        return None
    }
    None
}
