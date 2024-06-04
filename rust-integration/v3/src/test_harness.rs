use anyhow::Result;
use jupiter_amm_interface::{Amm, KeyedAccount, QuoteParams};
use solana_client::rpc_client::RpcClient;
use std::collections::HashMap;
use std::env;

use crate::constants::PROGRAM_ID;
use crate::obric_v3_amm::ObricV3Amm;

pub struct AmmTestHarness {
    pub client: RpcClient,
}

impl AmmTestHarness {
    pub fn new() -> Self {
        let rpc_string = env::var("SOLANA_RPC").unwrap();
        let rpc_url = rpc_string.as_str();
        Self {
            client: RpcClient::new(rpc_url),
        }
    }
    pub fn get_all_keyed_account(&self) -> Result<Vec<KeyedAccount>> {
        let accounts = self.client.get_program_accounts(&PROGRAM_ID).unwrap();
        let keyed_accounts = &mut vec![];
        for (key, account) in accounts {
            if account.data.len() == 762usize {
                keyed_accounts.push(KeyedAccount {
                    key,
                    account,
                    params: None,
                })
            }
        }
        Ok(keyed_accounts.clone())
    }

    pub fn update_amm(&self, amm: &mut dyn Amm) {
        let accounts_to_update = amm.get_accounts_to_update();

        let accounts_map = self
            .client
            .get_multiple_accounts(&accounts_to_update)
            .unwrap()
            .iter()
            .enumerate()
            .fold(HashMap::new(), |mut m, (index, account)| {
                if let Some(account) = account {
                    m.insert(accounts_to_update[index], account.clone());
                }
                m
            });
        amm.update(&accounts_map).unwrap();
    }
}

#[test]
fn test_quote() {
    use crate::test_harness::AmmTestHarness;
    use num::pow;

    let test_harness = AmmTestHarness::new();
    let all_keyed_account = test_harness.get_all_keyed_account().unwrap();

    for keyed_account in all_keyed_account {
        let amm = &mut ObricV3Amm::from_keyed_account(&keyed_account).unwrap();
        test_harness.update_amm(amm);

        println!("Pool: {}, {}", amm.state.mint_x, amm.state.mint_y);

        let in_amount = pow(10, usize::from(amm.state.decimals_x));
        let quote = amm
            .quote(&QuoteParams {
                input_mint: amm.state.mint_x,
                in_amount,
                output_mint: amm.state.mint_y,
            })
            .unwrap();

        println!(
            "  Token mints: from {}, to {}",
            amm.state.mint_x, amm.state.mint_y
        );
        println!("  In amount: {}", in_amount);
        println!(
            "  Out amount: {:?}, Fee amount: {:?}",
            quote.out_amount, quote.fee_amount
        );

        let in_amount = pow(10, usize::from(amm.state.decimals_y));
        let quote = amm
            .quote(&QuoteParams {
                input_mint: amm.state.mint_y,
                in_amount,
                output_mint: amm.state.mint_x,
            })
            .unwrap();

        println!(
            "\n  Token mints: from {}, to {}",
            amm.state.mint_y, amm.state.mint_x
        );
        println!("  In amount: {}", in_amount);
        println!(
            "  Out amount: {:?}, Fee amount: {:?}\n",
            quote.out_amount, quote.fee_amount
        );
    }
}
