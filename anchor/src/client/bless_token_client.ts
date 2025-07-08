import {
  Keypair,
  PublicKey,
  Transaction,
  TransactionInstruction,
  TransactionSignature,
} from "@solana/web3.js";
import { TxOptions } from "./base";
import { BlsBaseClient } from "./base_client";
import { Account } from "@solana/spl-token";

export class BlessTokenAccounts {
  public preseedSale: PublicKey;
  public seedSale: PublicKey;
  public advisors: PublicKey;
  public team: PublicKey;
  public ecosystem: PublicKey;
  public foundation: PublicKey;
  public liquidityProvision: PublicKey;
  public tgeMarketing: PublicKey;
  public airdrop: PublicKey;
  public communityRewards: PublicKey;
}

export class BlsTokenClient {
  baseClient: BlsBaseClient;

  constructor(base: BlsBaseClient) {
    this.baseClient = base;
  }

  public async initialBlessTokenState(
    accounts: BlessTokenAccounts,
    blessMint: PublicKey,
    mintAuthority: Keypair,
    txOptions: TxOptions = {},
  ): Promise<TransactionSignature> {
    let preIxs: TransactionInstruction[] = [];
    if (txOptions?.preInstructions) {
      preIxs = txOptions?.preInstructions;
    }
    const payer: PublicKey = txOptions.signer || this.baseClient.getSigner();
    const tx = await this.baseClient.program.methods
      .initial()
      .accountsPartial({
        payer,
        blessMint,
        airdrop: accounts.airdrop!,
        communityRewards: accounts.communityRewards,
        advisors: accounts.advisors,
        ecosystem: accounts.ecosystem,
        foundation: accounts.foundation,
        liquidityProvision: accounts.liquidityProvision,
        preseedSale: accounts.preseedSale,
        seedSale: accounts.seedSale,
        tgtMarketing: accounts.tgeMarketing,
        team: accounts.team,
        currentAuthority: mintAuthority.publicKey,
      })
      .preInstructions(preIxs)
      .transaction();
    const versioned = await this.baseClient.getVersionedTransaction({
      tx,
      ...txOptions,
    });
    return this.baseClient.sendAndConfirm(versioned, txOptions.signerKeypair);
  }
}
