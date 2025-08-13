import {
  Keypair,
  SignatureWithBytes,
  Signer,
  parseSerializedSignature,
} from "@mysten/sui/cryptography";
import {
  LoginRequest,
  AccountPositionLeverageUpdateRequestSignedFields,
  CreateOrderRequestSignedFields,
  WithdrawRequestSignedFields,
  AccountAuthorizationRequestSignedFields,
  AdjustMarginOperation,
  AdjustIsolatedMarginRequestSignedFields,
} from "./api";
import {
  DryRunTransactionBlockResponse,
  SuiBlocks,
  SuiClient,
  SuiTransactionBlockResponse,
  TransactionBlock,
} from "@firefly-exchange/library-sui";

export interface IBluefinSigner {
  getAddress(): string;
  signLeverageUpdateRequest: (
    fields: AccountPositionLeverageUpdateRequestSignedFields
  ) => Promise<string>;
  signOrderRequest: (fields: CreateOrderRequestSignedFields) => Promise<string>;
  signWithdrawRequest: (fields: WithdrawRequestSignedFields) => Promise<string>;
  signAccountAuthorizationRequest: (fields: AccountAuthorizationRequestSignedFields, isAuthorize: boolean) => Promise<string>;
  signAdjustIsolatedMarginRequest: (fields: AdjustIsolatedMarginRequestSignedFields) => Promise<string>;
  signLoginRequest: (request: LoginRequest) => Promise<string>;
  signTx: (txb: Uint8Array | TransactionBlock) => Promise<SignatureWithBytes>;
  executeSponsoredTx: (txBytes: string, userSignature: string, sponsorSignature: string, suiClient: SuiClient) => Promise<DryRunTransactionBlockResponse | SuiTransactionBlockResponse>;
  executeTx: (
    txb: TransactionBlock,
    suiClient: SuiClient
  ) => Promise<DryRunTransactionBlockResponse | SuiTransactionBlockResponse>;
}

export interface ISigner {
  toSuiAddress(): string;
  isUIWallet(): boolean;
}


export function makeSigner<T extends Keypair>(
  keyPair: T,
  isUIWallet: boolean,
): T & ISigner {
  return Object.assign(keyPair, {
    toSuiAddress: () => keyPair.getPublicKey().toSuiAddress(),
    isUIWallet: () => isUIWallet
  });
}

// ------- ISerializables ---------

// Enums
enum ClientPayloadType {
  WithdrawRequest = "Bluefin Pro Withdrawal",
  OrderRequest = "Bluefin Pro Order",
  LeverageAdjustment = "Bluefin Pro Leverage Adjustment",
  AuthorizeAccount = "Bluefin Pro Authorize Account",
  AdjustIsolatedMargin = "Bluefin Pro Margin Adjustment",
}

enum PositionType {
  Isolated = "ISOLATED",
  Cross = "CROSS",
}

// Interfaces
interface UIWithdrawRequest {
  type: string;
  eds: string;
  assetSymbol: string;
  account: string;
  amount: string;
  salt: string;
  signedAt: string;
}

interface UICreateOrderRequest {
  type: string;
  ids: string;
  account: string;
  market: string;
  price: string;
  quantity: string;
  leverage: string;
  side: string;
  positionType: string;
  expiration: string;
  salt: string;
  signedAt: string;
}

interface UIUpdateAccountPositionLeverageRequest {
  type: string;
  ids: string;
  account: string;
  market: string;
  leverage: string;
  salt: string;
  signedAt: string;
}

interface UIAuthorizeAccountRequest {
  type: string;
  ids: string;
  account: string;
  user: string;
  status: boolean;
  salt: string;
  signedAt: string;
}

interface UIDeAuthorizeAccountRequest {
  type: string;
  ids: string;
  account: string;
  user: string;
  status: boolean;
  salt: string;
  signedAt: string;
}

interface UIAdjustIsolatedMarginRequest {
  type: string;
  ids: string;
  account: string;
  market: string;
  add: boolean;
  amount: string;
  salt: string;
  signedAt: string;
}

