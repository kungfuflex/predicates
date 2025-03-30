# System Patterns

## Architecture Overview
The pair-equality.wasm project implements a predicate contract that enforces the quantities of alkanes sent to it in a two-party trade using the alkane framework. The architecture follows a modular approach with clear separation of concerns:

```
pair-equality.wasm
├── Core Predicate Logic (EqualityPredicateAlkane)
├── Message Handling (EqualityPredicateAlkaneMessage)
├── Predicate Interface (EqualityPredicate trait)
└── AlkaneResponder Implementation
```

## Key Components

### EqualityPredicateAlkane
The main implementation of the predicate contract. It handles:
- Contract initialization with proper guards
- Validation of incoming alkanes against specified parameters
- Enforcement of exactly two alkanes in the transaction
- Comprehensive error handling

### EqualityPredicateAlkaneMessage
An enum that defines the message structure for opcode-based dispatch:
- Uses the MessageDispatch derive macro
- Defines all supported operations with their parameters
- Provides a clean interface for opcode handling

### EqualityPredicate Trait
A shared trait that provides common predicate functionality:
- Initialization guard
- Common validation methods

## Design Patterns

### Storage Pattern
The contract uses StoragePointer for persistent state management:
- `/initialized` - Guards against multiple initializations

### Security Patterns
1. **Initialization Guard**
   - Uses `observe_initialization()` to prevent multiple initializations
   - Sets a flag in storage to track initialization state
   - Returns descriptive error messages on failure

2. **Validation Pattern**
   - Enforces exactly two alkanes in the transaction
   - Validates specific sequence numbers and amounts for both parties
   - Provides clear error messages when validation fails

3. **Overflow Protection**
   - Uses `overflow_error` checks for all numeric operations
   - Provides descriptive error messages
   - Prevents integer overflow vulnerabilities

### Message Dispatch Pattern
The contract uses the MessageDispatch derive macro for clean opcode handling:
- Automatically generates opcode dispatch logic
- Provides type safety for parameters and return values
- Simplifies the implementation of the AlkaneResponder trait
- Follows the standardized opcode format:
  - 0: Initialize
  - 7: Filter

## Component Relationships
- EqualityPredicateAlkane implements the EqualityPredicate trait
- EqualityPredicateAlkaneMessage defines the message structure
- MessageDispatch handles opcode routing
- AlkaneResponder trait is implemented for execution