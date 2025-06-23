use model_manager::{
    ConverterConfig, ConverterFactory, CoreValue, DataConverter, DefaultConverter,
    DefaultValueFactory, JsonConverter, JsonConverterConfig, ValueFactory,
};

#[test]
fn test_json_converter_basic() {
    let converter = DefaultConverter::create_json_converter();

    // Test from JSON
    let json_input = r#"{"name":"Test User","age":30,"active":true}"#.to_string();
    let dynamic_value = converter.from_external(json_input).unwrap();

    assert!(dynamic_value.is_object());
    assert_eq!(
        dynamic_value
            .get("name")
            .unwrap()
            .unwrap()
            .as_str()
            .unwrap(),
        "Test User"
    );
    assert_eq!(
        dynamic_value
            .get("age")
            .unwrap()
            .unwrap()
            .as_number()
            .unwrap(),
        30.0
    );
    assert_eq!(
        dynamic_value
            .get("active")
            .unwrap()
            .unwrap()
            .as_bool()
            .unwrap(),
        true
    );

    // Test to JSON
    let json_output = converter.to_external(&dynamic_value).unwrap();
    assert!(json_output.contains("Test User"));
    assert!(json_output.contains("30"));
    assert!(json_output.contains("true"));

    println!(
        "JSON Basic Test - INPUT: {}",
        r#"{"name":"Test User","age":30,"active":true}"#
    );
    println!("JSON Basic Test - OUTPUT: {}", json_output);
}

#[test]
fn test_json_converter_complex_data() {
    let converter = DefaultConverter::create_json_converter();

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
        }"#
    .to_string();

    let dynamic_value = converter.from_external(complex_json).unwrap();

    // Verify complex structure
    assert!(dynamic_value.is_object());

    let user = dynamic_value.get("user").unwrap().unwrap();
    assert_eq!(
        user.get("name").unwrap().unwrap().as_str().unwrap(),
        "Ana García"
    );

    let profile = user.get("profile").unwrap().unwrap();
    assert_eq!(
        profile
            .get("department")
            .unwrap()
            .unwrap()
            .as_str()
            .unwrap(),
        "Engineering"
    );
    assert_eq!(
        profile.get("level").unwrap().unwrap().as_number().unwrap(),
        5.0
    );

    let items = dynamic_value.get("items").unwrap().unwrap();
    let items_array = items.as_array().unwrap().unwrap();
    assert_eq!(items_array.len(), 2);
    assert_eq!(
        items_array[0]
            .get("id")
            .unwrap()
            .unwrap()
            .as_number()
            .unwrap(),
        1.0
    );

    // Test back to JSON
    let json_output = converter.to_external(&dynamic_value).unwrap();
    assert!(json_output.contains("Ana García"));
    assert!(json_output.contains("Engineering"));

    println!("Complex JSON Test - Conversion successful");
    println!("OUTPUT length: {} characters", json_output.len());
}

#[test]
fn test_json_converter_batch_operations() {
    let converter = DefaultConverter::create_json_converter();

    // Test batch conversion from JSON
    let json_inputs = vec![
        r#"{"id":1,"name":"User 1"}"#.to_string(),
        r#"{"id":2,"name":"User 2"}"#.to_string(),
        r#"{"id":3,"name":"User 3"}"#.to_string(),
    ];

    let dynamic_values = converter.from_external_batch(json_inputs).unwrap();
    assert_eq!(dynamic_values.len(), 3);

    for (i, value) in dynamic_values.iter().enumerate() {
        assert_eq!(
            value.get("id").unwrap().unwrap().as_number().unwrap(),
            (i + 1) as f64
        );
        assert_eq!(
            value.get("name").unwrap().unwrap().as_str().unwrap(),
            format!("User {}", i + 1)
        );
    }

    // Test batch conversion to JSON
    let json_outputs = converter.to_external_batch(&dynamic_values).unwrap();
    assert_eq!(json_outputs.len(), 3);

    for (i, json) in json_outputs.iter().enumerate() {
        assert!(json.contains(&format!("User {}", i + 1)));
        assert!(json.contains(&format!("\"id\":{}", i + 1)));
    }

    println!(
        "Batch conversion: {} items processed successfully",
        dynamic_values.len()
    );
}

#[test]
fn test_json_converter_pretty_printing() {
    let converter = DefaultConverter::create_json_converter();

    // Create test data
    let mut data = DefaultValueFactory::create_object();
    data.set("name", DefaultValueFactory::create_string("Pretty Test"))
        .unwrap();
    data.set("age", DefaultValueFactory::create_number(25.0).unwrap())
        .unwrap();

    let mut nested = DefaultValueFactory::create_object();
    nested
        .set(
            "department",
            DefaultValueFactory::create_string("Engineering"),
        )
        .unwrap();
    data.set("profile", nested).unwrap();

    // Test regular JSON output
    let regular_json = converter.to_external(&data).unwrap();
    assert!(!regular_json.contains('\n')); // Should be compact

    // Test pretty JSON output
    let pretty_json = converter.to_json_pretty(&data).unwrap();
    assert!(pretty_json.contains('\n')); // Should be formatted
    assert!(pretty_json.contains("  ")); // Should have indentation

    println!("Regular JSON: {}", regular_json);
    println!("Pretty JSON:\n{}", pretty_json);
}

#[test]
fn test_json_converter_error_handling() {
    let converter = DefaultConverter::create_json_converter();

    // Test invalid JSON
    let invalid_json = r#"{"name": "Test", "age": }"#.to_string(); // Missing value
    let result = converter.from_external(invalid_json);
    assert!(result.is_err());

    if let Err(error) = result {
        assert!(error.to_string().contains("JSON parse error"));
        println!("Error handling test - Expected error: {}", error);
    }

    // Test large JSON (exceeds size limit)
    let large_json = "\"".to_string() + &"x".repeat(20 * 1024 * 1024) + "\""; // 20MB string
    let large_result = converter.from_external(large_json);
    assert!(large_result.is_err());

    if let Err(error) = large_result {
        assert!(error.to_string().contains("too large"));
        println!("Size limit test - Expected error: {}", error);
    }
}

#[test]
fn test_json_converter_with_custom_config() {
    let custom_config = JsonConverterConfig {
        base: ConverterConfig {
            pretty_output: true,
            max_direct_conversion_size: 1024, // 1KB limit
            ..Default::default()
        },
        indent_size: 4,
        ..Default::default()
    };

    let converter = DefaultConverter::create_json_converter_with_config(custom_config);

    // Test pretty output by default
    let mut data = DefaultValueFactory::create_object();
    data.set("test", DefaultValueFactory::create_string("custom config"))
        .unwrap();

    let json_output = converter.to_external(&data).unwrap();
    assert!(json_output.contains('\n')); // Should be pretty by default

    println!("Custom config test - Pretty output: {}", json_output);

    // Test size limit
    let large_input = "\"".to_string() + &"x".repeat(2048) + "\""; // 2KB > 1KB limit
    let large_result = converter.from_external(large_input);
    assert!(large_result.is_err());

    println!("Custom config test - Size limit enforced correctly");
}
