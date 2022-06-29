use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{CloseAccount, Mint, Token, TokenAccount, Transfer,InitializeMint}};
use anchor_lang::solana_program::entrypoint::ProgramResult;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod hello {
    use super::*;

   //RPC Request Handlers Context<T> Generic 
   pub fn initialize_mint(mut _ctx: Context<InitializeMintAccount>, _mint_authority:Pubkey,_mint_amount: u8) -> ProgramResult {
        let mint = &mut _ctx.accounts;

        let initializing_instruction = InitializeMint{
            mint: mint.mint_account.to_account_info(),
            rent: mint.author.to_account_info(),
        };
        let ctx_cpi = CpiContext::new(mint.spl_program.to_account_info(), initializing_instruction);
        let authority : Pubkey = _mint_authority;
        let freeze_authority : Option<&Pubkey> = Some(mint.author.owner);
        let decimals : u8 = _mint_amount ;
        anchor_spl::token::initialize_mint(ctx_cpi,decimals,&authority,freeze_authority);

        Ok(())
    }

}

#[derive(Accounts)]                     //derive macro implementing Accounts trait, allowing a struct to transform from the untrusted &[AccountInfo] slice given to a Solana program into a validated struct of deserialized account types. 
pub struct InitializeMintAccount<'info> {       //Intializes mint
    #[account(init,payer=author,space = 8+8)]
    pub mint_account : Account<'info,Mint>, // Accountinfo wrapper - >Account desrialised
    #[account(mut)]                                //Mutable channges to the account
    pub author: Signer<'info>, 
    pub system_program:Program<'info,System>,
    pub spl_program :Program <'info,Token>,
}
