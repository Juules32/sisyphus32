use std::{mem, sync::Mutex};

use crate::{bit_move::ScoringMove, zobrist::ZobristKey};

#[cfg(all(feature = "transposition_table", not(feature = "small_transposition_table")))]
const TT_SIZE: usize = 100_000;

#[cfg(any(feature = "small_transposition_table", not(feature = "transposition_table")))]
const TT_SIZE: usize = 1_000;

#[cfg(feature = "tt_two_tier")]
static mut TRANSPOSITION_TABLE: [Mutex<TTSlot>; TT_SIZE] = [const { Mutex::new(TTSlot { main_entry: None, secondary_entry: None }) }; TT_SIZE];

#[cfg(feature = "tt_always_replace")]
static mut TRANSPOSITION_TABLE: [Mutex<TTSlot>; TT_SIZE] = [const { Mutex::new(TTSlot { entry: None }) }; TT_SIZE];

pub struct TranspositionTable;

#[cfg(feature = "tt_two_tier")]
struct TTSlot {
    main_entry: Option<TTEntry>,
    secondary_entry: Option<TTEntry>,
}

#[cfg(feature = "tt_always_replace")]
struct TTSlot {
    entry: Option<TTEntry>,
}

#[derive(Clone, Copy)]
pub struct TTEntry {
    pub zobrist_key: ZobristKey,
    pub best_move: ScoringMove,
    pub depth: u16,
    pub flag: TTNodeType,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TTNodeType {
    Exact,
    LowerBound, // β cutoff
    UpperBound, // α fail
}

#[cfg(feature = "tt_two_tier")]
impl TranspositionTable {
    // Store using a two-tier approach: https://www.chessprogramming.org/Transposition_Table#Two-tier_System
    #[inline(always)]
    pub fn store(zobrist_key: ZobristKey, entry: TTEntry) {
        unsafe {
            let index = (zobrist_key.0 as usize) % TT_SIZE;
            let mut slot = TRANSPOSITION_TABLE[index].lock().unwrap();

            if let Some(existing_entry) = slot.main_entry {
                if entry.depth >= existing_entry.depth {
                    slot.main_entry = Some(entry);
                } else {
                    slot.secondary_entry = Some(entry);
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

    #[inline(always)]
    pub fn reset() {
        unsafe { TRANSPOSITION_TABLE = mem::zeroed() };
    }
}

#[cfg(feature = "tt_always_replace")]
impl TranspositionTable {
    // Store using a two-tier approach: https://www.chessprogramming.org/Transposition_Table#Two-tier_System
    #[inline(always)]
    pub fn store(zobrist_key: ZobristKey, entry: TTEntry) {
        unsafe {
            let index = (zobrist_key.0 as usize) % TT_SIZE;
            let mut slot = TRANSPOSITION_TABLE[index].lock().unwrap();
            slot.entry = Some(entry);
        }
    }

    #[inline(always)]
    pub fn probe(zobrist_key: ZobristKey) -> Option<TTEntry> {
        unsafe {
            let index = (zobrist_key.0 as usize) % TT_SIZE;
            let slot = TRANSPOSITION_TABLE[index].lock().unwrap();

            if let Some(entry) = slot.entry {
                if entry.zobrist_key == zobrist_key {
                    return Some(entry);
                }
            }

            None
        }
    }
}
