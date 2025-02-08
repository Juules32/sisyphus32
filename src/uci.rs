use std::{io::{self, BufRead}, process::exit};

use crate::{pl, position::Position};

pub fn print_info(text: &str) {
    pl!("info ".to_owned() + text);
}

pub struct UCIParseError(&'static str);

#[derive(Default)]
pub struct UCI {
    pub position: Position
}

impl UCI {
    pub fn init(&mut self) {
        let mut lines = io::stdin().lock().lines();
        while let Some(Ok(line)) = lines.next() {
            self.parse_line(line);
        }
    }
    
    fn parse_line(&mut self, line: String) {
        let mut words = line.split_whitespace();
        let result: Result<(), UCIParseError> = match words.next() {
            Some(keyword) => {
                match keyword {
                    "quit" | "exit" => exit(0),
                    "go" => self.parse_go(words),
                    "position" => self.parse_position(line),
                    _ => Ok(()),
                }
            }
            None => Ok(()),
        };

        match result {
            Err(UCIParseError(message)) => { pl!(message); },
            _ => (),
        }
    }
    
    fn parse_position(&mut self, line: String) -> Result<(), UCIParseError> {
        todo!()
    }
    
    fn parse_go<'a, I>(&self, words: I) -> Result<(), UCIParseError> where I: Iterator<Item = &'a str> {
        todo!()
    }
}
