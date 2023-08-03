use anchor_lang::prelude::*;
use num::{pow, integer::Roots};
use crate::consts::MILLION;

#[account]
#[derive(Default, Debug)]
pub struct SSTradingPair {
    pub is_initialized: bool,

    pub x_price_feed_id: Pubkey,
    pub y_price_feed_id: Pubkey,

    pub reserve_x: Pubkey,
    pub reserve_y: Pubkey,

    pub protocol_fee_x: Pubkey,
    pub protocol_fee_y: Pubkey,

    pub bump: u8,
    // mints
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,

    pub concentration: u64,
    pub big_k: u128,
    pub target_x: u64,

    pub cumulative_volume: u64,

    pub mult_x: u64,
    pub mult_y: u64,
    pub fee_millionth: u64,
    pub rebate_percentage: u64,
    pub protocol_fee_share_thousandth: u64,

    pub volume_record: [u64; 8],

    pub padding: [u64; 32],
}

impl SSTradingPair{

    #[inline(never)]
    pub fn update_price(
        &mut self,
        price_x: u64,
        price_y: u64,
        x_decimals: u8,
        y_decimals: u8,
    ) -> Result<()> {
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
    pub fn get_target_xy(
        &self,
        current_x: u64,
        current_y: u64,
    ) -> Result<(u64, u64)> {
        let value_x = (current_x as u128).checked_mul(self.mult_x as u128).unwrap();
        let value_y = (current_y as u128).checked_mul(self.mult_y as u128).unwrap();
        let value_total = value_x.checked_add(value_y).unwrap();

        let target_x = self.target_x;
        let target_x_value = (target_x as u128).checked_mul(self.mult_x as u128).unwrap();
        let target_y_value = value_total.checked_sub(target_x_value).unwrap();
        let target_y = target_y_value.checked_div(self.mult_y as u128).unwrap() as u64;
        Ok((target_x, target_y))
    }
    /**
    Returns (output_to_user, fee_to_protocol)
     */
    #[inline(never)]
    pub fn quote_x_to_y(
        &self,
        input_x: u64,
        current_x: u64,
        current_y: u64,
    ) -> Result<(u64, u64, u64)> {
        if input_x == 0 {
            return Ok((0u64, 0u64, 0u64));
        }

        let (target_x, _target_y) =
            self.get_target_xy(current_x, current_y)?;

        // 0. get target_x on curve-K
        let big_k = self.big_k;
        //target_x_K = sqrt(big_k / p), where p = mult_x / mult_y
        let target_x_k = big_k.checked_mul(self.mult_y as u128).unwrap().checked_div(self.mult_x as u128).unwrap().sqrt();

        // 1. find current (x,y) on curve-K
        let current_x_k = target_x_k.checked_sub(target_x as u128).unwrap().checked_add(current_x as u128).unwrap();
        let current_y_k = big_k.checked_div(current_x_k).unwrap();

        // 2. find new (x, y) on curve-K
        let new_x_k = current_x_k.checked_add(input_x as u128).unwrap();
        let new_y_k = big_k.checked_div(new_x_k).unwrap();

        let output_before_fee_y: u64 = current_y_k.checked_sub(new_y_k).unwrap().try_into().unwrap();
        if output_before_fee_y >= current_y{
            return Ok((0u64, 0u64, 0u64));
        }
        let fee_before_rebate_y = output_before_fee_y.checked_mul(self.fee_millionth).unwrap() / MILLION;
        let rebate_ratio =
            std::cmp::min(input_x, target_x.checked_sub(std::cmp::min(target_x, current_x)).unwrap()).checked_mul(100).unwrap() / input_x;
        let rebate_y = (fee_before_rebate_y.checked_mul(rebate_ratio).unwrap() / 100).checked_mul(self.rebate_percentage).unwrap() / 100;
        let fee_y = fee_before_rebate_y.checked_sub(rebate_y).unwrap();
        let output_after_fee_y = output_before_fee_y.checked_sub(fee_y).unwrap();

        let protocol_fee_y = fee_y.checked_mul(self.protocol_fee_share_thousandth).unwrap() / 1000;
        let lp_fee_y = fee_y.checked_sub(protocol_fee_y).unwrap();

        Ok((output_after_fee_y, protocol_fee_y, lp_fee_y))
    }

    /**
    Returns (output_to_user, fee_to_protocol, fee_to_reserve_x)
     */
    #[inline(never)]
    pub fn quote_y_to_x(
        &self,
        input_y: u64,
        current_x: u64,
        current_y: u64,
    ) -> Result<(u64, u64, u64)> {
        if input_y == 0 {
            return Ok((0u64, 0u64, 0u64));
        }

        let (target_x, target_y) =
            self.get_target_xy(current_x, current_y)?;

        // 0. get target_x on curve-K
        let big_k = self.big_k;
        //target_x_K = sqrt(big_k / p), where p = mult_x / mult_y
        let target_x_k = big_k.checked_mul(self.mult_y as u128).unwrap().checked_div(self.mult_x as u128).unwrap().sqrt();

        // 1. find current (x, y) on curve-K
        let current_x_k = target_x_k.checked_sub(target_x as u128).unwrap().checked_add(current_x as u128).unwrap();
        let current_y_k = big_k.checked_div(current_x_k).unwrap();

        // 2. find new (x, y) on curve-K
        let new_y_k = current_y_k.checked_add(input_y as u128).unwrap();
        let new_x_k = big_k.checked_div(new_y_k).unwrap();

        let output_before_fee_x: u64 = current_x_k.checked_sub(new_x_k).unwrap().try_into().unwrap();
        if output_before_fee_x >= current_x {
            return Ok((0u64, 0u64, 0u64));
        }

        let fee_before_rebate_x = output_before_fee_x.checked_mul(self.fee_millionth).unwrap() / MILLION;
        let rebate_ratio =
            std::cmp::min(input_y, target_y.checked_sub(std::cmp::min(target_y, current_y)).unwrap()).checked_mul(100).unwrap()  / input_y;
        let rebate_x = (fee_before_rebate_x.checked_mul(rebate_ratio).unwrap() / 100).checked_mul(self.rebate_percentage).unwrap() / 100;
        let fee_x = fee_before_rebate_x.checked_sub(rebate_x).unwrap();
        let output_after_fee_x = output_before_fee_x.checked_sub(fee_x).unwrap();

        let protocol_fee_x = fee_x.checked_mul(self.protocol_fee_share_thousandth).unwrap() / 1000;
        let lp_fee_x = fee_x.checked_sub(protocol_fee_x).unwrap();

        Ok((output_after_fee_x, protocol_fee_x, lp_fee_x))
    }
}