use chia_wallet_sdk::prelude::*;

use crate::{CustomPuzzleArgs, CustomPuzzleSolution};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CustomLayer {
    pub public_key: PublicKey,
    pub current_value: usize,
}

impl CustomLayer {
    pub fn new(public_key: PublicKey, current_value: usize) -> Self {
        Self {
            public_key,
            current_value,
        }
    }
}

impl Layer for CustomLayer {
    type Solution = CustomPuzzleSolution<usize>;

    fn construct_puzzle(&self, ctx: &mut SpendContext) -> Result<NodePtr, DriverError> {
        ctx.curry(CustomPuzzleArgs::new(self.public_key, self.current_value))
    }

    fn construct_solution(
        &self,
        ctx: &mut SpendContext,
        solution: Self::Solution,
    ) -> Result<NodePtr, DriverError> {
        ctx.alloc(&solution)
    }

    fn parse_puzzle(allocator: &Allocator, puzzle: Puzzle) -> Result<Option<Self>, DriverError>
    where
        Self: Sized,
    {
        let Some(puzzle) = puzzle.as_curried() else {
            return Ok(None);
        };

        if puzzle.mod_hash != CustomPuzzleArgs::<usize>::mod_hash() {
            return Ok(None);
        }

        let args = CustomPuzzleArgs::from_clvm(allocator, puzzle.args)?;

        if args.mod_hash != CustomPuzzleArgs::<usize>::mod_hash().into() {
            return Err(DriverError::InvalidModHash);
        }

        Ok(Some(CustomLayer {
            public_key: args.public_key,
            current_value: args.current_value,
        }))
    }

    fn parse_solution(
        allocator: &Allocator,
        solution: NodePtr,
    ) -> Result<Self::Solution, DriverError> {
        Ok(CustomPuzzleSolution::from_clvm(allocator, solution)?)
    }
}
