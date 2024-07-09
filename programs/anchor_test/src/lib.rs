use anchor_lang::prelude::*;

declare_id!("4tb5PnCdk4epKjoje3qEXyM9xWpUPkTQZY4FexNxmRJX");

#[program]
pub mod anchor_test {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
