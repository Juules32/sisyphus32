#![allow(static_mut_refs)]

use std::ops::BitXor;

use crate::{ScoringMove, ZobristKey};

const TT_INIT_BYTES_SIZE: usize = 16; // 16MB

#[cfg(not(feature = "unit_lockless_hashing"))]
static mut TRANSPOSITION_TABLE: Vec<std::sync::Mutex<TTSlot>> = vec![];

#[cfg(feature = "unit_lockless_hashing")]
static mut TRANSPOSITION_TABLE: Vec<TTSlot> = vec![];

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

#[cfg(feature = "unit_tt_two_tier")]
impl TTSlot {
    const EMPTY: TTSlot = TTSlot { main_entry: None, secondary_entry: None };
}

#[cfg(not(feature = "unit_tt_two_tier"))]
impl TTSlot {
    const EMPTY: TTSlot = TTSlot { entry: None };
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
    pub node_type: TTNodeType,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TTNodeType {
    Exact,
    LowerBound, // β cutoff aka. Fail-high
    UpperBound, // α fail aka. Fail-low
}

impl TranspositionTable {
    pub unsafe fn init() {
        Self::resize(TT_INIT_BYTES_SIZE);
    }
}

#[cfg(not(feature = "unit_lockless_hashing"))]
impl TranspositionTable {
    #[inline(always)]
    pub fn reset() {
        unsafe { 
            TRANSPOSITION_TABLE = (0..TRANSPOSITION_TABLE.len())
                .map(|_| std::sync::Mutex::new(TTSlot::EMPTY))
                .collect();
        }
    }

    #[inline(always)]
    pub fn resize(size_mb: usize) {
        unsafe {
            TRANSPOSITION_TABLE = (0..size_mb * 1_000_000 / size_of::<TTSlot>())
                .map(|_| std::sync::Mutex::new(TTSlot::EMPTY))
                .collect();
        }
    }

    #[inline(always)]
    fn get_slot(zobrist_key: ZobristKey) -> std::sync::MutexGuard<'static, TTSlot> {
        unsafe {
            let index = (zobrist_key.0 as usize) % TRANSPOSITION_TABLE.len();
            TRANSPOSITION_TABLE[index].lock().unwrap()
        }
    }

    #[inline(always)]
    fn verify_key(zobrist_key: ZobristKey, entry: &TTEntry) -> bool {
        entry.zobrist_key == zobrist_key
    }

    #[inline(always)]
    fn store_entry(entry: &mut Option<TTEntry>, zobrist_key: ZobristKey, data: TTData) {
        *entry = Some(TTEntry::new(zobrist_key, data));
    }
}

#[cfg(feature = "unit_lockless_hashing")]
impl TranspositionTable {
    #[inline(always)]
    pub fn reset() {
        unsafe {
            TRANSPOSITION_TABLE = (0..TRANSPOSITION_TABLE.len())
                .map(|_| TTSlot::EMPTY)
                .collect();
        }
    }

    #[inline(always)]
    pub fn resize(size_mb: usize) {
        unsafe {
            TRANSPOSITION_TABLE = (0..size_mb * 1_000_000 / size_of::<TTSlot>())
                .map(|_| TTSlot::EMPTY)
                .collect();
        }
    }

    #[inline(always)]
    fn get_slot(zobrist_key: ZobristKey) -> &'static mut TTSlot {
        unsafe {
            let index = (zobrist_key.0 as usize) % TRANSPOSITION_TABLE.len();
            &mut TRANSPOSITION_TABLE[index]
        }
    }

    #[inline(always)]
    fn verify_key(zobrist_key: ZobristKey, entry: &TTEntry) -> bool {
        entry.zobrist_key ^ entry.data == zobrist_key
    }

    #[inline(always)]
    fn store_entry(entry: &mut Option<TTEntry>, zobrist_key: ZobristKey, data: TTData) {
        *entry = Some(TTEntry::new(zobrist_key ^ data, data));
    }
}

#[cfg(feature = "unit_tt_two_tier")]
impl TranspositionTable {
    // Store using a two-tier approach: https://www.chessprogramming.org/Transposition_Table#Two-tier_System
    #[inline(always)]
    pub fn store(zobrist_key: ZobristKey, data: TTData) {
        #[allow(unused_mut)]
        let mut slot = Self::get_slot(zobrist_key);
        if let Some(existing_entry) = slot.main_entry {
            if data.depth >= existing_entry.data.depth {
                Self::store_entry(&mut slot.main_entry, zobrist_key, data);
            } else {
                Self::store_entry(&mut slot.secondary_entry, zobrist_key, data);
            }
        } else {
            Self::store_entry(&mut slot.main_entry, zobrist_key, data);
        }
    }

    #[inline(always)]
    pub fn probe(zobrist_key: ZobristKey) -> Option<TTData> {
        let slot = Self::get_slot(zobrist_key);

        if let Some(entry) = slot.main_entry {
            if Self::verify_key(zobrist_key, &entry) {
                return Some(entry.data);
            }
        }

        if let Some(entry) = slot.secondary_entry {
            if Self::verify_key(zobrist_key, &entry) {
                return Some(entry.data);
            }
        }

        None
    }
}

#[cfg(not(feature = "unit_tt_two_tier"))]
impl TranspositionTable {
    // Store using a two-tier approach: https://www.chessprogramming.org/Transposition_Table#Two-tier_System
    #[inline(always)]
    pub fn store(zobrist_key: ZobristKey, data: TTData) {
        let mut slot = Self::get_slot(zobrist_key);
        Self::store_entry(&mut slot.entry, zobrist_key, data);
    }

    #[inline(always)]
    pub fn probe(zobrist_key: ZobristKey) -> Option<TTData> {
        let slot = Self::get_slot(zobrist_key);

        if let Some(entry) = slot.entry {
            if Self::verify_key(zobrist_key, &entry) {
                return Some(entry.data);
            }
        }

        None
    }
}

impl BitXor<TTData> for ZobristKey {
    type Output = ZobristKey;

    #[inline(always)]
    #[cfg(feature = "unit_bb_array")]
    fn bitxor(self, rhs: TTData) -> Self::Output {
        unsafe { std::mem::transmute::<u64, ZobristKey>(self.0 ^ std::mem::transmute::<TTData, u64>(rhs)) }
    }

    #[inline(always)]
    #[cfg(feature = "unit_bb")]
    // NOTE: This should NEVER happen. The function is defined only because of compile time errors
    // that arise when a bit move is not 16 bits in size, which results in TTData being more than
    // 64 bits in size.
    fn bitxor(self, rhs: TTData) -> Self::Output {
        unsafe { std::mem::zeroed() }
    }
}
