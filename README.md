The program instructions can be called in two ways:

1- via CPI (in this case another program calls to the to do_message instruction). This one is the more efficient. For this you need:

- add the dependency to your solana program in the Cargo.toml: 
```rust
aleph-solana-contract = { version="0.1.0", features = [ "no-entrypoint" ] }
```
This is the crate published: https://crates.io/crates/aleph-solana-contract
- include an account in the context of the instruction you want to publish a message:
```rust
/// CHECK: contraint added to force using actual aleph message program
#[account(address = aleph_solana_contract::ID, executable)]
pub messages_program: UncheckedAccount<'info>
```
- build the cpi in the instruction logic:
```rust
solana_program::program::invoke(
    &Instruction {
        program_id: ctx.accounts.messages_program.key(),
        accounts: vec![
            AccountMeta::new(ctx.accounts.sender.key(), true), 
            AccountMeta::new_readonly(ctx.accounts.messages_program.key(), false)
        ],
        data: aleph_solana_contract::instruction::DoMessage {
            msgtype: "aggregate".to_string(),
            msgcontent: format!("{{key:'',content:{{}}, channel:''}}", content),
        }
        .data(),
    },
    &[
        ctx.accounts.authority.to_account_info().clone(),
        ctx.accounts.messages_program.to_account_info().clone(),
    ],
)?;
```
- you need to import in the instruction: anchor_lang::InstructionData (to be able to use the data trait when building the instruction)

[**EXAMPLE**](https://github.com/ricardocr987/brick/blob/master/programs/brick/src/instructions/create_token.rs)

2- including the aleph message instruction inside the transaction in the client, so its creating an specific instruction inside an Transaction.

current related discussion in the anchor repo to improve this: add emit_cpi to allow programs to store logs in CPI data:

https://github.com/coral-xyz/anchor/pull/2438

https://github.com/coral-xyz/anchor/issues/2408

https://github.com/coral-xyz/anchor/pull/2438#discussion_r1155121788

Anchor (Solana framework) events are emitted through logs. There are two options:

- with the emit! macro: more efficient, it serializes the data
- with the msg! macro: creates a log with the raw json without serialization

Benchmark of these options: https://github.com/coral-xyz/anchor/issues/863#issuecomment-1101779506
