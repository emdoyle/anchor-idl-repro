use anchor_lang::prelude::*;

#[account]
pub struct InitializeData {
    pub name: String,
    pub owner: Pubkey,
}

impl InitializeData {
    pub const MAX_SIZE: usize = 64;
}
