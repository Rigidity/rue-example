use chia_wallet_sdk::{
    chia::puzzle_types::singleton::SingletonSolution, driver::SingletonLayer, prelude::*,
};

use crate::{CustomLayer, CustomPuzzleSolution, CustomSingletonInfo};

pub type CustomSingleton = Singleton<CustomSingletonInfo>;

pub trait CustomSingletonExt {
    fn spend(
        &self,
        ctx: &mut SpendContext,
        new_value: usize,
    ) -> Result<CustomSingleton, DriverError>;
}

impl CustomSingletonExt for CustomSingleton {
    fn spend(
        &self,
        ctx: &mut SpendContext,
        new_value: usize,
    ) -> Result<CustomSingleton, DriverError> {
        let layers = SingletonLayer::new(
            self.info.launcher_id,
            CustomLayer::new(self.info.public_key, self.info.value),
        );

        let coin_spend = layers.construct_coin_spend(
            ctx,
            self.coin,
            SingletonSolution {
                lineage_proof: self.proof,
                amount: self.coin.amount,
                inner_solution: CustomPuzzleSolution::new(new_value, self.coin.amount),
            },
        )?;

        ctx.insert(coin_spend);

        Ok(self.child_with(
            CustomSingletonInfo::new(self.info.launcher_id, self.info.public_key, new_value),
            self.coin.amount,
        ))
    }
}
