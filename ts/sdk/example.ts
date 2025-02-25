import {
  AccountDataStream,
  TradeTypeEnum,
  TransactionTypeEnum,
  KlineInterval,
  CandlePriceType,
  OrderType,
  OrderSide,
  MarketDataStreamName,
  MarketSubscriptionStreams,
  MarketStreamMessage,
  MarketEventType,
  AccountStreamMessage,
  AccountEventType,
  OrderTimeInForce,
} from "./index";

import { Ed25519Keypair } from "@mysten/sui/keypairs/ed25519";
import { BluefinProSdk, OrderParams } from "./index";
import { BluefinRequestSigner, makeAddressableKeyPair } from "./index";
import { hexToBytes } from "@noble/hashes/utils";

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

async function main() {
  // Create wallet from mnemonic
  // const suiWallet = Ed25519Keypair.deriveKeypair(
  //   // "dilemma salmon lake ceiling moral glide cute that ginger float area aunt vague remind cage mother concert inch dizzy present proud program time urge",
  //   "know puzzle puzzle table miss member token image loop velvet skin legend clarify affair wisdom alert lucky unveil mean two question nice spatial grape"
  // );

  const suiWallet = Ed25519Keypair.fromSecretKey(
    hexToBytes(
      "3427d19dcf5781f0874c36c78aec22c03acda435d69efcbf249e8821793567a1"
    )
  );

  logger.info(`Sui Address: ${suiWallet.getPublicKey().toSuiAddress()}`);

  const bfSigner = new BluefinRequestSigner(makeAddressableKeyPair(suiWallet));
  const client = new BluefinProSdk(bfSigner, "devnet", undefined);
  await client.initialize();

  try {
    // Get Market Data from Exchange Data API
    const exchangeInfo = (await client.exchangeDataApi.getExchangeInfo()).data;
    logger.info(`Exchange Info: ${JSON.stringify(exchangeInfo)}`);

    // Find SUI-PERP market
    const perpMarket = exchangeInfo.markets.find(m => m.symbol === 'SUI-PERP');
    if (!perpMarket) {
      throw new Error('SUI-PERP market not found');
    }
    logger.info(`Selected market: ${JSON.stringify(perpMarket)}`);

    const symbol = perpMarket.symbol;

    // Get market data
    const candleStick = (
      await client.exchangeDataApi.getCandlestickData(
        symbol,
        KlineInterval._1m,
        CandlePriceType.Oracle
      )
    ).data;
    logger.info(`Candle stick: ${JSON.stringify(candleStick)}`);

    const depth = (await client.exchangeDataApi.getOrderbookDepth(symbol)).data;
    logger.info(`Depth: ${JSON.stringify(depth)}`);

    const ticker = (await client.exchangeDataApi.getMarketTicker(symbol)).data;
    logger.info(`Exchange Market ticker: ${JSON.stringify(ticker)}`);

    const recentTrades = (await client.exchangeDataApi.getRecentTrades(symbol))
      .data;
    logger.info(`Recent Trades: ${JSON.stringify(recentTrades)}`);

    const fundingRateHistory = (
      await client.exchangeDataApi.getFundingRateHistory(symbol)
    ).data;
    logger.info(`Funding Rate History: ${JSON.stringify(fundingRateHistory)}`);

    // Account Data API calls
    const accountTrades = (
      await client.accountDataApi.getAccountTrades(
        symbol,
        Date.now() - 10000000,
        Date.now(),
        1000,
        TradeTypeEnum.Order,
        1
      )
    ).data;
    logger.info(`Trades History ${JSON.stringify(accountTrades)}`);

    const depositHistory = (
      await client.accountDataApi.getAccountTransactionHistory(
        [TransactionTypeEnum.Deposit, TransactionTypeEnum.Withdraw],
        symbol,
        Date.now() - 10000000,
        Date.now(),
        1000,
        1
      )
    ).data;
    logger.info(`Deposits history: ${JSON.stringify(depositHistory)}`);

    const accountDetails = (await client.accountDataApi.getAccountDetails())
      .data;
    logger.info(`Account Details: ${JSON.stringify(accountDetails)}`);

    const accountPreferences = (
      await client.accountDataApi.getAccountPreferences()
    ).data;
    logger.info(`Account Preferences: ${JSON.stringify(accountPreferences)}`);

    // Set up WebSocket listeners
    const accountDataListener = await client.createAccountDataStreamListener(
      "devnet",
      handleAccountDataEvent
    );
    const marketDataListener = await client.createMarketDataStreamListener(
      "devnet",
      handleMarketDataEvent
    );

    await accountDataListener.send(
      JSON.stringify({
        method: "Subscribe",
        dataStreams: [
          AccountDataStream.AccountOrderUpdate,
          AccountDataStream.AccountPositionUpdate,
          AccountDataStream.AccountTradeUpdate,
          AccountDataStream.AccountTransactionUpdate,
          AccountDataStream.AccountUpdate,
        ],
      })
    );

    await marketDataListener.send(
      JSON.stringify({
        method: "Subscribe",
        dataStreams: [
          {
            symbol: symbol,
            streams: [
              MarketDataStreamName.MarkPrice,
              MarketDataStreamName.RecentTrade,
              MarketDataStreamName.OraclePrice,
              MarketDataStreamName.Ticker,
              MarketDataStreamName.TickerAll,
              MarketDataStreamName.DiffDepth500Ms,
              MarketDataStreamName.PartialDepth5,
            ],
          },
        ],
      })
    );

    // Place order
    const orderParams: OrderParams = {
      clientOrderId: "123456",
      type: OrderType.Limit,
      symbol: symbol,
      priceE9: "10000000",
      quantityE9: "100000000000",
      side: OrderSide.Long,
      leverageE9: "1000000000",
      isIsolated: false,
      expiresAtUtcMillis: Date.now() + 6 * 60 * 1000,
      postOnly: false,
      reduceOnly: false,
      timeInForce: OrderTimeInForce.Gtt,
    };

    const orderCreationResult = (await client.createOrder(orderParams)).data;
    logger.info(
      `Order Creation Result: ${JSON.stringify(orderCreationResult)}`
    );

    logger.info("Update Leverage to 2");
    await client.updateLeverage(symbol, "2000000000");

    // Withdraw 10 USD
    await client.withdraw("USDC", "10000000000");
    logger.info("Withdraw request success");

    // Keep connection alive
    await new Promise((resolve) => setTimeout(resolve, 50000));
  } finally {
    await client.dispose();
  }
}

// Run the main function
main().catch((error) => {
  console.error("Error in main:", error);
  process.exit(1);
});
