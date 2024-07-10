use anchor_lang::prelude::*;

declare_id!("8r48Y774vYRB4oMPVmScktP4QZYy28fN5BYKYJam49av");

#[program]
pub mod anchor_counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
