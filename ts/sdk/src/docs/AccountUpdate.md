# AccountUpdate

Account information for the data stream.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
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
**updatedAtMillis** | **number** | Last update time in milliseconds since Unix epoch. | [default to undefined]
**assets** | [**Array&lt;Asset&gt;**](Asset.md) |  | [default to undefined]
**authorizedAccounts** | **Array&lt;string&gt;** | The accounts that are authorized to trade on behalf of the current account. | [default to undefined]

## Example

```typescript
import { AccountUpdate } from '@bluefin/api-client';

const instance: AccountUpdate = {
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
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
