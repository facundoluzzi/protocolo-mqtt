mod tests {
    use server::wildcard::wildcard_handler::Wildcard;

    #[test]
    pub fn testing_simple_astherisc_on_the_end() {
        let vec: Vec<String> = "Futbol/Boca/*".split('/').map(|s| s.to_string()).collect();
        let wildcard = Wildcard::init(vec);

        let test_case = "Futbol/Boca/CantidadDeGoles".to_owned();
        let result_case = wildcard.verify_topic(test_case);

        assert_eq!(result_case, true);
    }

    #[test]
    pub fn testing_simple_astherisc_at_the_end_of_a_word() {
        let vec: Vec<String> = "Futbol/Boca/Cant*"
            .split('/')
            .map(|s| s.to_string())
            .collect();
        let wildcard = Wildcard::init(vec);

        let test_case = "Futbol/Boca/CantidadDeGoles".to_owned();
        let result_case = wildcard.verify_topic(test_case);

        let test_case = "Futbol/Boca/CantidadDePartidos".to_owned();
        let result2_case = wildcard.verify_topic(test_case);

        assert_eq!(result_case, true);
        assert_eq!(result2_case, true);
    }

    #[test]
    pub fn testing_simple_astherisc_at_the_end_of_a_word_and_greater_than_on_the_end() {
        let vec: Vec<String> = "Futbol/Boca/Cant*/>"
            .split('/')
            .map(|s| s.to_string())
            .collect();
        let wildcard = Wildcard::init(vec);

        let test_case = "Futbol/Boca/CantidadDeGoles/2021/4".to_owned();
        let result_case = wildcard.verify_topic(test_case);

        let test_case = "Futbol/Boca/CantidadDePartidos/2020/-2".to_owned();
        let result2_case = wildcard.verify_topic(test_case);

        assert_eq!(result_case, true);
        assert_eq!(result2_case, true);
    }

    #[test]
    pub fn testing_simple_astherisc_on_the_end_should_failed() {
        let vec: Vec<String> = "Futbol/Boca/*".split('/').map(|s| s.to_string()).collect();
        let wildcard = Wildcard::init(vec);

        let test_case = "Futbol/Boca/CantidadDeGoles/3".to_owned();
        let result_case = wildcard.verify_topic(test_case);

        assert_ne!(result_case, true);
    }

    #[test]
    pub fn testing_multiple_astherisc() {
        let vec: Vec<String> = "Futbol/Boca/*/4/*"
            .split('/')
            .map(|s| s.to_string())
            .collect();
        let wildcard = Wildcard::init(vec);

        let test_case = "Futbol/Boca/CantidadDeGoles/4/2021".to_owned();
        let result_case = wildcard.verify_topic(test_case);

        assert_eq!(result_case, true);
    }

    #[test]
    pub fn testing_greater_than_on_the_end() {
        let vec: Vec<String> = "Futbol/Boca/>".split('/').map(|s| s.to_string()).collect();
        let wildcard = Wildcard::init(vec);

        let test_case = "Futbol/Boca/CantidadDeGoles/2021/-2".to_owned();
        let result_case = wildcard.verify_topic(test_case);

        assert_eq!(result_case, true);
    }

    #[test]
    pub fn testing_with_multiple_astheriscs_and_greater_than_on_the_end() {
        let vec: Vec<String> = "Futbol/*/CantidadDeGoles/*/>"
            .split('/')
            .map(|s| s.to_string())
            .collect();
        let wildcard = Wildcard::init(vec);

        let test_case = "Futbol/Boca/CantidadDeGoles/2020/2/Empate".to_owned();
        let test2_case = "Futbol/River/CantidadDeGoles/2019/1/Ganador".to_owned();

        let result_case = wildcard.verify_topic(test_case);
        let result2_case = wildcard.verify_topic(test2_case);

        assert_eq!(result_case, true);
        assert_eq!(result2_case, true);
    }

    #[test]
    pub fn testing_with_multiple_astheriscs_and_greater_than_on_the_end_should_failed() {
        let vec: Vec<String> = "Futbol/*/CantidadDeGoles/*/>"
            .split('/')
            .map(|s| s.to_string())
            .collect();
        let wildcard = Wildcard::init(vec);

        let test_case = "Futbol/Boca/CantidadDeGoles".to_owned();

        let result_case = wildcard.verify_topic(test_case);
        assert_ne!(result_case, true);
    }
}
