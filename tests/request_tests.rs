mod common;

#[test]
fn request_roundtrip_preserves_identifier() {
    let roundtrip = common::sample_request()
        .bridge_roundtrip()
        .expect("request roundtrip should succeed");
    assert_eq!(roundtrip.identifier, "request-id");
    assert!(roundtrip.trigger.is_some());
}
