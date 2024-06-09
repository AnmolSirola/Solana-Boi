use anchor_lang::prelude::*;

declare_id!("2A3NpxfsUaL2z5TfPEbQ75bQUDZEYGNaMox1MFyzE4xP");

#[program]
pub mod helloworld {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
