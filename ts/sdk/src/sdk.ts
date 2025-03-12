import {
  OrderType,
  OrderTimeInForce,
  SelfTradePreventionType,
  OrderSide,
  WithdrawRequestSignedFields,
  CancelOrdersRequest,
  AccountPositionLeverageUpdateRequestSignedFields,
  CreateOrderRequestSignedFields,
  CreateOrderRequest,
  AccountDataApi,
  AuthApi,
  ExchangeApi,
  TradeApi,
  LoginRequest,
  MarketStreamMessage,
  AccountStreamMessage,
  LoginResponse,
  ContractsConfig,
  Asset1,
  AccountAuthorizationRequestSignedFields,
} from "./api";

import { Configuration } from "./configuration";
import { IBluefinSigner } from "./request-signer";
import { WebSocket } from "ws";
import { IAsset, TxBuilder } from "@firefly-exchange/library-sui/dist/src/v3";
import {
  CoinUtils,
  SuiClient,
  TransactionBlock,
} from "@firefly-exchange/library-sui";

interface EnvironmentConfig {
  [key: string]: {
    authHost: string;
    apiHost: string;
    tradeHost: string;
    marketWsHost: string;
    accountWsHost: string;
  };
}

const environmentConfig: EnvironmentConfig = {
  mainnet: {
    authHost: "https://auth.api.sui-prod.bluefin.io",
    apiHost: "https://api.sui-prod.bluefin.io",
    tradeHost: "https://trade.api.sui-prod.bluefin.io",
    marketWsHost: "wss://stream.api.sui-prod.bluefin.io/ws/market",
    accountWsHost: "wss://stream.api.sui-prod.bluefin.io/ws/account",
  },
  testnet: {
    authHost: "https://auth.api.sui-staging.bluefin.io",
    apiHost: "https://api.sui-staging.bluefin.io",
    tradeHost: "https://trade.api.sui-staging.bluefin.io",
    marketWsHost: "wss://stream.api.sui-staging.bluefin.io/ws/market",
    accountWsHost: "wss://stream.api.sui-staging.bluefin.io/ws/account",
  },
  devnet: {
    authHost: "https://auth.api.sui-dev.bluefin.io",
    apiHost: "https://api.sui-dev.bluefin.io",
    tradeHost: "https://trade.api.sui-dev.bluefin.io",
    marketWsHost: "wss://stream.api.sui-dev.bluefin.io/ws/market",
    accountWsHost: "wss://stream.api.sui-dev.bluefin.io/ws/account",
  },
};

export interface OrderParams {
  clientOrderId: string;
  type: OrderType;
  symbol: string;
  priceE9: string;
  quantityE9: string;
  side: OrderSide;
  leverageE9: string;
  isIsolated: boolean;
  expiresAtUtcMillis: number;
  reduceOnly?: boolean;
  postOnly?: boolean;
  timeInForce?: OrderTimeInForce;
  triggerPriceE9?: string;
  selfTradePreventionType?: SelfTradePreventionType;
}

enum Services {
  Account,
  Exchange,
  Trade,
  Auth,
  MarketWebsocket,
  AccountWebsocket,
}

export interface BluefinProSdkOptions {
  currentAccountAddress?: string;

  refreshToken?: string;
  refreshTokenValidForSeconds?: number;

  // if needed to point to different services
  authHost?: string;
  apiHost?: string;
  tradeHost?: string;
  marketWsHost?: string;
  accountWsHost?: string;
}

export class BluefinProSdk {
  private readonly configs: Partial<Record<Services, Configuration>> = {};
  public readonly exchangeDataApi: ExchangeApi;
  public readonly accountDataApi: AccountDataApi;
  private readonly tradeApi: TradeApi;
  private readonly authApi: AuthApi;
  private tokenResponse: LoginResponse | null;
  private tokenSetAtSeconds: number | null;
  private isConnected: boolean;
  private updateTokenInterval: NodeJS.Timeout | null;
  private contractsConfig: ContractsConfig | undefined;
  private assets: Array<Asset1> | undefined;
  private txBuilder: TxBuilder | undefined;
  private currentAccountAddress: string | undefined;

