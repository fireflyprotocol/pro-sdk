# AccountStreamMessagePayload

The payload of the message, which varies based on the event type.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**groupId** | **string** | The optional group ID of the account. | [optional] [default to undefined]
**tradingFees** | [**TradingFees**](TradingFees.md) |  | [optional] [default to undefined]
**canTrade** | **boolean** | If the user can trade. | [default to undefined]
**canDeposit** | **boolean** | If the current user can deposit to the account. | [default to undefined]
**canWithdraw** | **boolean** | If the current user can withdraw from the account. | [default to undefined]
**crossEffectiveBalanceE9** | **string** | Total effective balance in USD (e9 format). | [default to undefined]
**crossMarginRequiredE9** | **string** | The sum of initial margin required across all cross positions (e9 format). | [default to undefined]
**totalOrderMarginRequiredE9** | **string** | The sum of initial margin required across all open orders (e9 format). | [default to undefined]
**marginAvailableE9** | **string** | The amount of margin available to open new positions and orders (e9 format). | [default to undefined]
**crossMaintenanceMarginRequiredE9** | **string** | The sum of maintenance margin required across all cross positions (e9 format). | [default to undefined]
**crossMaintenanceMarginAvailableE9** | **string** | The amount of margin available before liquidation (e9 format). | [default to undefined]
**crossMaintenanceMarginRatioE9** | **string** | The ratio of the maintenance margin required to the account value (e9 format). | [default to undefined]
**crossLeverageE9** | **string** | The leverage of the account (e9 format). | [default to undefined]
**totalUnrealizedPnlE9** | **string** | Total unrealized profit (e9 format). | [default to undefined]
**crossUnrealizedPnlE9** | **string** | Unrealized profit of cross positions (e9 format). | [default to undefined]
**crossUnrealizedLossE9** | **string** | An implicitly negative number that sums only the losses of all cross positions. | [default to undefined]
**crossAccountValueE9** | **string** | The total value of the cross account, combining the cross effective balance and unrealized PnL across all cross positions, and subtracting any pending funding payments on any cross position.  | [default to undefined]
**totalAccountValueE9** | **string** | The total value of the account, combining the total effective balance and unrealized PnL across all positions, and subtracting any pending funding payments on any position.  | [default to undefined]
**updatedAtMillis** | **number** | The last update time for the position in milliseconds. | [default to undefined]
**assets** | [**Array&lt;Asset&gt;**](Asset.md) |  | [default to undefined]
**authorizedAccounts** | **Array&lt;string&gt;** | Deprecated: Replaced with authorizedWallets. | [default to undefined]
**authorizedWallets** | [**Array&lt;AuthorizedWallet&gt;**](AuthorizedWallet.md) | The wallets that are authorized to trade on behalf of the current account. | [default to undefined]
**trade** | [**Trade**](Trade.md) |  | [default to undefined]
**orderHash** | **string** | The unique hash of the order. | [default to undefined]
**clientOrderId** | **string** | The client-provided order ID. | [optional] [default to undefined]
**symbol** | **string** | The symbol of the market. | [default to undefined]
**accountAddress** | **string** | The address of the account. | [default to undefined]
**priceE9** | **string** | The price of the order in scientific notation with 9 decimal places. | [default to undefined]
**quantityE9** | **string** | The quantity of the order in scientific notation with 9 decimal places. | [default to undefined]
**filledQuantityE9** | **string** | The filled quantity of the order in scientific notation with 9 decimal places. | [default to undefined]
**side** | [**PositionSide**](PositionSide.md) |  | [default to undefined]
**leverageE9** | **string** | The leverage of the order in scientific notation with 9 decimal places. | [default to undefined]
**isIsolated** | **boolean** | Indicates if the position is isolated. | [default to undefined]
**salt** | **string** | A unique salt for the order. | [default to undefined]
**expiresAtMillis** | **number** | The expiration timestamp of the order in milliseconds. | [default to undefined]
**signedAtMillis** | **number** | The signing timestamp of the order in milliseconds. | [default to undefined]
**signerAddress** | **string** | The address of the signer of the order request. | [default to undefined]
**type** | [**OrderType**](OrderType.md) |  | [default to undefined]
**reduceOnly** | **boolean** | Indicates if the order is reduce-only. | [default to undefined]
**postOnly** | **boolean** | Indicates if the order is post-only. | [default to undefined]
**timeInForce** | [**OrderTimeInForce**](OrderTimeInForce.md) |  | [default to undefined]
**triggerPriceE9** | **string** | The trigger price for stop-limit or stop-market orders. | [optional] [default to undefined]
**status** | [**OrderStatus**](OrderStatus.md) |  | [default to undefined]
**selfTradePreventionType** | [**SelfTradePreventionType**](SelfTradePreventionType.md) |  | [default to undefined]
**createdAtMillis** | **number** | The timestamp of the order creation in milliseconds. | [default to undefined]
**cancellationReason** | [**OrderCancelReason**](OrderCancelReason.md) |  | [default to undefined]
**failureToCancelReason** | [**OrderCancellationFailureReason**](OrderCancellationFailureReason.md) |  | [optional] [default to undefined]
**remainingQuantityE9** | **string** | The remaining quantity of the order. | [default to undefined]
**transactionType** | [**TransactionType**](TransactionType.md) |  | [default to undefined]
**amountE9** | **string** | The amount of the transaction in scientific notation with 9 decimal places. | [default to undefined]
**assetSymbol** | **string** | The symbol of the asset. | [optional] [default to undefined]
**tradeId** | **string** | The trade ID associated with the transaction. | [optional] [default to undefined]
**executedAtMillis** | **number** | The timestamp when the transaction was executed in milliseconds. | [default to undefined]
**avgEntryPriceE9** | **string** | The average entry price for the position. | [default to undefined]
**clientSetLeverageE9** | **string** | The leverage applied to the position. | [default to undefined]
**liquidationPriceE9** | **string** | The liquidation price of the position. | [default to undefined]
**markPriceE9** | **string** | The current mark price of the position. | [default to undefined]
**notionalValueE9** | **string** | The notional value of the position. | [default to undefined]
**sizeE9** | **string** | The size of the position. | [default to undefined]
**unrealizedPnlE9** | **string** | The unrealized profit and loss for the position. | [default to undefined]
**marginRequiredE9** | **string** | The margin required for the position. | [default to undefined]
**maintenanceMarginE9** | **string** | The maintenance margin required for the position. | [default to undefined]
**isolatedMarginE9** | **string** | The isolated margin applied to the position. | [default to undefined]
**reason** | **string** | The reason for the failure. | [default to undefined]
**reasonCode** | [**CommandFailureReasonCode**](CommandFailureReasonCode.md) |  | [optional] [default to undefined]
**failedCommandType** | **string** | The type of command that failed. | [default to undefined]
**failedCommandTypeCode** | [**FailedCommandType**](FailedCommandType.md) |  | [optional] [default to undefined]
**failedAtMillis** | **number** | The timestamp when the command failed in milliseconds. | [default to undefined]

