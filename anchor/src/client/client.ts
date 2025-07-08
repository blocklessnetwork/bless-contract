import { BlsBaseClient } from "./base_client";
import { BlsTokenClient } from "./bless_token_client";
import { BlsClientConfig } from "./config";

export class BlsClient extends BlsBaseClient {
  private _blessTokenClient: BlsTokenClient;

  public constructor(config?: BlsClientConfig) {
    super(config);
  }

  get blessTokenClient() {
    if (this._blessTokenClient == null) {
      this._blessTokenClient = new BlsTokenClient(this);
    }
    return this._blessTokenClient;
  }
}
