use crate::BBRules;
use anchor_lang::prelude::*;

pub fn get_admin() -> Pubkey {
    return Pubkey::new(b"5xoiEAwcXa7drut8TFDPtYnoyAkyUZNgZTrRxE3koJY7");
}

pub fn validate_bb_and_donuts(
    rules: Box<Account<BBRules>>,
    bb_mint: Pubkey,
    d1_mint: Pubkey,
    d2_mint: Pubkey,
    d3_mint: Pubkey,
    d4_mint: Pubkey,
    d5_mint: Pubkey,
    d6_mint: Pubkey,
    d7_mint: Pubkey,
    d8_mint: Pubkey,
    d9_mint: Pubkey,
) -> bool {
    if bb_mint.key() != rules.bb_mint {
        return false;
    }
    if d1_mint.key() != rules.d1_mint {
        return false;
    }
    if d2_mint.key() != rules.d2_mint {
        return false;
    }
    if d3_mint.key() != rules.d3_mint {
        return false;
    }
    if d4_mint.key() != rules.d4_mint {
        return false;
    }
    if d5_mint.key() != rules.d5_mint {
        return false;
    }
    if d6_mint.key() != rules.d6_mint {
        return false;
    }
    if d7_mint.key() != rules.d7_mint {
        return false;
    }
    if d8_mint.key() != rules.d8_mint {
        return false;
    }
    if d9_mint.key() != rules.d9_mint {
        return false;
    }
    return true;
}
