#[cfg(test)]
mod tests {
    use model_manager::dynamic::application::dynamic_value_converter::DynamicValueConverter;
    use model_manager::dynamic::infrastructure::json_to_dynamic_value_converter::JsonToDynamicValueConverter;
    use model_manager::DynamicValue;

    #[test]
    fn convert_valid_json_string_to_dynamic_value() {
        let converter = JsonToDynamicValueConverter;
        let json_str = r#"{"key": "value"}"#;
        let result = converter.convert(json_str);
        assert!(result.is_ok());
        let dynamic_value = result.unwrap();
        assert_eq!(dynamic_value.get("key").unwrap().as_str(), Some("value".to_string()));
    }

    #[test]
    fn convert_invalid_json_string_returns_error() {
        let converter = JsonToDynamicValueConverter;
        let json_str = r#"{"key": "value""#; // Missing closing brace
        let result = converter.convert(json_str);
        assert!(result.is_err());
    }

    #[test]
    fn convert_empty_json_string_to_dynamic_value() {
        let converter = JsonToDynamicValueConverter;
        let json_str = r#"{}"#;
        let result = converter.convert(json_str);
        assert!(result.is_ok());
        let dynamic_value = result.unwrap();
        assert!(dynamic_value.is_empty());
    }

    #[test]
    fn convert_json_array_to_dynamic_value() {
        let converter = JsonToDynamicValueConverter;
        let json_str = r#"["value1", 12.5]"#;
        let result = converter.convert(json_str);
        assert!(result.is_ok());
        let dynamic_value = result.unwrap();
        let array = dynamic_value.as_array().unwrap();
        //assert_eq!(array[0].as_str(), Some("value1"));
        assert_eq!(array[1].as_number(), Some(12.5));
    }

    #[test]
    fn convert_complex_json_to_dynamic_value() {
        let converter = JsonToDynamicValueConverter;
        let json_str = r#"
        {
            "array": [
                {"key1": "value1"},
                {"key2": 42}
            ],
            "object": {
                "nested_key": "nested_value"
            },
            "string": "a_string",
            "number": 3.14
        }
        "#;
        let result = converter.convert(json_str);
        assert!(result.is_ok());
        let dynamic_value = result.unwrap();

        let array = dynamic_value.get("array").unwrap().as_array().unwrap();
        assert_eq!(array[0].get("key1").unwrap().as_str(), Some("value1".to_string()));
        assert_eq!(array[1].get("key2").unwrap().as_number(), Some(42.0));

        let object = dynamic_value.get("object").unwrap().as_map().unwrap();
        assert_eq!(object.get("nested_key").unwrap().as_str(), Some("nested_value".to_string()));

        assert_eq!(dynamic_value.get("string").unwrap().as_str(), Some("a_string".to_string()));
        assert_eq!(dynamic_value.get("number").unwrap().as_number(), Some(3.14));
    }
}