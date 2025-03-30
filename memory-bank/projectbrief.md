# Alkanes Monorepo

## Overview
A collection of secure and efficient alkane contracts for various use cases. This monorepo follows current best practices and security patterns while providing specialized functionality for different alkane contract types.

## Core Requirements

### Monorepo Structure
- Must organize related alkane contracts under a single workspace
- Must maintain separate crates for different contract types
- Must share common patterns and utilities across contracts
- Must provide consistent documentation across all contracts

### Contract Requirements
- Each contract must follow the alkane framework standards
- Each contract must implement proper initialization and security measures
- Each contract must provide comprehensive error handling
- Each contract must be thoroughly tested

## Current Contract Types

### pair-equality
A predicate alkane contract that enforces the quantities of alkanes sent to it in a two-party trade. This contract validates that exactly two alkanes are present in the transaction and that they match the required sequence numbers and amounts.

## Technical Implementation
- Use Rust 2021 edition for all contracts
- Use cargo workspace for managing multiple crates
- Compile to WebAssembly for deployment
- Implement comprehensive test suites for all contracts

## Security Patterns
- Use proper initialization guards in all contracts
- Validate all numeric operations for overflow
- Implement comprehensive error handling
- Follow best practices for each contract type