// Conversion functions
function toUIWithdrawRequest(
  val: WithdrawRequestSignedFields
): UIWithdrawRequest {
  return {
    type: ClientPayloadType.WithdrawRequest,
    eds: val.edsId,
    assetSymbol: val.assetSymbol,
    account: val.accountAddress,
    amount: val.amountE9,
    salt: val.salt,
    signedAt: val.signedAtMillis.toString(),
  };
}

function toUICreateOrderRequest(
  val: CreateOrderRequestSignedFields
): UICreateOrderRequest {
  return {
    type: ClientPayloadType.OrderRequest,
    ids: val.idsId,
    account: val.accountAddress,
    market: val.symbol,
    price: val.priceE9,
    quantity: val.quantityE9,
    leverage: val.leverageE9,
    side: val.side.toString(),
    positionType: val.isIsolated ? PositionType.Isolated : PositionType.Cross,
    expiration: val.expiresAtMillis.toString(),
    salt: val.salt,
    signedAt: val.signedAtMillis.toString(),
  };
}

function toUIUpdateAccountPositionLeverageRequest(
  val: AccountPositionLeverageUpdateRequestSignedFields
): UIUpdateAccountPositionLeverageRequest {
  return {
    type: ClientPayloadType.LeverageAdjustment,
    ids: val.idsId,
    account: val.accountAddress,
    market: val.symbol,
    leverage: val.leverageE9,
    salt: val.salt,
    signedAt: val.signedAtMillis.toString(),
  };
}

function toUIAuthorizeAccountRequest(
  val: AccountAuthorizationRequestSignedFields
): UIAuthorizeAccountRequest {
  return {
    type: ClientPayloadType.AuthorizeAccount,
    ids: val.idsId,
    account: val.accountAddress,
    user: val.authorizedAccountAddress,
    status: true,
    salt: val.salt,
    signedAt: val.signedAtMillis.toString(),
  };
}

function toUIDeAuthorizeAccountRequest(
  val: AccountAuthorizationRequestSignedFields
): UIDeAuthorizeAccountRequest {
  return {
    type: ClientPayloadType.AuthorizeAccount,
    ids: val.idsId,
    account: val.accountAddress,
    user: val.authorizedAccountAddress,
    status: false,
    salt: val.salt,
    signedAt: val.signedAtMillis.toString(),
  };
}

function toUIAdjustIsolatedMarginRequest(
  val: AdjustIsolatedMarginRequestSignedFields
): UIAdjustIsolatedMarginRequest {
  return {
    type: ClientPayloadType.AdjustIsolatedMargin,
    ids: val.idsId,
    account: val.accountAddress,
    market: val.symbol,
    add: val.operation === AdjustMarginOperation.Add,
    amount: val.quantityE9,
    salt: val.salt,
    signedAt: val.signedAtMillis.toString(),
  };
}

// ---------- Utils ----------

function toJson(
  val:
    | UICreateOrderRequest
    | UIUpdateAccountPositionLeverageRequest
    | UIWithdrawRequest
    | UIAuthorizeAccountRequest
    | UIDeAuthorizeAccountRequest
    | UIAdjustIsolatedMarginRequest
): string {
  return JSON.stringify(val, null, 2);
}

// --------------------------------

export class BluefinRequestSigner implements IBluefinSigner {
  constructor(
    private readonly wallet: Pick<Signer, "signPersonalMessage"> & ISigner
  ) {}

  /**
   * Sign the login request
   */
  async signLoginRequest(loginRequest: LoginRequest): Promise<string> {
    const loginRequestJson = JSON.stringify(loginRequest);

    const signedMessage = await this.wallet.signPersonalMessage(
      new TextEncoder().encode(loginRequestJson)
    );
    return signedMessage.signature;
  }

