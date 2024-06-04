use crate::constants::PROGRAM_ID;
use anchor_lang::prelude::Pubkey;
use anchor_lang::AccountDeserialize;
use anyhow::Result;
use jupiter_amm_interface::{
    Amm, KeyedAccount, Quote, QuoteParams, Swap, SwapAndAccountMetas, SwapParams,
};
use larix_lending::state::reserve::Reserve;
use obric_solana_v3::consts;
use obric_solana_v3::state::PriceFeed;
use obric_solana_v3::state::SSTradingPair;
use solana_sdk::account::Account;
use solana_sdk::instruction::AccountMeta;
use std::collections::HashMap;

pub struct ObricV3Amm {
    pub key: Pubkey,
    pub state: SSTradingPair,
    pub obligation: Pubkey,
    pub larix_reserve_x: Option<Reserve>,
    pub larix_reserve_y: Option<Reserve>,
}

impl Amm for ObricV3Amm {
    fn label(&self) -> String {
        return String::from("Obric v3");
    }

    fn key(&self) -> Pubkey {
        return self.key;
    }

    fn from_keyed_account(keyed_account: &KeyedAccount) -> Result<Self> {
        let data = &mut &keyed_account.account.data.clone()[0..];
        let ss_trading_pair = SSTradingPair::try_deserialize(data).unwrap();
        let (obligation, _) = Pubkey::find_program_address(
            &[
                consts::LARIX_OBLIGATION_SEED.as_bytes(),
                ss_trading_pair.mint_x.to_bytes().as_ref(),
                ss_trading_pair.mint_y.to_bytes().as_ref(),
            ],
            &PROGRAM_ID,
        );
        Ok(Self {
            key: keyed_account.key,
            state: ss_trading_pair,
            obligation,
            larix_reserve_x: None,
            larix_reserve_y: None,
        })
    }

    fn get_reserve_mints(&self) -> Vec<Pubkey> {
        return [self.state.mint_x, self.state.mint_y].to_vec();
    }

