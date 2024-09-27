use std::env::current_dir;
use std::path::Path;
use std::fs::{read_to_string, read_dir};
use std::io::{Result, ErrorKind, Error};

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
    
    let front = if let Some(v) = path.file_name().unwrap().to_str() {
        v
    } else {
        return Err(Error::new(ErrorKind::Other, "Shit"))
    };
    let back = read_to_string(path)?;

    Ok(Card {
        front: front.to_owned(),
        back,
    })
}
