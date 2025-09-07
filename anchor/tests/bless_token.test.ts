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
  getAccount,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
import { BlessTokenAccounts } from "../src/client/bless_token_client";
import { expect } from "chai";

describe("bless token tests.", () => {
  const client = new BlsClient();
  const provider = client.provider;
  const wallet: Keypair = Keypair.generate();
  const mintAuthority: Keypair = Keypair.generate();
  const connection = client.connection;
  const blessTokenClient = client.blessTokenClient;
  const accts = new BlessTokenAccounts();
  let mintKey = Keypair.fromSecretKey(
    Uint8Array.from([
      54, 85, 164, 158, 45, 106, 125, 208, 143, 84, 31, 75, 79, 226, 133, 229,
      138, 121, 240, 44, 109, 55, 193, 58, 202, 166, 183, 236, 150, 177, 141,
      231, 100, 11, 5, 53, 163, 194, 33, 90, 157, 152, 167, 23, 76, 74, 143,
      227, 68, 11, 37, 60, 172, 146, 90, 227, 3, 191, 25, 55, 171, 216, 233, 99,
    ]),
  );
  let mint: PublicKey = new PublicKey(
    "8bDhoMgPoD4rb4Jh5zra4ZPRvHQAHFypAFJniCSFt7FP",
  );
  const metadata = {
    name: "tBless test token",
    symbol: "tBless",
    uri: "https://raw.githubusercontent.com/blocklessnetwork/bless-token/refs/heads/devnet/metadata.json",
  };
  // client.setWallet(new anchor.Wallet(wallet));
  const wallets = Array.from(Array(15), () => Keypair.generate());
  const userTokenAccount: PublicKey[] = [];

  let mintAuth = Keypair.fromSecretKey(
    Uint8Array.from([
      230, 156, 190, 106, 226, 43, 241, 114, 207, 57, 81, 154, 128, 2, 68, 162,
      89, 48, 136, 91, 137, 32, 205, 178, 93, 254, 12, 3, 255, 1, 179, 99, 124,
      63, 21, 249, 157, 46, 142, 250, 81, 13, 151, 65, 216, 85, 136, 135, 93,
      72, 249, 166, 178, 81, 194, 94, 71, 253, 171, 72, 207, 75, 155, 101,
    ]),
  );

  const log = async (signature: string): Promise<string> => {
    console.log(
      "Your transaction signature: https://explorer.solana.com/transaction/" +
        `${signature}?cluster=custom&customUrl=${connection?.rpcEndpoint}`,
    );
    return signature;
  };
  // before("airdrop and create ata", async () => {
  //   const tx = new Transaction();
  //   console.log(provider.publicKey!.toBase58());
  //   tx.instructions = [
  //     ...[wallet, ...wallets].map((k) =>
  //       SystemProgram.transfer({
  //         fromPubkey: provider.publicKey!,
  //         toPubkey: k.publicKey,
  //         lamports: 0.2 * LAMPORTS_PER_SOL,
  //       }),
  //     ),
  //   ];
  //   await provider.sendAndConfirm!(tx, []).then((s) => {
  //     log(s);
  //     console.log("Air Drop Success.");
  //   });
  //   mint = await createMint(
  //     connection,
  //     wallet,
  //     mintAuthority.publicKey,
  //     mintAuthority.publicKey,
  //     9,
  //     mintKey,
  //   );
  //   for (const wal of wallets) {
  //     let tokenAddr: Account = await getOrCreateAssociatedTokenAccount(
  //       connection,
  //       wal,
  //       mint!,
  //       wal.publicKey,
  //     );
  //     userTokenAccount.push(tokenAddr.address);
  //   }
  // });

  // it("initial", async () => {
  //   accts.walletCommunityRewards = userTokenAccount[10];
  //   accts.walletEcosystemLiquidityprovisionTgtmarketing = userTokenAccount[11];
  //   accts.walletFoundation = userTokenAccount[12];
  //   accts.walletInvestor = userTokenAccount[13];
  //   accts.walletTeamAdvisor = userTokenAccount[14];
  //   await blessTokenClient.initialBlessTokenState(accts, mint!, mintAuthority, {
  //     signer: wallet!.publicKey,
  //     signerKeypair: [wallet!, mintAuthority],
  //   });

  //   const sum = async (array: PublicKey[]) => {
  //     let s = new anchor.BN(0);
  //     for (const wallt of array) {
  //       let am = (await getAccount(connection, wallt)).amount;
  //       s = s.add(new anchor.BN(am));
  //     }
  //     return s;
  //   };
  //   let sumVal = await sum([
  //     accts.walletCommunityRewards,
  //     accts.walletEcosystemLiquidityprovisionTgtmarketing,
  //     accts.walletFoundation,
  //     accts.walletInvestor,
  //     accts.walletTeamAdvisor,
  //   ]);
  //   console.log(sumVal.toString());
  //   expect(sumVal.div(new anchor.BN(1_000_000_000)).toNumber()).eq(
  //     10_000_000_000,
  //   );
  // });

  // it("initial meta state", async () => {
  //   await blessTokenClient.initialBlessTokenMetaState(mint!);
  // });

  it("create metadata", async () => {
    const metaPda = blessTokenClient.getMetadataSync(mint!);
    console.log(metaPda);
    const hx = await blessTokenClient.createMetadata(mint!, metaPda, metadata, {
      signer: provider.wallet!.publicKey,
      signerKeypair: [provider.wallet!.payer!],
    });
  });

  it("update metadata", async () => {
    const metaPda = blessTokenClient.getMetadataSync(mint!);
    console.log(metaPda);
    const hx = await blessTokenClient.updateMetadata(mint!, metaPda, metadata, {
      signer: provider.wallet!.publicKey,
      signerKeypair: [provider.wallet!.payer!],
    });
  });
});
