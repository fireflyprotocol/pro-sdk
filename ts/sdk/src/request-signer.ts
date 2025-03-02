import {
  Keypair,
  Signer,
  parseSerializedSignature,
} from "@mysten/sui/cryptography";
import {
  LoginRequest,
  AccountPositionLeverageUpdateRequestSignedFields,
  CreateOrderRequestSignedFields,
  WithdrawRequestSignedFields,
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
  signLoginRequest: (request: LoginRequest) => Promise<string>;
  executeTx: (
    txb: TransactionBlock,
    suiClient: SuiClient
  ) => Promise<DryRunTransactionBlockResponse | SuiTransactionBlockResponse>;
}

export interface IAddressable {
  getAddress(): string;
}

export function makeAddressableKeyPair<T extends Keypair>(
  keyPair: T
): T & IAddressable {
  return Object.assign(keyPair, {
    getAddress: () => keyPair.getPublicKey().toSuiAddress(),
  });
}

// ------- ISerializables ---------

// Enums
enum ClientPayloadType {
  WithdrawRequest = "Bluefin Pro Withdrawal",
  OrderRequest = "Bluefin Pro Order",
  LeverageAdjustment = "Bluefin Pro Leverage Adjustment",
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
    signedAt: val.signedAtUtcMillis.toString(),
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
    expiration: val.expiresAtUtcMillis.toString(),
    salt: val.salt,
    signedAt: val.signedAtUtcMillis.toString(),
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
    signedAt: val.signedAtUtcMillis.toString(),
  };
}

// ---------- Utils ----------

function toJson(
  val:
    | UICreateOrderRequest
    | UIUpdateAccountPositionLeverageRequest
    | UIWithdrawRequest
): string {
  return JSON.stringify(val, null, 2);
}

// --------------------------------

export class BluefinRequestSigner implements IBluefinSigner {
  constructor(
    private readonly wallet: Pick<Signer, "signPersonalMessage"> & IAddressable
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

  async executeTx(txb: TransactionBlock, suiClient: SuiClient) {
    return SuiBlocks.execCall(txb, suiClient, this.wallet, false);
  }

  /**
   * Get the wallet's address
   */
  getAddress(): string {
    return this.wallet.getAddress();
  }
}
