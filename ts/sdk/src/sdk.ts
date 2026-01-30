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
  AssetConfig,
  AccountAuthorizationRequestSignedFields,
  AdjustIsolatedMarginRequestSignedFields,
  AdjustMarginOperation,
  RewardsApi,
  UpdateAccountPreferenceRequest,
  SponsorTxRequest,
  AccountGroupIdPatch,
} from './api';

import { Configuration } from './configuration';
import { IBluefinSigner } from './request-signer';
import { WebSocket } from 'ws';
import { IAsset, TxBuilder } from '@firefly-exchange/library-sui/v3';
import {
  CoinUtils,
  SuiClient,
  TransactionBlock,
  Ed25519Keypair,
  decodeSuiPrivateKey,
  SuiBlocks,
} from '@firefly-exchange/library-sui';
// RewardsDistributorInteractor for reward claiming
import { RewardsDistributorInteractor } from '@firefly-exchange/library-sui/index';
import { fromBase64, toHex } from '@mysten/bcs';

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
    authHost: 'https://auth.api.sui-prod.bluefin.io',
    apiHost: 'https://api.sui-prod.bluefin.io',
    tradeHost: 'https://trade.api.sui-prod.bluefin.io',
    marketWsHost: 'wss://stream.api.sui-prod.bluefin.io/ws/market',
    accountWsHost: 'wss://stream.api.sui-prod.bluefin.io/ws/account',
  },
  testnet: {
    authHost: 'https://auth.api.sui-staging.bluefin.io',
    apiHost: 'https://api.sui-staging.bluefin.io',
    tradeHost: 'https://trade.api.sui-staging.bluefin.io',
    marketWsHost: 'wss://stream.api.sui-staging.bluefin.io/ws/market',
    accountWsHost: 'wss://stream.api.sui-staging.bluefin.io/ws/account',
  },
  devnet: {
    authHost: 'https://auth.api.sui-dev.bluefin.io',
    apiHost: 'https://api.sui-dev.bluefin.io',
    tradeHost: 'https://trade.api.sui-dev.bluefin.io',
    marketWsHost: 'wss://stream.api.sui-dev.bluefin.io/ws/market',
    accountWsHost: 'wss://stream.api.sui-dev.bluefin.io/ws/account',
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
  expiresAtMillis: number;
  reduceOnly?: boolean;
  postOnly?: boolean;
  timeInForce?: OrderTimeInForce;
  triggerPriceE9?: string;
  selfTradePreventionType?: SelfTradePreventionType;
  signerAddress?: string;
}

export interface ClaimPayload {
  target: string;
  receiver: string;
  amount: string;
  expiry: string;
  nonce: string;
  type: number;
}

export interface BatchClaimParams {
  sigPayload: ClaimPayload;
  signature: string;
  rewardType: 'Blue' | 'Sui' | 'Wal' | string; // Reward type to look up coin from contract config
}

enum Services {
  Account,
  Exchange,
  Rewards,
  Trade,
  Auth,
  MarketWebsocket,
  AccountWebsocket,
}

export interface BluefinProSdkOptions {
  currentAccountAddress?: string;
  refreshToken?: string;
  refreshTokenValidForSeconds?: number;
  disableLoginPromptOnLogout?: boolean;
  currentTimeMs?: number;
  onLogout?: () => void;
  onAccessTokenUpdate?: (accessToken: string) => void;

  // if needed to point to different services
  authHost?: string;
  apiHost?: string;
  tradeHost?: string;
  marketWsHost?: string;
  accountWsHost?: string;
}

export interface InitializeOptions {
  refreshTokenValidForSeconds?: number;
  readOnly?: boolean;
}

