import { BlsBaseClient } from "./base_client";
import { BlsTimeClient } from "./bless_time_client";
import { BlsClientConfig } from "./config";

export class BlsClient extends BlsBaseClient {
  private _blessTimeClient: BlsTimeClient;

  public constructor(config?: BlsClientConfig) {
    super(config);
  }

  get blessTimeClient() {
    if (this._blessTimeClient == null) {
      this._blessTimeClient = new BlsTimeClient(this);
    }
    return this._blessTimeClient;
  }
}
