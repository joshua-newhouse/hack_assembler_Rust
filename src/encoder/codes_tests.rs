#[cfg(test)]
mod codes_tests {
    use crate::encoder::codes::Codes;

    #[test]
    fn comp_codes_are_probably_valid() {
        let codes = Codes::new();

        codes.comp_codes.iter()
            .for_each(|(k, v)| {
                if k.contains('M') {
                    assert!(v.starts_with('1'))
                } else {
                    assert!(v.starts_with('0'))
                }

                assert!(3 >= k.len());
                assert_eq!(7, v.len());
            })
    }

    #[test]
    fn jump_codes_are_probably_valid() {
        let codes = Codes::new();

        codes.jump_codes.iter()
            .for_each(|(k, v)| {
                assert!(3 == k.len() || k.is_empty());
                assert_eq!(3, v.len());
            })
    }

    #[test]
    fn dest_codes_are_probably_valid() {
        let codes = Codes::new();

        codes.jump_codes.iter()
            .for_each(|(k, v)| {
                assert!(3 >= k.len());
                assert_eq!(3, v.len());
            })
    }
}
