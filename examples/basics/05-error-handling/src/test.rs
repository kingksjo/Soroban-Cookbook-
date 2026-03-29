#[cfg(test)]
mod tests {
    use soroban_sdk::{
        testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
        Address, Env, IntoVal, Symbol,
    };

    use crate::{ContractError, ErrorDemoContract, ErrorDemoContractClient};

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    fn setup() -> (Env, ErrorDemoContractClient<'static>, Address) {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, ErrorDemoContract);
        let client = ErrorDemoContractClient::new(&env, &contract_id);
        let admin = Address::generate(&env);

        client.initialize(&admin);

        (env, client, admin)
    }

    // =======================================================================
    // Typed error tests
    // =======================================================================

    #[test]
    fn test_error_zero_amount_deposit() {
        let (_, client, _) = setup();
        let user = Address::generate(&client.env);

        let result = client.try_deposit(&user, &0);
        assert_eq!(result, Err(Ok(ContractError::ZeroAmount)));
    }

    #[test]
    fn test_error_zero_amount_withdraw() {
        let (_, client, _) = setup();
        let user = Address::generate(&client.env);

        let result = client.try_withdraw(&user, &0);
        assert_eq!(result, Err(Ok(ContractError::ZeroAmount)));
    }

    #[test]
    fn test_error_insufficient_balance() {
        let (_, client, _) = setup();
        let user = Address::generate(&client.env);

        let result = client.try_withdraw(&user, &100);
        assert_eq!(result, Err(Ok(ContractError::InsufficientBalance)));
    }

    #[test]
    fn test_error_contract_paused_deposit() {
        let (_, client, admin) = setup();
        client.pause(&admin);

        let user = Address::generate(&client.env);
        let result = client.try_deposit(&user, &50);
        assert_eq!(result, Err(Ok(ContractError::ContractPaused)));
    }

    #[test]
    fn test_error_contract_paused_withdraw() {
        let (_, client, admin) = setup();
        client.pause(&admin);

        let user = Address::generate(&client.env);
        let result = client.try_withdraw(&user, &50);
        assert_eq!(result, Err(Ok(ContractError::ContractPaused)));
    }

    // =======================================================================
    // Panic tests
    // =======================================================================

    #[test]
    fn test_panic_double_initialise() {
        let (_, client, admin) = setup();

        let result = client.try_initialize(&admin);
        assert!(result.is_err());
    }

    #[test]
    fn test_panic_with_error_unauthorized_pause() {
        let (_, client, _) = setup();
        let non_admin = Address::generate(&client.env);

        let result = client.try_pause(&non_admin);
        assert!(result.is_err());
    }

    #[test]
    fn test_panic_impossible_branch() {
        let (_, client, _) = setup();

        assert_eq!(client.status_label(&0), Symbol::new(&client.env, "ok"));
        assert_eq!(client.status_label(&1), Symbol::new(&client.env, "paused"));
        assert_eq!(client.status_label(&2), Symbol::new(&client.env, "error"));

        let result = client.try_status_label(&99);
        assert!(result.is_err());
    }

    // =======================================================================
    // Happy-path tests
    // =======================================================================

    #[test]
    fn test_happy_path_deposit_withdraw() {
        let (_, client, _) = setup();
        let user = Address::generate(&client.env);

        let after_deposit = client.deposit(&user, &200);
        assert_eq!(after_deposit, 200);

        let after_withdraw = client.withdraw(&user, &75);
        assert_eq!(after_withdraw, 125);

        assert_eq!(client.balance(&user), 125);
    }

    #[test]
    fn test_pause_unpause_cycle() {
        let (_, client, admin) = setup();
        let user = Address::generate(&client.env);

        client.deposit(&user, &100);

        client.pause(&admin);
        assert!(client.is_paused());

        assert_eq!(client.try_deposit(&user, &50), Err(Ok(ContractError::ContractPaused)));
        assert_eq!(client.try_withdraw(&user, &50), Err(Ok(ContractError::ContractPaused)));

        client.unpause(&admin);
        assert!(!client.is_paused());

        assert_eq!(client.deposit(&user, &50), 150);
        assert_eq!(client.withdraw(&user, &150), 0);
    }
}
