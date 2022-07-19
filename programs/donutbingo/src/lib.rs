use anchor_lang::prelude::*;
use anchor_spl::token::{
    Mint, /*MintTo,
          Token,*/
    TokenAccount,
};
mod bingolist;
mod burn;
use anchor_spl::token;
use solana_program::pubkey::Pubkey;

declare_id!("5xoiEAwcXa7drut8TFDPtYnoyAkyUZNgZTrRxE3koJY7");

#[program]
pub mod donutbingo {

    use super::*;

    pub fn bingo(
        ctx: Context<BingoContext>,
        _rules_bump: u8,
        _treasury_bump: u8,
        bingo_bump: u8,
    ) -> Result<()> {
        msg!("Verifying selected donuts");
        if !bingolist::validate_bb_and_donuts(
            ctx.accounts.rules.clone(),
            ctx.accounts.bb_mint.key(),
            ctx.accounts.d1_mint.key(),
            ctx.accounts.d2_mint.key(),
            ctx.accounts.d3_mint.key(),
            ctx.accounts.d4_mint.key(),
            ctx.accounts.d5_mint.key(),
            ctx.accounts.d6_mint.key(),
            ctx.accounts.d7_mint.key(),
            ctx.accounts.d8_mint.key(),
            ctx.accounts.d9_mint.key(),
        ) {
            return Err(DonutErrorCode::WrongCombination.into());
        }

        msg!("Burning donuts + BB");
        let _burn_tx = burn::bb_and_donuts(
            ctx.accounts.token_program.clone(),
            ctx.accounts.player.clone(),
            ctx.accounts.bb_mint.clone(),
            ctx.accounts.d1_mint.clone(),
            ctx.accounts.d2_mint.clone(),
            ctx.accounts.d3_mint.clone(),
            ctx.accounts.d4_mint.clone(),
            ctx.accounts.d5_mint.clone(),
            ctx.accounts.d6_mint.clone(),
            ctx.accounts.d7_mint.clone(),
            ctx.accounts.d8_mint.clone(),
            ctx.accounts.d9_mint.clone(),
            ctx.accounts.bb.clone(),
            ctx.accounts.d1.clone(),
            ctx.accounts.d2.clone(),
            ctx.accounts.d3.clone(),
            ctx.accounts.d4.clone(),
            ctx.accounts.d5.clone(),
            ctx.accounts.d6.clone(),
            ctx.accounts.d7.clone(),
            ctx.accounts.d8.clone(),
            ctx.accounts.d9.clone(),
        );

        msg!("Rewarding 9.bit Membership");
        let _reward_player_ = anchor_spl::token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.treasury.to_account_info(),
                    to: ctx.accounts.user_reward_account.to_account_info(),
                    authority: ctx.accounts.instance.to_account_info(),
                },
                &[&[b"bingo".as_ref(), &[bingo_bump]]],
            ),
            1,
        );

        Ok(())
    }

    pub fn fill(ctx: Context<RewardsContext>, bingo_bump: u8) -> Result<()> {
        msg!("Treasury: {:?} ", ctx.accounts.treasury);
        msg!("Bumps: {:?} ", ctx.bumps.into_iter());

        let rules: &mut Account<BBRules> = &mut ctx.accounts.rules;
        rules.bump = bingo_bump;
        rules.bb_mint = ctx.accounts.bb_mint.key();
        rules.d1_mint = ctx.accounts.d1_mint.key();
        rules.d2_mint = ctx.accounts.d2_mint.key();
        rules.d3_mint = ctx.accounts.d3_mint.key();
        rules.d4_mint = ctx.accounts.d4_mint.key();
        rules.d5_mint = ctx.accounts.d5_mint.key();
        rules.d6_mint = ctx.accounts.d6_mint.key();
        rules.d7_mint = ctx.accounts.d7_mint.key();
        rules.d8_mint = ctx.accounts.d8_mint.key();
        rules.d9_mint = ctx.accounts.d9_mint.key();

        let _fill_treasury_tx = anchor_spl::token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.reward_account.to_account_info(),
                    to: ctx.accounts.treasury.to_account_info(),
                    authority: ctx.accounts.creator.to_account_info(),
                },
            ),
            1,
        );
        msg!("Membership added to treasury!");
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bingo_bump: u8)]
pub struct RewardsContext<'info> {
    /// CHECK:  
    #[account(
        mut,
        seeds=[
            b"bingo".as_ref(),
        ],
        bump = bingo_bump
    )]
    pub instance: AccountInfo<'info>,

    #[account(
        init,
        payer = creator,
        space = 8 + 1 + 10 * 32,
        seeds=[
            b"rules".as_ref(),
            bb_mint.key().as_ref()
        ],
        bump,
    )]
    pub rules: Account<'info, BBRules>,

    #[account(
        init,
        payer = creator,
        seeds = [
            b"treasury".as_ref(),
            reward.key().as_ref()
        ],
        bump,
        token::mint = reward,
        token::authority = instance
    )]
    pub treasury: Account<'info, TokenAccount>,

    #[account(
        mut,
        address = bingolist::get_admin() @ DonutErrorCode::WrongCombination
    )]
    pub creator: Signer<'info>,
    #[account(mut)]
    pub reward_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub reward: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub bb_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub d1_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub d2_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub d3_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub d4_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub d5_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub d6_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub d7_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub d8_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub d9_mint: Box<Account<'info, Mint>>,

    // System
    pub token_program: Program<'info, token::Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
pub struct BBRules {
    pub bump: u8,
    pub bb_mint: Pubkey,
    pub d1_mint: Pubkey,
    pub d2_mint: Pubkey,
    pub d3_mint: Pubkey,
    pub d4_mint: Pubkey,
    pub d5_mint: Pubkey,
    pub d6_mint: Pubkey,
    pub d7_mint: Pubkey,
    pub d8_mint: Pubkey,
    pub d9_mint: Pubkey,
}

#[derive(Accounts)]
#[instruction(rules_bump: u8, treasury_bump: u8, bingo_bump: u8)]

pub struct BingoContext<'info> {
    /// CHECK:  
    #[account(
        mut,
        seeds=[
            b"bingo".as_ref(),
        ],
        bump = bingo_bump
    )]
    pub instance: AccountInfo<'info>,

    #[account(
        seeds=[
            b"rules".as_ref(),
            bb_mint.key().as_ref()
        ],
        bump = rules_bump,
    )]
    pub rules: Box<Account<'info, BBRules>>,

    #[account(
        mut,
        seeds = [
            b"treasury".as_ref(),
            reward_mint.key().as_ref()
        ],
        bump = treasury_bump,

    )]
    pub treasury: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub bb: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub d1: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub d2: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub d3: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub d4: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub d5: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub d6: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub d7: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub d8: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub d9: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub bb_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub d1_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub d2_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub d3_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub d4_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub d5_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub d6_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub d7_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub d8_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub d9_mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub player: Signer<'info>,

    pub token_program: Program<'info, token::Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,

    #[account(mut)]
    pub reward_mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub user_reward_account: Box<Account<'info, TokenAccount>>,
}

#[error_code]
pub enum DonutErrorCode {
    #[msg("Wrong combination of BB and Donuts, please check collection")]
    WrongCombination,
}
