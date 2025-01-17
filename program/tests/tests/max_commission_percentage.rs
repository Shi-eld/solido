use lido::error::LidoError;
use lido::state::ListEntry;

use solana_program_test::tokio;
use solana_sdk::signature::Keypair;

use testlib::assert_solido_error;
use testlib::solido_context::Context;

#[tokio::test]
async fn test_set_max_commission_percentage() {
    let mut context = Context::new_with_maintainer_and_validator().await;
    let validator = &context.get_solido().await.validators.entries[0];

    // increase max_commission_percentage
    let result = context.try_set_max_commission_percentage(context.max_commission_percentage + 1);
    assert_eq!(result.await.is_ok(), true);

    let solido = context.get_solido().await.lido;
    assert_eq!(
        solido.max_commission_percentage,
        context.max_commission_percentage + 1
    );

    let result = context.try_deactivate_validator_if_commission_exceeds_max(*validator.pubkey());
    assert_eq!(result.await.is_ok(), true);

    // check validator is not deactivated
    let validator = &context.get_solido().await.validators.entries[0];
    assert_eq!(validator.active, true);

    // Increase max_commission_percentage above 100%
    assert_solido_error!(
        context.try_set_max_commission_percentage(101).await,
        LidoError::ValidationCommissionOutOfBounds
    );

    // decrease max_commission_percentage
    let result = context.try_set_max_commission_percentage(context.max_commission_percentage - 1);
    assert_eq!(result.await.is_ok(), true);

    let result = context.try_deactivate_validator_if_commission_exceeds_max(*validator.pubkey());
    assert_eq!(result.await.is_ok(), true);

    // check validator is deactivated
    let validator = &context.get_solido().await.validators.entries[0];
    assert_eq!(validator.active, false);
}

#[tokio::test]
async fn test_close_vote_account() {
    let mut context = Context::new_with_maintainer_and_validator().await;
    let vote_account = context.validator.as_ref().unwrap().vote_account;

    let validator = &context.get_solido().await.validators.entries[0];
    assert_eq!(validator.active, true);

    let keypair_bytes = context
        .validator
        .as_ref()
        .unwrap()
        .withdraw_authority
        .to_bytes();

    let withdraw_authority = Keypair::from_bytes(&keypair_bytes).unwrap();

    let result = context.try_close_vote_account(&vote_account, &withdraw_authority);
    assert_eq!(result.await.is_ok(), true);

    let result = context.try_deactivate_validator_if_commission_exceeds_max(*validator.pubkey());
    assert_eq!(result.await.is_ok(), true);

    let validator = &context.get_solido().await.validators.entries[0];
    assert_eq!(validator.active, false);
}
