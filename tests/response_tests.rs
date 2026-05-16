mod common;

use usernotifications::prelude::*;

#[test]
fn response_helpers_match_known_identifiers() {
    let response = common::sample_response();
    assert!(response.is_default_action());
    assert_ne!(default_action_identifier(), dismiss_action_identifier());
}
