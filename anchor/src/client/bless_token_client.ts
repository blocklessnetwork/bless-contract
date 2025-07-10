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
  public marketMaking: PublicKey;
  public tgtMarketing: PublicKey;
  public communityAirdrop: PublicKey;
  public communityIncentives: PublicKey;
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
      .blessTokenInitial()
      .accountsPartial({
        payer,
        blessMint,
        communityAirdrop: accounts.communityAirdrop!,
        communityIncentives: accounts.communityIncentives,
        advisors: accounts.advisors,
        ecosystem: accounts.ecosystem,
        foundation: accounts.foundation,
        marketMaking: accounts.marketMaking,
        preseedSale: accounts.preseedSale,
        seedSale: accounts.seedSale,
        tgtMarketing: accounts.tgtMarketing,
        team: accounts.team,
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

  public async fundBlessToken(
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
      .fundBlessToken()
      .accountsPartial({
        payer,
        blessMint,
        communityAirdrop: accounts.communityAirdrop!,
        communityIncentives: accounts.communityIncentives,
        advisors: accounts.advisors,
        ecosystem: accounts.ecosystem,
        foundation: accounts.foundation,
        marketMaking: accounts.marketMaking,
        preseedSale: accounts.preseedSale,
        seedSale: accounts.seedSale,
        tgtMarketing: accounts.tgtMarketing,
        team: accounts.team,
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
