# Bluefin Pro SDK for Rust

## Overview

The **Bluefin Pro SDK for Rust** allows developers to interact with the Bluefin Pro platform using generated client
libraries from OpenAPI 3.0 specifications. This SDK enables seamless interaction with REST and WebSocket APIs to perform
various trading operations, such as placing orders, withdrawing funds, and subscribing to market feeds.

Generate rust client code command
```openapi-generator generate --input-spec-root-directory ../resources -c gen/config.yaml -g rust-axum -o gen/bluefin-api```

## Features

- **REST API Client**: Interact with Bluefin Pro’s REST APIs to perform operations such as creating orders, managing
  accounts, and retrieving trading data.
- **WebSocket API Client**: Subscribable channels for real-time market data and updates, generated from OpenAPI specs.
- **Signature Utilities**: Built-in cryptographic utilities to sign `create order` and `withdraw` requests securely.
- **Environment Support**: Example configurations and code provided to connect to **TESTNET** or **MAINNET**
  environments.

---

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
bluefin-sdk = "0.1.0"
```

*Note*: Replace `0.1.0` with the latest version available.

---

## Resources and API Support

### REST APIs

The REST APIs are grouped by functionality and defined within the OpenAPI files located in the `resources` folder of the
repository. These APIs include:

- **Authentication**: Authenticate through Bluefin and get the Authorization token for REST API and WebSocket operations
- **Trade**: Place or cancel orders, fetch open orders.
- **Account Management**: View and manage user account information, update account leverage for market addresses.
- **Exchange Info**: View the public information of the exchange

> Example usage for REST APIs can be found in the `examples` folder.

### WebSocket APIs

WebSocket APIs enable real-time market data and event subscription. Documentation for WebSocket messages is available
within the OpenAPI specs in `resources/websocket-api.yaml` file.

- **Market**: Real-time market updates such as Ticker, Recent Trades, Oracle Price and more
- **Account**: Real-time account updates such as Order Updates, Balance Changes and more (requires authentication token)

> Example usage for WebSocket APIs can be found in the `examples` folder. Websocket examples are suffixed by
`-stream`

---

## Examples

The SDK provides example code to demonstrate how to use the APIs.

- **Connecting to TESTNET or MAINNET**: The examples are configured for both TESTNET and MAINNET environments. You can
  toggle between environments based on your use case.
- **Signing Requests**: Ensure secure interactions by leveraging the built-in signing utilities before making
  `create order`, `update leverage` and `withdraw` requests.

To run examples, navigate to the `examples` directory and execute them using:

```bash
cargo run --example <example_name>
```

> Replace `<example_name>` with the specific example file you wish to execute.

> The SDK provides a test user with hex encoded ed25519 public key and private key available to run examples through
> TEST_ACCOUNT_ADDRESS, TEST_ACCOUNT_PUBLIC_KEY, TEST_ACCOUNT_PRIVATE_KEY

---

## Utility Functions

The SDK comes with built-in support for signing specific API requests:

- **Sign `Create Order` Requests**: Cryptographically sign your order requests securely.
- **Sign `Withdraw` Requests**: Cryptographically sign your withdrawal requests.
- **Sign `Update Leverage` Requests**: Cryptographically sign your update leverage requests.

For other API interactions, refer to the examples folder for detailed guidance.

---

## Questions?

If you encounter an issue or have any questions, please feel free to reach out via GitHub issues or the support team.
