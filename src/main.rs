use anyhow::Result;
use chia_wallet_sdk::{
    chia::puzzle_types::{EveProof, Proof},
    prelude::*,
};
use rue_example::{CustomSingleton, CustomSingletonExt, CustomSingletonInfo};

fn main() -> Result<()> {
    let mut sim = Simulator::new();
    let mut ctx = SpendContext::new();

    // Give Alice 1 mojo
    let alice = sim.bls(1);

    let launcher = Launcher::new(alice.coin.coin_id(), 1);

    let info = CustomSingletonInfo::new(launcher.coin().coin_id(), alice.pk, 0);

    let (launch_singleton, singleton_coin) =
        launcher.spend(&mut ctx, info.inner_puzzle_hash().into(), ())?;

    StandardLayer::new(alice.pk).spend(&mut ctx, alice.coin, launch_singleton)?;

    let mut singleton = CustomSingleton::new(
        singleton_coin,
        Proof::Eve(EveProof {
            parent_parent_coin_info: alice.coin.coin_id(),
            parent_amount: alice.coin.amount,
        }),
        info,
    );

    for i in 1..=10 {
        singleton = singleton.spend(&mut ctx, i)?;
    }

    sim.spend_coins(ctx.take(), &[alice.sk])?;

    println!("Simulator spends were successful");

    Ok(())
}
