import { Provider, Wallet } from "@coral-xyz/anchor";

export enum ClusterNetwork {
  Mainnet = "mainnet-beta",
  Testnet = "testnet",
  Devnet = "devnet",
  Custom = "custom",
}

export type BlsClientConfig = {
  provider?: Provider;
  wallet?: Wallet;
  cluster?: ClusterNetwork;
  jupiterApi?: string;
};
