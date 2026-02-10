# Batch Claim Rewards - Integration Guide

## Overview

The Bluefin Pro SDK provides a `batchClaimRewards` method to claim multiple rewards in a single on-chain transaction. This guide walks you through the integration process.

## Prerequisites

- Bluefin Pro SDK installed (`@bluefin-exchange/pro-sdk`)
- A funded Sui wallet
- SDK initialized and authenticated

## Step-by-Step Integration

### Step 1: Initialize the SDK

```typescript
import { 
  BluefinProSdk, 
  BluefinRequestSigner, 
  makeSigner,
  BatchClaimParams 
} from "@bluefin-exchange/pro-sdk";
import { SuiClient, Ed25519Keypair } from "@firefly-exchange/library-sui";

// Create wallet from private key or mnemonic
const wallet = Ed25519Keypair.fromSecretKey(hexToBytes("YOUR_PRIVATE_KEY"));
// OR from mnemonic:
// const wallet = Ed25519Keypair.deriveKeypair("your mnemonic phrase here");

// Initialize signer and SDK
const signer = new BluefinRequestSigner(makeSigner(wallet, false));
const sdk = new BluefinProSdk(
  signer,
  "mainnet", // or "testnet" for staging
  new SuiClient({ url: "https://fullnode.mainnet.sui.io:443" })
);

await sdk.initialize();
```

### Step 2: Fetch Available Rewards

```typescript
// Get rewards for a specific campaign
const userAddress = wallet.getPublicKey().toSuiAddress();

const response = await sdk.rewardsDataApi.getCampaignRewards(
  "TRADE_AND_EARN",  // Campaign name
  userAddress
);

const campaignRewards = response.data;
console.log("Available rewards:", campaignRewards);
```

### Step 3: Filter Claimable Rewards

Only rewards with a valid `claimSignature` can be claimed:

```typescript
// Transform API response to BatchClaimParams format
const claimPayloads: BatchClaimParams[] = campaignRewards
  .filter((reward) => reward.claimSignature && reward.claimSignature.length > 0)
  .flatMap((reward) =>
    reward.claimSignature!.map((claimSig) => ({
      sigPayload: {
        target: claimSig.sigPayload.target,
        receiver: claimSig.sigPayload.receiver,
        amount: claimSig.sigPayload.amount,
        expiry: claimSig.sigPayload.expiry,
        nonce: claimSig.sigPayload.nonce,
        type: claimSig.sigPayload.type,
      },
      signature: claimSig.signature,
      rewardType: claimSig.rewardType, // 'Blue', 'Sui', 'Wal', etc.
    }))
  );

console.log(`Found ${claimPayloads.length} claimable rewards`);
```

### Step 4: Execute Batch Claim

```typescript
if (claimPayloads.length > 0) {
  try {
    const txResult = await sdk.batchClaimRewards(claimPayloads);
    console.log("Claim successful!");
    console.log("Transaction digest:", txResult.digest);
    console.log("Status:", txResult.effects?.status);
  } catch (error) {
    console.error("Claim failed:", error);
  }
} else {
  console.log("No rewards available to claim");
}
```

---

## Complete Example

```typescript
import { 
  BluefinProSdk, 
  BluefinRequestSigner, 
  makeSigner,
  BatchClaimParams 
} from "@bluefin-exchange/pro-sdk";
import { SuiClient, Ed25519Keypair } from "@firefly-exchange/library-sui";
import { hexToBytes } from "@noble/hashes/utils";

async function claimRewards() {
  // 1. Setup
  const wallet = Ed25519Keypair.fromSecretKey(hexToBytes("YOUR_PRIVATE_KEY"));
  const signer = new BluefinRequestSigner(makeSigner(wallet, false));
  const sdk = new BluefinProSdk(
    signer,
    "mainnet",
    new SuiClient({ url: "https://fullnode.mainnet.sui.io:443" })
  );
  await sdk.initialize();

  // 2. Fetch rewards
  const userAddress = wallet.getPublicKey().toSuiAddress();
  const { data: rewards } = await sdk.rewardsDataApi.getCampaignRewards(
    "TRADE_AND_EARN",
    userAddress
  );

  // 3. Filter claimable rewards
  const claimPayloads: BatchClaimParams[] = rewards
    .filter((r) => r.claimSignature?.length > 0)
    .flatMap((r) =>
      r.claimSignature!.map((sig) => ({
        sigPayload: {
          target: sig.sigPayload.target,
          receiver: sig.sigPayload.receiver,
          amount: sig.sigPayload.amount,
          expiry: sig.sigPayload.expiry,
          nonce: sig.sigPayload.nonce,
          type: sig.sigPayload.type,
        },
        signature: sig.signature,
        rewardType: sig.rewardType,
      }))
    );

  // 4. Claim
  if (claimPayloads.length > 0) {
    const tx = await sdk.batchClaimRewards(claimPayloads);
    console.log("Success! TX:", tx.digest);
  } else {
    console.log("No rewards to claim");
  }

  await sdk.dispose();
}

claimRewards();
```

---

## Data Types Reference

```typescript
interface BatchClaimParams {
  sigPayload: {
    target: string;    // Reward pool object ID
    receiver: string;  // User's address
    amount: string;    // Reward amount
    expiry: string;    // Signature expiry timestamp
    nonce: string;     // Unique nonce
    type: number;      // Claim type identifier
  };
  signature: string;   // Server-provided signature
  rewardType: string;  // 'Blue', 'Sui', 'Wal', etc.
}
```

---

## Notes

- **Gas fees**: The transaction requires SUI for gas. Ensure your wallet has sufficient balance.
- **Signature expiry**: Claim signatures have an expiry time. Fetch fresh signatures if claims fail.
- **Reward types**: Different campaigns may offer different reward tokens (BLUE, SUI, WAL, etc.)

---

## Troubleshooting

| Error | Cause | Solution |
|-------|-------|----------|
| `No claimable rewards found` | No rewards with valid signatures | Check campaign status and eligibility |
| `verify_signature` error | Signature expired or invalid | Fetch fresh rewards from API |
| `Insufficient gas` | Not enough SUI for transaction | Add SUI to wallet for gas fees |

---

## Support

For questions or issues, contact Bluefin support or refer to the SDK documentation.
