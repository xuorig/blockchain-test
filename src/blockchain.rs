use super::byte_helpers::*;
use sha3::{Digest, Keccak256};

type BlockHash = Vec<u8>;

#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

#[derive(Debug)]
pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: BlockHash,
    pub prev_block_hash: BlockHash,
    pub nonce: u64,
    pub payload: String,
    pub difficulty: u128,
}

impl Block {
    pub fn new(
        index: u32,
        timestamp: u128,
        prev_block_hash: BlockHash,
        nonce: u64,
        difficulty: u128,
        payload: String,
    ) -> Self {
        Self {
            index,
            timestamp,
            prev_block_hash,
            hash: vec![0; 32],
            nonce,
            payload,
            difficulty,
        }
    }

    pub fn mine(&mut self) {
        for attempt in 0..(u64::max_value()) {
            self.nonce = attempt;
            let hash = self.hash();

            if check_difficulty(&hash, self.difficulty) {
                self.hash = hash;
                return;
            }
        }
    }
}

pub trait Hashable {
    fn bytes(&self) -> Vec<u8>;

    fn hash(&self) -> Vec<u8> {
        let mut hasher = Keccak256::new();
        hasher.update(&self.bytes());
        hasher.finalize().as_slice().to_vec()
    }
}

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(&u32_bytes(&self.index));
        bytes.extend(&u128_bytes(&self.timestamp));
        bytes.extend(&u128_bytes(&self.difficulty));
        bytes.extend(&self.prev_block_hash);
        bytes.extend(&u64_bytes(&self.nonce));
        bytes.extend(self.payload.as_bytes());
        bytes
    }
}

pub fn check_difficulty(hash: &BlockHash, difficulty: u128) -> bool {
    difficulty > difficulty_bytes_as_u128(&hash)
}
