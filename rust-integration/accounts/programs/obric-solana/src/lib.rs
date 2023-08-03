pub mod state;
pub mod errors;
pub mod consts;

use anchor_lang::prelude::*;
use crate::state::{PriceFeed, SSTradingPair};
use crate::errors::ObricError;
use anchor_spl::token::{Mint, Token, TokenAccount};


declare_id!("AYBqaywuDVd3SYQkrLLJ27WRin5MrfJdQNHZGgc4LmHA");

#[program]
pub mod obric_solana {
    use super::*;
    use crate::{SwapXToY, SwapYToX};

    pub fn swap_x_to_y(_ctx: Context<SwapXToY>, _input_x: u64, _min_output_amt: u64) -> Result<()> {

        Ok(())
    }

    pub fn swap_y_to_x(_ctx: Context<SwapYToX>, _input_y: u64, _min_output_amt: u64) -> Result<()> {
        Ok(())
    }
}
#[derive(Accounts)]
pub struct SwapXToY<'info> {
    #[account(
    mut,
    seeds = [consts::TRADING_PAIR_SEED.as_bytes(), mint_x.key().as_ref(), mint_y.key().as_ref()],
    bump = trading_pair.bump
    )]
    pub trading_pair: Box<Account<'info, SSTradingPair>>,

    pub mint_x: Box<Account<'info, Mint>>,

    pub mint_y: Box<Account<'info, Mint>>,

    #[account(mut, address = trading_pair.reserve_x)]
    pub reserve_x: Box<Account<'info, TokenAccount>>,

    #[account(mut, address = trading_pair.reserve_y)]
    pub reserve_y: Box<Account<'info, TokenAccount>>,

    #[account(
    mut,
    constraint = user_token_account_x.owner == user.key(),
    constraint = user_token_account_x.mint == mint_x.key(),
    )]
    pub user_token_account_x: Box<Account<'info, TokenAccount>>,

    #[account(
    mut,
    constraint = user_token_account_y.owner == user.key(),
    constraint = user_token_account_y.mint == mint_y.key(),
    )]
    pub user_token_account_y: Box<Account<'info, TokenAccount>>,

    #[account(
    mut,
    address = trading_pair.protocol_fee_y
    )]
    pub protocol_fee_y: Box<Account<'info, TokenAccount>>,

    #[account(address = trading_pair.x_price_feed_id @ ObricError::InvalidPriceAccount)]
    pub x_price_feed: Box<Account<'info, PriceFeed>>,

    #[account(address = trading_pair.y_price_feed_id @ ObricError::InvalidPriceAccount)]
    pub y_price_feed: Box<Account<'info, PriceFeed>>,

    pub user: Signer<'info>,

    pub token_program: Program<'info, Token>,
}
#[derive(Accounts)]
pub struct SwapYToX<'info> {
    #[account(
    mut,
    seeds = [consts::TRADING_PAIR_SEED.as_bytes(), mint_x.key().as_ref(), mint_y.key().as_ref()],
    bump = trading_pair.bump
    )]
    pub trading_pair: Box<Account<'info, SSTradingPair>>,

    pub mint_x: Box<Account<'info, Mint>>,

    pub mint_y: Box<Account<'info, Mint>>,

    #[account(mut, address = trading_pair.reserve_x)]
    pub reserve_x: Box<Account<'info, TokenAccount>>,

    #[account(mut, address = trading_pair.reserve_y)]
    pub reserve_y: Box<Account<'info, TokenAccount>>,

    #[account(
    mut,
    constraint = user_token_account_x.owner == user.key(),
    constraint = user_token_account_x.mint == mint_x.key(),
    )]
    pub user_token_account_x: Box<Account<'info, TokenAccount>>,

    #[account(
    mut,
    constraint = user_token_account_y.owner == user.key(),
    constraint = user_token_account_y.mint == mint_y.key(),
    )]
    pub user_token_account_y: Box<Account<'info, TokenAccount>>,

    #[account(
    mut,
    address = trading_pair.protocol_fee_x
    )]
    pub protocol_fee_x: Box<Account<'info, TokenAccount>>,

    #[account(address = trading_pair.x_price_feed_id @ ObricError::InvalidPriceAccount)]
    pub x_price_feed: Box<Account<'info, PriceFeed>>,

    #[account(address = trading_pair.y_price_feed_id @ ObricError::InvalidPriceAccount)]
    pub y_price_feed: Box<Account<'info, PriceFeed>>,

    pub user: Signer<'info>,

    pub token_program: Program<'info, Token>,
}