  /**
   * Sign an order request
   */
  async signOrderRequest(
    signedFields: CreateOrderRequestSignedFields
  ): Promise<string> {
    const orderJson = toJson(toUICreateOrderRequest(signedFields));

    const signedMessageSerialized = await this.wallet.signPersonalMessage(
      new TextEncoder().encode(orderJson)
    );

    const parsedSignature = parseSerializedSignature(
      signedMessageSerialized.signature
    );

    if (parsedSignature.signatureScheme == "MultiSig") {
      throw new Error("MultiSig not supported");
    }

    return signedMessageSerialized.signature;
  }

  /**
   * Sign a withdraw request
   */
  async signWithdrawRequest(
    withdrawRequestSignedFields: WithdrawRequestSignedFields
  ): Promise<string> {
    const requestJson = toJson(
      toUIWithdrawRequest(withdrawRequestSignedFields)
    );

    const signedMessageSerialized = await this.wallet.signPersonalMessage(
      new TextEncoder().encode(requestJson)
    );

    const parsedSignature = parseSerializedSignature(
      signedMessageSerialized.signature
    );

    if (parsedSignature.signatureScheme == "MultiSig") {
      throw new Error("MultiSig not supported");
    }

    return signedMessageSerialized.signature;
  }

  /**
   * Sign a leverage update request
   */
  async signLeverageUpdateRequest(
    signedFields: AccountPositionLeverageUpdateRequestSignedFields
  ): Promise<string> {
    const requestJson = toJson(
      toUIUpdateAccountPositionLeverageRequest(signedFields)
    );

    const signedMessageSerialized = await this.wallet.signPersonalMessage(
      new TextEncoder().encode(requestJson)
    );

    const parsedSignature = parseSerializedSignature(
      signedMessageSerialized.signature
    );

    if (parsedSignature.signatureScheme == "MultiSig") {
      throw new Error("MultiSig not supported");
    }

    return signedMessageSerialized.signature;
  }

  /**
   * Sign an account authorization request
   */
  async signAccountAuthorizationRequest(
    signedFields: AccountAuthorizationRequestSignedFields,
    is_authorize: boolean
  ): Promise<string> {
    const requestJson = toJson(
      is_authorize
        ? toUIAuthorizeAccountRequest(signedFields)
        : toUIDeAuthorizeAccountRequest(signedFields)
    );

    const signedMessageSerialized = await this.wallet.signPersonalMessage(
      new TextEncoder().encode(requestJson)
    );

    const parsedSignature = parseSerializedSignature(
      signedMessageSerialized.signature
    );

    if (parsedSignature.signatureScheme == "MultiSig") {
      throw new Error("MultiSig not supported");
    }

    return signedMessageSerialized.signature;
  }

  /**
   * Sign an isolated margin adjustment request
   */
  async signAdjustIsolatedMarginRequest(
    signedFields: AdjustIsolatedMarginRequestSignedFields
  ): Promise<string> {
    const requestJson = toJson(
      toUIAdjustIsolatedMarginRequest(signedFields)
    );

    const signedMessageSerialized = await this.wallet.signPersonalMessage(
      new TextEncoder().encode(requestJson)
    );

    const parsedSignature = parseSerializedSignature(
      signedMessageSerialized.signature
    );

    if (parsedSignature.signatureScheme == "MultiSig") {
      throw new Error("MultiSig not supported");
    }

    return signedMessageSerialized.signature;
  }

  async signTx(txb: Uint8Array | TransactionBlock) {
    return SuiBlocks.signTxBlock(txb, this.wallet, this.wallet.isUIWallet());
  }

  async executeSponsoredTx(txBytes: string, userSignature: string, sponsorSignature: string, suiClient: SuiClient) {
    return SuiBlocks.executeSponsoredTxBlock(txBytes, userSignature, sponsorSignature, suiClient);
  }

  async executeTx(txb: TransactionBlock, suiClient: SuiClient) {
    return SuiBlocks.execCall(txb, suiClient, this.wallet, false, this.wallet.isUIWallet());
  }

  /**
   * Get the wallet's address
   */
  getAddress(): string {
    return this.wallet.toSuiAddress();
  }

  /**
   * Get the wallet type (ui wallet or backend)
   */
  isUIWallet(): boolean {
      return this.wallet.isUIWallet();
  }
}
