use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod consensus_gauge {
    use super::*;

    pub fn create_gauge(ctx: Context<CreateGauge>) -> Result<()> {
        let gauge_state = &mut ctx.accounts.gauge_state;
        gauge_state.score = 0;
        gauge_state.authority = *ctx.accounts.user.key; // Save who created it
        Ok(())
    }

    pub fn signal_agree(ctx: Context<UpdateGauge>) -> Result<()> {
        let gauge_state = &mut ctx.accounts.gauge_state;
        gauge_state.score += 1;
        Ok(())
    }

    pub fn signal_disagree(ctx: Context<UpdateGauge>) -> Result<()> {
        let gauge_state = &mut ctx.accounts.gauge_state;
        gauge_state.score -= 1;
        Ok(())
    }
}

// Context for the `create_gauge` instruction
#[derive(Accounts)]
pub struct CreateGauge<'info> {
    #[account(
        init,   // Tells anchor to create this account
        payer = user,
        space = GaugeState::LEN // We must define how much space to reserve
    )]
    pub gauge_state: Account<'info, GaugeState>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// Context for the `signal_agree` and `signal_disagree` instructions
#[derive(Accounts)]
pub struct UpdateGauge<'info> {
    #[account(mut)] // "mut" means we can change its data
    pub gauge_state: Account<'info, GaugeState>,

    pub user: Signer<'info>,
}

// This is the structure of our "GaugeState" account
#[account]
pub struct GaugeState {
    pub score: i64,
    pub authority: Pubkey,
}

impl GaugeState {
    // We define a constant "LEN" for the account's size
    // 8 bytes: Anchor's internal discriminator (always needed)
    // 8 bytes: score (i64)
    // 32 bytes: authority (Pubkey)
    // Total = 48 bytes
    const LEN: usize = 8 + 8 + 32;
}
