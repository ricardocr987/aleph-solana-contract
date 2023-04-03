Anchor (Solana framework) events are emitted through logs. There are two options:

- with the emit! macro: more efficient, it serializes the data
- with the msg! macro: creates a log with the raw json without serialization

Benchmark of these options: https://github.com/coral-xyz/anchor/issues/863#issuecomment-1101779506

Possible problems:
- Anchor PR in works: add emit_cpi to allow programs to store logs in CPI data (Cross program invocation, it makes sense to take this into account, in case we want to allow another solana program to call this program to emit events to aleph network directly from their program). This will make easier and more reliable to index events.

https://github.com/coral-xyz/anchor/pull/2438
https://github.com/coral-xyz/anchor/issues/2408
https://github.com/coral-xyz/anchor/pull/2438#discussion_r1155121788

As a realistic option for it to work at the moment, programs that want to emit events to aleph network should create the instructions from the client and not from the program, ie:

Build a transaction with:
- ixns of the custom program
- do_message ix to emit the message