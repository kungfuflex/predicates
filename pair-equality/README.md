# pair-equality

A secure and efficient predicate alkane contract that enforces the quantities of alkanes sent to it in a two-party trade.

## Overview

This contract implements a predicate that validates incoming alkanes against specified parameters. It ensures that exactly two alkanes are present in the transaction and that they match the required sequence numbers and amounts.

## Features

- Validates that exactly two alkanes are present in the transaction
- Enforces specific sequence numbers and amounts for both parties
- Provides clear error messages when validation fails
- Uses proper initialization guard to prevent multiple initializations
- Implements comprehensive error handling

## Usage

The contract exposes two main operations:

1. **Initialize** (opcode 0): Initializes the contract with proper guards
2. **Filter** (opcode 7): Validates incoming alkanes against specified parameters

## Transaction Examples

### PSBT-Based Trade Using pair-equality

This example demonstrates how to create and sign a PSBT (Partially Signed Bitcoin Transaction) that uses the pair-equality contract to exchange two alkanes in an atomic swap.

#### Scenario

Imagine we have:
- Two alkanes with IDs `[2, 10]` (value: 1,000,000) and `[2, 11]` (value: 500,000) from different parties
- The pair-equality alkane is deployed at ID `[2, 20]`
- Transaction outputs (vouts) 0 and 1 are for the trade recipients
- vout 2 is for the OP_RETURN containing the Runestone

#### Complete TypeScript Example

```typescript
import * as btc from "@scure/btc-signer";
import { encodeRunestoneProtostone } from "alkanes/lib/protorune/proto_runestone_upgrade.js";
import { ProtoStone } from "alkanes/lib/protorune/protostone.js";
import { ProtoruneRuneId } from "alkanes/lib/protorune/protoruneruneid.js";
import { ProtoruneEdict } from "alkanes/lib/protorune/protoruneedict.js";
import { encipher } from "alkanes/lib/bytes.js";

// Define the network parameters
const NETWORK_PARAMS = {
  bip32: {
    public: 0x043587cf,
    private: 0x04358394,
  },
};

// Create a new transaction
const transaction = new btc.Transaction();

// Add inputs from both parties
// Each input contains an alkane and exactly 546 sats (dust amount)
transaction.addInput({
  txid: "party1_txid_containing_alkane_2_10",
  index: 0,
  witnessUtxo: { script: party1Script, amount: 546n },
});

transaction.addInput({
  txid: "party2_txid_containing_alkane_2_11",
  index: 0,
  witnessUtxo: { script: party2Script, amount: 546n },
});

// Add fee input (exact amount needed for the transaction fee)
transaction.addInput({
  txid: "fee_payment_txid",
  index: 0,
  witnessUtxo: { script: feePaymentScript, amount: 1000n }, // Exact fee amount
});

// Add outputs for the recipients (dust amount to hold the alkanes)
transaction.addOutputAddress("party1_recipient_address", 546n, NETWORK_PARAMS);
transaction.addOutputAddress("party2_recipient_address", 546n, NETWORK_PARAMS);

// Note: No change output is needed as we're using exact amounts

// Create the Protostone array for the pair-equality contract
// Note: ProtoStones are indexed starting at tx.outputs.length + 1
// In this example, we'll have 3 outputs (2 recipients + 1 OP_RETURN),
// so the first ProtoStone index would be 3
const protostones = [
  // First Protostone: Call the pair-equality contract with Filter opcode
  new ProtoStone({
    protocolTag: 1n,
    message: {
      // pointer to the second ProtoStone (index 4) for success path
      pointer: 4,
      // pointer to the third ProtoStone (index 5) for refund path
      refundPointer: 5,
      calldata: encipher([
        2n,                // block of pair-equality contract
        20n,               // tx of pair-equality contract
        7n,                // opcode for Filter
        10n,               // sequence_left
        1000000n,          // amount_left
        11n,               // sequence_right
        500000n            // amount_right
      ])
    }
  }),
  
  // Second ProtoStone: Define edicts for the first alkane (success path)
  new ProtoStone({
    protocolTag: 1n,
    message: {
      pointer: 1,  // Point to output 1 (opposite of where edict transfers to)
      refundPointer: 1,
      calldata: new Uint8Array(0)
    },
    edicts: [
      {
        id: new ProtoruneRuneId(2n, 10n),
        amount: 1000000n,
        output: 0  // Transfer to first recipient
      }
    ]
  }),
  
  // Third ProtoStone: Define edicts for the second alkane (success path)
  new ProtoStone({
    protocolTag: 1n,
    message: {
      pointer: 0,  // Point to output 0 (opposite of where edict transfers to)
      refundPointer: 0,
      calldata: new Uint8Array(0)
    },
    edicts: [
      {
        id: new ProtoruneRuneId(2n, 11n),
        amount: 500000n,
        output: 1  // Transfer to second recipient
      }
    ]
  })
];

// Encode the Runestone with the Protostones
const runestone = encodeRunestoneProtostone({
  protostones: protostones
}).encodedRunestone;

// Add the Runestone as an OP_RETURN output
transaction.addOutput({
  script: runestone,
  amount: 0n
});

// Party 1 signs their input
transaction.sign(party1PrivateKey);

// Sign fee input (if fee provider is party 1)
transaction.sign(feePrivateKey);

// Export the PSBT for Party 2 to sign
const psbtHex = transaction.toPSBT();

// --- On Party 2's side ---
const psbt = btc.Transaction.fromPSBT(psbtHex);

// Party 2 signs their input
psbt.sign(party2PrivateKey);

// Sign fee input (if fee provider is party 2)
psbt.sign(feePrivateKey);

// Finalize the transaction
psbt.finalize();

// Extract the final transaction
const finalTxHex = psbt.extract();

// Broadcast the transaction
// broadcastTransaction(finalTxHex);
```

