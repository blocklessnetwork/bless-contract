import { BlsBaseClient } from "./base_client";

export class BlsTimeClient {
  baseClient: BlsBaseClient;

  constructor(base: BlsBaseClient) {
    this.baseClient = base;
  }
}
