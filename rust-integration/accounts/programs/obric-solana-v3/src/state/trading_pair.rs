use anchor_lang::prelude::*;
use num::pow;

use crate::{consts, errors::ObricError};

#[account]
#[derive(Default, Debug)]
pub struct SSTradingPair {
    pub is_initialized: bool,

    pub x_price_feed_id: Pubkey,
    pub y_price_feed_id: Pubkey,

    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,

    pub reserve_x_ctoken: Pubkey,
    pub reserve_y_ctoken: Pubkey,

    pub protocol_fee_x: Pubkey,
    pub protocol_fee_y: Pubkey,

    pub bump: u8,
    // mints
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,

    // these numbers are synced with lending interface, and are updated after every tx
    pub deposit_x: u64,
    pub borrow_x: u64,

    pub deposit_y: u64,
    pub borrow_y: u64,

    pub target_y: u64,

    pub concentration: u64,
    pub big_k: u128,

    pub cumulative_volume: u64,

    pub mult_x: u64,
    pub mult_y: u64,
    pub fee_millionth: u64,
    pub rebate_percentage: u64,
    pub protocol_fee_share_thousandth: u64,

    pub decimals_x: u8,
    pub decimals_y: u8,

    pub volume_records: [u64; 8],

    pub padding: [u8; 6],
    pub padding1: [u64; 15],
    pub padding2: [u64; 16],
}

impl SSTradingPair {
    pub const LEN: usize = 8 + 1 + 32 * 8 + 1 + 32 * 2 + 8 * 6 + 16 + 8 * 6 + 2 + 254; // 8 for internal anchor

    #[inline(never)]
    pub fn update_price(
        &mut self,
        price_x: u64,
        price_y: u64,
    ) -> Result<()> {
        let x_decimals = self.decimals_x;
        let y_decimals = self.decimals_y;
        let (x_deci_mult, y_deci_mult) = if x_decimals > y_decimals {
            (1 as u64, pow(10, usize::from(x_decimals - y_decimals)))
        } else if y_decimals > x_decimals {
            (pow(10, usize::from(y_decimals - x_decimals)), 1 as u64)
        } else {
            (1 as u64, 1 as u64)
        };

        self.mult_x = price_x.checked_mul(x_deci_mult).unwrap();
        self.mult_y = price_y.checked_mul(y_deci_mult).unwrap();

        Ok(())
    }

    pub fn update_target_y(&mut self, new_target_y: u64) -> Result<()> {
        let target_y_k = (new_target_y as u128)
            .checked_mul(self.concentration as u128)
            .unwrap();
        let target_x_k = target_y_k
            .checked_mul(self.mult_y as u128)
            .unwrap()
            .checked_div(self.mult_x as u128)
            .unwrap();
        self.target_y = new_target_y;
        self.big_k = target_x_k.checked_mul(target_y_k).unwrap();

        Ok(())
    }


    pub fn compute_target_y(&self) -> u64 {
        let deposit_x_value = self.deposit_x.checked_mul(self.mult_x).unwrap();
        let borrow_x_value = self.borrow_x.checked_mul(self.mult_x).unwrap();
        let deposit_y_value = self.deposit_y.checked_mul(self.mult_y).unwrap();
        let borrow_y_value = self.borrow_y.checked_mul(self.mult_y).unwrap();
        let deposit_value = deposit_x_value.checked_add(deposit_y_value).unwrap();
        let borrow_value = borrow_x_value.checked_add(borrow_y_value).unwrap();
        let net_value = deposit_value.checked_sub(borrow_value).unwrap();
        let target_y = net_value.checked_div(self.mult_y).unwrap();

        target_y
    }

