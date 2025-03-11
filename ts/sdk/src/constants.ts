import { EnvironmentConfigurations } from "./types";

export const ENVIRONMENT_CONFIGURATIONS: EnvironmentConfigurations = {
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