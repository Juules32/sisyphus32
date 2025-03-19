use std::{mem, sync::Mutex};

use crate::{bit_move::ScoringMove, zobrist::ZobristKey};

#[cfg(all(feature = "unit_tt", not(feature = "unit_small_tt")))]
const TT_SIZE: usize = 100_000;

#[cfg(any(feature = "unit_small_tt", not(feature = "unit_tt")))]
const TT_SIZE: usize = 1_000;

#[cfg(feature = "unit_tt_two_tier")]
static mut TRANSPOSITION_TABLE: [Mutex<TTSlot>; TT_SIZE] = [const { Mutex::new(TTSlot { main_entry: None, secondary_entry: None }) }; TT_SIZE];

#[cfg(not(feature = "unit_tt_two_tier"))]
static mut TRANSPOSITION_TABLE: [Mutex<TTSlot>; TT_SIZE] = [const { Mutex::new(TTSlot { entry: None }) }; TT_SIZE];

pub struct TranspositionTable;

#[cfg(feature = "unit_tt_two_tier")]
struct TTSlot {
    main_entry: Option<TTEntry>,
    secondary_entry: Option<TTEntry>,
}

#[cfg(not(feature = "unit_tt_two_tier"))]
struct TTSlot {
    entry: Option<TTEntry>,
}

#[derive(Clone, Copy)]
pub struct TTEntry {
    pub zobrist_key: ZobristKey,
    pub data: TTData,
}

impl TTEntry {
    #[inline(always)]
    fn new(zobrist_key: ZobristKey, data: TTData) -> TTEntry {
        TTEntry { zobrist_key, data }
    }
}

#[derive(Clone, Copy)]
pub struct TTData {
    pub best_move: ScoringMove,
    pub depth: u16,
    pub flag: TTNodeType,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TTNodeType {
    Exact,
    LowerBound, // β cutoff aka. Fail-high
    UpperBound, // α fail aka. Fail-low
}

impl TranspositionTable {
    #[inline(always)]
    pub fn reset() {
        unsafe { TRANSPOSITION_TABLE = mem::zeroed() };
    }
}

#[cfg(feature = "unit_tt_two_tier")]
impl TranspositionTable {
    // Store using a two-tier approach: https://www.chessprogramming.org/Transposition_Table#Two-tier_System
    #[inline(always)]
    pub fn store(zobrist_key: ZobristKey, data: TTData) {
        unsafe {
            let index = (zobrist_key.0 as usize) % TT_SIZE;
            let mut slot = TRANSPOSITION_TABLE[index].lock().unwrap();

            if let Some(existing_entry) = slot.main_entry {
                if data.depth >= existing_entry.data.depth {
                    slot.main_entry = Some(TTEntry::new(zobrist_key, data));
                } else {
                    slot.secondary_entry = Some(TTEntry::new(zobrist_key, data));
                }
            } else {
                slot.main_entry = Some(TTEntry::new(zobrist_key, data));
            }
        }
    }

    #[inline(always)]
    pub fn probe(zobrist_key: ZobristKey) -> Option<TTData> {
        unsafe {
            let index = (zobrist_key.0 as usize) % TT_SIZE;
            let slot = TRANSPOSITION_TABLE[index].lock().unwrap();

            if let Some(entry) = slot.main_entry {
                if entry.zobrist_key == zobrist_key {
                    return Some(entry.data);
                }
            }

            if let Some(entry) = slot.secondary_entry {
                if entry.zobrist_key == zobrist_key {
                    return Some(entry.data);
                }
            }

            None
        }
    }
}

#[cfg(not(feature = "unit_tt_two_tier"))]
impl TranspositionTable {
    // Store using a two-tier approach: https://www.chessprogramming.org/Transposition_Table#Two-tier_System
    #[inline(always)]
    pub fn store(zobrist_key: ZobristKey, data: TTData) {
        unsafe {
            let index = (zobrist_key.0 as usize) % TT_SIZE;
            let mut slot = TRANSPOSITION_TABLE[index].lock().unwrap();
            slot.entry = Some(TTEntry::new(zobrist_key, data));
        }
    }

    #[inline(always)]
    pub fn probe(zobrist_key: ZobristKey) -> Option<TTData> {
        unsafe {
            let index = (zobrist_key.0 as usize) % TT_SIZE;
            let slot = TRANSPOSITION_TABLE[index].lock().unwrap();

            if let Some(entry) = slot.entry {
                if entry.zobrist_key == zobrist_key {
                    return Some(entry.data);
                }
            }

            None
        }
    }
}
