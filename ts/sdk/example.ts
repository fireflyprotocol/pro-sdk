import {
  AccountDataStream,
  TradeType,
  TransactionType,
  KlineInterval,
  CandlePriceType,
  OrderType,
  OrderSide,
  MarketDataStreamName,
  MarketStreamMessage,
  MarketEventType,
  AccountStreamMessage,
  AccountEventType,
  OrderTimeInForce,
  OrderParams,
  Market,
  BluefinRequestSigner,
  BluefinProSdk,
  makeSigner,
} from "./index";

import { hexToBytes } from "@noble/hashes/utils";
import { SuiClient, Ed25519Keypair } from "@firefly-exchange/library-sui";

import { bech32 } from "@scure/base";

// Configure logging
const logger = {
  info: (message: string) =>
    console.log(new Date().toISOString(), "-", message),
  error: (message: string) =>
    console.error(new Date().toISOString(), "-", message),
};

// Handle market data stream events
async function handleMarketDataEvent(msg: MarketStreamMessage): Promise<void> {
  if (msg.event == MarketEventType.OraclePriceUpdate) {
    logger.info(`OraclePriceUpdate: ${JSON.stringify(msg)}`);
  }
  if (msg.event == MarketEventType.MarkPriceUpdate) {
    logger.info(`MarkPriceUpdate: ${JSON.stringify(msg)}`);
  }
  if (msg.event == MarketEventType.MarketPriceUpdate) {
    logger.info(`MarketPriceUpdate: ${JSON.stringify(msg)}`);
  }
  if (msg.event == MarketEventType.CandlestickUpdate) {
    logger.info(`CandlestickUpdate: ${JSON.stringify(msg)}`);
  }
  if (msg.event == MarketEventType.OrderbookDiffDepthUpdate) {
    logger.info(`OrderbookDiffDepthUpdate: ${JSON.stringify(msg)}`);
  }
  if (msg.event == MarketEventType.OrderbookPartialDepthUpdate) {
    logger.info(`OrderbookPartialDepthUpdate: ${JSON.stringify(msg)}`);
  }
  if (msg.event == MarketEventType.RecentTradesUpdates) {
    logger.info(`RecentTradesUpdates: ${JSON.stringify(msg)}`);
  }
  if (msg.event == MarketEventType.TickerAllUpdate) {
    logger.info(`TickerAllUpdate: ${JSON.stringify(msg)}`);
  }
  if (msg.event == MarketEventType.TickerUpdate) {
    logger.info(`TickerUpdate: ${JSON.stringify(msg)}`);
  }
}

// Handle account data stream events
async function handleAccountDataEvent(
  msg: AccountStreamMessage
): Promise<void> {
  if (msg.event == AccountEventType.AccountUpdate) {
    logger.info(`AccountUpdate: ${JSON.stringify(msg)}`);
  }
  if (msg.event == AccountEventType.AccountTradeUpdate) {
    logger.info(`AccountTradeUpdate: ${JSON.stringify(msg)}`);
  }
  if (msg.event == AccountEventType.AccountPositionUpdate) {
    logger.info(`AccountPositionUpdate: ${JSON.stringify(msg)}`);
  }
  if (msg.event == AccountEventType.AccountOrderUpdate) {
    // if (msg.actual_instance instanceof ActiveOrderUpdate) {
    //   logger.info(`ActiveOrderUpdate: ${JSON.stringify(msg.actual_instance)}`);
    // }
    // if (msg.actual_instance instanceof OrderCancellationUpdate) {
    //   logger.info(
    //     `OrderCancellationUpdate: ${JSON.stringify(msg.actual_instance)}`
    //   );
    // }
  }
}

export function base64ToHex(data: string): string {
  return Buffer.from(data, "base64").toString("hex");
}

export function bech32ToHex(bech32String: string): string {
  const { words } = bech32.decode(bech32String as any);
  const bytes = bech32.fromWords(words);
  return Buffer.from(bytes).toString("hex").slice(2); // skip leading "00"
}

async function main() {
  const keys = Ed25519Keypair.deriveKeypair(
    "hair scan buyer choice pretty budget involve day rare manual history scrap"
  );

  const public_key = base64ToHex(keys.getPublicKey().toBase64());
  const private_key = bech32ToHex(keys.getSecretKey());

  // Should match the address of the original passphrase (above).
  let address = Ed25519Keypair.fromSecretKey(hexToBytes(private_key)).toSuiAddress();

  console.log({ public_key, private_key, address });
}

// Run the main function
main().catch((error) => {
  console.error("Error in main:", error);
  process.exit(1);
});