#### Step-by-Step Explanation

1. **Transaction Setup**:
   - Create a new transaction with inputs from both parties
   - Add outputs for both recipients (outputs 0 and 1)
   - We'll add an OP_RETURN output (output 2) containing the Runestone

2. **Protostone Creation and Indexing**:
   - ProtoStones are indexed starting at `tx.outputs.length + 1`
   - With 3 outputs (2 recipients + 1 OP_RETURN), the first ProtoStone index is 3
   - First ProtoStone (index 3): Contains the message to call the pair-equality contract
     - `pointer: 4` points to the second ProtoStone (success path)
     - `refundPointer: 5` points to the third ProtoStone (refund path)
   - Second ProtoStone (index 4):
     - Defines the edict to transfer the first alkane to output 0
     - Has `pointer: 1` pointing to the opposite output of where its edict transfers to
   - Third ProtoStone (index 5):
     - Defines the edict to transfer the second alkane to output 1
     - Has `pointer: 0` pointing to the opposite output of where its edict transfers to

3. **Pointer Mechanism**:
   - If the pair-equality contract validates successfully, execution follows the `pointer` path
   - If validation fails, execution follows the `refundPointer` path
   - This creates an atomic swap mechanism where either both transfers happen or neither does
   - Each ProtoStone with edicts must have a pointer to the opposite output:
     - If an edict transfers to output 0, its pointer must point to output 1
     - If an edict transfers to output 1, its pointer must point to output 0
   - This cross-pointing mechanism ensures proper handling of the alkanes in both success and refund scenarios

4. **Runestone Encoding**:
   - Use `encodeRunestoneProtostone` to create a Runestone that embeds the Protostones
   - Add the Runestone as an OP_RETURN output in the transaction

5. **PSBT Signing Process**:
   - Party 1 partially signs the transaction and creates a PSBT
   - Party 2 receives the PSBT, signs their input, and finalizes the transaction
   - The final transaction is broadcast to the network

6. **Contract Execution**:
   - When the transaction is mined, the pair-equality contract validates:
     - Exactly two alkanes are present
     - The alkanes match the specified sequence numbers and amounts
   - If validation passes:
     - Execution follows the `pointer` path (index 4)
     - Both alkanes are transferred to their respective outputs
   - If validation fails:
     - Execution follows the `refundPointer` path (index 5)
     - The transaction is rejected or funds are returned to original owners

