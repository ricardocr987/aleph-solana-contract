import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AlephSolanaContract } from "../target/types/aleph_solana_contract";
import { createFundedWallet } from "./utils/createFundedWallet";
import { assert } from "chai";

type MessageEvent = {
  name: string,
  data: {
    timestamp: BigInt,
    address: anchor.web3.PublicKey,
    messageType: string,
    messageContent: string,
  }
}

function isMessageEvent(event: any): event is MessageEvent {
  return event.data.messageType !== undefined
}

describe("aleph-solana-contract", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.AlephSolanaContract as Program<AlephSolanaContract>;
  const confirmOptions: anchor.web3.ConfirmOptions = { commitment: "confirmed" };

  it("Test event emit", async () => {
    const sender = await createFundedWallet(provider, 20);
    const tx = await program.methods
      .doMessage("message_type", "message_content")
      .accounts({
        sender: sender.publicKey,
      })
      .signers(
        sender instanceof (anchor.Wallet as any)
          ? []
          : [sender]
      )
      .rpc(confirmOptions);

    const rawTx = await provider.connection.getTransaction(tx, {
      commitment: "confirmed",
    });
    const eventParser = new anchor.EventParser(program.programId, new anchor.BorshCoder(program.idl));
    const events = eventParser.parseLogs(rawTx.meta.logMessages);
    for (let event of events) {
      if (isMessageEvent(event)) {
        assert.equal(Number(event.data.timestamp), rawTx.blockTime);
        assert.equal(event.data.address.toString(), sender.publicKey.toString());
        assert.equal(event.data.messageType, "message_type");
        assert.equal(event.data.messageContent, "message_content");
      }
    }
  });
});