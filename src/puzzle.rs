use std::borrow::Cow;

use chia_wallet_sdk::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ToClvm, FromClvm)]
#[clvm(curry)]
pub struct CustomPuzzleArgs<T = NodePtr> {
    pub mod_hash: Bytes32,
    pub public_key: PublicKey,
    pub current_value: T,
}

impl<T> CustomPuzzleArgs<T> {
    pub fn new(public_key: PublicKey, current_value: T) -> Self {
        Self {
            mod_hash: CUSTOM_PUZZLE.hash.into(),
            public_key,
            current_value,
        }
    }
}

impl<T> Mod for CustomPuzzleArgs<T> {
    fn mod_hash() -> TreeHash {
        CUSTOM_PUZZLE.hash
    }

    fn mod_reveal() -> Cow<'static, [u8]> {
        Cow::Owned(CUSTOM_PUZZLE.reveal.clone())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ToClvm, FromClvm)]
#[clvm(list)]
pub struct CustomPuzzleSolution<T = NodePtr> {
    pub new_value: T,
    pub my_amount: u64,
}

impl<T> CustomPuzzleSolution<T> {
    pub fn new(new_value: T, my_amount: u64) -> Self {
        Self {
            new_value,
            my_amount,
        }
    }
}

struct _CustomPuzzleMod;

compile_rue!(_CustomPuzzleMod = CUSTOM_PUZZLE, ".");
