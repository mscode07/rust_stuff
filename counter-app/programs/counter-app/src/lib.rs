use anchor_lang::{declare_id, program};
use anchor_lang::prelude::*;

declare_id!("2phbC62wekpw95XuBk4i1KX4uA8zBUWmYbiTMhicSuBV");

#[program]

pub mod hello_solana{
    use anchor_lang::{context::Context, solana_program::msg};
    use super::*;
    pub fn hello(_ctx: Context<Hello>) -> Result<()>{
        msg!("hello Solana!!");
        msg!("Our program {}", &id());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Hello{}