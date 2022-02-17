use anchor_lang::prelude::*;
use crate::package2::InitializeData;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = InitializeData::MAX_SIZE + 8)]
    pub initialize_data_account: Account<'info, InitializeData>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}