import {
  Keypair,
  PublicKey,
  Transaction,
  TransactionInstruction,
  TransactionSignature,
} from "@solana/web3.js";
import { TxOptions } from "./base";
import { BlsBaseClient } from "./base_client";

export class BlessTokenAccounts {
  public walletInvestor: PublicKey;
  public walletTeamAdvisor: PublicKey;
  public walletFoundation: PublicKey;
  public walletEcosystemLiquidityprovisionTgtmarketing: PublicKey;
  public walletCommunityRewards: PublicKey;
}

export class BlsTokenClient {
  baseClient: BlsBaseClient;

  constructor(base: BlsBaseClient) {
    this.baseClient = base;
  }

  public async fetchBlessState(mint: PublicKey) {
    let state = PublicKey.findProgramAddressSync(
      [Buffer.from("bless_contract_state"), mint.toBuffer()],
      this.baseClient.programId,
    );
    return await this.baseClient.program.account.blessTokenState.fetch(
      state[0],
    );
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
      .initializeBlessToken()
      .accountsPartial({
        payer,
        blessMint,
        walletInvestor: accounts.walletInvestor,
        walletTeamAdvisor: accounts.walletTeamAdvisor,
        walletFoundation: accounts.walletFoundation,
        walletEcosystemLiquidityprovisionTgtmarketing:
          accounts.walletEcosystemLiquidityprovisionTgtmarketing,
        walletCommunityRewards: accounts.walletCommunityRewards,
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
