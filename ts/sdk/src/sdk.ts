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
  AccountPositionLeverageUpdateRequest,
  LoginRequest,
  WithdrawRequest,
  MarketStreamMessagePayload,
  MarketStreamMessage,
  AccountStreamMessage,
  LoginResponse,
  ContractsConfig,
} from "./api";

import { Configuration } from "./configuration";
import { IBluefinSigner } from "./request-signer";
import { WebSocket } from "ws";

interface EnvironmentConfig {
  [key: string]: {
    authHost: string;
    apiHost: string;
    tradeHost: string;
  };
}

const environmentConfig: EnvironmentConfig = {
  mainnet: {
    authHost: "https://auth.api.sui-prod.bluefin.io",
    apiHost: "https://api.sui-prod.bluefin.io",
    tradeHost: "https://trade.api.sui-prod.bluefin.io",
  },
  testnet: {
    authHost: "https://auth.api.sui-staging.bluefin.io",
    apiHost: "https://api.sui-staging.bluefin.io",
    tradeHost: "https://trade.api.sui-staging.bluefin.io",
  },
  devnet: {
    authHost: "https://auth.api.sui-dev.bluefin.io",
    apiHost: "https://api.sui-dev.bluefin.io",
    tradeHost: "https://trade.api.sui-dev.bluefin.io",
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
}

type BasePathConfig = {
  authHost: string | null;
  apiHost: string | null;
  tradeHost: string | null;
};

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

  constructor(
    private readonly bfSigner: IBluefinSigner,
    environment: "mainnet" | "testnet" | "devnet" = "mainnet",
    private currentAccountAddress: string | null = null,
    basePathConfig: BasePathConfig | null = null
  ) {
    this.isConnected = false;
    this.updateTokenInterval = null;
    this.contractsConfig = undefined;
    this.tokenResponse = null;
    this.tokenSetAtSeconds = null;

    const basePaths = {
      authHost:
        basePathConfig && basePathConfig?.authHost
          ? basePathConfig.authHost
          : environmentConfig[environment].authHost,

      apiHost:
        basePathConfig && basePathConfig?.apiHost
          ? basePathConfig.apiHost
          : environmentConfig[environment].apiHost,

      tradeHost:
        basePathConfig && basePathConfig?.tradeHost
          ? basePathConfig.tradeHost
          : environmentConfig[environment].tradeHost,
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
  }

  private generateSalt(): string {
    return (Date.now() + Math.floor(Math.random() * 1000000)).toString();
  }

  public async initialize(): Promise<void> {
    await this.setContractsConfig();
    await this.loginAndUpdateToken();
    this.updateTokenInterval = setInterval(() => this.refreshToken(), 10000);
    this.isConnected = true;
  }

  private async setContractsConfig() {
    const response = await this.exchangeDataApi.getExchangeInfo();
    this.contractsConfig = response.data.contractsConfig;
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
      loginRequest
    );
    this.tokenResponse = response.data;
  }

  public async getAccessToken() {
    if (!this.tokenResponse) {
      await this.login();
    }
    return this.tokenResponse!.accessToken;
  }

  public async getOpenOrders(symbol: string) {
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

    return await this.tradeApi.putLeverageUpdate(request);
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

    const [signature, orderHash] = await this.bfSigner.signOrderRequest(
      signedFields
    );

    const createOrderRequest: CreateOrderRequest = {
      signedFields,
      signature,
      orderHash,
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
      (asset) => asset.symbol === assetSymbol
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

    const request = await this.bfSigner.signWithdrawRequest(signedFields);

    await this.tradeApi.postWithdraw(request);
    console.log("Withdraw request sent:", request);
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

  private async refreshToken(): Promise<void> {
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
    handler: (data: AccountStreamMessage) => Promise<void>
  ): Promise<WebSocket> {
    return new Promise((resolve, reject) => {
      if (!this.tokenResponse) {
        throw new Error("Missing tokenResponse");
      }
      const ws = new WebSocket(
        "wss://stream.api.sui-staging.bluefin.io/ws/account",
        {
          headers: {
            Authorization: `Bearer ${this.tokenResponse.accessToken}`,
          },
        }
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
    handler: (data: MarketStreamMessage) => Promise<void>
  ): Promise<WebSocket> {
    return new Promise((resolve, reject) => {
      const ws = new WebSocket(
        "wss://stream.api.sui-staging.bluefin.io/ws/market"
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
