# Bluefin Pro SDK for TypeScript/JavaScript

TypeScript/JavaScript SDK for interacting with the Bluefin Pro exchange API and Sui blockchain contracts.

[![npm version](https://badge.fury.io/js/@bluefin-exchange%2Fpro-sdk.svg)](https://www.npmjs.com/package/@bluefin-exchange/pro-sdk)

## Installation

Install the SDK using npm or yarn:

```bash
npm install @bluefin-exchange/pro-sdk
# or
yarn add @bluefin-exchange/pro-sdk
```

### Peer Dependencies

The SDK requires the following peer dependencies:

```bash
npm install @mysten/sui
```

## Quick Start

### 1. Initialize the SDK

```typescript
import {
  BluefinProSdk,
  BluefinRequestSigner,
  makeSigner,
} from "@bluefin-exchange/pro-sdk";
import { SuiClient, Ed25519Keypair } from "@firefly-exchange/library-sui";
import { hexToBytes } from "@noble/hashes/utils";

// Create wallet from private key
const wallet = Ed25519Keypair.fromSecretKey(
  hexToBytes("YOUR_PRIVATE_KEY_HEX")
);

// Or create wallet from mnemonic
// const wallet = Ed25519Keypair.deriveKeypair("your mnemonic phrase here");

// Create request signer
const signer = new BluefinRequestSigner(makeSigner(wallet, false));

// Initialize SDK client
const client = new BluefinProSdk(
  signer,
  "mainnet", // or "testnet" for staging, "devnet" for development
  new SuiClient({ url: "https://fullnode.mainnet.sui.io:443" })
);

// Initialize the SDK (authenticates and loads exchange configuration)
await client.initialize();
```

### 2. Get Market Data

```typescript
// Get exchange information and available markets
const exchangeInfo = await client.exchangeDataApi.getExchangeInfo();
console.log("Available markets:", exchangeInfo.data.markets);

// Get market ticker
const ticker = await client.exchangeDataApi.getMarketTicker("BTC-PERP");
console.log("BTC-PERP ticker:", ticker.data);

// Get orderbook depth
const orderbook = await client.exchangeDataApi.getOrderbookDepth("BTC-PERP");
console.log("Orderbook:", orderbook.data);

// Get recent trades
const trades = await client.exchangeDataApi.getRecentTrades("BTC-PERP");
console.log("Recent trades:", trades.data);
```

### 3. Place an Order

```typescript
import { OrderType, OrderSide, OrderTimeInForce } from "@bluefin-exchange/pro-sdk";

const orderParams = {
  clientOrderId: "unique-order-id-123",
  type: OrderType.Limit,
  symbol: "BTC-PERP",
  priceE9: "45000000000000", // Price in E9 format (45000 * 1e9)
  quantityE9: "100000000", // Quantity in E9 format (0.1 BTC * 1e9)
  side: OrderSide.Long,
  leverageE9: "2000000000", // 2x leverage
  isIsolated: false,
  expiresAtMillis: Date.now() + 24 * 60 * 60 * 1000, // 24 hours
  postOnly: false,
  reduceOnly: false,
  timeInForce: OrderTimeInForce.Gtt,
};

const result = await client.createOrder(orderParams);
console.log("Order created:", result.data);
```

### 4. Get Account Information

```typescript
import { TradeType } from "@bluefin-exchange/pro-sdk";

// Get account details
const accountDetails = await client.accountDataApi.getAccountDetails(
  wallet.getPublicKey().toSuiAddress()
);
console.log("Account details:", accountDetails.data);

// Get account trade history
const tradeHistory = await client.accountDataApi.getAccountTrades(
  "BTC-PERP",
  Date.now() - 7 * 24 * 60 * 60 * 1000, // Last 7 days
  Date.now(),
  100,
  TradeType.Order,
  1
);
console.log("Trade history:", tradeHistory.data);
```

## WebSocket Streams

### Market Data Stream

Subscribe to real-time market data updates:

```typescript
import { MarketDataStreamName } from "@bluefin-exchange/pro-sdk";

// Create market data stream listener
const marketListener = await client.createMarketDataStreamListener(
  async (msg) => {
    console.log("Market update:", msg);
  }
);

// Subscribe to market data streams
await marketListener.send(
  JSON.stringify({
    method: "Subscribe",
    dataStreams: [
      {
        symbol: "BTC-PERP",
        streams: [
          MarketDataStreamName.Ticker,
          MarketDataStreamName.RecentTrade,
          MarketDataStreamName.PartialDepth5,
        ],
      },
    ],
  })
);
```

### Account Data Stream

Subscribe to account updates:

```typescript
import { AccountDataStream } from "@bluefin-exchange/pro-sdk";

// Create account data stream listener
const accountListener = await client.createAccountDataStreamListener(
  async (msg) => {
    console.log("Account update:", msg);
  }
);

// Subscribe to account data streams
await accountListener.send(
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
```

## Advanced Features

### Deposit and Withdraw

```typescript
// Deposit USDC to exchange (amount in base units)
await client.deposit("1000000000"); // 1 USDC (1 * 1e9)

// Withdraw USDC from exchange
await client.withdraw("USDC", "500000000"); // 0.5 USDC (0.5 * 1e9)
```

### Update Leverage

```typescript
// Update leverage for a specific market
await client.updateLeverage("BTC-PERP", "5000000000"); // 5x leverage
```

### Adjust Isolated Margin

```typescript
// Add margin to isolated position
await client.adjustIsolatedMargin("BTC-PERP", "100000000000", true); // Add 100 USDC
```

### Batch Claim Rewards

```typescript
// Get available rewards
const rewards = await client.rewardsDataApi.getCampaignRewards(
  "TRADE_AND_EARN",
  wallet.getPublicKey().toSuiAddress()
);

// Claim rewards (see docs/batch-claim-rewards-guide.md for detailed guide)
const claimPayloads = /* transform rewards to BatchClaimParams */;
const txResult = await client.batchClaimRewards(claimPayloads);
```

See [docs/batch-claim-rewards-guide.md](./docs/batch-claim-rewards-guide.md) for a complete guide on claiming rewards.

## API Reference

The SDK provides access to the following API categories:

- **ExchangeDataApi**: Market data, tickers, orderbooks, candles, funding rates
- **AccountDataApi**: Account details, positions, orders, trade history, transactions
- **TradeApi**: Order placement, cancellation, leverage updates
- **AuthApi**: Authentication and session management
- **RewardsApi**: Campaign rewards and claiming

For detailed API documentation, see the auto-generated docs in the [src/docs](./src/docs) directory.

## Environment Configuration

The SDK supports three environments:

- **mainnet**: Production environment
- **testnet**: Staging environment for testing
- **devnet**: Development environment

Each environment has different API endpoints configured automatically.

See [example.ts](./example.ts) for the complete example code.

