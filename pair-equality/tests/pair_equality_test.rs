use alkanes_runtime::runtime::AlkaneResponder;
use anyhow::Result;
use alkanes_support::id::AlkaneId;
use alkanes_support::parcel::AlkaneTransferParcel;
use alkanes_support::parcel::AlkaneTransfer;

mod mock;
use mock::{MockAlkaneResponder, MockContext};

use pair_equality::{EqualityPredicateAlkane, EqualityPredicate};

#[test]
fn test_initialization() -> Result<()> {
    // Create the EqualityPredicateAlkane instance
    let alkane = EqualityPredicateAlkane::default();
    
    // Test initialization
    alkane.observe_initialization()?;
    
    // Second initialization should fail
    let result = alkane.observe_initialization();
    assert!(result.is_err());
    
    Ok(())
}

#[test]
fn test_filter_success() -> Result<()> {
    // Create a mock context with two alkanes
    let mut mock_context = MockContext::default();
    
    // Set up test parameters
    let sequence_left = 123u128;
    let amount_left = 100u128;
    let sequence_right = 456u128;
    let amount_right = 200u128;
    
    // Create alkane IDs
    let left_id = AlkaneId { block: 2, tx: sequence_left };
    let right_id = AlkaneId { block: 2, tx: sequence_right };
    
    // Create alkane transfers
    let left_transfer = AlkaneTransfer {
        id: left_id.clone(),
        value: amount_left,
    };
    
    let right_transfer = AlkaneTransfer {
        id: right_id.clone(),
        value: amount_right,
    };
    
    // Set up incoming alkanes
    let mut transfers = Vec::new();
    transfers.push(left_transfer);
    transfers.push(right_transfer);
    mock_context.incoming_alkanes = AlkaneTransferParcel(transfers);
    
    // Create the responder
    let responder = MockAlkaneResponder::new(mock_context);
    
    // Create the EqualityPredicateAlkane instance
    let alkane = EqualityPredicateAlkane::default();
    
    // Initialize the contract
    alkane.observe_initialization()?;
    
    // Test filter with matching parameters
    let _context = responder.context()?;
    let result = alkane.filter(sequence_left, amount_left, sequence_right, amount_right);
    
    // Verify the filter succeeded
    assert!(result.is_ok());
    
    Ok(())
}

#[test]
fn test_filter_wrong_parameters() -> Result<()> {
    // Create a mock context with two alkanes
    let mut mock_context = MockContext::default();
    
    // Set up test parameters
    let sequence_left = 123u128;
    let amount_left = 100u128;
    let sequence_right = 456u128;
    let amount_right = 200u128;
    
    // Create alkane IDs
    let left_id = AlkaneId { block: 2, tx: sequence_left };
    let right_id = AlkaneId { block: 2, tx: sequence_right };
    
    // Create alkane transfers
    let left_transfer = AlkaneTransfer {
        id: left_id.clone(),
        value: amount_left,
    };
    
    let right_transfer = AlkaneTransfer {
        id: right_id.clone(),
        value: amount_right,
    };
    
    // Set up incoming alkanes
    let mut transfers = Vec::new();
    transfers.push(left_transfer);
    transfers.push(right_transfer);
    mock_context.incoming_alkanes = AlkaneTransferParcel(transfers);
    
    // Create the responder
    let responder = MockAlkaneResponder::new(mock_context);
    
    // Create the EqualityPredicateAlkane instance
    let alkane = EqualityPredicateAlkane::default();
    
    // Initialize the contract
    alkane.observe_initialization()?;
    
    // Test filter with wrong parameters
    let _context = responder.context()?;
    let result = alkane.filter(999u128, amount_left, sequence_right, amount_right);
    
    // Verify the filter failed
    assert!(result.is_err());
    
    Ok(())
}

#[test]
fn test_filter_wrong_alkane_count() -> Result<()> {
    // Create a mock context with only one alkane
    let mut mock_context = MockContext::default();
    
    // Set up test parameters
    let sequence_left = 123u128;
    let amount_left = 100u128;
    
    // Create alkane ID
    let left_id = AlkaneId { block: 2, tx: sequence_left };
    
    // Create alkane transfer
    let left_transfer = AlkaneTransfer {
        id: left_id.clone(),
        value: amount_left,
    };
    
    // Set up incoming alkanes with only one transfer
    let mut transfers = Vec::new();
    transfers.push(left_transfer);
    mock_context.incoming_alkanes = AlkaneTransferParcel(transfers);
    
    // Create the responder
    let responder = MockAlkaneResponder::new(mock_context);
    
    // Create the EqualityPredicateAlkane instance
    let alkane = EqualityPredicateAlkane::default();
    
    // Initialize the contract
    alkane.observe_initialization()?;
    
    // Test filter with only one alkane
    let _context = responder.context()?;
    let result = alkane.filter(sequence_left, amount_left, 456u128, 200u128);
    
    // Verify the filter failed due to wrong alkane count
    assert!(result.is_err());
    
    Ok(())
}