    /*
    When quoting, we assume X and Y are equally priced. We adjust our output amounts by the price of X only after the
    quote has been computed.

    here we do not compute current_x_K or target_x_K, instead we compute:
    - current_xp_K = current_x_K * mult_x / mult_y
    - target_xp_K = target_x_K * mult_x / mult_y = target_y_K

    At "target"/equilibrium:
    - target_y = deposited amount of Y
    - target_y_K = deposited amount of Y * concentration
    - targeted deposit_x = targeted borrow_x = 0
    - targeted deposit_y = target_y
    */
    pub fn get_pool_values_for_quoting(&self) -> Result<(u128, u64, u64, u64, u64)> {
        // u64 or u128 here?
        let target_y_k = self.concentration.checked_mul(self.target_y).unwrap();
        let target_x_k = target_y_k
            .checked_mul(self.mult_y)
            .unwrap()
            .checked_div(self.mult_x)
            .unwrap();
        let current_y_k = target_y_k
            .checked_add(self.deposit_y).unwrap().checked_sub(self.target_y).unwrap();
        let current_x_k = target_x_k
            .checked_add(self.deposit_x).unwrap().checked_sub(self.borrow_x).unwrap();
        let big_k = (current_x_k as u128)
            .checked_mul(current_y_k as u128)
            .unwrap();

        let temp = target_x_k
            .checked_div(self.concentration)
            .unwrap()
            .checked_add(self.deposit_x)
            .unwrap();
        let available_x = if temp > self.borrow_x {
            temp.checked_sub(self.borrow_x).unwrap()
        } else {
            0
        };
        let available_y = self.deposit_y;

        Ok((big_k, current_x_k, current_y_k, available_x, available_y))
    }

    /**
    Returns (output_to_user, fee_to_protocol)
    */
    pub fn quote_x_to_y(&self, input_x: u64) -> Result<(u64, u64, u64)> {
        let (big_k, current_x_k, current_y_k, _available_x, available_y) =
            self.get_pool_values_for_quoting()?;

        // 2. find new (x, y) on curve-K
        let new_x_k = current_x_k.checked_add(input_x).unwrap();
        let new_y_k: u64 = big_k
            .checked_div(new_x_k as u128)
            .unwrap()
            .try_into()
            .unwrap();

        let output_before_fee_y = current_y_k.checked_sub(new_y_k).unwrap();
        require!(
            output_before_fee_y < available_y,
            ObricError::InsufficientActiveY
        );

        let fee_y = output_before_fee_y
            .checked_mul(self.fee_millionth)
            .unwrap()
            .checked_div(consts::MILLION)
            .unwrap();
        let output_after_fee_y = output_before_fee_y.checked_sub(fee_y).unwrap();

        let protocol_fee_y = fee_y
            .checked_mul(self.protocol_fee_share_thousandth)
            .unwrap()
            .checked_div(1000)
            .unwrap();
        let lp_fee_y = fee_y.checked_sub(protocol_fee_y).unwrap();

        Ok((output_after_fee_y, protocol_fee_y, lp_fee_y))
    }

    /**
    Returns (output_to_user, fee_to_protocol, fee_to_reserve_x)
    */
    pub fn quote_y_to_x(&self, input_y: u64) -> Result<(u64, u64, u64)> {
        let (big_k, current_x_k, current_y_k, available_x, _available_y) =
            self.get_pool_values_for_quoting()?;

        // 2. find new (x, y) on curve-K
        let new_y_k = current_y_k.checked_add(input_y).unwrap();
        let new_x_k: u64 = big_k
            .checked_div(new_y_k as u128)
            .unwrap()
            .try_into()
            .unwrap();

        let output_before_fee_x = current_x_k.checked_sub(new_x_k).unwrap();
        require!(
            output_before_fee_x < available_x,
            ObricError::InsufficientActiveX
        );

        let fee_x = output_before_fee_x
            .checked_mul(self.fee_millionth)
            .unwrap()
            .checked_div(consts::MILLION)
            .unwrap();
        let output_after_fee_x = output_before_fee_x.checked_sub(fee_x).unwrap();

        let protocol_fee_x = fee_x
            .checked_mul(self.protocol_fee_share_thousandth)
            .unwrap()
            .checked_div(1000)
            .unwrap();
        let lp_fee_x = fee_x.checked_sub(protocol_fee_x).unwrap();

        Ok((output_after_fee_x, protocol_fee_x, lp_fee_x))
    }
}
