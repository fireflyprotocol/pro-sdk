import {
  OrderType,
  OrderTimeInForce,
  SelfTradePreventionType,
  OrderSide,
} from "./api";

interface ConfigurationParams {
  authHost: string;
  apiHost: string;
  tradeHost: string;
  marketWsHost: string;
  accountWsHost: string;
}

export type Environment = "mainnet" | "testnet" | "devnet";
export type EnvironmentConfigurations = Record<Environment, ConfigurationParams>;

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

export enum Services {
  Account,
  Exchange,
  Trade,
  Auth,
  MarketWebsocket,
  AccountWebsocket,
}

export type BasePathConfig = {
  authHost: string | null;
  apiHost: string | null;
  tradeHost: string | null;
  marketWsHost: string | null;
  accountWsHost: string | null;
};
