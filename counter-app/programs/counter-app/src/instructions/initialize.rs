use anchor_lang::prelude::*;

use crate::state::Counter;

#[derive(Accounts)]
pub struct Initialize<'info>{

    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 8
    )]
    pub counter: Account<'info, Counter>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}


pub fn handler(ctx: Context<Initialize>) -> Result<()> {


    

    Ok(())

}