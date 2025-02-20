use std::sync::Mutex;

use crate::{bit_move::ScoringMove, zobrist::ZobristKey};

#[cfg(all(feature = "transposition_table", not(feature = "small_transposition_table")))]
const TT_SIZE: usize = 100_000;

#[cfg(any(feature = "small_transposition_table", not(feature = "transposition_table")))]
const TT_SIZE: usize = 1;

static mut TRANSPOSITION_TABLE: [Mutex<TTSlot>; TT_SIZE] = [const { Mutex::new(TTSlot { main_entry: None, secondary_entry: None }) }; TT_SIZE];

pub struct TranspositionTable {
    
}

struct TTSlot {
    main_entry: Option<TTEntry>,
    secondary_entry: Option<TTEntry>,
}

#[derive(Clone, Copy)]
pub struct TTEntry {
    pub zobrist_key: ZobristKey, // Unique position identifier
    pub best_move: ScoringMove, // Best move found
    pub depth: u8, // Depth at which this entry was recorded
    pub flag: TTNodeType, // Type of node (Exact, LowerBound, UpperBound)
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TTNodeType {
    Exact,
    LowerBound, // β cutoff
    UpperBound, // α fail
}

impl TranspositionTable {
    // Store using a two-level approach
    #[inline(always)]
    pub fn store(zobrist_key: ZobristKey, entry: TTEntry) {
        unsafe {
            let index = (zobrist_key.0 as usize) % TT_SIZE;
            let mut slot = TRANSPOSITION_TABLE[index].lock().unwrap();

            if let Some(existing_entry) = slot.main_entry {
                if entry.depth >= existing_entry.depth {
                    slot.main_entry = Some(entry); // Replace main if depth is better
                } else {
                    slot.secondary_entry = Some(entry); // Store in secondary slot
                }
            } else {
                slot.main_entry = Some(entry);
            }
        }
    }

    #[inline(always)]
    pub fn probe(zobrist_key: ZobristKey) -> Option<TTEntry> {
        unsafe {
            let index = (zobrist_key.0 as usize) % TT_SIZE;
            let slot = TRANSPOSITION_TABLE[index].lock().unwrap();

            if let Some(entry) = slot.main_entry {
                if entry.zobrist_key == zobrist_key {
                    return Some(entry);
                }
            }

            if let Some(entry) = slot.secondary_entry {
                if entry.zobrist_key == zobrist_key {
                    return Some(entry);
                }
            }

            None
        }
    }
}
