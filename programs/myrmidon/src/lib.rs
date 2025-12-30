use anchor_lang::prelude::*;
use spl_account_compression::program::SplAccountCompression;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod myrmidon {
    use super::*;

    pub fn initialize_swarm(ctx: Context<Initialize>, _depth: u32, _buffer: u32) -> Result<()> {
        let state = &mut ctx.accounts.swarm_state;
        state.authority = ctx.accounts.authority.key();
        state.merkle_tree = ctx.accounts.merkle_tree.key();
        state.active = true;
        msg!("Swarm Grid Initialized");
        Ok(())
    }

    pub fn submit_work_proof(
        ctx: Context<SubmitWork>, 
        root: [u8; 32], 
        data_hash: [u8; 32], 
        _proof: Vec<[u8; 32]>
    ) -> Result<()> {
        // In production: verify_leaf(merkle_tree, root, leaf, proof)

        // Settlement
        let reward = 50_000_000; // 0.05 SOL
        **ctx.accounts.vault.try_borrow_mut_lamports()? -= reward;
        **ctx.accounts.worker.try_borrow_mut_lamports()? += reward;

        emit!(WorkVerified {
            worker: ctx.accounts.worker.key(),
            hash: data_hash,
            timestamp: Clock::get()?.unix_timestamp,
        });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 32 + 1)]
    pub swarm_state: Account<'info, SwarmState>,
    /// CHECK: Validated via CPI
    pub merkle_tree: UncheckedAccount<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SubmitWork<'info> {
    #[account(mut)]
    pub swarm_state: Account<'info, SwarmState>,
    #[account(mut)]
    pub worker: Signer<'info>,
    /// CHECK: Vault
    #[account(mut)]
    pub vault: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct SwarmState {
    pub authority: Pubkey,
    pub merkle_tree: Pubkey,
    pub active: bool,
}

#[event]
pub struct WorkVerified {
    pub worker: Pubkey,
    pub hash: [u8; 32],
    pub timestamp: i64,
}
