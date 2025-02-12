import { blake2b } from "@noble/hashes/blake2b";
import * as crypto from "crypto";
import {
  Keypair,
  Signer,
  parseSerializedSignature,
  SIGNATURE_SCHEME_TO_FLAG,
} from "@mysten/sui/cryptography";
import { bcs } from "@mysten/sui/bcs";
import {
  LoginRequest,
  WithdrawRequest,
  AccountPositionLeverageUpdateRequest,
  AccountPositionLeverageUpdateRequestSignedFields,
  CreateOrderRequestSignedFields,
  WithdrawRequestSignedFields,
} from "./api";
import { toBase64UrlEncoded } from "./utils";

export interface IBluefinSigner {
  getAddress(): string;
  signLeverageUpdateRequest: (
    fields: AccountPositionLeverageUpdateRequestSignedFields
  ) => Promise<AccountPositionLeverageUpdateRequest>;
  signOrderRequest: (
    fields: CreateOrderRequestSignedFields
  ) => Promise<[string, string]>;
  signWithdrawRequest: (
    fields: WithdrawRequestSignedFields
  ) => Promise<WithdrawRequest>;
  signLoginRequest: (request: LoginRequest) => Promise<string>;
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

export const OrderRequestSignedFieldsBCS = bcs.struct(
  "OrderRequestSignedFields",
  {
    symbol: bcs.string(),
    accountAddress: bcs.Address,
    priceE9: bcs.u64(),
    quantityE9: bcs.u64(),
    leverageE9: bcs.u64(),
    side: bcs.string(),
    isIsolated: bcs.bool(),
    expiresAtUtcMillis: bcs.u64(),
    salt: bcs.u64(),
    idsId: bcs.Address,
    signedAtUtcMillis: bcs.u64(),
  }
);

export const LeverageUpdateRequestSignedFieldsBCS = bcs.struct(
  "LeverageUpdateRequestSignedFields",
  {
    accountAddress: bcs.Address,
    symbol: bcs.string(),
    leverageE9: bcs.u64(),
    salt: bcs.u64(),
    idsId: bcs.Address,
    signedAtUtcMillis: bcs.u64(),
  }
);

export const WithdrawRequestSignedFieldsBCS = bcs.struct(
  "WithdrawRequestSignedFields",
  {
    assetSymbol: bcs.string(),
    accountAddress: bcs.Address,
    amountE9: bcs.u64(),
    salt: bcs.u64(),
    edsId: bcs.Address,
    signedAtUtcMillis: bcs.u64(),
  }
);

export const SerializedSignatureBCS = bcs.struct("SerializedSignature", {
  signature: bcs.vector(bcs.u8()),
  publicKey: bcs.vector(bcs.u8()),
  scheme: bcs.u8(),
});

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
    orderRequestSignedFields: CreateOrderRequestSignedFields
  ): Promise<[string, string]> {
    const signableFields: CreateOrderRequestSignedFields = {
      symbol: orderRequestSignedFields.symbol,
      accountAddress: orderRequestSignedFields.accountAddress,
      priceE9: orderRequestSignedFields.priceE9,
      quantityE9: orderRequestSignedFields.quantityE9,
      leverageE9: orderRequestSignedFields.leverageE9,
      side: orderRequestSignedFields.side,
      isIsolated: orderRequestSignedFields.isIsolated,
      expiresAtUtcMillis: orderRequestSignedFields.expiresAtUtcMillis,
      salt: orderRequestSignedFields.salt,
      idsId: orderRequestSignedFields.idsId,
      signedAtUtcMillis: orderRequestSignedFields.signedAtUtcMillis,
    };

    const orderBytes =
      OrderRequestSignedFieldsBCS.serialize(signableFields).toBytes();

    const digest = await (global.crypto ? global.crypto : crypto).subtle.digest(
      "SHA-256",
      orderBytes
    );

    const walletHack: Signer = <Signer>(<unknown>this.wallet);

    const signedMessageSerialized = await this.wallet.signPersonalMessage(
      orderBytes
    );

    const signedMessage = await walletHack.sign(Buffer.from(digest));

    const parsedSignature = parseSerializedSignature(
      signedMessageSerialized.signature
    );

    if (parsedSignature.signatureScheme == "MultiSig") {
      throw new Error("MultiSig not supported");
    }

    const signature = SerializedSignatureBCS.serialize({
      signature: signedMessage,
      publicKey: parsedSignature.publicKey,
      scheme: SIGNATURE_SCHEME_TO_FLAG[parsedSignature.signatureScheme],
    }).toHex();

    return [signature, Buffer.from(digest).toString("hex")];
  }

  /**
   * Sign a withdraw request
   */
  async signWithdrawRequest(
    withdrawRequestSignedFields: WithdrawRequestSignedFields
  ): Promise<WithdrawRequest> {
    const signableFields: WithdrawRequestSignedFields = {
      assetSymbol: withdrawRequestSignedFields.assetSymbol,
      accountAddress: withdrawRequestSignedFields.accountAddress,
      amountE9: withdrawRequestSignedFields.amountE9,
      salt: withdrawRequestSignedFields.salt,
      edsId: withdrawRequestSignedFields.edsId,
      signedAtUtcMillis: withdrawRequestSignedFields.signedAtUtcMillis,
    };

    const requestBytes =
      WithdrawRequestSignedFieldsBCS.serialize(signableFields).toBytes();

    const digest = await (global.crypto ? global.crypto : crypto).subtle.digest(
      "SHA-256",
      requestBytes
    );

    const walletHack: Signer = <Signer>(<unknown>this.wallet);

    const signedMessageSerialized = await this.wallet.signPersonalMessage(
      requestBytes
    );

    const signedMessage = await walletHack.sign(Buffer.from(digest));

    const parsedSignature = parseSerializedSignature(
      signedMessageSerialized.signature
    );

    if (parsedSignature.signatureScheme == "MultiSig") {
      throw new Error("MultiSig not supported");
    }

    const signature = SerializedSignatureBCS.serialize({
      signature: signedMessage,
      publicKey: parsedSignature.publicKey,
      scheme: SIGNATURE_SCHEME_TO_FLAG[parsedSignature.signatureScheme],
    }).toHex();

    return {
      signature,
      requestHash: Buffer.from(digest).toString("hex"),
      signedFields: withdrawRequestSignedFields,
    };
  }

  /**
   * Sign a leverage update request
   */
  async signLeverageUpdateRequest(
    requestSignedFields: AccountPositionLeverageUpdateRequestSignedFields
  ): Promise<AccountPositionLeverageUpdateRequest> {
    const signableFields: AccountPositionLeverageUpdateRequestSignedFields = {
      symbol: requestSignedFields.symbol,
      accountAddress: requestSignedFields.accountAddress,
      leverageE9: requestSignedFields.leverageE9,
      salt: requestSignedFields.salt,
      idsId: requestSignedFields.idsId,
      signedAtUtcMillis: requestSignedFields.signedAtUtcMillis,
    };

    const requestBytes =
      LeverageUpdateRequestSignedFieldsBCS.serialize(signableFields).toBytes();

    const digest = await (global.crypto ? global.crypto : crypto).subtle.digest(
      "SHA-256",
      requestBytes
    );

    const walletHack: Signer = <Signer>(<unknown>this.wallet);

    const signedMessageSerialized = await this.wallet.signPersonalMessage(
      requestBytes
    );

    const signedMessage = await walletHack.sign(Buffer.from(digest));

    const parsedSignature = parseSerializedSignature(
      signedMessageSerialized.signature
    );

    if (parsedSignature.signatureScheme == "MultiSig") {
      throw new Error("MultiSig not supported");
    }

    const signature = SerializedSignatureBCS.serialize({
      signature: signedMessage,
      publicKey: parsedSignature.publicKey,
      scheme: SIGNATURE_SCHEME_TO_FLAG[parsedSignature.signatureScheme],
    }).toHex();

    return {
      signature,
      requestHash: Buffer.from(digest).toString("hex"),
      signedFields: requestSignedFields,
    };
  }

  /**
   * Get the wallet's address
   */
  getAddress(): string {
    return this.wallet.getAddress();
  }
}
