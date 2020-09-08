use crate::utils::{accounts, create_context, setup};

mod utils;

/// Test various invalid inputs to function calls.
#[test]
fn test_invalid_input() {
    let (mut fake_external, vm_config, fees_config) = setup();
    let mut context = create_context(&mut fake_external, &vm_config, &fees_config, accounts(1), 10);
    assert!(context.get_nonce(vec![]).is_err());
    assert!(context.get_balance(vec![]).is_err());
    assert!(context.get_code(vec![]).is_err());
    assert!(context.get_storage_at(vec![]).is_err());
    assert!(context.view_call_function(vec![]).is_err());
    assert!(context.view_call_function(vec![0u8; 20]).is_err());
    assert!(context.call_function(vec![]).is_err());
    assert!(context.call_function(vec![0u8; 20]).is_err());
    assert!(context.deposit(vec![]).is_err());
    assert!(context.withdraw(vec![]).is_err());
    assert!(context.transfer(vec![]).is_err());
}