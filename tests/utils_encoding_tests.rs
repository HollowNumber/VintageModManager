#[cfg(test)]
mod tests {
    use utils::encoding::decode_mod_string;
    use utils::Encoder;

    #[test]
    fn test_encode() {
        let encoder = Encoder::new();
        let data = b"hello";
        let encoded = encoder.encode(data);
        assert_eq!(encoded, "aGVsbG8=");
    }

    #[test]
    fn test_decode() {
        let encoder = Encoder::new();
        let data = "aGVsbG8=";
        let decoded = encoder.decode(data).unwrap();
        assert_eq!(decoded, b"hello");
    }

    #[test]
    fn test_encode_mod_string() {
        let encoder = Encoder::new();
        let data = &[1, 2, 3, 4, 5];
        let encoded = encoder.encode_mod_string(data);
        assert_eq!(encoded, "MSYyJjMmNCY1");
    }

    #[test]
    fn test_decode_mod_string() {
        let encoder = Encoder::new();
        let data = "MSYyJjMmNCY1".to_string();
        let decoded = encoder.decode_mod_string(data).unwrap();
        assert_eq!(decoded, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_decode_mod_string_error() {
        let encoder = Encoder::new();
        let data = "invalid_base64".to_string();
        let result = encoder.decode_mod_string(data);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_mod_string() {
        let data = &[1, 2, 3, 4, 5];
        let mod_string = create_mod_string(data);
        assert_eq!(mod_string, "1&2&3&4&5");

        fn create_mod_string(data: &[u16]) -> String {
            let mod_string = data
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("&");
            mod_string
        }
    }

    #[test]
    fn test_decode_mod_string_function() {
        let data = "MSYyJjMmNCY1";
        let decoded = decode_mod_string(data).unwrap();
        assert_eq!(decoded, "1&2&3&4&5");
    }

    #[test]
    fn test_decode_mod_string_function_error() {
        let data = "invalid_base64";
        let result = decode_mod_string(data);
        assert!(result.is_none());
    }
}