  constructor(
    private readonly bfSigner: IBluefinSigner,
    private environment: "mainnet" | "testnet" | "devnet" = "mainnet",
    private suiClient: SuiClient,
    opts?: BluefinProSdkOptions,
  ) {
    this.currentAccountAddress = opts?.currentAccountAddress;
    this.isConnected = false;
    this.updateTokenInterval = null;
    this.contractsConfig = undefined;
    this.tokenResponse = null;
    this.tokenSetAtSeconds = null;

    if (opts?.refreshToken && opts?.refreshTokenValidForSeconds) {
      this.tokenResponse = {
        accessToken: "",
        accessTokenValidForSeconds: 0,
        refreshToken: opts.refreshToken,
        refreshTokenValidForSeconds: opts.refreshTokenValidForSeconds,
      };
    }

    const defaultConfig = environmentConfig[this.environment];

    const basePaths = {
      authHost: opts?.authHost ?? defaultConfig.authHost,
      apiHost: opts?.apiHost ?? defaultConfig.apiHost,
      tradeHost: opts?.tradeHost ?? defaultConfig.tradeHost,
      marketWsHost: opts?.marketWsHost ?? defaultConfig.marketWsHost,
      accountWsHost: opts?.accountWsHost ?? defaultConfig.accountWsHost,
    };

    const authApiConfig = new Configuration({
      basePath: basePaths.authHost,
    });
    this.configs[Services.Auth] = authApiConfig;
    this.authApi = new AuthApi(authApiConfig);

    const exchangeApiConfig = new Configuration({
      basePath: basePaths.apiHost,
    });
    this.configs[Services.Exchange] = exchangeApiConfig;
    this.exchangeDataApi = new ExchangeApi(exchangeApiConfig);

    const accountDataApiConfig = new Configuration({
      basePath: basePaths.apiHost,
    });
    this.configs[Services.Account] = accountDataApiConfig;
    this.accountDataApi = new AccountDataApi(accountDataApiConfig);

    const tradeApiConfig = new Configuration({
      basePath: basePaths.tradeHost,
    });
    this.configs[Services.Trade] = tradeApiConfig;
    this.tradeApi = new TradeApi(tradeApiConfig);

    const marketWsConfig = new Configuration({
      basePath: basePaths.marketWsHost,
    });
    this.configs[Services.MarketWebsocket] = marketWsConfig;

    const accountWsConfig = new Configuration({
      basePath: basePaths.accountWsHost,
    });
    this.configs[Services.AccountWebsocket] = accountWsConfig;
  }

  public getTokenResponse() {
    return this.tokenResponse;
  }

  private generateSalt(): string {
    return (Date.now() + Math.floor(Math.random() * 1000000)).toString();
  }

  private async initializeTxBuilder() {
    this.txBuilder = new TxBuilder({
      AdminCap: "",
      ExternalDataStore: this.contractsConfig?.edsId || "",
      InternalDataStore: this.contractsConfig?.idsId || "",
      Operators: {
        admin: this.contractsConfig?.operators.admin || "",
        fee: this.contractsConfig?.operators.fee || "",
        funding: this.contractsConfig?.operators.funding || "",
        pruning: "",
        sequencer: this.contractsConfig?.operators.sequencer || "",
      },
      Package: this.contractsConfig?.currentContractAddress || "",
      Perpetuals: {},
      SupportedAssets:
        this.assets?.reduce((agg: Record<string, IAsset>, x: Asset1) => {
          agg[x.symbol] = { ...x, coinType: x.assetType };
          return agg;
        }, {}) || {},
      TreasuryCap: "",
      UpgradeCap: "",
    });
  }

  public async initialize(): Promise<void> {
    await this.setContractsConfig();
    await this.initializeTxBuilder();
    await this.loginAndUpdateToken();
    this.updateTokenInterval = setInterval(() => this.refreshToken(), 10000);
    this.isConnected = true;
  }

  private async setContractsConfig() {
    const response = await this.exchangeDataApi.getExchangeInfo();
    this.contractsConfig = response.data.contractsConfig;
    this.assets = response.data.assets;
  }

  private async loginAndUpdateToken(): Promise<void> {
    await this.login();
    this.configs[Services.Account]!.accessToken =
      this.tokenResponse!.accessToken;
    this.configs[Services.Trade]!.accessToken = this.tokenResponse!.accessToken;
  }