## Example

```typescript
import { AccountStreamMessagePayload } from '@bluefin/api-client';

const instance: AccountStreamMessagePayload = {
    groupId,
    tradingFees,
    canTrade,
    canDeposit,
    canWithdraw,
    crossEffectiveBalanceE9,
    crossMarginRequiredE9,
    totalOrderMarginRequiredE9,
    marginAvailableE9,
    crossMaintenanceMarginRequiredE9,
    crossMaintenanceMarginAvailableE9,
    crossMaintenanceMarginRatioE9,
    crossLeverageE9,
    totalUnrealizedPnlE9,
    crossUnrealizedPnlE9,
    crossUnrealizedLossE9,
    crossAccountValueE9,
    totalAccountValueE9,
    updatedAtMillis,
    assets,
    authorizedAccounts,
    authorizedWallets,
    trade,
    orderHash,
    clientOrderId,
    symbol,
    accountAddress,
    priceE9,
    quantityE9,
    filledQuantityE9,
    side,
    leverageE9,
    isIsolated,
    salt,
    expiresAtMillis,
    signedAtMillis,
    signerAddress,
    type,
    reduceOnly,
    postOnly,
    timeInForce,
    triggerPriceE9,
    status,
    selfTradePreventionType,
    createdAtMillis,
    cancellationReason,
    failureToCancelReason,
    remainingQuantityE9,
    transactionType,
    amountE9,
    assetSymbol,
    tradeId,
    executedAtMillis,
    avgEntryPriceE9,
    clientSetLeverageE9,
    liquidationPriceE9,
    markPriceE9,
    notionalValueE9,
    sizeE9,
    unrealizedPnlE9,
    marginRequiredE9,
    maintenanceMarginE9,
    isolatedMarginE9,
    reason,
    reasonCode,
    failedCommandType,
    failedCommandTypeCode,
    failedAtMillis,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
