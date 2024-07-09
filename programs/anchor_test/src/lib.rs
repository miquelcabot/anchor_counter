use anchor_lang::prelude::*;

declare_id!("AwVPCWBAwYgVrmC4CPmhaY1skNmuZSgqwMoLy1nDtJxT");

#[program]
pub mod anchor_test {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