    fn has_dynamic_accounts(&self) -> bool {
        true
    }

    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        if self.larix_reserve_y.is_none() {
            return [
                self.key,
                self.state.x_price_feed_id,
                self.state.y_price_feed_id,
                consts::mint_to_larix_reserve(&self.state.mint_x).unwrap(),
                consts::mint_to_larix_reserve(&self.state.mint_y).unwrap(),
            ]
            .to_vec();
        } else {
            return [
                self.key,
                self.state.x_price_feed_id,
                self.state.y_price_feed_id,
            ]
            .to_vec();
        }
    }

    fn update(&mut self, accounts_map: &HashMap<Pubkey, Account>) -> Result<()> {
        let trading_pair_data = &mut &accounts_map.get(&self.key).unwrap().data[0..];
        let trading_pair = SSTradingPair::try_deserialize(trading_pair_data).unwrap();
        self.state = trading_pair;

        let price_x_data = &mut &accounts_map.get(&self.state.x_price_feed_id).unwrap().data[0..];
        let price_y_data = &mut &accounts_map.get(&self.state.y_price_feed_id).unwrap().data[0..];
        let price_x_fee = &PriceFeed::try_deserialize(price_x_data).unwrap();
        let price_y_fee = &PriceFeed::try_deserialize(price_y_data).unwrap();
        let price_x = price_x_fee.price_normalized()?.price as u64;
        let price_y = price_y_fee.price_normalized()?.price as u64;
        self.state.update_price(price_x, price_y)?;
        let target_y = self.state.compute_target_y();
        let _ = self.state.update_target_y(target_y)?;
        Ok(())
    }

    fn quote(&self, quote_params: &QuoteParams) -> Result<Quote> {
        let (output_after_fee, protocol_fee, _) = if quote_params.input_mint.eq(&self.state.mint_x)
        {
            self.state
                .quote_x_to_y(quote_params.in_amount.clone())
                .unwrap()
        } else if quote_params.input_mint.eq(&self.state.mint_y) {
            self.state
                .quote_y_to_x(quote_params.in_amount.clone())
                .unwrap()
        } else {
            (0u64, 0u64, 0u64)
        };
        Ok(Quote {
            out_amount: output_after_fee,
            fee_amount: protocol_fee,
            fee_mint: quote_params.output_mint,
            ..Quote::default()
        })
    }

    fn clone_amm(&self) -> Box<dyn Amm + Send + Sync> {
        let state = &self.state;
        Box::new(Self {
            key: self.key,
            state: SSTradingPair {
                is_initialized: state.is_initialized,
                x_price_feed_id: state.x_price_feed_id,
                y_price_feed_id: state.y_price_feed_id,
                reserve_x: state.reserve_x,
                reserve_y: state.reserve_y,
                reserve_x_ctoken: state.reserve_x_ctoken,
                reserve_y_ctoken: state.reserve_y_ctoken,

                protocol_fee_x: state.protocol_fee_x,
                protocol_fee_y: state.protocol_fee_y,
                bump: state.bump,
                mint_x: state.mint_x,
                mint_y: state.mint_y,

                deposit_x: state.deposit_x,
                borrow_x: state.borrow_x,

                deposit_y: state.deposit_y,
                borrow_y: state.borrow_y,

                target_y: state.target_y,

                concentration: state.concentration,
                big_k: state.big_k,
                cumulative_volume: state.cumulative_volume,
                mult_x: state.mult_x,
                mult_y: state.mult_y,
                fee_millionth: state.fee_millionth,
                rebate_percentage: state.rebate_percentage,
                protocol_fee_share_thousandth: state.protocol_fee_share_thousandth,
                decimals_x: state.decimals_x,
                decimals_y: state.decimals_y,
                volume_records: state.volume_records,
                padding: state.padding,
                volume_time_records: state.volume_time_records,
                padding2: state.padding2,
            },
            obligation: self.obligation,
            larix_reserve_x: self.larix_reserve_x.clone(),
            larix_reserve_y: self.larix_reserve_y.clone(),
        })
    }

    fn program_id(&self) -> Pubkey {
        PROGRAM_ID
    }

    fn get_swap_and_account_metas(&self, swap_params: &SwapParams) -> Result<SwapAndAccountMetas> {
        let (user_token_account_x, user_token_account_y, protocol_fee) =
            if swap_params.source_mint.eq(&self.state.mint_x) {
                (
                    swap_params.source_token_account,
                    swap_params.destination_token_account,
                    self.state.protocol_fee_y,
                )
            } else {
                (
                    swap_params.destination_token_account,
                    swap_params.source_token_account,
                    self.state.protocol_fee_x,
                )
            };
        let larix_reserve_x = &self.larix_reserve_x.as_ref().unwrap();
        let larix_reserve_y = &self.larix_reserve_y.as_ref().unwrap();
        let mut account_metas = vec![
            AccountMeta::new(self.key(), false),
            AccountMeta::new_readonly(self.state.mint_x, false),
            AccountMeta::new_readonly(self.state.mint_y, false),
            AccountMeta::new(larix_reserve_x.collateral.mint_pubkey, false),
            AccountMeta::new(larix_reserve_y.collateral.mint_pubkey, false),
            AccountMeta::new(user_token_account_x, false),
            AccountMeta::new(user_token_account_y, false),
            AccountMeta::new(self.state.reserve_x, false),
            AccountMeta::new(self.state.reserve_y, false),
            AccountMeta::new(self.state.reserve_x_ctoken, false),
            AccountMeta::new(self.state.reserve_y_ctoken, false),
            AccountMeta::new(protocol_fee, false),
            AccountMeta::new_readonly(self.state.x_price_feed_id, false),
            AccountMeta::new_readonly(self.state.y_price_feed_id, false),
            AccountMeta::new(larix_reserve_x.liquidity.supply_pubkey, false),
            AccountMeta::new(larix_reserve_y.liquidity.supply_pubkey, false),
            AccountMeta::new(larix_reserve_x.collateral.supply_pubkey, false),
            AccountMeta::new(larix_reserve_y.collateral.supply_pubkey, false),
            AccountMeta::new(
                consts::mint_to_larix_reserve(&self.state.mint_x).unwrap(),
                false,
            ),
            AccountMeta::new(
                consts::mint_to_larix_reserve(&self.state.mint_y).unwrap(),
                false,
            ),
            AccountMeta::new(self.obligation, false),
            AccountMeta::new(larix_reserve_x.lending_market, false),
            AccountMeta::new(consts::larix::market::authority::id(), false),
            AccountMeta::new_readonly(larix_reserve_x.liquidity.params_2, false),
            AccountMeta::new_readonly(larix_reserve_y.liquidity.params_2, false),
            AccountMeta::new_readonly(swap_params.token_transfer_authority, true),
            AccountMeta::new_readonly(anchor_spl::token::spl_token::id(), false),
            AccountMeta::new_readonly(larix_lending::id(), false),
        ];
        if swap_params.source_mint.eq(&self.state.mint_y) {
            account_metas.push(AccountMeta::new_readonly(
                consts::larix::oracle::id(),
                false,
            ));
            account_metas.push(AccountMeta::new_readonly(consts::mints::larix::id(), false));
            account_metas.push(AccountMeta::new(
                larix_reserve_x.liquidity.fee_receiver,
                false,
            ))
        }
        Ok(SwapAndAccountMetas {
            swap: Swap::Saber,
            account_metas,
        })
    }
}