export class BluefinProSdk {
  private static readonly TOKEN_REFRESH_THRESHOLD_PERCENTAGE = 0.8;
  private readonly configs: Partial<Record<Services, Configuration>> = {};
  public readonly exchangeDataApi: ExchangeApi;
  public readonly rewardsDataApi: RewardsApi;
  public readonly accountDataApi: AccountDataApi;
  private readonly tradeApi: TradeApi;
  private readonly authApi: AuthApi;
  private initializeOptions: InitializeOptions | undefined;
  private tokenResponse: LoginResponse | null;
  private tokenSetAtSeconds: number | null;
  private isConnected: boolean;
  private updateTokenTimeout: ReturnType<typeof setTimeout> | null;
  private contractsConfig: ContractsConfig | undefined;
  private assets: Array<AssetConfig> | undefined;
  private txBuilder: TxBuilder | undefined;
  private currentAccountAddress: string | undefined;
  private disableLoginPromptOnLogout: boolean;
  private onLogout?: () => void;
  private onAccessTokenUpdate?: (accessToken: string) => void;
  private isRefreshing: boolean;
  private refreshTokenPromise: Promise<void> | null;
  private visibilityChangeHandler?: () => void;
  private onlineHandler?: () => void;
  private offlineHandler?: () => void;
  private timeOffsetMs: number;

