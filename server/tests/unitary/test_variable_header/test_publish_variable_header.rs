use server::variable_header::publish_variable_header::get_variable_header;
use server::variable_header::publish_variable_header::verify_publish_wilcard;

#[test]
fn should_get_variable_header_in_publish_successfully() {
    let vec: &[u8] = &[0x00, 0x03, 0x41, 0x2F, 0x42, 0x00, 0x00];
    let (topic, packet_identifier, _) = get_variable_header(vec, 1).unwrap();
    assert_eq!(topic, "A/B".to_owned());
    assert_eq!(packet_identifier.unwrap(), [0x00, 0x00]);
}

#[test]
fn should_not_get_variable_header_in_publish_successfully() {
    let vec: &[u8] = &[0x00, 0x03, 0x41, 0x2F, 0x42, 0x00, 0x00];
    let (topic, packet_identifier, _) = get_variable_header(vec, 0).unwrap();
    assert_eq!(topic, "A/B".to_owned());
    assert_eq!(packet_identifier, None);
}

#[test]
fn should_verify_topic_successfully() {
    let topic = "A/B".to_string();
    let is_valid = verify_publish_wilcard(topic);
    assert_eq!(is_valid, true);
}

#[test]
fn should_verify_topic_a_successfully() {
    let topic = "A".to_string();
    let is_valid = verify_publish_wilcard(topic);
    assert_eq!(is_valid, true);
}

#[test]
fn should_fail_verify_topic_with_slash() {
    let topic = "A/".to_string();
    let is_valid = verify_publish_wilcard(topic);
    assert_eq!(is_valid, false);
}

#[test]
fn should_fail_verify_topic_with_slash_at_the_beginning() {
    let topic = "/A".to_string();
    let is_valid = verify_publish_wilcard(topic);
    assert_eq!(is_valid, false);
}

#[test]
fn should_fail_verify_topic_with_numeral() {
    let topic = "A/B#".to_string();
    let is_valid = verify_publish_wilcard(topic);
    assert_eq!(is_valid, false);
}

#[test]
fn should_fail_verify_topic_with_dolar_sign() {
    let topic = "A/B$".to_string();
    let is_valid = verify_publish_wilcard(topic);
    assert_eq!(is_valid, false);
}

#[test]
fn should_fail_verify_topic_with_plus_sign() {
    let topic = "A/B+".to_string();
    let is_valid = verify_publish_wilcard(topic);
    assert_eq!(is_valid, false);
}
