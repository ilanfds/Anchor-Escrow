use anchor_lang::prelude::*;

#[account(discriminator=1)]
#[derive(InitSpace)]

pub struct Escrow{
   pub seed: u64,
   pub maker: PubKey,
   pub mint_a: PubKey,
   pub mint_b: PubKey,
   pub receive: u64,
   pub bump: u8,
}