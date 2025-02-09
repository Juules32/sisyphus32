use std::{io::{self, BufRead}, process::exit};

use crate::{bit_move::BitMove, fen::{self, FenParseError}, move_flag::MoveFlag, pl, position::Position, square::{Square, SquareParseError}};

pub fn print_info(text: &str) {
    pl!("info ".to_owned() + text);
}

pub struct UCIParseError(pub &'static str);

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

    fn parse_move_string(&mut self, move_string: &str) -> Result<BitMove, UCIParseError> {
        if move_string.len() == 4 || move_string.len() == 5 {
            let source = Square::try_from(&move_string[0..2]).map_err(|SquareParseError(msg)| UCIParseError(msg))?;
            let target = Square::try_from(&move_string[2..4]).map_err(|SquareParseError(msg)| UCIParseError(msg))?;
            let promotion_piece_option = if move_string.len() == 5 {
                Some(&move_string[4..5])
            } else {
                None
            };

            let ms = self.position.generate_moves();
            for m in ms.iter() {
                let s = m.source();
                let t = m.target();
                let f = m.flag();
                
                if source == s && target == t {
                    match promotion_piece_option {
                        Some(promotion_piece_string) => {
                            match promotion_piece_string {
                                "q" => if f == MoveFlag::PromoQ { return Ok(*m) },
                                "r" => if f == MoveFlag::PromoR { return Ok(*m) },
                                "b" => if f == MoveFlag::PromoB { return Ok(*m) },
                                "n" => if f == MoveFlag::PromoN { return Ok(*m) },
                                _ => return Err(UCIParseError("Found illegal promotion piece string!"))
                            }
                        },
                        None => return Ok(*m),
                    }
                }
            }

            Err(UCIParseError("Couldn't find a pseudo-legal move!"))
        } else {
            Err(UCIParseError("Couldn't parse move with illegal amount of characters!"))
        }
    }
    
    fn parse_position(&mut self, line: String) -> Result<(), UCIParseError> {
        let fen_index_option = line.find("fen");
        let startpos_index_option = line.find("startpos");
        let moves_index_option = line.find("moves");

        if let Some(fen_index) = fen_index_option {
            let fen_string = {
                match moves_index_option {
                    Some(moves_index) => &line[fen_index + 3..moves_index].trim(),
                    None => &line[fen_index + 3..].trim(),
                }
            };
            self.position = fen::parse(fen_string).map_err(|FenParseError(msg)| UCIParseError(msg))?;
        } else if let Some(_) = startpos_index_option {
            self.position = fen::parse(fen::STARTING_POSITION).map_err(|FenParseError(msg)| UCIParseError(msg))?;
        } else {
            return Err(UCIParseError("Neither fen nor startpos found!"));
        }

        if let Some(moves_index) = moves_index_option {
            for move_string in line[moves_index + 5..].trim().split_whitespace() {
                let pseudo_legal_move = self.parse_move_string(move_string)?;
                if !self.position.make_move(pseudo_legal_move) {
                    return Err(UCIParseError("Found illegal move while parsing moves!"))
                }
            }
        }

        pl!(self.position);
        Ok(())
    }
    
    fn parse_go<'a, I>(&self, words: I) -> Result<(), UCIParseError> where I: Iterator<Item = &'a str> {
        todo!()
    }
}
