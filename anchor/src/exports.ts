// Here we export some useful types and functions for interacting with the Anchor program.
import { AnchorProvider, Program } from "@coral-xyz/anchor";
import { Cluster, PublicKey } from "@solana/web3.js";
import BlessTimeProgramIDL from "../target/idl/bless_time.json";
import type { BlessTime } from "../target/types/bless_time";

// Re-export the generated IDL and type
export { BlessTimeProgramIDL, BlessTime };

// The programId is imported from the program IDL.
export const NODE_REGISTRATION_PROGRAM_ID = new PublicKey(
  BlessTimeProgramIDL.address,
);

export type BlessTimeProgram = Program<BlessTime>;

// This is a helper function to get the Registration Anchor program.
export function getBlessTimeProgram(
  provider: AnchorProvider,
  address?: PublicKey,
): Program<BlessTime> {
  return new Program<BlessTime>(
    {
      ...BlessTimeProgramIDL,
      address: address ? address.toBase58() : BlessTimeProgramIDL.address,
    } as BlessTime,
    provider,
  );
}

// This is a helper function to get the program ID for the Registration program depending on the cluster.
export function getBlessTimeProgramId(cluster: Cluster) {
  switch (cluster) {
    case "devnet":
    case "testnet":
      // This is the program ID for the Registration program on devnet and testnet.
      return new PublicKey("93HzBKdD4w8jfBBdnbjdDs9NeiJB6xHfkrSTmVxLTiQD");
    case "mainnet-beta":
    default:
      return NODE_REGISTRATION_PROGRAM_ID;
  }
}