  private async login(): Promise<void> {
    console.log("Logging in to get the access token");
    this.tokenSetAtSeconds = Date.now() / 1000;

    if (!this.currentAccountAddress) {
      this.currentAccountAddress = this.bfSigner.getAddress();
    }

    console.log(`Logging in as ${this.currentAccountAddress}`);

    if (this.tokenResponse && this.tokenResponse.refreshTokenValidForSeconds) {
      try {
        const response = await this.authApi.authTokenRefreshPut({
          refreshToken: this.tokenResponse.refreshToken,
        });
        this.tokenResponse = response.data;
        return;
      } catch (e) {
        console.error("Error refreshing token:", e); // skipping refresh in favour of login
      }
    }

    const loginRequest: LoginRequest = {
      accountAddress: this.currentAccountAddress,
      signedAtUtcMillis: Date.now(),
      audience: "api",
    };

    const signature = await this.bfSigner.signLoginRequest(loginRequest);
    const response = await this.authApi.authV2TokenPost(
      signature,
      loginRequest,
    );
    this.tokenResponse = response.data;
  }

  public async getAccessToken() {
    if (!this.tokenResponse) {
      await this.login();
    }
    return this.tokenResponse!.accessToken;
  }

  public async getOpenOrders(symbol?: string) {
    await this.setAccessToken();
    return await this.tradeApi.getOpenOrders(symbol);
  }

  public async updateLeverage(symbol: string, leverageE9: string) {
    if (!this.contractsConfig) {
      throw new Error("Missing contracts config");
    }

    const signedFields: AccountPositionLeverageUpdateRequestSignedFields = {
      accountAddress: this.currentAccountAddress!,
      idsId: this.contractsConfig.idsId,
      symbol: symbol,
      leverageE9: leverageE9,
      salt: this.generateSalt(),
      signedAtUtcMillis: Date.now(),
    };

    const request = await this.bfSigner.signLeverageUpdateRequest(signedFields);

    return await this.tradeApi.putLeverageUpdate({
      signedFields,
      signature: request,
      requestHash: "",
    });
  }

  public async createOrder(params: OrderParams): Promise<any> {
    if (!this.contractsConfig) {
      throw new Error("Missing contracts config");
    }

    const signedFields: CreateOrderRequestSignedFields = {
      symbol: params.symbol,
      idsId: this.contractsConfig.idsId,
      accountAddress: this.currentAccountAddress!,
      priceE9: params.priceE9,
      quantityE9: params.quantityE9,
      side: params.side,
      leverageE9: params.leverageE9,
      isIsolated: params.isIsolated,
      salt: this.generateSalt(),
      expiresAtUtcMillis: params.expiresAtUtcMillis,
      signedAtUtcMillis: Date.now(),
    };

    const signature = await this.bfSigner.signOrderRequest(signedFields);

    const createOrderRequest: CreateOrderRequest = {
      signedFields,
      signature,
      orderHash: "",
      clientOrderId: params.clientOrderId,
      type: params.type,
      reduceOnly: params.reduceOnly ?? false,
      postOnly: params.postOnly ?? false,
      timeInForce: params.timeInForce ?? OrderTimeInForce.Gtt,
      triggerPriceE9: params.triggerPriceE9,
      selfTradePreventionType: params.selfTradePreventionType,
    };
    console.log("Creating order:", createOrderRequest);
    return await this.tradeApi.postCreateOrder(createOrderRequest);
  }

  public async cancelOrder(cancelOrdersRequest: CancelOrdersRequest) {
    return await this.tradeApi.cancelOrders(cancelOrdersRequest);
  }

  public async withdraw(assetSymbol: string, amountE9: string) {
    const exchangeInfo = await this.exchangeDataApi.getExchangeInfo();
    const asset = exchangeInfo.data.assets.find(
      (asset) => asset.symbol === assetSymbol,
    );

    if (!asset) {
      throw new Error(`Asset ${assetSymbol} not found`);
    }

    if (!this.contractsConfig) {
      throw new Error("Missing contractsConfig");
    }

    const signedFields: WithdrawRequestSignedFields = {
      assetSymbol,
      edsId: this.contractsConfig.edsId,
      accountAddress: this.currentAccountAddress!,
      amountE9,
      salt: this.generateSalt(),
      signedAtUtcMillis: Date.now(),
    };

    const signature = await this.bfSigner.signWithdrawRequest(signedFields);

    await this.tradeApi.postWithdraw({
      signedFields,
      signature,
      requestHash: "",
    });
    console.log("Withdraw request sent:", signedFields);
  }

  public async authorizeAccount(accountAddress: string) {
    if (!this.contractsConfig) {
      throw new Error("Missing contractsConfig");
    }

    const signedFields: AccountAuthorizationRequestSignedFields = {
      accountAddress: this.currentAccountAddress!,
      idsId: this.contractsConfig.idsId,
      authorizedAccountAddress: accountAddress,
      salt: this.generateSalt(),
      signedAtUtcMillis: Date.now(),
    };

    const signature = await this.bfSigner.signAccountAuthorizationRequest(
      signedFields,
      true,
    );

    await this.tradeApi.putAuthorizeAccount({
      signedFields,
      signature,
      requestHash: "",
    });
    console.log("Authorize account request sent:", signedFields);
  }

