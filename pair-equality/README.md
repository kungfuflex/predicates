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