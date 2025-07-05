import * as anchor from "@coral-xyz/anchor";
import * as ed from "@noble/ed25519";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";
import { BlsClient } from "../src/client/client";
import { BlsError } from "../src/client/errors";
import { expect } from "chai";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";

describe("NodeRegistration", () => {
  const client = new BlsClient();
  const provider = client.provider;
  const wallet: Keypair = Keypair.generate();
  const connection = client.connection;
  const blessTimeClient = client.blessTimeClient;
  client.setWallet(new anchor.Wallet(wallet));
  const backendSignerKeypair = Keypair.generate();
  const backendSigner = backendSignerKeypair.publicKey;

  const wallets = Array.from(Array(10), () => Keypair.generate());

  const log = async (signature: string): Promise<string> => {
    console.log(
      "Your transaction signature: https://explorer.solana.com/transaction/" +
        `${signature}?cluster=custom&customUrl=${connection?.rpcEndpoint}`,
    );
    return signature;
  };

  before("Air Drop", async () => {
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
  });
});
