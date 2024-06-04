pub mod consts;
pub mod errors;
pub mod state;

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::{
    errors::ObricError,
    state::{
        larix::{Obligation, Reserve},
        PriceFeed, SSTradingPair,
    },
};
declare_id!("4DDLcmzLRosAUgTNSHXDuAHmuE1CACA193L3QTPYyz9j");

#[program]
pub mod obric_solana_v3 {
    use super::*;

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

    #[account(mut)]
    pub mint_x_ctoken: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub mint_y_ctoken: Box<Account<'info, Mint>>,

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

    #[account(mut, address = trading_pair.reserve_x)]
    pub reserve_x: Box<Account<'info, TokenAccount>>,

    #[account(mut, address = trading_pair.reserve_y)]
    pub reserve_y: Box<Account<'info, TokenAccount>>,

    #[account(mut, address = trading_pair.reserve_x_ctoken)]
    pub reserve_x_ctoken: Box<Account<'info, TokenAccount>>,

    #[account(mut, address = trading_pair.reserve_y_ctoken)]
    pub reserve_y_ctoken: Box<Account<'info, TokenAccount>>,

    #[account(
    mut,
    address = trading_pair.protocol_fee_y
    )]
    pub protocol_fee_y: Box<Account<'info, TokenAccount>>,

    #[account(address = trading_pair.x_price_feed_id @ ObricError::InvalidPriceAccount)]
    pub x_price_feed: Box<Account<'info, PriceFeed>>,

    #[account(address = trading_pair.y_price_feed_id @ ObricError::InvalidPriceAccount)]
    pub y_price_feed: Box<Account<'info, PriceFeed>>,

    #[account(
    mut,
    constraint = larix_reserve_liquidity_supply_x.mint == mint_x.key(),
    constraint = larix_reserve_liquidity_supply_x.owner == consts::larix::market::authority::ID,
    address = larix_reserve_x.liquidity.supply_pubkey
    )]
    pub larix_reserve_liquidity_supply_x: Box<Account<'info, TokenAccount>>,

    #[account(
    mut,
    constraint = larix_reserve_liquidity_supply_y.mint == mint_y.key(),
    constraint = larix_reserve_liquidity_supply_y.owner == consts::larix::market::authority::ID,
    address = larix_reserve_y.liquidity.supply_pubkey
    )]
    pub larix_reserve_liquidity_supply_y: Box<Account<'info, TokenAccount>>,

    #[account(
    mut,
    constraint = larix_destination_reserve_ctoken_x.mint == mint_x_ctoken.key(),
    constraint = larix_destination_reserve_ctoken_x.owner == larix_market_authority.key(),
    address = larix_reserve_x.collateral.supply_pubkey
    )]
    pub larix_destination_reserve_ctoken_x: Box<Account<'info, TokenAccount>>,

    #[account(
    mut,
    constraint = larix_destination_reserve_ctoken_y.mint == mint_y_ctoken.key(),
    constraint = larix_destination_reserve_ctoken_y.owner == larix_market_authority.key(),
    address = larix_reserve_y.collateral.supply_pubkey
    )]
    pub larix_destination_reserve_ctoken_y: Box<Account<'info, TokenAccount>>,

    #[account(
    mut,
    address = consts::mint_to_larix_reserve(&mint_x.key())? @ ObricError::InvalidLarixReserveKey
    )]
    pub larix_reserve_x: Box<Account<'info, Reserve>>,

    #[account(
    mut,
    address = consts::mint_to_larix_reserve(&mint_y.key())? @ ObricError::InvalidLarixReserveKey
    )]
    pub larix_reserve_y: Box<Account<'info, Reserve>>,

    #[account(
    mut,
    seeds = [consts::LARIX_OBLIGATION_SEED.as_bytes(), mint_x.key().as_ref(), mint_y.key().as_ref()],
    bump
    )]
    pub larix_obligation: Box<Account<'info, Obligation>>,

    #[account(
    mut,
    address = consts::larix::market::ID,
    )]
    /// CHECK: larix lending market
    pub larix_lending_market: UncheckedAccount<'info>,

    #[account(
    address = consts::larix::market::authority::ID,
    )]
    /// CHECK: larix lending market authority
    pub larix_market_authority: UncheckedAccount<'info>,

    /// CHECK:
    pub larix_x_oracle: UncheckedAccount<'info>,

    /// CHECK:
    pub larix_y_oracle: UncheckedAccount<'info>,

    pub user: Signer<'info>,

    pub token_program: Program<'info, Token>,

    #[account(address = larix_lending::ID @ ObricError::InvalidLarixProgram)]
    /// CHECK: larix program
    pub larix_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct SwapYToX<'info> {
    #[account(mut)]
    pub trading_pair: Box<Account<'info, SSTradingPair>>,

    // pub mint_x: Box<Account<'info, Mint>>,
    // pub mint_y: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub mint_x_ctoken: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub mint_y_ctoken: Box<Account<'info, Mint>>,

    #[account(
    mut,
    constraint = user_token_account_x.owner == user.key(),
    constraint = user_token_account_x.mint == trading_pair.mint_x,
    )]
    pub user_token_account_x: Box<Account<'info, TokenAccount>>,

    #[account(
    mut,
    constraint = user_token_account_y.owner == user.key(),
    constraint = user_token_account_y.mint == trading_pair.mint_y,
    )]
    pub user_token_account_y: Box<Account<'info, TokenAccount>>,

    #[account(mut, address = trading_pair.reserve_x)]
    pub reserve_x: Box<Account<'info, TokenAccount>>,

    #[account(mut, address = trading_pair.reserve_y)]
    pub reserve_y: Box<Account<'info, TokenAccount>>,

    #[account(mut, address = trading_pair.reserve_x_ctoken)]
    pub reserve_x_ctoken: Box<Account<'info, TokenAccount>>,

    #[account(mut, address = trading_pair.reserve_y_ctoken)]
    pub reserve_y_ctoken: Box<Account<'info, TokenAccount>>,

    #[account(
    mut,
    address = trading_pair.protocol_fee_x
    )]
    pub protocol_fee_x: Box<Account<'info, TokenAccount>>,

    #[account(address = trading_pair.x_price_feed_id @ ObricError::InvalidPriceAccount)]
    pub x_price_feed: Box<Account<'info, PriceFeed>>,

    #[account(address = trading_pair.y_price_feed_id @ ObricError::InvalidPriceAccount)]
    pub y_price_feed: Box<Account<'info, PriceFeed>>,

    #[account(
    mut,
    address = larix_reserve_x.liquidity.supply_pubkey,
    )]
    pub larix_reserve_liquidity_supply_x: Box<Account<'info, TokenAccount>>,

    #[account(
    mut,
    address = larix_reserve_y.liquidity.supply_pubkey
    )]
    pub larix_reserve_liquidity_supply_y: Box<Account<'info, TokenAccount>>,

    #[account(
    mut,
    address = larix_reserve_x.collateral.supply_pubkey
    )]
    pub larix_destination_reserve_ctoken_x: Box<Account<'info, TokenAccount>>,

    #[account(
    mut,
    address = larix_reserve_y.collateral.supply_pubkey
    )]
    pub larix_destination_reserve_ctoken_y: Box<Account<'info, TokenAccount>>,

    #[account(
    mut,
    address = consts::mint_to_larix_reserve(&trading_pair.mint_x)? @ ObricError::InvalidLarixReserveKey
    )]
    pub larix_reserve_x: Box<Account<'info, Reserve>>,

    #[account(
    mut,
    address = consts::mint_to_larix_reserve(&trading_pair.mint_y)? @ ObricError::InvalidLarixReserveKey
    )]
    pub larix_reserve_y: Box<Account<'info, Reserve>>,

    #[account(
    mut,
    seeds = [consts::LARIX_OBLIGATION_SEED.as_bytes(), trading_pair.mint_x.as_ref(), trading_pair.mint_y.key().as_ref()],
    bump
    )]
    pub larix_obligation: Box<Account<'info, Obligation>>,

    #[account(
    mut,
    address = consts::larix::market::ID,
    )]
    /// CHECK: larix market
    pub larix_lending_market: UncheckedAccount<'info>,

    #[account(
    address = consts::larix::market::authority::ID,
    )]
    /// CHECK: larix market authority
    pub larix_market_authority: UncheckedAccount<'info>,

    /// CHECK:
    pub larix_x_oracle: UncheckedAccount<'info>,

    /// CHECK:
    pub larix_y_oracle: UncheckedAccount<'info>,

    pub user: Signer<'info>,

    pub token_program: Program<'info, Token>,

    #[account(address = larix_lending::ID @ ObricError::InvalidLarixProgram)]
    /// CHECK: larix program
    pub larix_program: UncheckedAccount<'info>,

    #[account(address = consts::larix::oracle::ID)]
    /// CHECK: larix oracle program
    pub larix_oracle_program: UncheckedAccount<'info>,

    #[account(address = consts::mints::larix::ID)]
    pub larix_mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    /// CHECK: not an existing account
    pub larix_reserve_fee_receiver_x: UncheckedAccount<'info>,
}
