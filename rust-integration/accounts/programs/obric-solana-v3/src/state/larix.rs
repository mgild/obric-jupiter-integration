use crate::errors::ObricError;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_pack::Pack;
use core::ops::Deref;
use larix_lending::math::{Decimal, TryAdd, TryDiv, TrySub};
use larix_lending::state::obligation::Obligation as LarixObligation;
use larix_lending::state::reserve::Reserve as LarixReserve;

#[derive(Clone)]
pub struct Reserve(LarixReserve);

impl anchor_lang::IdlBuild for Reserve {}

impl anchor_lang::AccountDeserialize for Reserve {
    fn try_deserialize_unchecked(data: &mut &[u8]) -> Result<Self> {
        let reserve = LarixReserve::unpack(data)
            .map_err(|_x| error!(ObricError::LarixAccountDeserializeFailed))?;

        return Ok(Reserve(reserve));
    }
}

impl anchor_lang::AccountSerialize for Reserve {
    fn try_serialize<W: std::io::Write>(&self, _writer: &mut W) -> std::result::Result<(), Error> {
        Err(error!(ObricError::TryToSerializeLarixAccount))
    }
}

impl anchor_lang::Owner for Reserve {
    fn owner() -> Pubkey {
        return larix_lending::ID;
    }
}

impl Deref for Reserve {
    type Target = LarixReserve;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Reserve {
    pub fn get_ctoken_exchange_rate(&self) -> Result<Decimal> {
        let available_amount = Decimal::from(self.liquidity.available_amount);
        let total_borrow = self.liquidity.borrowed_amount_wads;
        // let accumulated_protocol_fees = self.liquidity.accumulated_protocol_fees_wads;
        let unclaimed_protocol_fees = self.liquidity.owner_unclaimed;

        let total_supply = available_amount
            .try_add(total_borrow)?
            .try_sub(unclaimed_protocol_fees)?;

        // let total_supply = self.liquidity.total_supply()?;

        let mint_total_supply = self.collateral.mint_total_supply;

        let ctoken_exchange_rate = total_supply.try_div(mint_total_supply)?;

        Ok(ctoken_exchange_rate)
    }
}
#[account]
pub struct CtokenInfo {
    pub exchange_rate: u64,
    pub fee_receiver: Pubkey, // useless
    pub available_amount: u64,
    pub total_borrow: u64,
    pub unclaimed_protocol_fees: u64,
}

#[derive(Clone)]
pub struct Obligation(LarixObligation);

impl anchor_lang::IdlBuild for Obligation {}

impl anchor_lang::AccountDeserialize for Obligation {
    fn try_deserialize_unchecked(data: &mut &[u8]) -> Result<Self> {
        let obligation = LarixObligation::unpack(data)
            .map_err(|_x| error!(ObricError::LarixAccountDeserializeFailed))?;

        return Ok(Obligation(obligation));
    }
}

impl anchor_lang::AccountSerialize for Obligation {
    fn try_serialize<W: std::io::Write>(&self, _writer: &mut W) -> std::result::Result<(), Error> {
        Err(error!(ObricError::TryToSerializeLarixAccount))
    }
}

impl anchor_lang::Owner for Obligation {
    fn owner() -> Pubkey {
        return larix_lending::id();
    }
}

impl Deref for Obligation {
    type Target = LarixObligation;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
