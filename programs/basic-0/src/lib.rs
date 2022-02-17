mod package1;
mod package2;

use anchor_lang::prelude::*;
use package1::*;
use package2::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
mod basic_0 {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, name: String) -> ProgramResult {
        let data_account = &mut ctx.accounts.initialize_data_account;
        let authority = &ctx.accounts.authority;
        data_account.name = name;
        data_account.owner = authority.key();
        Ok(())
    }
}

