use tokio::task::LocalSet;
use model_manager::{
    DynamicValue, DefaultValue, DefaultConverterFactory, ConverterFactory,
    JsonConverter, DataConverter, JsonConverterConfig, ConverterConfig,
};

#[tokio::test]
async fn test_json_converter_basic() {
    let local_set = LocalSet::new();

    local_set.run_until(async {
        let converter = DefaultConverterFactory::create_json_converter();

        // Test from JSON
        let json_input = r#"{"name":"Test User","age":30,"active":true}"#.to_string();
        let dynamic_value = converter.from_external(json_input).await.unwrap();

        assert!(dynamic_value.is_object());
        assert_eq!(dynamic_value.get("name").await.unwrap().unwrap().as_str().unwrap(), "Test User");
        assert_eq!(dynamic_value.get("age").await.unwrap().unwrap().as_number().unwrap(), 30.0);
        assert_eq!(dynamic_value.get("active").await.unwrap().unwrap().as_bool().unwrap(), true);

        // Test to JSON
        let json_output = converter.to_external(&dynamic_value).await.unwrap();
        assert!(json_output.contains("Test User"));
        assert!(json_output.contains("30"));
        assert!(json_output.contains("true"));

        println!("JSON Basic Test - INPUT: {}", r#"{"name":"Test User","age":30,"active":true}"#);
        println!("JSON Basic Test - OUTPUT: {}", json_output);
    }).await;
}

#[tokio::test]
async fn test_json_converter_complex_data() {
    let local_set = LocalSet::new();

    local_set.run_until(async {
        let converter = DefaultConverterFactory::create_json_converter();

        // Test complex JSON structure
        let complex_json = r#"{
            "user": {
                "name": "Ana García",
                "profile": {
                    "department": "Engineering",
                    "level": 5
                }
            },
            "items": [
                {"id": 1, "value": "Item 1"},
                {"id": 2, "value": "Item 2"}
            ],
            "metadata": {
                "created": "2025-01-15",
                "version": 1.2
            }
        }"#.to_string();

        let dynamic_value = converter.from_external(complex_json).await.unwrap();

        // Verify complex structure
        assert!(dynamic_value.is_object());

        let user = dynamic_value.get("user").await.unwrap().unwrap();
        assert_eq!(user.get("name").await.unwrap().unwrap().as_str().unwrap(), "Ana García");

        let profile = user.get("profile").await.unwrap().unwrap();
        assert_eq!(profile.get("department").await.unwrap().unwrap().as_str().unwrap(), "Engineering");
        assert_eq!(profile.get("level").await.unwrap().unwrap().as_number().unwrap(), 5.0);

        let items = dynamic_value.get("items").await.unwrap().unwrap();
        let items_array = items.as_array().await.unwrap().unwrap();
        assert_eq!(items_array.len(), 2);
        assert_eq!(items_array[0].get("id").await.unwrap().unwrap().as_number().unwrap(), 1.0);

        // Test back to JSON
        let json_output = converter.to_external(&dynamic_value).await.unwrap();
        assert!(json_output.contains("Ana García"));
        assert!(json_output.contains("Engineering"));

        println!("Complex JSON Test - Conversion successful");
        println!("OUTPUT length: {} characters", json_output.len());
    }).await;
}

