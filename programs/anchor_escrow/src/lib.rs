use anchor_lang::prelude::*;

declare_id!("F6FQZ5gTn7jSPyzhKfAKVhyaGRWNx4dAKYnk38Jmk9Lo");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
