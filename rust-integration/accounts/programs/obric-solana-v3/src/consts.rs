use anchor_lang::prelude::*;

use crate::errors::ObricError;

pub mod admin {
    use anchor_lang::declare_id;
    declare_id!("obrrzdC5QzRHopGwe8jqUWkKa73AVLyAoEgRpa9V13i");
}

pub mod mints {
    pub mod sol {
        use anchor_lang::declare_id;
        declare_id!("So11111111111111111111111111111111111111112");
    }
    pub mod usdc {
        use anchor_lang::declare_id;
        declare_id!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
    }
    pub mod usdt {
        use anchor_lang::declare_id;
        declare_id!("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB");
    }
    pub mod stsol {
        use anchor_lang::declare_id;
        declare_id!("7dHbWXmci3dT8UFYWYZweBLXgycu7Y3iL6trKn1Y7ARj");
    }
    pub mod msol {
        use anchor_lang::declare_id;
        declare_id!("mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So");
    }
    pub mod eth_portal {
        use anchor_lang::declare_id;
        declare_id!("7vfCXTUXx5WJV5JADk17DUJ4ksgau7utNKj4b963voxs");
    }

    pub mod larix {
        use anchor_lang::declare_id;
        declare_id!("Lrxqnh6ZHKbGy3dcrCED43nsoLkM1LTzU2jRfWe8qUC");
    }
}
/*
pub mod solend_market {
    use anchor_lang::declare_id;
    declare_id!("4UpD2fh7xH3VP9QQaXtsS1YY3bxzWhtfpks7FatyKvdY");
}

pub mod solend_market_authority {
    use anchor_lang::declare_id;
    declare_id!("DdZR6zRFiUt4S5mg7AV1uKB2z1f1WzcNYCaTEEWPAuby");
}

pub mod solend_reserves {
    pub mod main_pool {
        pub mod sol {
            use anchor_lang::declare_id;
            declare_id!("8PbodeaosQP19SjYFx855UMqWxH2HynZLdBXmsrbac36");
        }
        pub mod usdc {
            use anchor_lang::declare_id;
            declare_id!("BgxfHJDzm44T7XG68MYKx7YisTjZu73tVovyZSjJMpmw");
        }
        pub mod usdt {
            use anchor_lang::declare_id;
            declare_id!("8K9WC8xoh2rtQNY7iEGXtPvfbDCi563SdWhCAhuMP2xE");
        }
        pub mod stsol {
            use anchor_lang::declare_id;
            declare_id!("5sjkv6HD8wycocJ4tC4U36HHbvgcXYqcyiPRUkncnwWs");
        }
        pub mod msol {
            use anchor_lang::declare_id;
            declare_id!("CCpirWrgNuBVLdkP2haxLTbD6XqEgaYuVXixbbpxUB6");
        }
        pub mod eth_portal {
            use anchor_lang::declare_id;
            declare_id!("CPDiKagfozERtJ33p7HHhEfJERjvfk1VAjMXAFLrvrKP");
        }
    }
}

pub fn mint_to_solend_reserve (mint_ref: &Pubkey) -> Result<Pubkey>{
    let mint = *mint_ref;
    if mint == mints::sol::ID {
        Ok(solend_reserves::main_pool::sol::ID)
    } else if mint == mints::usdc::ID {
        Ok(solend_reserves::main_pool::usdc::ID)
    } else if mint == mints::usdt::ID {
        Ok(solend_reserves::main_pool::usdt::ID)
    } else if mint == mints::stsol::ID {
        Ok(solend_reserves::main_pool::stsol::ID)
    } else if mint == mints::msol::ID {
        Ok(solend_reserves::main_pool::msol::ID)
    } else if mint == mints::eth_portal::ID {
        Ok(solend_reserves::main_pool::eth_portal::ID)

    } else {
        err!(ObricError::NoLarixReserveFoundForMint)
    }
}
*/

pub const TRADING_PAIR_SEED: &str = "trading_pair";
pub const FEE_RECORDS_SEED: &str = "fee_records";

pub const MILLION: u64 = 1000000;

pub const SOLEND_OBLIGATION_SPACE: usize = 1300;

pub const LARIX_OBLIGATION_SEED: &str = "larix_obligation";
pub const LARIX_OBLIGATION_SPACE: usize = 1092;

// TODO: update larix ids
pub mod larix {
    pub mod market {
        use anchor_lang::declare_id;
        declare_id!("5geyZJdffDBNoMqEbogbPvdgH9ue7NREobtW8M3C1qfe");
        pub mod authority {
            use anchor_lang::declare_id;
            declare_id!("BxnUi6jyYbtEEgkBq4bPLKzDpSfWVAzgyf3TF2jfC1my");
        }
    }

    pub mod reserves {
        pub mod main_pool {
            pub mod sol {
                use anchor_lang::declare_id;
                declare_id!("2RcrbkGNcfy9mbarLCCRYdW3hxph7pSbP38x35MR2Bjt");
            }
            pub mod usdc {
                use anchor_lang::declare_id;
                declare_id!("Emq1qT9MyyB5eHfftF5thYme84hoEwh4TCjm31K2Xxif");
            }
            pub mod usdt {
                use anchor_lang::declare_id;
                declare_id!("DC832AzxQMGDaVLGiRQfRCkyXi6PUPjQyQfMbVRRjtKA");
            }
            /*
            pub mod stsol {
                use anchor_lang::declare_id;
                declare_id!("5sjkv6HD8wycocJ4tC4U36HHbvgcXYqcyiPRUkncnwWs");
            }
            pub mod msol {
                use anchor_lang::declare_id;
                declare_id!("CCpirWrgNuBVLdkP2haxLTbD6XqEgaYuVXixbbpxUB6");
            }
            pub mod eth_portal {
                use anchor_lang::declare_id;
                declare_id!("CPDiKagfozERtJ33p7HHhEfJERjvfk1VAjMXAFLrvrKP");
            }
            */
        }
    }

    pub mod oracle {
        use anchor_lang::declare_id;
        declare_id!("GMjBguH3ceg9wAHEMdY5iZnvzY6CgBACBDvkWmjR7upS");
    }
}

pub fn mint_to_larix_reserve(mint_ref: &Pubkey) -> Result<Pubkey> {
    let mint = *mint_ref;
    if mint == mints::sol::ID {
        Ok(larix::reserves::main_pool::sol::ID)
    } else if mint == mints::usdc::ID {
        Ok(larix::reserves::main_pool::usdc::ID)
    } else if mint == mints::usdt::ID {
        Ok(larix::reserves::main_pool::usdt::ID)
    } else {
        err!(ObricError::NoLarixReserveFoundForMint)
    }
}
