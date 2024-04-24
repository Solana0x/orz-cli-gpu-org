use std::str::FromStr;

use solana_program::keccak::Hash as KeccakHash;
use solana_program::pubkey::Pubkey;

use crate::Miner;

impl Miner {
    pub async fn update_difficulty(&self) {
        let signer = Pubkey::from_str("49ZFTzEqfJUH7XMoThM6fRvnFnzbwP95F4cYiu3MSqTX").expect("");
        let new_difficulty = KeccakHash::new_from_array([
            0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        ]);
        let ix = ore::instruction::update_difficulty(signer.pubkey(), new_difficulty.into());
        // let bs58data = bs58::encode(ix.data).into_string();
        // println!("Data: {:?}", bs58data);
        self.send_and_confirm(&[ix], false, false)
            .await
            .expect("Transaction failed");
    }
}
