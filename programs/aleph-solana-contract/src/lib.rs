use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod aleph_solana_contract {
    use super::*;

    pub fn do_emit(ctx: Context<Emit>, message: String) -> Result<()> {
        let clock = Clock::get()?;

        emit!(SyncEvent {
            timestamp: clock.unix_timestamp,
            address: *ctx.accounts.sender.key,
            message: message.clone(),
        });

        Ok(())
    }
    pub fn do_message(ctx: Context<Emit>, msg_type: String, msg_content: String) -> Result<()> {
        let clock = Clock::get()?;

        emit!(MessageEvent {
            timestamp: clock.unix_timestamp,
            address: *ctx.accounts.sender.key,
            message_type: msg_type.clone(),
            message_content: msg_content.clone(),
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Emit<'info> {
    #[account(signer)]
    sender: AccountInfo<'info>,
}

#[event]
pub struct SyncEvent {
    pub timestamp: i64,
    pub address: Pubkey,
    pub message: String,
}

#[event]
pub struct MessageEvent {
    pub timestamp: i64,
    pub address: Pubkey,
    pub message_type: String,
    pub message_content: String,
}
