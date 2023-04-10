use anchor_lang::prelude::*;

declare_id!("ALepH1n9jxScbz45aZhBYVa35zxBNbKSvL6rWQpb4snc");

#[program]
pub mod aleph_solana_contract {
    use super::*;

    pub fn do_emit(ctx: Context<Emit>, message: String) -> Result<()> {
        emit!(SyncEvent {
            address: *ctx.accounts.sender.key,
            message: message.clone(),
        });

        Ok(())
    }
    pub fn do_message(ctx: Context<Emit>, msgtype: String, msgcontent: String) -> Result<()> {
        emit!(MessageEvent {
            address: *ctx.accounts.sender.key,
            msgtype: msgtype.clone(),
            msgcontent: msgcontent.clone(),
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Emit<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
}

#[event]
pub struct SyncEvent {
    pub address: Pubkey,
    pub message: String,
}

#[event]
pub struct MessageEvent {
    pub address: Pubkey,
    pub msgtype: String,
    pub msgcontent: String,
}
