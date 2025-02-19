use std::sync::Mutex;

use crate::{bit_move::BitMove, zobrist::ZobristKey};

const TT_SIZE: usize = 1_000_000;

struct TranspositionTable {
    table: [Mutex<TTSlot>; TT_SIZE],
}

struct TTSlot {
    main_entry: Option<TTEntry>,
    secondary_entry: Option<TTEntry>,
}

#[derive(Clone, Copy)]
struct TTEntry {
    zobrist_key: ZobristKey, // Unique position identifier
    best_move: Option<BitMove>, // Best move found
    score: i32, // Evaluation score
    depth: i32, // Depth at which this entry was recorded
    flag: TTNodeType, // Type of node (Exact, LowerBound, UpperBound)
}

#[derive(Clone, Copy)]
enum TTNodeType {
    Exact,
    LowerBound, // β cutoff
    UpperBound, // α fail
}

impl Default for TranspositionTable {
    fn default() -> Self {
        Self {
            table: std::array::from_fn(|_| Mutex::new(TTSlot { 
                main_entry: None, 
                secondary_entry: None 
            })),
        }
    }
}

impl TranspositionTable {
    pub fn new() -> Self {
        Self::default()
    }

    // Store using a two-level approach
    #[inline(always)]
    pub fn store(&self, zobrist_key: ZobristKey, entry: TTEntry) {
        let index = (zobrist_key.0 as usize) % TT_SIZE;
        let mut slot = self.table[index].lock().unwrap();

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

    #[inline(always)]
    pub fn probe(&self, zobrist_key: ZobristKey) -> Option<TTEntry> {
        let index = (zobrist_key.0 as usize) % TT_SIZE;
        let slot = self.table[index].lock().unwrap();

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
