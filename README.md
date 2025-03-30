# predicates

alkanes to enforce constraints on their inputs, useful when composed with protostones that should edict value conditionally based on an agreement between different signers on inputs to a transaction

### pair-equality

A predicate alkane contract that enforces the quantities of alkanes sent to it in a two-party trade. This contract validates that exactly two alkanes are present in the transaction and that they match the required sequence numbers and amounts.

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

## Project Structure

```
alkanes/
├── Cargo.toml          - Workspace configuration
├── pair-equality/      - Pair Equality Predicate Alkane
│   ├── Cargo.toml      - Package configuration
│   ├── src/            - Source code
│   └── tests/          - Tests
└── ... (future alkanes)
