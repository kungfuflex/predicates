# pair-equality.wasm

## Overview
A secure and efficient predicate alkane contract that enforces the quantities of alkanes sent to it in a two-party trade. This contract follows current best practices and security patterns while providing validation functionality for alkane transfers.

## Core Requirements

### Predicate Functionality
- Must validate incoming alkanes against specified parameters
- Must enforce exactly two alkanes in the transaction
- Must support proper initialization sequence with authentication
- Must follow the alkane predicate format for consistency

### Validation Controls
- Must enforce specific sequence and amount requirements for both parties
- Must validate that exactly two alkanes are present in the transaction
- Must provide clear error messages when validation fails

### Security Requirements
- Must use proper initialization guard via observe_initialization()
- Must validate all numeric operations for overflow
- Must ensure proper validation of incoming alkanes
- Must provide secure forwarding of valid transactions

## Opcode Specification 

### Standard Operations
- 0: Initialize()
- 7: Filter(sequence_left: u128, amount_left: u128, sequence_right: u128, amount_right: u128)

## Technical Implementation
- Use MessageDispatch derive macro for opcode handling
- Use declare_alkane! macro for proper runtime integration
- Implement proper validation of incoming alkanes
- Implement all operations with comprehensive error handling

## Security Patterns
- Call observe_initialization() in Initialize operation
- Validate that exactly two alkanes are present in the transaction
- Ensure all numeric operations use overflow_error checks
- Provide clear error messages for validation failures