#[tokio::test]
async fn test_json_converter_batch_operations() {
    let local_set = LocalSet::new();

    local_set.run_until(async {
        let converter = DefaultConverterFactory::create_json_converter();

        // Test batch conversion from JSON
        let json_inputs = vec![
            r#"{"id":1,"name":"User 1"}"#.to_string(),
            r#"{"id":2,"name":"User 2"}"#.to_string(),
            r#"{"id":3,"name":"User 3"}"#.to_string(),
        ];

        let dynamic_values = converter.from_external_batch(json_inputs).await.unwrap();
        assert_eq!(dynamic_values.len(), 3);

        for (i, value) in dynamic_values.iter().enumerate() {
            assert_eq!(value.get("id").await.unwrap().unwrap().as_number().unwrap(), (i + 1) as f64);
            assert_eq!(value.get("name").await.unwrap().unwrap().as_str().unwrap(), format!("User {}", i + 1));
        }

        // Test batch conversion to JSON
        let json_outputs = converter.to_external_batch(&dynamic_values).await.unwrap();
        assert_eq!(json_outputs.len(), 3);

        for (i, json) in json_outputs.iter().enumerate() {
            assert!(json.contains(&format!("User {}", i + 1)));
            assert!(json.contains(&format!("\"id\":{}", i + 1)));
        }

        println!("Batch conversion: {} items processed successfully", dynamic_values.len());
    }).await;
}

#[tokio::test]
async fn test_json_converter_pretty_printing() {
    let local_set = LocalSet::new();

    local_set.run_until(async {
        let converter = DefaultConverterFactory::create_json_converter();

        // Create test data
        let mut data = DefaultValue::new_object();
        data.set("name", DefaultValue::from_str("Pretty Test")).await.unwrap();
        data.set("age", DefaultValue::from_number(25.0).unwrap()).await.unwrap();

        let mut nested = DefaultValue::new_object();
        nested.set("department", DefaultValue::from_str("Engineering")).await.unwrap();
        data.set("profile", nested).await.unwrap();

        // Test regular JSON output
        let regular_json = converter.to_external(&data).await.unwrap();
        assert!(!regular_json.contains('\n')); // Should be compact

        // Test pretty JSON output
        let pretty_json = converter.to_json_pretty(&data).await.unwrap();
        assert!(pretty_json.contains('\n')); // Should be formatted
        assert!(pretty_json.contains("  ")); // Should have indentation

        println!("Regular JSON: {}", regular_json);
        println!("Pretty JSON:\n{}", pretty_json);
    }).await;
}

#[tokio::test]
async fn test_json_converter_error_handling() {
    let local_set = LocalSet::new();

    local_set.run_until(async {
        let converter = DefaultConverterFactory::create_json_converter();

        // Test invalid JSON
        let invalid_json = r#"{"name": "Test", "age": }"#.to_string(); // Missing value
        let result = converter.from_external(invalid_json).await;
        assert!(result.is_err());

        if let Err(error) = result {
            assert!(error.to_string().contains("JSON parse error"));
            println!("Error handling test - Expected error: {}", error);
        }

        // Test large JSON (exceeds size limit)
        let large_json = "\"".to_string() + &"x".repeat(20 * 1024 * 1024) + "\""; // 20MB string
        let large_result = converter.from_external(large_json).await;
        assert!(large_result.is_err());

        if let Err(error) = large_result {
            assert!(error.to_string().contains("too large"));
            println!("Size limit test - Expected error: {}", error);
        }
    }).await;
}

#[tokio::test]
async fn test_json_converter_with_custom_config() {
    let local_set = LocalSet::new();

    local_set.run_until(async {
        let custom_config = JsonConverterConfig {
            base: ConverterConfig {
                pretty_output: true,
                max_direct_conversion_size: 1024, // 1KB limit
                ..Default::default()
            },
            indent_size: 4,
            ..Default::default()
        };

        let converter = DefaultConverterFactory::create_json_converter_with_config(custom_config);

        // Test pretty output by default
        let mut data = DefaultValue::new_object();
        data.set("test", DefaultValue::from_str("custom config")).await.unwrap();

        let json_output = converter.to_external(&data).await.unwrap();
        assert!(json_output.contains('\n')); // Should be pretty by default

        println!("Custom config test - Pretty output: {}", json_output);

        // Test size limit
        let large_input = "\"".to_string() + &"x".repeat(2048) + "\""; // 2KB > 1KB limit
        let large_result = converter.from_external(large_input).await;
        assert!(large_result.is_err());

        println!("Custom config test - Size limit enforced correctly");
    }).await;
}