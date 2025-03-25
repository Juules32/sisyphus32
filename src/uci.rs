use std::{io::{self, BufRead}, process::exit, sync::{atomic::Ordering, mpsc, Arc}, thread};

use thiserror::Error;

use crate::{bit_move::BitMove, color::Color, eval_position::EvalPosition, fen::{FenParseError, FenString}, move_flag::MoveFlag, move_generation::{Legal, MoveGeneration}, perft::Perft, position::Position, search::Search, square::{Square, SquareParseError}, syzygy::SyzygyTablebase, transposition_table::TranspositionTable};

#[derive(Error, Debug)]
pub enum UciParseError {
    #[error("Couldn't parse uci keyword")]
    KeywordParseError,

    #[error("Couldn't parse parameter: {0}")]
    ParamParseError(&'static str),

    #[error("Couldn't parse parameter value: {0}")]
    ParamValueParseError(&'static str),

    #[error("Couldn't parse uci option")]
    OptionParseError,

    #[error("{0}")]
    MoveStringParseError(#[from] MoveStringParseError),

    #[error("{0}")]
    FenParseError(#[from] FenParseError),
}

#[derive(Error, Debug)]
pub enum MoveStringParseError {
    #[error("Illegal move string length")]
    LengthParseError,

    #[error("Illegal promotion piece")]
    PromotionPieceParseError(String),
    
    #[error("Couldn't find pseudo-legal move: {0}")]
    IllegalMove(String),

    #[error("{0}")]
    SquareParseError(#[from] SquareParseError),
}

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
            if let Err(error) = self.parse_line(line) {
                eprintln!("{}!", error.to_string());
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
        let words: Vec<_> = line.split_whitespace().collect();
        match words.first().map(|s| s.to_owned()) {
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
                    "go" => self.parse_go(&words),
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
                        self.parse_setoption(&line, &words)
                    }
                    _ => Err(UciParseError::KeywordParseError),
                }
            }
            None => Ok(()),
        }
    }

    fn parse_setoption(&mut self, line: &str, words: &[&str]) -> Result<(), UciParseError> {
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
                Err(_) => Err(UciParseError::ParamValueParseError("Threads")),
            }
        } else if line.starts_with("setoption name SyzygyPath value ") {
            let path = words.last().unwrap();
            self.search.tablebase = Arc::new(Some(SyzygyTablebase::from_directory(path).unwrap()));
            println!("Set syzygy path to {path} successfully!");
            Ok(())
        } else {
            Err(UciParseError::OptionParseError)
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
            self.position = fen_string.parse()?;
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
            return Err(UciParseError::ParamParseError("Neither fen nor startpos found"));
        }

        if let Some(moves_index) = moves_index_option {
            for move_string in line[moves_index + 5..].split_whitespace() {
                let bit_move = Self::parse_move_string(&self.position, move_string)?;
                self.position.make_move(bit_move);
            }
        }
        Ok(())
    }

    fn parse_parameter_value<T: std::str::FromStr>(words: &[&str], key: &str, error: UciParseError) -> Result<Option<T>, UciParseError> {
        match words.iter().position(|&word| word == key) {
            Some(word_index) => match words.get(word_index + 1) {
                Some(&value) => value.parse::<T>().map(Some).map_err(|_| error),
                None => Err(error),
            },
            None => Ok(None),
        }
    }
    
    fn parse_go(&mut self, words: &[&str]) -> Result<(), UciParseError> {
        let depth: Option<u16> = Self::parse_parameter_value(words, "depth", UciParseError::ParamValueParseError("depth"))?;
        let perft_depth: Option<u16> = Self::parse_parameter_value(words, "perft", UciParseError::ParamValueParseError("perft depth"))?;
        let move_time: Option<u128> = Self::parse_parameter_value(words, "movetime", UciParseError::ParamValueParseError("movetime"))?;
        let total_time: Option<u128> = Self::parse_parameter_value(&words, match self.position.side {
            Color::White => "wtime",
            Color::Black => "btime",
        }, UciParseError::ParamValueParseError("wtime/btime"))?;
        let increment_time: Option<u128> = Self::parse_parameter_value(&words, match self.position.side {
            Color::White => "winc",
            Color::Black => "binc",
        }, UciParseError::ParamValueParseError("winc/binc"))?;

        if let Some(perft_depth) = perft_depth {
            Perft::perft_test(&self.position, perft_depth, true);
            return Ok(());
        }

        let stop_time = if move_time.is_some() {
            move_time
        } else {
            Search::calculate_stop_time(total_time, increment_time)
        };

        self.search.go(&self.position, depth, stop_time);
        Ok(())
    }
    
    #[inline(always)]
    pub fn parse_move_string(position: &Position, move_string: &str) -> Result<BitMove, MoveStringParseError> {
        if move_string.len() == 4 || move_string.len() == 5 {
            let source = Square::try_from(&move_string[0..2])?;
            let target = Square::try_from(&move_string[2..4])?;
            let promotion_piece_option = if move_string.len() == 5 {
                Some(&move_string[4..5])
            } else {
                None
            };

            for m in MoveGeneration::generate_moves::<BitMove, Legal>(position) {
                let s = m.source();
                let t = m.target();
                let f = m.flag_option();
                
                if source == s && target == t {
                    match promotion_piece_option {
                        Some(promotion_piece_string) => {
                            match promotion_piece_string {
                                "q" => if f == Some(MoveFlag::PromoQ) { return Ok(m); },
                                "r" => if f == Some(MoveFlag::PromoR) { return Ok(m); },
                                "b" => if f == Some(MoveFlag::PromoB) { return Ok(m); },
                                "n" => if f == Some(MoveFlag::PromoN) { return Ok(m); },
                                _ => return Err(MoveStringParseError::PromotionPieceParseError(promotion_piece_string.to_string()))
                            }
                        },
                        None => return Ok(m),
                    }
                }
            }

            Err(MoveStringParseError::IllegalMove(move_string.to_string()))
        } else {
            Err(MoveStringParseError::LengthParseError)
        }
    }
}