  constructor(
    private readonly bfSigner: IBluefinSigner,
    private environment: 'mainnet' | 'testnet' | 'devnet' = 'mainnet',
    private suiClient: SuiClient,
    opts?: BluefinProSdkOptions
  ) {
    this.currentAccountAddress = opts?.currentAccountAddress;
    this.isConnected = false;
    this.updateTokenTimeout = null;
    this.contractsConfig = undefined;
    this.tokenResponse = null;
    this.tokenSetAtSeconds = null;
    this.disableLoginPromptOnLogout = opts?.disableLoginPromptOnLogout ?? false;
    this.onLogout = opts?.onLogout;
    this.onAccessTokenUpdate = opts?.onAccessTokenUpdate;
    this.isRefreshing = false;
    this.refreshTokenPromise = null;
    this.visibilityChangeHandler = undefined;
    this.onlineHandler = undefined;
    this.offlineHandler = undefined;
    
    // Initialize time offset based on provided currentTimeMs
    if (opts?.currentTimeMs !== undefined) {
      this.timeOffsetMs = opts.currentTimeMs - Date.now();
    } else {
      this.timeOffsetMs = 0;
    }

    if (opts?.refreshToken && opts?.refreshTokenValidForSeconds) {
      this.tokenResponse = {
        accessToken: '',
        accessTokenValidForSeconds: 0,
        refreshToken: opts.refreshToken,
        refreshTokenValidForSeconds: opts.refreshTokenValidForSeconds,
      };
      // Set timestamp when refresh token was provided
      this.tokenSetAtSeconds = Date.now() / 1000;
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

    const boundGetAccessToken = this.getAccessToken.bind(this);

    const exchangeApiConfig = new Configuration({
      basePath: basePaths.apiHost,
    });
    exchangeApiConfig.accessToken = boundGetAccessToken;
    this.configs[Services.Exchange] = exchangeApiConfig;
    this.exchangeDataApi = new ExchangeApi(exchangeApiConfig);

    const rewardsApiConfig = new Configuration({
      basePath: basePaths.apiHost,
    });
    rewardsApiConfig.accessToken = boundGetAccessToken;
    this.configs[Services.Rewards] = rewardsApiConfig;
    this.rewardsDataApi = new RewardsApi(rewardsApiConfig);

    const accountDataApiConfig = new Configuration({
      basePath: basePaths.apiHost,
    });
    accountDataApiConfig.accessToken = boundGetAccessToken;
    this.configs[Services.Account] = accountDataApiConfig;
    this.accountDataApi = new AccountDataApi(accountDataApiConfig);

    const tradeApiConfig = new Configuration({
      basePath: basePaths.tradeHost,
    });
    tradeApiConfig.accessToken = boundGetAccessToken;
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

  public createWallet() {
    const wallet = Ed25519Keypair.generate();
    const signerKey = wallet.getSecretKey();
    const keyPair = decodeSuiPrivateKey(signerKey);

    const publicAddress = wallet.toSuiAddress();

    return {
      privateKey: toHex(keyPair.secretKey),
      publicAddress,
    };
  }

  public getTokenResponse() {
    return this.tokenResponse;
  }

  private generateSalt(): string {
    return (Date.now() + Math.floor(Math.random() * 1000000)).toString();
  }

  private getCurrentTimeMs(): number {
    return Date.now() + this.timeOffsetMs;
  }

  public updateCurrentTimeMs(currentTimeMs: number): void {
    this.timeOffsetMs = currentTimeMs - Date.now();
  }

  private isRefreshTokenValid(): boolean {
    if (!this.tokenResponse?.refreshToken || !this.tokenSetAtSeconds) {
      return false;
    }

    const currentTimeSeconds = Date.now() / 1000;
    const refreshTokenExpiryTime =
      this.tokenSetAtSeconds + this.tokenResponse.refreshTokenValidForSeconds;

    // Add 60 second buffer to prevent using token right at expiry
    return currentTimeSeconds < refreshTokenExpiryTime - 60;
  }

  private isAccessTokenExpired(): boolean {
    if (!this.tokenResponse?.accessToken || !this.tokenSetAtSeconds) {
      return true;
    }

    const currentTimeSeconds = Date.now() / 1000;
    const tokenLifetimeSeconds = this.tokenResponse.accessTokenValidForSeconds;
    const refreshAtLifetimePercentage =
      BluefinProSdk.TOKEN_REFRESH_THRESHOLD_PERCENTAGE;
    const refreshAtSeconds =
      this.tokenSetAtSeconds +
      tokenLifetimeSeconds * refreshAtLifetimePercentage;

    // Token is considered "expired" if we've passed the 80% lifetime mark
    return currentTimeSeconds >= refreshAtSeconds;
  }

  private async initializeTxBuilder() {
    this.txBuilder = new TxBuilder({
      AdminCap: '',
      ExternalDataStore: this.contractsConfig?.edsId || '',
      InternalDataStore: this.contractsConfig?.idsId || '',
      Operators: {
        admin: this.contractsConfig?.operators.admin || '',
        fee: this.contractsConfig?.operators.fee || '',
        funding: this.contractsConfig?.operators.funding || '',
        pruning: '',
        sequencer: this.contractsConfig?.operators.sequencer || '',
      },
      Package: this.contractsConfig?.currentContractAddress || '',
      Perpetuals: {},
      SupportedAssets:
        this.assets?.reduce((agg: Record<string, IAsset>, x: AssetConfig) => {
          agg[x.symbol] = { ...x, coinType: x.assetType };
          return agg;
        }, {}) || {},
      TreasuryCap: '',
      UpgradeCap: '',
    });
  }

  public async initialize(options?: InitializeOptions): Promise<void> {
    this.initializeOptions = options;
    await this.setContractsConfig();
    await this.initializeTxBuilder();
    await this.loginAndUpdateToken();
    this.setupVisibilityChangeListener();
    this.setupNetworkChangeListener();
  }

  private async setContractsConfig() {
    const response = await this.exchangeDataApi.getExchangeInfo();
    this.contractsConfig = response.data.contractsConfig;
    this.assets = response.data.assets;
  }

  private async loginAndUpdateToken(): Promise<void> {
    await this.login();

    // Safety check - if login failed or logout was called, tokenResponse might be null
    if (!this.tokenResponse) {
      throw new Error('Login failed - no token response available');
    }

    this.isConnected = true;

    // Notify about token refresh
    this.onAccessTokenUpdate?.(this.tokenResponse.accessToken);

    // Schedule the next token refresh
    this.scheduleTokenRefresh();
  }

  private scheduleTokenRefresh(): void {
    // Clear any existing timeout
    if (this.updateTokenTimeout) {
      clearTimeout(this.updateTokenTimeout);
      this.updateTokenTimeout = null;
    }

    if (
      !this.isConnected ||
      !this.tokenResponse ||
      !this.tokenSetAtSeconds
    ) {
      return;
    }

    // Calculate when to refresh: at 80% of token lifetime (or 20% before expiry)
    const currentTimeSeconds = Date.now() / 1000;
    const tokenLifetimeSeconds = this.tokenResponse.accessTokenValidForSeconds;
    const refreshAtLifetimePercentage =
      BluefinProSdk.TOKEN_REFRESH_THRESHOLD_PERCENTAGE;
    const refreshAtSeconds =
      this.tokenSetAtSeconds +
      tokenLifetimeSeconds * refreshAtLifetimePercentage;
    const millisecondsUntilRefresh =
      (refreshAtSeconds - currentTimeSeconds) * 1000;

    const delayMs = Math.max(millisecondsUntilRefresh, 0);

    console.log(
      `Scheduling token refresh in ${Math.round(delayMs / 1000)} seconds`
    );

    this.updateTokenTimeout = setTimeout(() => {
      this.refreshToken(true);
    }, delayMs);
  }

  private setupVisibilityChangeListener(): void {
    // Only set up in browser environment
    if (typeof document !== 'undefined') {
      this.visibilityChangeHandler = () => {
        if (document.visibilityState === 'visible' && this.isConnected) {
          console.log('Page became visible, checking token status');
          this.handleVisible();
        }
      };

      document.addEventListener(
        'visibilitychange',
        this.visibilityChangeHandler
      );
    }
  }

  private setupNetworkChangeListener(): void {
    // Only set up in browser environment
    if (typeof navigator !== 'undefined' && 'onLine' in navigator) {
      window.addEventListener('online', this.handleOnline);
      window.addEventListener('offline', this.handleOffline);
    }
  }

  private async handleVisible(): Promise<void> {
    if (!this.tokenResponse || !this.tokenSetAtSeconds) {
      return;
    }

    this.refreshToken();
  }

  private async handleOnline(): Promise<void> {
    if (!this.isConnected || !this.tokenResponse || !this.tokenSetAtSeconds) {
      return;
    }

    console.log(
      'Network reconnected, resuming token refresh and checking token status'
    );

    this.refreshToken(true);
  }

  private handleOffline(): void {
    if (this.updateTokenTimeout) {
      clearTimeout(this.updateTokenTimeout);
      this.updateTokenTimeout = null;
    }
  }

  private async login(): Promise<void> {
    console.log('Logging in to get the access token');

    if (!this.currentAccountAddress) {
      this.currentAccountAddress = this.bfSigner.getAddress();
    }

    console.log(`Logging in as ${this.currentAccountAddress}`);

    // Check if we have a valid refresh token first
    if (this.tokenResponse?.refreshToken && this.isRefreshTokenValid()) {
      try {
        console.log('Attempting to refresh token using refresh token');
        const response = await this.authApi.authTokenRefreshPut({
          refreshToken: this.tokenResponse.refreshToken,
        });
        this.tokenResponse = response.data;
        this.tokenSetAtSeconds = Date.now() / 1000;
        console.log('Token refreshed successfully');
        return;
      } catch (e) {
        console.error('Error refreshing token:', e);

        // Throw the error to let the caller handle it
        throw new Error(
          `Token refresh failed: ${
            e instanceof Error ? e.message : 'Unknown error'
          }`
        );
      }
    }

    // Fallback to full login only if refresh token is not available or invalid
    // This should only happen during initial login or when refresh token is truly expired
    try {
      const loginRequest: LoginRequest = {
        accountAddress: this.currentAccountAddress,
        signedAtMillis: this.getCurrentTimeMs(),
        audience: 'api',
      };

      const signature = await this.bfSigner.signLoginRequest(loginRequest);
      const response = await this.authApi.authV2TokenPost(
        signature,
        loginRequest,
        this.initializeOptions?.refreshTokenValidForSeconds,
        this.initializeOptions?.readOnly
      );
      this.tokenResponse = response.data;
      this.tokenSetAtSeconds = Date.now() / 1000;
    } catch (e) {
      console.error('Full login failed:', e);

      // If this fails and we're in a mode where logout should be disabled, don't logout
      if (this.disableLoginPromptOnLogout) {
        throw new Error(
          `Login failed: ${e instanceof Error ? e.message : 'Unknown error'}`
        );
      }

      // Otherwise, throw the error and let the caller decide
      throw new Error(
        `Login failed: ${e instanceof Error ? e.message : 'Unknown error'}`
      );
    }
  }

  public async getAccessToken(): Promise<string> {
    if (!this.tokenResponse) {
      await this.login();
    }

    if (!this.tokenResponse) {
      throw new Error('Unable to obtain access token');
    }

    if (this.isConnected) {
      if (this.isAccessTokenExpired()) {
        await this.refreshToken();
      }
    } else if (this.isAccessTokenExpired()) {
      await this.login();
    }

    if (!this.tokenResponse) {
      throw new Error('Access token unavailable after refresh');
    }

    return this.tokenResponse.accessToken;
  }

  public async getOpenOrders(symbol?: string) {
    return await this.tradeApi.getOpenOrders(symbol);
  }

  public async getStandbyOrders(symbol?: string) {
    return await this.tradeApi.getStandbyOrders(symbol);
  }

  public async updateLeverage(symbol: string, leverageE9: string) {
    if (!this.contractsConfig) {
      throw new Error('Missing contracts config');
    }

    const signedFields: AccountPositionLeverageUpdateRequestSignedFields = {
      accountAddress: this.currentAccountAddress!,
      idsId: this.contractsConfig.idsId,
      symbol: symbol,
      leverageE9: leverageE9,
      salt: this.generateSalt(),
      signedAtMillis: this.getCurrentTimeMs(),
    };

    const request = await this.bfSigner.signLeverageUpdateRequest(signedFields);

    return await this.tradeApi.putLeverageUpdate({
      signedFields,
      signature: request,
    });
  }

  public async createOrder(params: OrderParams): Promise<any> {
    if (!this.contractsConfig) {
      throw new Error('Missing contracts config');
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
      expiresAtMillis: params.expiresAtMillis,
      signedAtMillis: this.getCurrentTimeMs(),
    };

    const signature = await this.bfSigner.signOrderRequest(signedFields);

    const createOrderRequest: CreateOrderRequest = {
      signedFields,
      signature,
      clientOrderId: params.clientOrderId,
      type: params.type,
      reduceOnly: params.reduceOnly ?? false,
      postOnly: params.postOnly ?? false,
      timeInForce: params.timeInForce,
      triggerPriceE9: params.triggerPriceE9,
      selfTradePreventionType: params.selfTradePreventionType,
    };
    console.log('Creating order:', createOrderRequest);
    return await this.tradeApi.postCreateOrder(createOrderRequest);
  }

  public async cancelOrder(cancelOrdersRequest: CancelOrdersRequest) {
    return await this.tradeApi.cancelOrders(cancelOrdersRequest);
  }

  public async cancelStandbyOrder(cancelOrdersRequest: CancelOrdersRequest) {
    return await this.tradeApi.cancelStandbyOrders(cancelOrdersRequest);
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
      throw new Error('Missing contractsConfig');
    }

    const signedFields: WithdrawRequestSignedFields = {
      assetSymbol,
      edsId: this.contractsConfig.edsId,
      accountAddress: this.currentAccountAddress!,
      amountE9,
      salt: this.generateSalt(),
      signedAtMillis: this.getCurrentTimeMs(),
    };

    const signature = await this.bfSigner.signWithdrawRequest(signedFields);

    await this.tradeApi.postWithdraw({
      signedFields,
      signature,
    });
    console.log('Withdraw request sent:', signedFields);
  }

  public async authorizeAccount(accountAddress: string, alias?: string) {
    if (!this.contractsConfig) {
      throw new Error('Missing contractsConfig');
    }

    const signedFields: AccountAuthorizationRequestSignedFields = {
      accountAddress: this.currentAccountAddress!,
      idsId: this.contractsConfig.idsId,
      authorizedAccountAddress: accountAddress,
      salt: this.generateSalt(),
      signedAtMillis: this.getCurrentTimeMs(),
    };

    const signature = await this.bfSigner.signAccountAuthorizationRequest(
      signedFields,
      true
    );

    await this.tradeApi.putAuthorizeAccount({
      signedFields,
      signature,
      alias,
    });
    console.log('Authorize account request sent:', signedFields);
  }

  public async deauthorizeAccount(accountAddress: string) {
    if (!this.contractsConfig) {
      throw new Error('Missing contractsConfig');
    }

    const signedFields: AccountAuthorizationRequestSignedFields = {
      accountAddress: this.currentAccountAddress!,
      idsId: this.contractsConfig.idsId,
      authorizedAccountAddress: accountAddress,
      salt: this.generateSalt(),
      signedAtMillis: this.getCurrentTimeMs(),
    };

    const signature = await this.bfSigner.signAccountAuthorizationRequest(
      signedFields,
      false
    );

    await this.tradeApi.putDeauthorizeAccount({
      signedFields,
      signature,
    });
    console.log('Deauthorize account request sent:', signedFields);
  }

  public async adjustIsolatedMargin(
    symbol: string,
    amountE9: string,
    add: boolean
  ) {
    if (!this.contractsConfig) {
      throw new Error('Missing contractsConfig');
    }

    const signedFields: AdjustIsolatedMarginRequestSignedFields = {
      symbol,
      idsId: this.contractsConfig.idsId,
      accountAddress: this.currentAccountAddress!,
      operation: add
        ? AdjustMarginOperation.Add
        : AdjustMarginOperation.Subtract,
      quantityE9: amountE9,
      salt: this.generateSalt(),
      signedAtMillis: this.getCurrentTimeMs(),
    };

    const signature = await this.bfSigner.signAdjustIsolatedMarginRequest(
      signedFields
    );

    await this.tradeApi.putAdjustIsolatedMargin({
      signedFields,
      signature,
    });
    console.log('Adjust isolated margin request sent:', signedFields);
  }

  public async getAccountPreferences() {
    return await this.accountDataApi.getAccountPreferences();
  }

  public async updateAccountPreferences(
    updateAccountPreferenceRequest: UpdateAccountPreferenceRequest
  ) {
    return await this.accountDataApi.putAccountPreferences(
      updateAccountPreferenceRequest
    );
  }

  // Update Account Group ID. 
  // If groupID is null, it will remove the account from its group.
  // An account may only belong to 1 group at a time.
  public async updateAccountGroupId(updateAccountGroupIdRequest: AccountGroupIdPatch) {
      return await this.accountDataApi.patchAccountGroupID(
        updateAccountGroupIdRequest
      );
  }

  public async deposit(amountE9: string, accountAddress?: string, args?: { sponsored?: boolean, fallbackToExecuteTx?: boolean }) {
    const assetSymbol = 'USDC';
    const txb = new TransactionBlock();

    const assetType = this.assets?.find(
      (x) => x.symbol === assetSymbol
    )?.assetType;
    if (!assetType) {
      throw new Error('Missing USDC asset type');
    }

    const [splitCoin, mergedCoin] = await CoinUtils.createCoinWithBalance(
      this.suiClient,
      txb,
      amountE9,
      assetType,
      this.currentAccountAddress || this.bfSigner.getAddress()
    );

    //build the tx
    this.txBuilder?.depositToAssetBank(
      assetSymbol,
      accountAddress ||
        this.currentAccountAddress ||
        this.bfSigner.getAddress(),
      amountE9,
      splitCoin,
      {
        txBlock: txb
      }
    );
    //add the transfer objects to the tx
    if (mergedCoin) {
      txb.transferObjects(
        [mergedCoin],
        this.currentAccountAddress || this.bfSigner.getAddress()
      );
    }
    if (splitCoin) {
      txb.transferObjects(
        [splitCoin],
        this.currentAccountAddress || this.bfSigner.getAddress()
      );
    }

    if(args && args.sponsored){
      // build the gasless tx payload bytes
      const gaslessTxPayloadBytes = await this.buildGaslessTxPayloadBytes(txb);

      try {
        const request: SponsorTxRequest = {
          txBytes: gaslessTxPayloadBytes,
        };

        // sponsor gas for the transaction
        const sponsorTxApiResponse = await this.accountDataApi.sponsorTx(request);
        const txBlockFromBytes = TransactionBlock.from(sponsorTxApiResponse.data.txBytes);

        // sign the transaction with user's wallet
        const userSignedTx = await this.bfSigner.signTx(txBlockFromBytes, this.suiClient);

        // execute the transaction with both user's signature and sponsor's signature
        const response = await this.bfSigner.executeSponsoredTx(userSignedTx.bytes, userSignedTx.signature, sponsorTxApiResponse.data.signature, this.suiClient);

        return response;
      } catch (error) {
        if(args?.fallbackToExecuteTx){
          return this.bfSigner.executeTx(txb, this.suiClient);
        }
        throw error;
      }
    }
    else {  
      return this.bfSigner.executeTx(txb, this.suiClient);
    }
  }

  /**
   * Batch claim rewards on-chain using RewardsDistributorInteractor
   * @param payload Array of claim payloads with signatures
   * @returns Transaction response
   */
  public async batchClaimRewards(payload: BatchClaimParams[]) {
    // Get the rewards contract config from API (same as useClaimConfigQuery)
    const { data } = await this.rewardsDataApi.getContractConfig();

    if (!data) {
      throw new Error('Failed to get rewards contract config');
    }

    // Get the signer from BluefinRequestSigner
    const signer = (this.bfSigner as any).wallet;

    // Create the RewardsDistributorInteractor
    const interactor = new RewardsDistributorInteractor(
      this.suiClient as any,
      data as any,
      signer,
      this.bfSigner.isUIWallet()
    );

    // Transform payload to the format expected by claimRewardsBatch
    const batch = payload.map(({ signature, sigPayload }) => ({
      signature,
      payload: sigPayload,
    }));

    // Execute the batch claim
    const tx = await interactor.claimRewardsBatch(batch);

    return tx;
  }

  public async refreshToken(force: boolean = false): Promise<void> {
    if (!this.isConnected) return;

    // If already refreshing, wait for that operation to complete
    if (this.isRefreshing) {
      if (this.refreshTokenPromise) {
        await this.refreshTokenPromise;
      }
      return;
    }

    // Check if token needs refreshing
    if (!this.isAccessTokenExpired() && !force) {
      return;
    }

    this.isRefreshing = true;

    try {
      this.refreshTokenPromise = this.performTokenRefresh();
      await this.refreshTokenPromise;
    } finally {
      this.isRefreshing = false;
      this.refreshTokenPromise = null;
    }
  }

  private async performTokenRefresh(): Promise<void> {
    const maxRetries = 5;
    let retryCount = 0;
    let consecutiveNetworkErrors = 0;

    while (retryCount < maxRetries) {
      try {
        console.log(
          `Refreshing token (attempt ${retryCount + 1}/${maxRetries})`
        );

        // Check if refresh token is still valid
        if (!this.isRefreshTokenValid()) {
          console.log('Refresh token is expired or invalid, triggering logout');
          this.logout();
          return;
        }

        await this.loginAndUpdateToken();
        console.log('Token refreshed successfully');
        consecutiveNetworkErrors = 0; // Reset network error counter on success
        return;
      } catch (error) {
        retryCount++;
        console.error(`Error refreshing token (attempt ${retryCount}):`, error);

        // Check if this is a network error that we should retry
        const isNetworkError =
          error instanceof Error &&
          (error.message.includes('Network Error') ||
            error.message.includes('ERR_NETWORK') ||
            error.message.includes('ECONNREFUSED') ||
            error.message.includes('timeout') ||
            error.message.includes('ERR_NETWORK_IO_SUSPENDED'));

        // Check if we're currently offline (browser environment only)
        const isOffline =
          typeof navigator !== 'undefined' &&
          'onLine' in navigator &&
          !navigator.onLine;

        if (isNetworkError) {
          consecutiveNetworkErrors++;
        }

        // If we're offline, don't count this as a "real" error - just wait for network to come back
        if (isOffline) {
          console.log(
            'User is offline, will retry when network comes back online'
          );
          return;
        }

        // If it's not a network error and login failed (tokenResponse is null), logout immediately
        if (!isNetworkError && !this.tokenResponse) {
          console.error('Login failed permanently, triggering logout');
          this.logout();
          return;
        }

        // If we've had too many consecutive network errors, pause refresh attempts temporarily
        if (consecutiveNetworkErrors >= 10) {
          // Clear any scheduled refresh
          if (this.updateTokenTimeout) {
            clearTimeout(this.updateTokenTimeout);
            this.updateTokenTimeout = null;
          }

          setTimeout(() => {
            console.log('Resuming token refresh attempts');
            this.scheduleTokenRefresh(); // Reschedule refresh when resuming
          }, 300000); // 5 minutes
          return;
        }

        if (retryCount >= maxRetries && !isOffline) {
          if (isNetworkError) {
            console.warn(
              'Max retries reached due to network errors, will retry later'
            );
            return; // Don't logout for network issues, just wait for next interval
          } else {
            console.error('Max retries reached, triggering logout');
            this.logout();
            return;
          }
        }

        // Exponential backoff: wait 2^retryCount seconds, but cap at 30 seconds for network errors
        const baseDelay = isNetworkError ? 5000 : 2000; // Start with 5s for network errors, 2s for others
        const delayMs = Math.min(
          baseDelay * Math.pow(2, retryCount - 1),
          30000
        );
        console.log(`Retrying in ${delayMs}ms...`);
        await new Promise((resolve) => setTimeout(resolve, delayMs));
      }
    }
  }

  public async createAccountDataStreamListener(
    handler: (data: AccountStreamMessage) => Promise<void>
  ): Promise<WebSocket> {
    const accessToken = await this.getAccessToken();
    return new Promise((resolve) => {
      const ws = new WebSocket(
        this.configs[Services.AccountWebsocket]!.basePath!,
        {
          headers: {
            Authorization: `Bearer ${accessToken}`,
          },
        }
      );
      ws.onmessage = async (event) => {
        await handler(JSON.parse(<string>event.data));
      };
      ws.on('open', () => {
        resolve(ws);
      });
    });
  }

  public async createMarketDataStreamListener(
    handler: (data: MarketStreamMessage) => Promise<void>
  ): Promise<WebSocket> {
    return new Promise((resolve) => {
      const ws = new WebSocket(
        this.configs[Services.MarketWebsocket]!.basePath!
      );
      ws.onmessage = async (event) => {
        await handler(JSON.parse(<string>event.data));
      };
      ws.on('open', () => {
        resolve(ws);
      });
    });
  }

  public async logout(): Promise<void> {
    console.log('Logging out');
    if (this.updateTokenTimeout) {
      clearTimeout(this.updateTokenTimeout);
      this.updateTokenTimeout = null;
    }
    this.tokenResponse = null;
    this.tokenSetAtSeconds = null;
    this.isConnected = false;

    this.onLogout?.();
  }

  public async dispose(): Promise<void> {
    console.log('Disposing SDK resources');

    if (this.updateTokenTimeout) {
      clearTimeout(this.updateTokenTimeout);
      this.updateTokenTimeout = null;
    }

    // Clean up visibility change listener
    if (this.visibilityChangeHandler && typeof document !== 'undefined') {
      document.removeEventListener(
        'visibilitychange',
        this.visibilityChangeHandler
      );
      this.visibilityChangeHandler = undefined;
    }

    // Clean up network change listeners
    if (typeof window !== 'undefined') {
      if (this.onlineHandler) {
        window.removeEventListener('online', this.onlineHandler);
        this.onlineHandler = undefined;
      }
      if (this.offlineHandler) {
        window.removeEventListener('offline', this.offlineHandler);
        this.offlineHandler = undefined;
      }
    }

    this.isConnected = false;
  }

  /**
   * @description
   * build gasless transaction payload bytes
   * @param tx transcation block
   * @returns string
   * */

  private buildGaslessTxPayloadBytes = async (txb: TransactionBlock): Promise<string> => {
    try {
      return await SuiBlocks.buildGaslessTxPayloadBytes(txb, this.suiClient);
    } catch (error) {
      throw new Error(error instanceof Error ? error.message : 'Build gasless tx payload bytes failed');
    }
  };
}
