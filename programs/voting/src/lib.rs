use anchor_lang::prelude::*;
/// Pending to modify Program ID 
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program] 

mod voting {
    use super::*;
    ///init ///
    pub fn initialize(ctx: Context<Initialize>, ticket: u64) -> Result<()> { //let's initialize our Voting Account
        let voting: &mut Account<Voting> = &mut ctx.accounts.voting;        
        voting.authority = ctx.accounts.admin.key(); 
        voting.claimed = 0; //This will be a count that will tell us if the user have request a ticket               
        voting.countyes = 0;           
        voting.ticket = ticket;
        voting.winner = winner;

        Ok(())
    }
    pub fn initialize_mint(ctx: Context<InitializeMint>) -> ProgramResult {
        ctx.accounts.mint.authority = ctx.accounts.payer.key();
        ctx.accounts.mint.supply = 0;
        Ok(())
    }

    pub fn initialize_token_account(ctx: Context<InitializeTokenAccount>) -> ProgramResult {
        ctx.accounts.token_account.owner = ctx.accounts.payer.key();
        ctx.accounts.token_account.mint = ctx.accounts.mint.key();
        ctx.accounts.token_account.amount = 0;
        Ok(())
    }

    pub fn mint(ctx: Context<MintCtx>, amount: u64) -> ProgramResult {
        assert!(ctx.accounts.mint.authority == ctx.accounts.authority.key());
        assert!(ctx.accounts.dst.mint == ctx.accounts.mint.key());
        ctx.accounts.mint.supply += amount;
        ctx.accounts.dst.amount += amount;
        msg!("total supply {}", ctx.accounts.mint.supply);
        msg!("dst amount {}", ctx.accounts.dst.amount);
        Ok(())
    }

    pub fn transfer(ctx: Context<Transfer>, amount: u64) -> ProgramResult {
        assert!(ctx.accounts.src.amount >= amount);
        assert!(ctx.accounts.src.owner == ctx.accounts.owner.key());
        assert!(ctx.accounts.src.mint == ctx.accounts.dst.mint);
        ctx.accounts.src.amount -= amount;
        ctx.accounts.dst.amount += amount;
        msg!("src supply {}", ctx.accounts.src.amount);
        msg!("dst amount {}", ctx.accounts.dst.amount);
        Ok(())
    }
    
    ///get a ticket////
    /// 
    
    pub fn get_ticket(ctx: Context<Claim>) ->Result<()>{
        let voting: &mut Account<Voting> = &mut ctx.accounts.voting;
        let voter: &mut Signer = &mut ctx.accounts.voter;
        Ok(())
        //transfer lamports to the lottery account

    }

    /*pub fn yes(ctx: Context) -> Result<()>{
        Ok(())
    }
    pub fn no(ctx: Context) -> Result<()>{
        Ok(())
    }
    */
    //Oracle checks if today It'S before than Deadline ////

    // 

    //
}
////// CONTEXTS///////////
pub struct Initialize<'info> {
    #[account(init, payer = admin, space = 8 + 180)]
    pub voting: Account<'info, Voting>,
    #[account(mut)]
    pub admin: Signer<'info>,    
    pub system_program: Program<'info, System>,
   // pub struct Claim<'info>{
    }

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(init, seeds = [payer.key().as_ref(),], bump, payer = payer, space = 8 + 40,)]
    mint: Account<'info, Mint>,
    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct InitializeTokenAccount<'info> {
    #[account(init, seeds = [payer.key().as_ref(), mint.key().as_ref(),], bump, space = 8 + 72, payer = payer,)]
    token_account: Account<'info, TokenAccount>,
    mint: Account<'info, Mint>,
    #[account(mut)]
    payer: Signer<'info>,
    system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct MintCtx<'info> {
    #[account(mut)]
    mint: Account<'info, Mint>,
    #[account(mut)]
    dst: Account<'info, TokenAccount>,
    authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut)]
    src: Account<'info, TokenAccount>,
    #[account(mut)]
    dst: Account<'info, TokenAccount>,
    owner: Signer<'info>,
}

// Currency
#[account]
struct Mint {
    pub authority: Pubkey,
    pub supply: u64,
}

#[account]
struct TokenAccount {
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
}
//////////////////////////////////
#[derive(Accounts)]

////////////////////

/// Accounts//////////////
//Errors/
#[error_code]
pub enum ErrorCode {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
}