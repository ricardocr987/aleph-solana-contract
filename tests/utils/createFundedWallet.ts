import { AnchorProvider, web3 } from "@coral-xyz/anchor";

export const createFundedWallet = async (
  provider: AnchorProvider,
  amount: number
): Promise<web3.Keypair> => {
  const user = new web3.Keypair();

  await provider.sendAndConfirm(
    new web3.Transaction().add(
      web3.SystemProgram.transfer({
        fromPubkey: provider.wallet.publicKey,
        toPubkey: user.publicKey,
        lamports: amount * web3.LAMPORTS_PER_SOL,
      })
    )
  );

  return user;
};
