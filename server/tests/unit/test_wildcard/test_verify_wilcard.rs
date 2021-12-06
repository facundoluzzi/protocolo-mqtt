mod tests {
    use server::wildcard::verify_wildcard;
    use server::wildcard::wildcard_result::WildcardResult::{HasWildcard, HasNoWildcard, InvalidWildcard};

    #[test]
    pub fn get_ok_on_verification_get_wilcard() {
        let topic_name: String = "Futbol/Boca/*".to_owned();

        match verify_wildcard::get_wilcard(topic_name) {
            HasWildcard(_wildcard) => assert_eq!(1, 1),
            _ => assert_eq!(0, 1),
        };

        let topic_name: String = "Futbol/Boca/Boca*".to_owned();
        match verify_wildcard::get_wilcard(topic_name) {
            HasWildcard(_wildcard) => assert_eq!(1, 1),
            _ => assert_eq!(0, 1),
        };

        let topic_name: String = "Futbol/*/Boca/*".to_owned();
        match verify_wildcard::get_wilcard(topic_name) {
            HasWildcard(_wildcard) => assert_eq!(1, 1),
            _ => assert_eq!(0, 1),
        };

        let topic_name: String = "Futbol/Boca/>".to_owned();
        match verify_wildcard::get_wilcard(topic_name) {
            HasWildcard(_wildcard) => assert_eq!(1, 1),
            _ => assert_eq!(0, 1),
        };

        let topic_name: String = "Futbol/*/Boca".to_owned();
        match verify_wildcard::get_wilcard(topic_name) {
            HasWildcard(_wildcard) => assert_eq!(1, 1),
            _ => assert_eq!(0, 1),
        };

        let topic_name: String = "Futbol/*/Boca/>".to_owned();
        match verify_wildcard::get_wilcard(topic_name) {
            HasWildcard(_wildcard) => assert_eq!(1, 1),
            _ => assert_eq!(0, 1),
        };
    }

    #[test]
    pub fn verification_wildcard_should_failed() {
        let topic_name: String = "Futbol/Boca/*goles".to_owned();
        match verify_wildcard::get_wilcard(topic_name) {
            HasNoWildcard => assert_eq!(1, 1),
            _ => assert_eq!(0, 1),
        };

        let topic_name: String = "Futbol/Boca/>goles".to_owned();
        match verify_wildcard::get_wilcard(topic_name) {
            HasNoWildcard => assert_eq!(1, 1),
            _ => assert_eq!(0, 1),
        };

        let topic_name: String = "Futbol/*Boca/goles".to_owned();
        match verify_wildcard::get_wilcard(topic_name) {
            HasNoWildcard => assert_eq!(1, 1),
            _ => assert_eq!(0, 1),
        };

        let topic_name: String = "Futbol/>/Boca/goles".to_owned();
        match verify_wildcard::get_wilcard(topic_name) {
            InvalidWildcard => assert_eq!(1, 1),
            _ => assert_eq!(0, 1),
        };

        let topic_name: String = "Futbol/>/Boca/goles/>".to_owned();
        match verify_wildcard::get_wilcard(topic_name) {
            InvalidWildcard => assert_eq!(1, 1),
            _ => assert_eq!(0, 1),
        };

        let topic_name: String = "Futbol/*/>/Boca/goles".to_owned();
        match verify_wildcard::get_wilcard(topic_name) {
            InvalidWildcard => assert_eq!(1, 1),
            _ => assert_eq!(0, 1),
        };

    }
}
