# OrderStatus

The statuses of the Orders  PENDING: The Order has been acknowledged and is currently pending  OPEN: The Order has passed all checks and is open on the order book.  PARTIALLY_FILLED: The Order has been matched for less than its full quantity. Part of the Order got filled and the rest is still OPEN  FILLED: The Order has been fully matched and filled and is no longer on the order book and no longer cancellable.  CANCELLED: The Order has been cancelled and is no longer able to be filled. It has been taken off the Order Book  REJECTED: The Order has been rejected and has not been put on the order book. 

## Enum

* `OPEN` (value: `'OPEN'`)

* `PENDING` (value: `'PENDING'`)

* `FILLED` (value: `'FILLED'`)

* `PARTIALLY_FILLED` (value: `'PARTIALLY_FILLED'`)

* `CANCELLED` (value: `'CANCELLED'`)

* `REJECTED` (value: `'REJECTED'`)

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


