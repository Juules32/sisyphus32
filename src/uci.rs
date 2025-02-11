use std::{io::{self, BufRead}, process::exit};

use crate::{bit_move::BitMove, eval, fen::{self, FenParseError}, move_flag::MoveFlag, perft, pl, position::Position, search, square::{Square, SquareParseError}};

pub struct UciParseError(pub &'static str);

pub struct Uci {
    pub position: Position
}

impl Default for Uci {
    fn default() -> Self {
        Self { position: Position::starting_position() }
    }
}

impl Uci {
    pub fn init(&mut self) {
        Self::print_uci_info();

        let mut lines = io::stdin().lock().lines();
        while let Some(Ok(line)) = lines.next() {
            if let Err(UciParseError(msg)) = self.parse_line(line) {
                eprintln!("{msg}");
            };
        }
    }

    fn print_uci_info() {
        pl!("id name Sisyphus32");
        pl!("id author Juules32");
        pl!("uciok");
    }
    
    fn parse_line(&mut self, line: String) -> Result<(), UciParseError> {
        let mut words = line.split_whitespace();
        match words.next() {
            Some(keyword) => {
                match keyword {
                    "quit" | "exit" => exit(0),
                    "go" => self.parse_go(words),
                    "position" => self.parse_position(&line),
                    "ucinewgame" => self.parse_position("position startpos"),
                    "uci" => {
                        Self::print_uci_info();
                        Ok(())
                    },
                    "eval" => {
                        pl!(eval::basic(&self.position).score);
                        Ok(())
                    },
                    "isready" => {
                        pl!("readyok");
                        Ok(())
                    },
                    "d" => {
                        pl!(self.position);
                        Ok(())
                    },
                    "bench" | "benchlong" => {
                        perft::long_perft_tests();
                        Ok(())
                    },
                    "benchmedium" => {
                        perft::medium_perft_tests();
                        Ok(())
                    }
                    "benchshort" => {
                        perft::short_perft_tests();
                        Ok(())
                    },
                    _ => Err(UciParseError("Couldn't parse keyword!")),
                }
            }
            None => Ok(()),
        }
    }

    fn parse_move_string(&mut self, move_string: &str) -> Result<BitMove, UciParseError> {
        if move_string.len() == 4 || move_string.len() == 5 {
            let source = Square::try_from(&move_string[0..2]).map_err(|SquareParseError(msg)| UciParseError(msg))?;
            let target = Square::try_from(&move_string[2..4]).map_err(|SquareParseError(msg)| UciParseError(msg))?;
            let promotion_piece_option = if move_string.len() == 5 {
                Some(&move_string[4..5])
            } else {
                None
            };

            let ms = self.position.generate_pseudo_legal_moves();
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
                                _ => return Err(UciParseError("Found illegal promotion piece string!"))
                            }
                        },
                        None => return Ok(*m),
                    }
                }
            }

            Err(UciParseError("Couldn't find a pseudo-legal move!"))
        } else {
            Err(UciParseError("Couldn't parse move with illegal amount of characters!"))
        }
    }
    
    fn parse_position(&mut self, line: &str) -> Result<(), UciParseError> {
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
            self.position = fen::parse(fen_string).map_err(|FenParseError(msg)| UciParseError(msg))?;
        } else if startpos_index_option.is_some() {
            self.position = fen::parse(fen::STARTING_POSITION).map_err(|FenParseError(msg)| UciParseError(msg))?;
        } else {
            return Err(UciParseError("Neither fen nor startpos found!"));
        }

        if let Some(moves_index) = moves_index_option {
            for move_string in line[moves_index + 5..].split_whitespace() {
                let pseudo_legal_move = self.parse_move_string(move_string)?;
                if !self.position.make_move(pseudo_legal_move) {
                    return Err(UciParseError("Found illegal move while parsing moves!"))
                }
            }
        }

        Ok(())
    }
    
    fn parse_go<'a, I>(&self, mut words: I) -> Result<(), UciParseError> where I: Iterator<Item = &'a str> {
        match words.next() {
            Some(word) => {
                match word {
                    "perft" => {
                        match words.next() {
                            Some(depth_string) => {
                                match depth_string.parse::<u8>() {
                                    Ok(depth) => {
                                        perft::perft_test(&self.position, depth, true);
                                        Ok(())
                                    },
                                    Err(_) => Err(UciParseError("Couldn't parse depth string!")),
                                }
                            },
                            None => Err(UciParseError("Didn't find perft depth!")),
                        }
                    },
                    "depth" => {
                        match words.next() {
                            Some(depth_string) => {
                                match depth_string.parse::<u8>() {
                                    Ok(depth) => {
                                        search::go(&mut self.position.clone(), depth);
                                        Ok(())
                                    },
                                    Err(_) => Err(UciParseError("Couldn't parse depth string!"))
                                }
                            },
                            None => Err(UciParseError("Didn't find depth string!")),
                        }
                    },
                    _ => {
                        search::go(&mut self.position.clone(), 5);
                        Err(UciParseError("Couldn't parse go command! Calculated go depth 5 instead."))
                    },
                }
            },
            None => {
                search::go(&mut self.position.clone(), 255);
                Ok(())
            },
        }
    }
}
