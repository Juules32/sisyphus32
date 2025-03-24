use std::{io::{self, BufRead}, process::exit, sync::{atomic::Ordering, mpsc, Arc}, thread};

use crate::{color::Color, eval_position::EvalPosition, fen::{FenParseError, FenString}, perft::Perft, position::{MoveStringParseError, Position}, search::Search, syzygy::SyzygyTablebase, transposition_table::TranspositionTable};

pub struct UciParseError(pub &'static str);

pub struct Uci {
    pub position: Position,
    pub search: Search,
}

impl Default for Uci {
    fn default() -> Self {
        Self {
            position: Position::starting_position(),
            search: Search::default(),
        }
    }
}

impl Uci {
    pub fn init(&mut self) {
        Self::print_uci_info();

        let (uci_command_tx, uci_command_rx) = mpsc::channel();

        let stop_calculating = self.search.stop_calculating.clone();
        
        thread::spawn(move || {
            let mut lines = io::stdin().lock().lines();
            while let Some(Ok(line)) = lines.next() {
                match line.as_str() {
                    "quit" | "exit" | "q" | "e" => exit(0),
                    "stop" | "s" => stop_calculating.store(true, Ordering::Relaxed),
                    _ => if uci_command_tx.send(line).is_err() {
                        break;
                    },
                }
            }
        });

        for line in uci_command_rx {
            if let Err(UciParseError(msg)) = self.parse_line(line) {
                eprintln!("{msg}");
            };
        }
    }

    fn print_uci_info() {
        println!("id name Sisyphus32");
        println!("id author Juules32");
        println!();
        println!("option name Threads type spin default 1 min 1 max 1024");
        println!("option name Clear Hash type button");
        println!("option name SyzygyPath type string default tables/syzygy");
        println!("uciok");
    }
    
    fn parse_line(&mut self, line: String) -> Result<(), UciParseError> {
        let mut words = line.split_whitespace();
        match words.next() {
            Some(keyword) => {
                match keyword {
                    "uci" => {
                        Self::print_uci_info();
                        Ok(())
                    },
                    "ucinewgame" => self.parse_position("position startpos"),
                    "isready" => {
                        println!("readyok");
                        Ok(())
                    },
                    "position" => self.parse_position(&line),
                    "go" => self.parse_go(&line),
                    "eval" => {
                        println!("{}", EvalPosition::eval(&self.position));
                        Ok(())
                    },
                    "display" | "d" => {
                        println!("{}", self.position);
                        Ok(())
                    },
                    "bench" | "benchmedium" => {
                        Perft::medium_perft_tests();
                        Ok(())
                    }
                    "benchlong" => {
                        Perft::long_perft_tests();
                        Ok(())
                    },
                    "benchshort" => {
                        Perft::short_perft_tests();
                        Ok(())
                    },
                    "setoption" => {
                        if line == "setoption name Clear Hash" {
                            TranspositionTable::reset();
                            println!("Reset transposition table successfully!");
                            Ok(())
                        } else if line.starts_with("setoption name Threads value ") {
                            match words.last().unwrap().parse() {
                                Ok(num_threads) => {
                                    self.search.threadpool = Arc::new(
                                        rayon::ThreadPoolBuilder::new()
                                            .num_threads(num_threads)
                                            .build()
                                            .unwrap()
                                    );
                                    println!("Set number of threads to {num_threads} successfully!");
                                    Ok(())
                                },
                                Err(_) => Err(UciParseError("Couldn't parse threads value!")),
                            }
                        } else if line.starts_with("setoption name SyzygyPath value ") {
                            let path = words.last().unwrap();
                            self.search.tablebase = Arc::new(Some(SyzygyTablebase::from_directory(path).unwrap()));
                            println!("Set syzygy path to {path} successfully!");
                            Ok(())
                        } else {
                            Err(UciParseError("Couldn't find option name!"))
                        }
                    }
                    _ => Err(UciParseError("Couldn't parse keyword!")),
                }
            }
            None => Ok(()),
        }
    }
    
    fn parse_position(&mut self, line: &str) -> Result<(), UciParseError> {
        let fen_index_option = line.find("fen");
        let startpos_index_option = line.find("startpos");
        let kiwipete_index_option = line.find("kiwipete");
        let rook_index_option = line.find("rook");
        let tricky_index_option = line.find("tricky");
        let tricky2_index_option = line.find("tricky2");
        let moves_index_option = line.find("moves");

        if let Some(fen_index) = fen_index_option {
            let fen_string = {
                FenString::from(match moves_index_option {
                    Some(moves_index) => line[fen_index + 3..moves_index].trim(),
                    None => line[fen_index + 3..].trim(),
                })
            };
            self.position = fen_string.parse().map_err(|FenParseError(msg)| UciParseError(msg))?;
        } else if startpos_index_option.is_some() {
            self.position = Position::starting_position();
        } else if kiwipete_index_option.is_some() {
            self.position = FenString::kiwipete().parse().unwrap();
        } else if rook_index_option.is_some() {
            self.position = FenString::rook().parse().unwrap();
        } else if tricky2_index_option.is_some() {
            self.position = FenString::tricky2().parse().unwrap();
        } else if tricky_index_option.is_some() {
            self.position = FenString::tricky().parse().unwrap();
        } else {
            return Err(UciParseError("Neither fen nor startpos found!"));
        }

        if let Some(moves_index) = moves_index_option {
            for move_string in line[moves_index + 5..].split_whitespace() {
                let bit_move = self.position.parse_move_string(move_string).map_err(|MoveStringParseError(msg)| UciParseError(msg))?;
                self.position.make_move(bit_move);
            }
        }
        Ok(())
    }
    
    fn parse_go(&mut self, line: &str) -> Result<(), UciParseError> {
        let words: Vec<_> = line.split_whitespace().collect();
        if let Some(perft_index) = words.iter().position(|&word| word == "perft") {
            match words.get(perft_index + 1) {
                Some(depth_string) => {
                    match depth_string.parse::<u8>() {
                        Ok(depth) => {
                            Perft::perft_test(&self.position, depth, true);
                            Ok(())
                        },
                        Err(_) => Err(UciParseError("Couldn't parse depth string!")),
                    }
                },
                None => Err(UciParseError("Didn't find perft depth!")),
            }
        } else if let Some(depth_index) = words.iter().position(|&word| word == "depth") {
            match words.get(depth_index + 1) {
                Some(depth_string) => {
                    match depth_string.parse::<u16>() {
                        Ok(depth) => {
                            self.search.go(&self.position, depth, None);
                            Ok(())
                        },
                        Err(_) => Err(UciParseError("Couldn't parse depth string!"))
                    }
                },
                None => Err(UciParseError("Didn't find depth string!")),
            }
        } else {
            if let Some(move_time_index) = words.iter().position(|&word| {
                word == "movetime"
            }) {
                match words.get(move_time_index + 1) {
                    Some(time_string) => {
                        match time_string.parse::<u128>() {
                            Ok(move_time) => {
                                self.search.go(&self.position, 255, Some(move_time));
                                return Ok(());
                            },
                            Err(_) => return Err(UciParseError("Couldn't parse time string!")),
                        }
                    },
                    None => return Err(UciParseError("Didn't find time string!")),
                }
            }

            let mut total_time = None;
            if let Some(time_index) = words.iter().position(|&word| {
                word == match self.position.side {
                    Color::White => "wtime",
                    Color::Black => "btime",
                }
            }) {
                match words.get(time_index + 1) {
                    Some(time_string) => {
                        match time_string.parse::<u128>() {
                            Ok(time) => {
                                total_time = Some(time)
                            },
                            Err(_) => return Err(UciParseError("Couldn't parse time string!")),
                        }
                    },
                    None => return Err(UciParseError("Didn't find time string!")),
                }
            }

            let mut increment = 0;
            if let Some(inc_index) = words.iter().position(|&word| {
                word == match self.position.side {
                    Color::White => "winc",
                    Color::Black => "binc",
                }
            }) {
                match words.get(inc_index + 1) {
                    Some(inc_string) => {
                        match inc_string.parse::<u128>() {
                            Ok(inc) => {
                                increment = inc
                            },
                            Err(_) => return Err(UciParseError("Couldn't parse increment string!")),
                        }
                    },
                    None => return Err(UciParseError("Didn't find increment string!")),
                }
            }

            self.search.go(&self.position, 255, Search::calculate_stop_time(total_time, increment));
            Ok(())
        }
    }
}
