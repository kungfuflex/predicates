# Technical Context

## Technologies Used

### Core Technologies
- **Rust** - Primary programming language
- **WebAssembly (WASM)** - Compilation target for the contract
- **Alkanes Framework** - Smart contract framework for predicate implementation
- **MessageDispatch** - Macro for opcode-based message dispatching

### Dependencies
- **alkanes-support** - Core support library for alkane contracts
- **alkanes-runtime** - Runtime support for alkane execution
- **metashrew-support** - Support library for metashrew compatibility
- **protorune-support** - Support for protorune protocol
- **ordinals** - Ordinals protocol integration
- **anyhow** - Error handling library
- **bitcoin** - Bitcoin protocol library

## Development Setup

### Project Structure
```
pair-equality/
├── Cargo.toml           - Project manifest
├── memory-bank/         - Documentation and context
├── scripts/             - Build and test scripts
│   └── test-wasm.sh     - Script for testing the WASM build
├── src/                 - Source code
│   └── lib.rs           - Main contract implementation with MessageDispatch
└── tests/               - Test files
    ├── mock.rs          - Mock implementations for testing
    └── pair_equality_test.rs - Test cases for the contract
```

### Build Configuration
The project is configured as both a cdylib (for WebAssembly compilation) and rlib (for Rust library usage):

```toml
[lib]
crate-type = ["cdylib", "rlib"]
```

## Technical Constraints

### Compatibility Requirements
- Must follow the alkane predicate format for consistency
- Must support proper initialization sequence with authentication
- Must maintain compatibility with existing alkane systems

### Security Requirements
- Must use proper initialization guard via observe_initialization()
- Must validate all numeric operations for overflow
- Must ensure proper validation of incoming alkanes
- Must provide secure forwarding of valid transactions

### Performance Considerations
- Validation operations should be efficient
- Storage operations should be optimized to minimize resource usage
- Numeric operations must be checked for overflow to prevent vulnerabilities

## Integration Points

### Message Dispatch
The contract uses the MessageDispatch derive macro for opcode handling:
```rust
#[derive(MessageDispatch)]
enum EqualityPredicateAlkaneMessage {
    #[opcode(0)]
    Initialize,
    
    #[opcode(7)]
    Filter { sequence_left: u128, amount_left: u128, sequence_right: u128, amount_right: u128 }
}
```

### Opcode Interface
The contract exposes a standardized opcode interface for external interaction:
- Initialize (0)
- Filter (7)
- All operations have comprehensive error handling

### Storage Interface
The contract uses StoragePointer for persistent state management:
- Key-value storage for initialization state

## Deployment Considerations
- The contract is compiled to WebAssembly for deployment
- Initialization must be performed before using the filter functionality
- The contract should be tested with various validation scenarios