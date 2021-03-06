use anchor_lang::prelude::*;

declare_id!("3vKFQUmdGGiHR6hK3tQ1JJnD3YCF4b4ktDUutgCsgScG");

#[program]
pub mod donation_solana {
    use super::*;

    // initial program state
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let donation_account = &mut ctx.accounts.donation_account;
        donation_account.owner = *ctx.accounts.user.to_account_info().key;
        Ok(())
    }

    // receiveing donations
    pub fn make_donation(
        ctx: Context<Donation>,
        lamports: u64,
    ) -> std::result::Result<(), anchor_lang::prelude::ProgramError> {
        ctx.accounts.donator.amount = ctx.accounts.donator.amount + lamports;
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.donation_account.key(),
            lamports,
        );

        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.donation_account.to_account_info(),
            ],
        )
    }

    //withdraw all donates to owner
    pub fn withdraw(ctx: Context<Withdraw>) -> std::result::Result<(), ProgramError> {
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.donation_account.key(),
            &ctx.accounts.donation_account.owner,
            ctx.accounts.donation_account.to_account_info().lamports(),
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.donation_account.to_account_info(),
                ctx.accounts.user.to_account_info(),
            ],
        )
    }
}

//data for initialize function
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 64 + 1024)]
    pub donation_account: Account<'info, DonationAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

//main program account that hold business logic
#[account]
pub struct DonationAccount {
    pub owner: Pubkey,
}

//donate function data context
#[derive(Accounts)]
pub struct Donation<'info> {
    #[account(mut)]
    pub donation_account: Account<'info, DonationAccount>,
    #[account(seeds = [b"seed".as_ref()], bump)]
    pub donator: Account<'info, Donator>,
    pub user: Signer<'info>,
}

//withdraw function data context
#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub donation_account: Account<'info, DonationAccount>,
    #[account(owner = user.key())]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct AllDonators<'info> {
    #[account(mut)]
    pub donation_account: Account<'info, DonationAccount>,
    pub user: Signer<'info>,
}

#[account]
#[derive(Default)]
pub struct Donator {
    pub amount: u64,
}
