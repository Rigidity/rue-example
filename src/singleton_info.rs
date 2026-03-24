use chia_wallet_sdk::prelude::*;

use crate::CustomPuzzleArgs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CustomSingletonInfo {
    pub launcher_id: Bytes32,
    pub public_key: PublicKey,
    pub value: usize, // just a counter, can be more complex if needed
}

impl CustomSingletonInfo {
    pub fn new(launcher_id: Bytes32, public_key: PublicKey, value: usize) -> Self {
        Self {
            launcher_id,
            public_key,
            value,
        }
    }
}

impl SingletonInfo for CustomSingletonInfo {
    fn launcher_id(&self) -> Bytes32 {
        self.launcher_id
    }

    fn inner_puzzle_hash(&self) -> TreeHash {
        CustomPuzzleArgs::new(self.public_key, self.value).curry_tree_hash()
    }
}
