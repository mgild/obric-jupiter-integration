use anchor_lang::prelude::*;
use core::ops::Deref;
use core::str::FromStr;
use pyth_sdk::Price;
use pyth_sdk_solana::state::load_price_account;

use crate::errors::ObricError;

#[derive(Clone)]
pub struct PriceFeed(pyth_sdk::PriceFeed);

impl PriceFeed {
    pub fn price_normalized(&self) -> Result<Price> {
        let p = self.0.get_price_unchecked();
        let price = p.scale_to_exponent(-3).unwrap();
        require!(price.price > 0, ObricError::NegativePrice);
        Ok(price)
    }
}

impl anchor_lang::IdlBuild for PriceFeed {}

impl anchor_lang::Owner for PriceFeed {
    fn owner() -> Pubkey {
        // Make sure the owner is the pyth oracle account on solana mainnet-beta
        let oracle_addr = "FsJ3A3u2vn5cTVofAjvy6y5kwABJAqYWpe4975bi2epH";
        return Pubkey::from_str(&oracle_addr).unwrap();
    }
}

impl anchor_lang::AccountDeserialize for PriceFeed {
    fn try_deserialize_unchecked(data: &mut &[u8]) -> Result<Self> {
        let account: &pyth_sdk_solana::state::SolanaPriceAccount = load_price_account(data).map_err(|_x| error!(ObricError::PythError))?;

        // Use a dummy key since the key field will be removed from the SDK
        let zeros: [u8; 32] = [0; 32];
        let dummy_key = Pubkey::try_from(zeros).unwrap();
        let feed = account.to_price_feed(&dummy_key.to_string().parse().unwrap());
        return Ok(PriceFeed(feed));
    }
}

impl anchor_lang::AccountSerialize for PriceFeed {
    fn try_serialize<W: std::io::Write>(&self, _writer: &mut W) -> std::result::Result<(), Error> {
        Err(error!(ObricError::TryToSerializePriceAccount))
    }
}

impl Deref for PriceFeed {
    type Target = pyth_sdk::PriceFeed;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