### Real-World Implementation Example

For a more complex real-world scenario, here's how you might implement a trade with error handling and verification:

```typescript
async function createAlkaneTrade(
  party1Input: { txid: string, index: number, privateKey: Uint8Array, script: Uint8Array },
  party2Input: { txid: string, index: number, privateKey: Uint8Array, script: Uint8Array },
  feeInput: { txid: string, index: number, privateKey: Uint8Array, script: Uint8Array, amount: bigint },
  alkane1: { block: bigint, tx: bigint, amount: bigint },
  alkane2: { block: bigint, tx: bigint, amount: bigint },
  pairEqualityContract: { block: bigint, tx: bigint },
  recipient1Address: string,
  recipient2Address: string
) {
  try {
    // Create transaction
    const tx = new btc.Transaction();
    
    // Add inputs - each containing an alkane with dust amount (546 sats)
    tx.addInput({
      txid: party1Input.txid,
      index: party1Input.index,
      witnessUtxo: {
        script: party1Input.script,
        amount: 546n  // Dust amount
      }
    });
    
    tx.addInput({
      txid: party2Input.txid,
      index: party2Input.index,
      witnessUtxo: {
        script: party2Input.script,
        amount: 546n  // Dust amount
      }
    });
    
    // Add fee input (exact amount needed for the transaction fee)
    tx.addInput({
      txid: feeInput.txid,
      index: feeInput.index,
      witnessUtxo: {
        script: feeInput.script,
        amount: feeInput.amount  // Exact fee amount (e.g., 1000 sats)
      }
    });
    
    // Add recipient outputs - dust amount to hold the alkanes
    tx.addOutputAddress(recipient1Address, 546n, NETWORK_PARAMS); // Dust amount
    tx.addOutputAddress(recipient2Address, 546n, NETWORK_PARAMS); // Dust amount
    
    // Note: No change output as we're using exact amounts
    
    // Calculate ProtoStone indices
    // With 3 outputs (2 recipients + 1 OP_RETURN), the first ProtoStone index is 3
    const firstProtostoneIndex = 3;
    const successPathIndex = firstProtostoneIndex + 1; // 4
    const refundPathIndex = firstProtostoneIndex + 2;  // 5
    
    // Create Protostones
    const protostones = [
      // First ProtoStone: Call the pair-equality contract
      new ProtoStone({
        protocolTag: 1n,
        message: {
          pointer: successPathIndex,        // Point to success path ProtoStone
          refundPointer: refundPathIndex,   // Point to refund path ProtoStone
          calldata: encipher([
            pairEqualityContract.block,
            pairEqualityContract.tx,
            7n, // Filter opcode
            alkane1.tx,
            alkane1.amount,
            alkane2.tx,
            alkane2.amount
          ])
        }
      }),
      
      // Second ProtoStone: Success path for first alkane
      new ProtoStone({
        protocolTag: 1n,
        message: {
          pointer: 1,  // Point to output 1 (opposite of where edict transfers to)
          refundPointer: 1,
          calldata: new Uint8Array(0)
        },
        edicts: [
          {
            id: new ProtoruneRuneId(alkane1.block, alkane1.tx),
            amount: alkane1.amount,
            output: 0  // Transfer to first recipient
          }
        ]
      }),
      
      // Third ProtoStone: Success path for second alkane
      new ProtoStone({
        protocolTag: 1n,
        message: {
          pointer: 0,  // Point to output 0 (opposite of where edict transfers to)
          refundPointer: 0,
          calldata: new Uint8Array(0)
        },
        edicts: [
          {
            id: new ProtoruneRuneId(alkane2.block, alkane2.tx),
            amount: alkane2.amount,
            output: 1  // Transfer to second recipient
          }
        ]
      })
    ];
    
    // Add Runestone output
    tx.addOutput({
      script: encodeRunestoneProtostone({ protostones }).encodedRunestone,
      amount: 0n
    });
    
    // Party 1 signs their input
    tx.sign(party1Input.privateKey);
    
    // Sign fee input (if fee provider is party 1)
    tx.sign(feeInput.privateKey);
    
    // Create PSBT for Party 2
    const psbtHex = tx.toPSBT();
    
    // Return PSBT for Party 2 to sign
    return psbtHex;
  } catch (error) {
    console.error("Error creating alkane trade:", error);
    throw error;
  }
}

// Usage example
const psbtHex = await createAlkaneTrade(
  { txid: "party1_txid", index: 0, privateKey: party1Key, script: party1Script },
  { txid: "party2_txid", index: 0, privateKey: party2Key, script: party2Script },
  { txid: "fee_txid", index: 0, privateKey: feeKey, script: feeScript, amount: 1000n },
  { block: 2n, tx: 10n, amount: 1000000n },
  { block: 2n, tx: 11n, amount: 500000n },
  { block: 2n, tx: 20n },
  "party1_recipient_address",
  "party2_recipient_address"
);

// Party 2 would then:
const psbt = btc.Transaction.fromPSBT(psbtHex);
// Sign party 2's input
psbt.sign(party2Input.privateKey);
// Sign fee input (if fee provider is party 2)
psbt.sign(feeInput.privateKey);
psbt.finalize();
const finalTxHex = psbt.extract();
// Broadcast transaction
```

