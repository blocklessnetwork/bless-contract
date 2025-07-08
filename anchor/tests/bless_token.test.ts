import * as anchor from "@coral-xyz/anchor";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";
import { BlsClient } from "../src/client/client";
import {
  Account,
  createMint,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
import { BlessTokenAccounts } from "../src/client/bless_token_client";

describe("bless token tests.", () => {
  const client = new BlsClient();
  const provider = client.provider;
  const wallet: Keypair = Keypair.generate();
  const mintAuthority: Keypair = Keypair.generate();
  const connection = client.connection;
  const blessTokenClient = client.blessTokenClient;
  let mint: PublicKey | null = null;
  client.setWallet(new anchor.Wallet(wallet));
  const wallets = Array.from(Array(10), () => Keypair.generate());
  const userTokenAccount: PublicKey[] = [];

  const log = async (signature: string): Promise<string> => {
    console.log(
      "Your transaction signature: https://explorer.solana.com/transaction/" +
        `${signature}?cluster=custom&customUrl=${connection?.rpcEndpoint}`,
    );
    return signature;
  };
  before("airdrop and create ata", async () => {
    const tx = new Transaction();
    tx.instructions = [
      ...[wallet, ...wallets].map((k) =>
        SystemProgram.transfer({
          fromPubkey: provider.publicKey!,
          toPubkey: k.publicKey,
          lamports: 2 * LAMPORTS_PER_SOL,
        }),
      ),
    ];
    await provider.sendAndConfirm!(tx, []).then((s) => {
      log(s);
      console.log("Air Drop Success.");
    });
    mint = await createMint(
      connection,
      wallet,
      mintAuthority.publicKey,
      mintAuthority.publicKey,
      6,
    );
    for (const wal of wallets) {
      let tokenAddr: Account = await getOrCreateAssociatedTokenAccount(
        connection,
        wal,
        mint!,
        wal.publicKey,
      );
      userTokenAccount.push(tokenAddr.address);
    }
  });

  it("initial", async () => {
    const accts = new BlessTokenAccounts();
    accts.advisors = userTokenAccount[0];
    accts.airdrop = userTokenAccount[1];
    accts.communityRewards = userTokenAccount[2];
    accts.ecosystem = userTokenAccount[3];
    accts.foundation = userTokenAccount[4];
    accts.liquidityProvision = userTokenAccount[5];
    accts.preseedSale = userTokenAccount[6];
    accts.seedSale = userTokenAccount[7];
    accts.team = userTokenAccount[8];
    accts.tgeMarketing = userTokenAccount[9];
    await blessTokenClient.initialBlessTokenState(accts, mint!, mintAuthority, {
      signer: wallet!.publicKey,
      signerKeypair: [wallet!, mintAuthority],
    });
  });
});
