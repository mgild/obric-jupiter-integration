use anchor_lang::prelude::*;
use core::ops::Deref;
use pyth_sdk::Price;
use pyth_sdk_solana::state::load_price_account;

use crate::errors::ObricError;
use anchor_lang::IdlBuild;

#[derive(Clone, Debug)]
pub struct PriceFeed(pyth_sdk::PriceFeed);

declare_id!("FsJ3A3u2vn5cTVofAjvy6y5kwABJAqYWpe4975bi2epH");

impl PriceFeed {
    pub fn price_normalized(&self) -> Result<Price> {
        let p = self.0.get_price_unchecked();
        let price = p.scale_to_exponent(-3).ok_or(ObricError::PythError)?;
        Ok(price)
    }
}

impl IdlBuild for PriceFeed {
}

impl Owner for PriceFeed {
    fn owner() -> Pubkey {
        return id();
    }
}

impl AccountDeserialize for PriceFeed {
    fn try_deserialize_unchecked(data: &mut &[u8]) -> Result<Self> {
        let account: &pyth_sdk_solana::state::SolanaPriceAccount = load_price_account(data).map_err(|_x| error!(ObricError::PythError))?;

        // Use a dummy key since the key field will be removed from the SDK
        let feed = account.to_price_feed(&ID.to_string().parse().unwrap());
        return Ok(PriceFeed(feed));
    }
}

impl AccountSerialize for PriceFeed {
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
