use anchor_lang::prelude::*;
// Pending to modify Program ID 
declare_id!("A9Gg3NHu1mkEX7JiA5ai7T6Dj1T61vbEMK8KSxm1zSai");

#[program] 

mod voting {
    use super::*;
    
    pub fn initialize_mint(ctx: Context<InitializeMint>) ->  Result<()> {
        ctx.accounts.mint.authority = ctx.accounts.payer.key();
        ctx.accounts.mint.supply = 0;
        Ok(())
    }

    pub fn initialize_token_account(ctx: Context<InitializeTokenAccount>) ->  Result<()> {
        ctx.accounts.token_account.owner = ctx.accounts.payer.key();
        ctx.accounts.token_account.mint = ctx.accounts.mint.key();
        ctx.accounts.token_account.amount = 0;
        Ok(())
    }

    pub fn mint(ctx: Context<MintCtx>, amount: u64) ->  Result<()> {
        assert!(ctx.accounts.mint.authority == ctx.accounts.authority.key());
        assert!(ctx.accounts.dst.mint == ctx.accounts.mint.key());
        ctx.accounts.mint.supply += amount;
        ctx.accounts.dst.amount += amount;
        msg!("total supply {}", ctx.accounts.mint.supply);
        msg!("dst amount {}", ctx.accounts.dst.amount);
        Ok(())
    }

    pub fn transfer(ctx: Context<Transfer>, amount: u64) ->  Result<()> {
        assert!(ctx.accounts.src.amount >= amount);
    
        assert!(ctx.accounts.src.owner == ctx.accounts.owner.key());
        assert!(ctx.accounts.src.mint == ctx.accounts.dst.mint);
        ctx.accounts.src.amount -= amount;
        ctx.accounts.dst.amount += amount;
        msg!("src supply {}", ctx.accounts.src.amount);
        msg!("dst amount {}", ctx.accounts.dst.amount);
        Ok(())
    }
    
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

