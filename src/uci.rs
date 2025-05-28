use std::{io::{self, BufRead}, process::exit, sync::{atomic::Ordering, mpsc}, thread};

use crate::{GlobalThreadPool, BitMove, Color, EvalPosition, FenString, HistoryHeuristic, KillerMoves, Legal, MoveFlag, MoveGeneration, MoveList, MoveStringParseError, Perft, Position, Search, Square, TranspositionTable, UciParseError};

const DEFAULT_TT_SIZE_MB: usize = 16;
const MIN_TT_SIZE_MB: usize = 1;
const MAX_TT_SIZE_MB: usize = 10_000;

const DEFAULT_NUM_THREADS: usize = 1;
const MIN_NUM_THREADS: usize = 0;
const MAX_NUM_THREADS: usize = 1024;

pub struct Uci {
    position: Position,
    search: Search,
}

impl Default for Uci {
    fn default() -> Self {
        let mut search = Search::default();
        search.show_uci_info();

        Self {
            position: Position::starting_position(),
            search,
        }
    }
}

impl Uci {
    pub fn init(&mut self) {
        println!("info string listening on stdin for uci commands");

        let (uci_command_tx, uci_command_rx) = mpsc::channel();

        let stop_calculating = self.search.get_stop_calculating();
        
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
                eprintln!("{error}!");
            };
        }
    }

    fn print_uci_info() {
        println!("id name Sisyphus32");
        println!("id author Juules32");
        println!();
        println!("option name Threads type spin default {DEFAULT_NUM_THREADS} min {MIN_NUM_THREADS} max {MAX_NUM_THREADS}");
        println!("option name Hash type spin default {DEFAULT_TT_SIZE_MB} min {MIN_TT_SIZE_MB} max {MAX_TT_SIZE_MB}");
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
                    "ucinewgame" => {
                        self.ucinewgame();
                        Ok(())
                    },
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
                    _ => Err(UciParseError::Keyword),
                }
            }
            None => Ok(()),
        }
    }

    fn ucinewgame(&mut self) {
        KillerMoves::reset();
        HistoryHeuristic::reset();
        TranspositionTable::reset();
        self.search.in_opening = true;
        self.position = Position::starting_position();
    }

    fn parse_setoption(&mut self, line: &str, words: &[&str]) -> Result<(), UciParseError> {
        if line == "setoption name Clear Hash" {
            TranspositionTable::reset();
            println!("info string transposition table reset successfully");
            Ok(())
        } else if line.starts_with("setoption name Threads value") {
            let num_threads = words.last().unwrap().parse().map_err(|_| UciParseError::ParamValue("Threads"))?;
            if num_threads > MAX_NUM_THREADS {
                return Err(UciParseError::ParamRange("Threads"));
            }
            
            GlobalThreadPool::set_threadpool(num_threads);
            println!("info string set threads to {num_threads} successfully");
            Ok(())
        } else if line.starts_with("setoption name SyzygyPath value") {
            let path = words.last().unwrap();
            self.search.set_tablebase(path);
            println!("info string set syzygy path to {path} successfully");
            Ok(())
        } else if line.starts_with("setoption name Hash value") {
            let tt_size_mb = words.last().unwrap().parse().map_err(|_| UciParseError::ParamValue("Transposition Table Size (MB)"))?;
            if tt_size_mb < MIN_TT_SIZE_MB || tt_size_mb > MAX_TT_SIZE_MB {
                return Err(UciParseError::ParamRange("Transposition Table Size (MB)"));
            }
            
            TranspositionTable::resize(tt_size_mb);
            println!("info string set transposition table size to {tt_size_mb}MB successfully");
            Ok(())
        } else {
            Err(UciParseError::Option)
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
            return Err(UciParseError::Param("Neither fen nor startpos found"));
        }

        self.search.zobrist_key_history = Vec::new();
        if let Some(moves_index) = moves_index_option {
            let move_strings: Vec<String> = line[moves_index + 5..]
                .split_whitespace()
                .map(|move_string| move_string.to_string())
                .collect();

            for move_string in &move_strings {
                let legal_moves = MoveGeneration::generate_moves::<BitMove, Legal>(&self.position);
                let bit_move = Self::parse_move_string(&legal_moves, move_string)?;
                self.position.make_move(bit_move);
                if bit_move.is_pp_capture_or_castle(&self.position) {
                    self.search.zobrist_key_history = Vec::new();
                } else {
                    self.search.zobrist_key_history.push(self.position.zobrist_key);
                }
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
        let depth: Option<usize> = Self::parse_parameter_value(words, "depth", UciParseError::ParamValue("depth"))?;
        let perft_depth: Option<u16> = Self::parse_parameter_value(words, "perft", UciParseError::ParamValue("perft depth"))?;
        let move_time: Option<u128> = Self::parse_parameter_value(words, "movetime", UciParseError::ParamValue("movetime"))?;
        let total_time: Option<u128> = Self::parse_parameter_value(words, match self.position.side {
            Color::White => "wtime",
            Color::Black => "btime",
        }, UciParseError::ParamValue("wtime/btime"))?;
        let increment_time: Option<u128> = Self::parse_parameter_value(words, match self.position.side {
            Color::White => "winc",
            Color::Black => "binc",
        }, UciParseError::ParamValue("winc/binc"))?;

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
    pub(crate) fn parse_move_string(move_list: &MoveList<BitMove>, move_string: &str) -> Result<BitMove, MoveStringParseError> {
        if move_string.len() == 4 || move_string.len() == 5 {
            let source = Square::try_from(&move_string[0..2])?;
            let target = Square::try_from(&move_string[2..4])?;
            let promotion_piece_option = if move_string.len() == 5 {
                Some(&move_string[4..5])
            } else {
                None
            };

            for &m in move_list.iter() {
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
