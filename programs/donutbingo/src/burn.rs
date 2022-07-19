use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::{Mint, TokenAccount};

pub fn bb_and_donuts<'info>(
    token_program: Program<'info, token::Token>,
    player: Signer<'info>,
    bb_mint: Box<Account<'info, Mint>>,
    d1_mint: Box<Account<'info, Mint>>,
    d2_mint: Box<Account<'info, Mint>>,
    d3_mint: Box<Account<'info, Mint>>,
    d4_mint: Box<Account<'info, Mint>>,
    d5_mint: Box<Account<'info, Mint>>,
    d6_mint: Box<Account<'info, Mint>>,
    d7_mint: Box<Account<'info, Mint>>,
    d8_mint: Box<Account<'info, Mint>>,
    d9_mint: Box<Account<'info, Mint>>,
    bb: Box<Account<'info, TokenAccount>>,
    d1: Box<Account<'info, TokenAccount>>,
    d2: Box<Account<'info, TokenAccount>>,
    d3: Box<Account<'info, TokenAccount>>,
    d4: Box<Account<'info, TokenAccount>>,
    d5: Box<Account<'info, TokenAccount>>,
    d6: Box<Account<'info, TokenAccount>>,
    d7: Box<Account<'info, TokenAccount>>,
    d8: Box<Account<'info, TokenAccount>>,
    d9: Box<Account<'info, TokenAccount>>,
) -> Result<()> {
    let _burn_bb_tx = anchor_spl::token::burn(
        CpiContext::new(
            token_program.to_account_info(),
            anchor_spl::token::Burn {
                mint: bb_mint.to_account_info(),
                from: bb.to_account_info(),
                authority: player.to_account_info(),
            },
        ),
        1,
    );

    let _burn_d1_tx = anchor_spl::token::burn(
        CpiContext::new(
            token_program.to_account_info(),
            anchor_spl::token::Burn {
                mint: d1_mint.to_account_info(),
                from: d1.to_account_info(),
                authority: player.to_account_info(),
            },
        ),
        1,
    );

    let _burn_d2_tx = anchor_spl::token::burn(
        CpiContext::new(
            token_program.to_account_info(),
            anchor_spl::token::Burn {
                mint: d2_mint.to_account_info(),
                from: d2.to_account_info(),
                authority: player.to_account_info(),
            },
        ),
        1,
    );

    let _burn_d3_tx = anchor_spl::token::burn(
        CpiContext::new(
            token_program.to_account_info(),
            anchor_spl::token::Burn {
                mint: d3_mint.to_account_info(),
                from: d3.to_account_info(),
                authority: player.to_account_info(),
            },
        ),
        1,
    );

    let _burn_d4_tx = anchor_spl::token::burn(
        CpiContext::new(
            token_program.to_account_info(),
            anchor_spl::token::Burn {
                mint: d4_mint.to_account_info(),
                from: d4.to_account_info(),
                authority: player.to_account_info(),
            },
        ),
        1,
    );

    let _burn_d5_tx = anchor_spl::token::burn(
        CpiContext::new(
            token_program.to_account_info(),
            anchor_spl::token::Burn {
                mint: d5_mint.to_account_info(),
                from: d5.to_account_info(),
                authority: player.to_account_info(),
            },
        ),
        1,
    );

    let _burn_d6_tx = anchor_spl::token::burn(
        CpiContext::new(
            token_program.to_account_info(),
            anchor_spl::token::Burn {
                mint: d6_mint.to_account_info(),
                from: d6.to_account_info(),
                authority: player.to_account_info(),
            },
        ),
        1,
    );

    let _burn_d7_tx = anchor_spl::token::burn(
        CpiContext::new(
            token_program.to_account_info(),
            anchor_spl::token::Burn {
                mint: d7_mint.to_account_info(),
                from: d7.to_account_info(),
                authority: player.to_account_info(),
            },
        ),
        1,
    );

    let _burn_d8_tx = anchor_spl::token::burn(
        CpiContext::new(
            token_program.to_account_info(),
            anchor_spl::token::Burn {
                mint: d8_mint.to_account_info(),
                from: d8.to_account_info(),
                authority: player.to_account_info(),
            },
        ),
        1,
    );

    let _burn_d9_tx = anchor_spl::token::burn(
        CpiContext::new(
            token_program.to_account_info(),
            anchor_spl::token::Burn {
                mint: d9_mint.to_account_info(),
                from: d9.to_account_info(),
                authority: player.to_account_info(),
            },
        ),
        1,
    );

    Ok(())
}