  public async deauthorizeAccount(accountAddress: string) {
    if (!this.contractsConfig) {
      throw new Error("Missing contractsConfig");
    }

    const signedFields: AccountAuthorizationRequestSignedFields = {
      accountAddress: this.currentAccountAddress!,
      idsId: this.contractsConfig.idsId,
      authorizedAccountAddress: accountAddress,
      salt: this.generateSalt(),
      signedAtUtcMillis: Date.now(),
    };

    const signature = await this.bfSigner.signAccountAuthorizationRequest(
      signedFields,
      false,
    );

    await this.tradeApi.putDeauthorizeAccount({
      signedFields,
      signature,
      requestHash: "",
    });
    console.log("Deauthorize account request sent:", signedFields);
  }

  public async deposit(amountE9: string, accountAddress?: string) {
    const assetSymbol = "USDC";
    const txb = new TransactionBlock();
    const assetType = this.assets?.find(
      (x) => x.symbol === assetSymbol,
    )?.assetType;
    if (!assetType) {
      throw new Error("Missing USDC asset type");
    }
    const [splitCoin, mergedCoin] = await CoinUtils.createCoinWithBalance(
      this.suiClient,
      txb,
      amountE9,
      assetType,
      this.currentAccountAddress || this.bfSigner.getAddress(),
    );

    this.txBuilder?.depositToAssetBank(
      assetSymbol,
      accountAddress ||
        this.currentAccountAddress ||
        this.bfSigner.getAddress(),
      amountE9,
      splitCoin,
      {
        txBlock: txb,
      },
    );

    if (mergedCoin) {
      txb.transferObjects(
        [mergedCoin],
        this.currentAccountAddress || this.bfSigner.getAddress(),
      );
    }
    if (splitCoin) {
      txb.transferObjects(
        [splitCoin],
        this.currentAccountAddress || this.bfSigner.getAddress(),
      );
    }

    return this.bfSigner.executeTx(txb, this.suiClient);
  }

  private async setAccessToken(): Promise<void> {
    await this.login();
    if (!this.tokenResponse) {
      throw new Error("Missing tokenResponse");
    }

    this.configs[Services.Account]!.accessToken =
      this.tokenResponse.accessToken;
    this.configs[Services.Trade]!.accessToken = this.tokenResponse.accessToken;
  }

  public async refreshToken(): Promise<void> {
    if (!this.isConnected) return;

    console.log("Checking token for refresh");
    if (
      !this.tokenResponse ||
      !this.tokenSetAtSeconds ||
      Date.now() / 1000 - this.tokenSetAtSeconds >
        this.tokenResponse.accessTokenValidForSeconds
    ) {
      console.log("Refreshing token");
      this.tokenSetAtSeconds = Date.now() / 1000;
      await this.loginAndUpdateToken();
    }
  }

  public async createAccountDataStreamListener(
    handler: (data: AccountStreamMessage) => Promise<void>,
  ): Promise<WebSocket> {
    return new Promise((resolve) => {
      if (!this.tokenResponse) {
        throw new Error("Missing tokenResponse");
      }
      const ws = new WebSocket(
        this.configs[Services.AccountWebsocket]!.basePath!,
        {
          headers: {
            Authorization: `Bearer ${this.tokenResponse.accessToken}`,
          },
        },
      );
      ws.onmessage = async (event) => {
        await handler(JSON.parse(<string>event.data));
      };
      ws.on("open", () => {
        resolve(ws);
      });
    });
  }

  public async createMarketDataStreamListener(
    handler: (data: MarketStreamMessage) => Promise<void>,
  ): Promise<WebSocket> {
    return new Promise((resolve) => {
      const ws = new WebSocket(
        this.configs[Services.MarketWebsocket]!.basePath!,
      );
      ws.onmessage = async (event) => {
        await handler(JSON.parse(<string>event.data));
      };
      ws.on("open", () => {
        resolve(ws);
      });
    });
  }

  public async dispose(): Promise<void> {
    console.log("Disposing SDK resources");

    if (this.updateTokenInterval) {
      clearInterval(this.updateTokenInterval);
    }

    this.isConnected = false;
  }
}
