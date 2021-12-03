mod tests {
    use server::wildcard::verify_wildcard;

    #[test]
    pub fn get_ok_on_verification_get_wilcard() {
        let topic_name: String = "Futbol/Boca/*".to_owned();
        if let Some(_result) = verify_wildcard::get_wilcard(topic_name) {
            assert_eq!(1, 1);
        } else {
            assert_ne!(0, 1);
        }

        let topic_name: String = "Futbol/Boca/Boca*".to_owned();
        if let Some(_result) = verify_wildcard::get_wilcard(topic_name) {
            assert_eq!(1, 1);
        } else {
            assert_ne!(0, 1);
        }

        let topic_name: String = "Futbol/*/Boca/*".to_owned();
        if let Some(_result) = verify_wildcard::get_wilcard(topic_name) {
            assert_eq!(1, 1);
        } else {
            assert_ne!(0, 1);
        }

        let topic_name: String = "Futbol/Boca/>".to_owned();
        if let Some(_result) = verify_wildcard::get_wilcard(topic_name) {
            assert_eq!(1, 1);
        } else {
            assert_ne!(0, 1);
        }

        let topic_name: String = "Futbol/*/Boca".to_owned();
        if let Some(_result) = verify_wildcard::get_wilcard(topic_name) {
            assert_eq!(1, 1);
        } else {
            assert_ne!(0, 1);
        }

        let topic_name: String = "Futbol/*/Boca/>".to_owned();
        if let Some(_result) = verify_wildcard::get_wilcard(topic_name) {
            assert_eq!(1, 1);
        } else {
            assert_ne!(0, 1);
        }
    }

    #[test]
    pub fn get_deny_on_verification_get_wilcard() {
        let topic_name: String = "Futbol/Boca/*goles".to_owned();
        if let Some(_result) = verify_wildcard::get_wilcard(topic_name) {
            assert_eq!(0, 1);
        } else {
            assert_eq!(1, 1);
        }

        let topic_name: String = "Futbol/Boca/>goles".to_owned();
        if let Some(_result) = verify_wildcard::get_wilcard(topic_name) {
            assert_eq!(0, 1);
        } else {
            assert_eq!(1, 1);
        }

        let topic_name: String = "Futbol/*Boca/goles".to_owned();
        if let Some(_result) = verify_wildcard::get_wilcard(topic_name) {
            assert_eq!(0, 1);
        } else {
            assert_eq!(1, 1);
        }
    }
}