This implementation demonstrates a complete workflow for creating and signing a PSBT that uses the pair-equality contract to facilitate a secure atomic swap of alkanes between two parties.

### Understanding the ProtoStone Pointer Mechanism

The pointer mechanism in ProtoStones is crucial for implementing atomic swaps with the pair-equality contract. Here's a detailed explanation:

#### ProtoStone Indexing

ProtoStones are indexed starting at `tx.outputs.length + 1`. For example, in a transaction with:
- Output 0: Recipient 1
- Output 1: Recipient 2
- Output 2: OP_RETURN (containing the Runestone)

The ProtoStone indices would be:
- First ProtoStone: Index 3
- Second ProtoStone: Index 4
- Third ProtoStone: Index 5

#### Cross-Pointing Requirement

Each ProtoStone that contains edicts must have its pointer pointing to the opposite output of where its edicts transfer to:

1. **Why opposite pointers are required**:
   - If an edict transfers to output 0, its pointer must point to output 1
   - If an edict transfers to output 1, its pointer must point to output 0

2. **Purpose of this cross-pointing**:
   - It ensures that the alkane transfer logic correctly handles both success and refund scenarios
   - It maintains the atomic nature of the swap by ensuring both parties either receive their alkanes or get refunded
   - It prevents partial execution where only one party would receive their alkane

#### Complete Pointer Flow

In our example:

1. **First ProtoStone** (index 3):
   - Contains the call to the pair-equality contract
   - `pointer: 4` - Points to the second ProtoStone for the success path
   - `refundPointer: 5` - Points to the third ProtoStone for the refund path

2. **Second ProtoStone** (index 4):
   - Contains edicts transferring the first alkane to output 0
   - `pointer: 1` - Points to output 1 (opposite of where its edict transfers to)
   - This ensures that if this ProtoStone is executed, the system knows to look at output 1 for related operations

3. **Third ProtoStone** (index 5):
   - Contains edicts transferring the second alkane to output 1
   - `pointer: 0` - Points to output 0 (opposite of where its edict transfers to)
   - This ensures that if this ProtoStone is executed, the system knows to look at output 0 for related operations

This carefully designed pointer structure ensures that the pair-equality contract can enforce the atomic swap guarantee: either both transfers happen exactly as specified, or neither does.

## Development

### Prerequisites

- Rust 2021 edition
- Cargo
- WebAssembly target

### Building

```bash
cargo build --target wasm32-unknown-unknown
```

### Testing

```bash
cargo test
```

## Security

This contract implements several security patterns:

- Initialization guard via observe_initialization()
- Validation of incoming alkanes against specified parameters
- Overflow protection for all numeric operations
- Comprehensive error handling