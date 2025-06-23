use model_manager::{
    CoreValue, DefaultModelManager, DefaultValueFactory, DynamicValue, ModelError, ModelManager,
    ModelManagerFactory, ModelResult, PathNavigation, ValueAnalysis, ValueFactory,
};

type Value = <DefaultValueFactory as ValueFactory>::Value;
type Manager = <DefaultModelManager as ModelManagerFactory<Value>>::Manager;

#[test]
fn test_get_by_path_single_level() {
    let mut data = DefaultValueFactory::create_object();
    data.set("name", DefaultValueFactory::create_string("John"))
        .unwrap();
    data.set("age", DefaultValueFactory::create_number(30.0).unwrap())
        .unwrap();
    data.set("active", DefaultValueFactory::create_bool(true))
        .unwrap();

    let name_result = data.get_by_path("name").unwrap();
    let age_result = data.get_by_path("age").unwrap();
    let active_result = data.get_by_path("active").unwrap();

    assert!(name_result.is_some());
    assert_eq!(name_result.unwrap().as_str().unwrap(), "John");

    assert!(age_result.is_some());
    assert_eq!(age_result.unwrap().as_number().unwrap(), 30.0);

    assert!(active_result.is_some());
    assert_eq!(active_result.unwrap().as_bool().unwrap(), true);
}

#[test]
fn test_get_by_path_nested_levels() {
    let mut root = DefaultValueFactory::create_object();

    let mut level1 = DefaultValueFactory::create_object();
    let mut level2 = DefaultValueFactory::create_object();
    level2
        .set("value", DefaultValueFactory::create_string("deep_value"))
        .unwrap();
    level1.set("level2", level2).unwrap();
    root.set("level1", level1).unwrap();

    let result = root.get_by_path("level1.level2.value").unwrap();

    assert!(result.is_some());
    assert_eq!(result.unwrap().as_str().unwrap(), "deep_value");
}

#[test]
fn test_get_by_path_nonexistent_paths() {
    let mut data = DefaultValueFactory::create_object();
    data.set("existing", DefaultValueFactory::create_string("value"))
        .unwrap();

    let nonexistent_field = data.get_by_path("nonexistent").unwrap();
    let nonexistent_nested = data.get_by_path("existing.nonexistent").unwrap();
    let completely_invalid = data.get_by_path("a.b.c.d.e").unwrap();

    assert!(nonexistent_field.is_none());
    assert!(nonexistent_nested.is_none());
    assert!(completely_invalid.is_none());
}

#[test]
fn test_get_by_path_empty_path() {
    let mut data = DefaultValueFactory::create_object();
    data.set("field", DefaultValueFactory::create_string("value"))
        .unwrap();

    let result = data.get_by_path("").unwrap();

    assert!(result.is_some());
    let returned_obj = result.unwrap();
    assert!(returned_obj.is_object());

    let field_value = returned_obj.get("field").unwrap().unwrap();
    assert_eq!(field_value.as_str().unwrap(), "value");
}

#[test]
fn test_has_path_existing_paths() {
    let mut root = DefaultValueFactory::create_object();
    root.set("simple", DefaultValueFactory::create_string("value"))
        .unwrap();

    let mut nested = DefaultValueFactory::create_object();
    nested
        .set("inner", DefaultValueFactory::create_number(42.0).unwrap())
        .unwrap();
    root.set("nested", nested).unwrap();

    let has_simple = root.has_path("simple").unwrap();
    let has_nested = root.has_path("nested").unwrap();
    let has_deep = root.has_path("nested.inner").unwrap();

    assert!(has_simple);
    assert!(has_nested);
    assert!(has_deep);
}

#[test]
fn test_has_path_nonexistent_paths() {
    let mut data = DefaultValueFactory::create_object();
    data.set("exists", DefaultValueFactory::create_string("value"))
        .unwrap();

    let has_nonexistent = data.has_path("nonexistent").unwrap();
    let has_partial = data.has_path("exists.nonexistent").unwrap();
    let has_deep_invalid = data.has_path("a.b.c").unwrap();

    assert!(!has_nonexistent);
    assert!(!has_partial);
    assert!(!has_deep_invalid);
}

#[test]
fn test_set_by_path_empty_path_error() {
    let mut data = DefaultValueFactory::create_object();

    let result = data.set_by_path("", DefaultValueFactory::create_string("value"));

    assert!(result.is_err());
    if let Err(ModelError::InvalidData(msg)) = result {
        assert!(msg.contains("Empty path"));
    } else {
        panic!("Expected InvalidData error");
    }
}

#[test]
fn test_path_navigation_with_different_types() {
    let mut data = DefaultValueFactory::create_object();
    data.set("string_val", DefaultValueFactory::create_string("text"))
        .unwrap();
    data.set(
        "number_val",
        DefaultValueFactory::create_number(123.45).unwrap(),
    )
    .unwrap();
    data.set("bool_val", DefaultValueFactory::create_bool(false))
        .unwrap();

    let mut array_val = DefaultValueFactory::create_array();
    array_val
        .push(DefaultValueFactory::create_string("item1"))
        .unwrap();
    array_val
        .push(DefaultValueFactory::create_string("item2"))
        .unwrap();
    data.set("array_val", array_val).unwrap();

    let string_result = data.get_by_path("string_val").unwrap().unwrap();
    let number_result = data.get_by_path("number_val").unwrap().unwrap();
    let bool_result = data.get_by_path("bool_val").unwrap().unwrap();
    let array_result = data.get_by_path("array_val").unwrap().unwrap();

    assert_eq!(string_result.as_str().unwrap(), "text");
    assert_eq!(number_result.as_number().unwrap(), 123.45);
    assert_eq!(bool_result.as_bool().unwrap(), false);
    assert!(array_result.is_array());
}

#[test]
fn test_path_navigation_max_depth_by_levels() {
    let mut root = DefaultValueFactory::create_object();

    let mut level10 = DefaultValueFactory::create_object();
    level10
        .set("bottom", DefaultValueFactory::create_string("bottom"))
        .unwrap();

    let mut level9 = DefaultValueFactory::create_object();
    level9.set("level10", level10).unwrap();

    let mut level8 = DefaultValueFactory::create_object();
    level8.set("level9", level9).unwrap();

    let mut level7 = DefaultValueFactory::create_object();
    level7.set("level8", level8).unwrap();

    let mut level6 = DefaultValueFactory::create_object();
    level6.set("level7", level7).unwrap();

    let mut level5 = DefaultValueFactory::create_object();
    level5.set("level6", level6).unwrap();

    let mut level4 = DefaultValueFactory::create_object();
    level4.set("level5", level5).unwrap();

    let mut level3 = DefaultValueFactory::create_object();
    level3.set("level4", level4).unwrap();

    let mut level2 = DefaultValueFactory::create_object();
    level2.set("level3", level3).unwrap();

    let mut level1 = DefaultValueFactory::create_object();
    level1.set("level2", level2).unwrap();

    root.set("level1", level1).unwrap();

    let path = "level1.level2.level3.level4.level5.level6.level7.level8.level9.level10.bottom";
    let result = root.get_by_path(&path).unwrap();

    assert!(result.is_some());
    assert_eq!(result.unwrap().as_str().unwrap(), "bottom");
}

#[test]
fn test_serde_dynamic_value_equals() {
    let value1 = DefaultValueFactory::create_string("test_value");
    let value2 = DefaultValueFactory::create_string("test_value");
    let value3 = DefaultValueFactory::create_string("different_value");

    let are_equal = value1.equals(&value2).unwrap();
    let are_different = value1.equals(&value3).unwrap();

    assert!(are_equal);
    assert!(!are_different);
}

#[test]
fn test_serde_get_path_parts() {
    let simple_path = "field";
    let nested_path = "level1.level2.level3";
    let complex_path = "user.profile.settings.theme";

    let simple_parts = Value::get_path_parts(simple_path);
    let nested_parts = Value::get_path_parts(nested_path);
    let complex_parts = Value::get_path_parts(complex_path);

    assert_eq!(simple_parts, vec!["field"]);
    assert_eq!(nested_parts, vec!["level1", "level2", "level3"]);
    assert_eq!(complex_parts, vec!["user", "profile", "settings", "theme"]);
}

#[test]
fn test_serde_is_valid_path() {
    let valid_simple = "field";
    let valid_nested = "level1.level2";
    let invalid_empty = "";
    let invalid_empty_part = "level1..level3";
    let invalid_start_dot = ".level1";
    let invalid_end_dot = "level1.";

    let simple_valid = Value::is_valid_path(valid_simple);
    let nested_valid = Value::is_valid_path(valid_nested);
    let empty_invalid = Value::is_valid_path(invalid_empty);
    let empty_part_invalid = Value::is_valid_path(invalid_empty_part);
    let start_dot_invalid = Value::is_valid_path(invalid_start_dot);
    let end_dot_invalid = Value::is_valid_path(invalid_end_dot);

    assert!(simple_valid);
    assert!(nested_valid);
    assert!(!empty_invalid);
    assert!(!empty_part_invalid);
    assert!(!start_dot_invalid);
    assert!(!end_dot_invalid);
}

#[test]
fn test_path_with_special_characters() {
    let mut data = DefaultValueFactory::create_object();
    data.set(
        "field-with-dash",
        DefaultValueFactory::create_string("dash_value"),
    )
    .unwrap();
    data.set(
        "field_with_underscore",
        DefaultValueFactory::create_string("underscore_value"),
    )
    .unwrap();
    data.set(
        "field with spaces",
        DefaultValueFactory::create_string("spaces_value"),
    )
    .unwrap();

    let dash_result = data.get_by_path("field-with-dash").unwrap();
    let underscore_result = data.get_by_path("field_with_underscore").unwrap();
    let spaces_result = data.get_by_path("field with spaces").unwrap();

    assert!(dash_result.is_some());
    assert_eq!(dash_result.unwrap().as_str().unwrap(), "dash_value");

    assert!(underscore_result.is_some());
    assert_eq!(
        underscore_result.unwrap().as_str().unwrap(),
        "underscore_value"
    );

    assert!(spaces_result.is_some());
    assert_eq!(spaces_result.unwrap().as_str().unwrap(), "spaces_value");
}

#[test]
fn test_path_navigation_through_array() {
    let mut root = DefaultValueFactory::create_object();

    let mut array = DefaultValueFactory::create_array();
    array
        .push(DefaultValueFactory::create_string("item"))
        .unwrap();
    root.set("array_field", array).unwrap();

    let result = root.get_by_path("array_field.0").unwrap();

    assert!(result.is_none());
}

#[test]
fn test_path_navigation_through_non_object() {
    let mut data = DefaultValueFactory::create_object();
    data.set(
        "string_field",
        DefaultValueFactory::create_string("just_a_string"),
    )
    .unwrap();

    let result = data.get_by_path("string_field.nonexistent").unwrap();

    assert!(result.is_none());
}

#[test]
fn test_empty_values_in_path_navigation() {
    let mut data = DefaultValueFactory::create_object();
    data.set("empty_string", DefaultValueFactory::create_string(""))
        .unwrap();
    data.set(
        "zero_number",
        DefaultValueFactory::create_number(0.0).unwrap(),
    )
    .unwrap();
    data.set("false_bool", DefaultValueFactory::create_bool(false))
        .unwrap();

    let empty_str = data.get_by_path("empty_string").unwrap();
    let zero_num = data.get_by_path("zero_number").unwrap();
    let false_bool = data.get_by_path("false_bool").unwrap();

    assert!(empty_str.is_some());
    assert_eq!(empty_str.unwrap().as_str().unwrap(), "");

    assert!(zero_num.is_some());
    assert_eq!(zero_num.unwrap().as_number().unwrap(), 0.0);

    assert!(false_bool.is_some());
    assert_eq!(false_bool.unwrap().as_bool().unwrap(), false);

    let has_empty = data.has_path("empty_string").unwrap();
    let has_zero = data.has_path("zero_number").unwrap();
    let has_false = data.has_path("false_bool").unwrap();

    assert!(has_empty);
    assert!(has_zero);
    assert!(has_false);
}

#[test]
fn test_path_navigation_performance_simple() {
    let mut data = DefaultValueFactory::create_object();
    for i in 0..100 {
        data.set(
            &format!("field_{}", i),
            DefaultValueFactory::create_number(i as f64).unwrap(),
        )
        .unwrap();
    }

    let start = std::time::Instant::now();
    for i in 0..100 {
        let result = data.get_by_path(&format!("field_{}", i)).unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().as_number().unwrap(), i as f64);
    }
    let duration = start.elapsed();

    assert!(
        duration.as_millis() < 10,
        "Path navigation too slow: {:?}",
        duration
    );
    println!("✅ 100 path accesses completed in {:?}", duration);
}

#[test]
fn test_deep_path_performance() {
    let mut current = DefaultValueFactory::create_object();
    current
        .set("value", DefaultValueFactory::create_string("deep"))
        .unwrap();

    for i in (1..=20).rev() {
        let mut parent = DefaultValueFactory::create_object();
        parent.set(&format!("level{}", i), current).unwrap();
        current = parent;
    }

    let start = std::time::Instant::now();
    let path = (1..=20)
        .map(|i| format!("level{}", i))
        .collect::<Vec<_>>()
        .join(".");

    for _ in 0..10 {
        let result = current.get_by_path(&path).unwrap();
        assert!(result.is_some());
    }
    let duration = start.elapsed();

    assert!(
        duration.as_millis() < 50,
        "Deep path navigation too slow: {:?}",
        duration
    );
    println!(
        "✅ 10 deep path accesses (20 levels) completed in {:?}",
        duration
    );
}

#[test]
fn test_path_navigation_memory_safety() {
    let mut data = DefaultValueFactory::create_object();

    let large_string = "x".repeat(1000);
    data.set(
        "large_field",
        DefaultValueFactory::create_string(&large_string),
    )
    .unwrap();

    let mut nested = DefaultValueFactory::create_object();
    nested
        .set(
            "inner_large",
            DefaultValueFactory::create_string(&large_string),
        )
        .unwrap();
    data.set("nested", nested).unwrap();

    for _ in 0..100 {
        let result1 = data.get_by_path("large_field").unwrap();
        let result2 = data.get_by_path("nested.inner_large").unwrap();

        assert!(result1.is_some());
        assert!(result2.is_some());

        assert_eq!(result1.unwrap().as_str().unwrap().len(), 1000);
        assert_eq!(result2.unwrap().as_str().unwrap().len(), 1000);
    }

    println!("✅ Memory safety maintained during cloning operations");
}

#[test]
fn test_trait_contract_path_methods() {
    fn test_path_contract<T: DynamicValue>(mut value: T) -> ModelResult<bool> {
        value.set("test", T::from_str("contract_test"))?;

        let exists = value.has_path("test")?;
        let retrieved = value.get_by_path("test")?;

        if let Some(val) = retrieved {
            Ok(exists && val.as_str().unwrap() == "contract_test")
        } else {
            Ok(false)
        }
    }

    let serde_value = DefaultValueFactory::create_object();
    let result = test_path_contract(serde_value).unwrap();

    assert!(result);
    println!("✅ Trait contract for path methods verified");
}

#[test]
fn test_set_by_path_simple() {
    let mut data = DefaultValueFactory::create_object();

    let result = data.set_by_path("name", DefaultValueFactory::create_string("John Doe"));

    assert!(result.is_ok());

    let retrieved = data.get("name").unwrap().unwrap();
    assert_eq!(retrieved.as_str().unwrap(), "John Doe");

    println!("✅ Simple set_by_path test passed");
}

#[test]
fn test_set_by_path_nested_creation() {
    let mut data = DefaultValueFactory::create_object();

    let result = data.set_by_path(
        "user.profile.name",
        DefaultValueFactory::create_string("Jane Smith"),
    );

    assert!(result.is_ok());

    let user = data.get("user").unwrap().unwrap();
    assert!(user.is_object());

    let profile = user.get("profile").unwrap().unwrap();
    assert!(profile.is_object());

    let name = profile.get("name").unwrap().unwrap();
    assert_eq!(name.as_str().unwrap(), "Jane Smith");

    let retrieved = data.get_by_path("user.profile.name").unwrap().unwrap();
    assert_eq!(retrieved.as_str().unwrap(), "Jane Smith");

    println!("✅ Nested creation test passed");
}

#[test]
fn test_set_by_path_deep_nesting() {
    let mut data = DefaultValueFactory::create_object();

    let deep_path = "level1.level2.level3.level4.level5.value";
    let result = data.set_by_path(deep_path, DefaultValueFactory::create_string("deep_value"));

    assert!(result.is_ok());

    let retrieved = data.get_by_path(deep_path).unwrap().unwrap();
    assert_eq!(retrieved.as_str().unwrap(), "deep_value");

    assert!(data.has_path("level1").unwrap());
    assert!(data.has_path("level1.level2").unwrap());
    assert!(data.has_path("level1.level2.level3").unwrap());
    assert!(data.has_path("level1.level2.level3.level4").unwrap());
    assert!(data.has_path("level1.level2.level3.level4.level5").unwrap());

    println!("✅ Deep nesting test passed");
}

#[test]
fn test_set_by_path_overwrite_existing() {
    let mut data = DefaultValueFactory::create_object();

    data.set_by_path("user.name", DefaultValueFactory::create_string("Old Name"))
        .unwrap();
    data.set_by_path(
        "user.age",
        DefaultValueFactory::create_number(25.0).unwrap(),
    )
    .unwrap();

    let result = data.set_by_path("user.name", DefaultValueFactory::create_string("New Name"));

    assert!(result.is_ok());

    let name = data.get_by_path("user.name").unwrap().unwrap();
    assert_eq!(name.as_str().unwrap(), "New Name");

    let age = data.get_by_path("user.age").unwrap().unwrap();
    assert_eq!(age.as_number().unwrap(), 25.0);

    println!("✅ Overwrite existing test passed");
}

#[test]
fn test_set_by_path_replace_non_object() {
    let mut data = DefaultValueFactory::create_object();
    data.set("user", DefaultValueFactory::create_string("not an object"))
        .unwrap();

    let result = data.set_by_path("user.name", DefaultValueFactory::create_string("John"));

    assert!(result.is_ok());

    let user = data.get("user").unwrap().unwrap();
    assert!(user.is_object());

    let name = user.get("name").unwrap().unwrap();
    assert_eq!(name.as_str().unwrap(), "John");

    println!("✅ Replace non-object test passed");
}

#[test]
fn test_set_by_path_different_data_types() {
    let mut data = DefaultValueFactory::create_object();

    data.set_by_path(
        "config.string_val",
        DefaultValueFactory::create_string("test"),
    )
    .unwrap();
    data.set_by_path(
        "config.number_val",
        DefaultValueFactory::create_number(42.5).unwrap(),
    )
    .unwrap();
    data.set_by_path("config.bool_val", DefaultValueFactory::create_bool(true))
        .unwrap();

    let string_val = data.get_by_path("config.string_val").unwrap().unwrap();
    assert_eq!(string_val.as_str().unwrap(), "test");

    let number_val = data.get_by_path("config.number_val").unwrap().unwrap();
    assert_eq!(number_val.as_number().unwrap(), 42.5);

    let bool_val = data.get_by_path("config.bool_val").unwrap().unwrap();
    assert_eq!(bool_val.as_bool().unwrap(), true);

    println!("✅ Different data types test passed");
}

#[test]
fn test_set_by_path_error_cases() {
    let mut data = DefaultValueFactory::create_object();

    let result = data.set_by_path("", DefaultValueFactory::create_string("value"));
    assert!(result.is_err());
    if let Err(ModelError::InvalidData(msg)) = result {
        assert!(msg.contains("Empty path"));
    }

    let result = data.set_by_path(
        "level1..level3",
        DefaultValueFactory::create_string("value"),
    );
    assert!(result.is_err());
    if let Err(ModelError::InvalidData(msg)) = result {
        assert!(msg.contains("empty segment"));
    }

    let result = data.set_by_path(".level1", DefaultValueFactory::create_string("value"));
    assert!(result.is_err());

    let result = data.set_by_path("level1.", DefaultValueFactory::create_string("value"));
    assert!(result.is_err());

    let mut non_object = DefaultValueFactory::create_string("not an object");
    let result = non_object.set_by_path("some.path", DefaultValueFactory::create_string("value"));
    assert!(result.is_err());
    if let Err(ModelError::InvalidData(msg)) = result {
        assert!(msg.contains("non-object root"));
    }

    println!("✅ Error cases test passed");
}

#[test]
fn test_set_by_path_complex_scenario() {
    let mut company = DefaultValueFactory::create_object();

    company
        .set_by_path(
            "info.name",
            DefaultValueFactory::create_string("TechCorp Inc"),
        )
        .unwrap();
    company
        .set_by_path(
            "info.founded",
            DefaultValueFactory::create_number(2020.0).unwrap(),
        )
        .unwrap();

    company
        .set_by_path(
            "departments.engineering.head",
            DefaultValueFactory::create_string("Alice Johnson"),
        )
        .unwrap();
    company
        .set_by_path(
            "departments.engineering.budget",
            DefaultValueFactory::create_number(500000.0).unwrap(),
        )
        .unwrap();

    company
        .set_by_path(
            "departments.sales.head",
            DefaultValueFactory::create_string("Bob Smith"),
        )
        .unwrap();
    company
        .set_by_path(
            "departments.sales.budget",
            DefaultValueFactory::create_number(300000.0).unwrap(),
        )
        .unwrap();

    company
        .set_by_path(
            "locations.headquarters.city",
            DefaultValueFactory::create_string("San Francisco"),
        )
        .unwrap();
    company
        .set_by_path(
            "locations.headquarters.country",
            DefaultValueFactory::create_string("USA"),
        )
        .unwrap();

    company
        .set_by_path(
            "locations.branch_office.city",
            DefaultValueFactory::create_string("London"),
        )
        .unwrap();
    company
        .set_by_path(
            "locations.branch_office.country",
            DefaultValueFactory::create_string("UK"),
        )
        .unwrap();

    assert_eq!(
        company
            .get_by_path("info.name")
            .unwrap()
            .unwrap()
            .as_str()
            .unwrap(),
        "TechCorp Inc"
    );
    assert_eq!(
        company
            .get_by_path("departments.engineering.head")
            .unwrap()
            .unwrap()
            .as_str()
            .unwrap(),
        "Alice Johnson"
    );
    assert_eq!(
        company
            .get_by_path("departments.sales.budget")
            .unwrap()
            .unwrap()
            .as_number()
            .unwrap(),
        300000.0
    );
    assert_eq!(
        company
            .get_by_path("locations.headquarters.city")
            .unwrap()
            .unwrap()
            .as_str()
            .unwrap(),
        "San Francisco"
    );

    assert!(company.has_path("info").unwrap());
    assert!(company.has_path("departments").unwrap());
    assert!(company.has_path("departments.engineering").unwrap());
    assert!(company.has_path("departments.sales").unwrap());
    assert!(company.has_path("locations").unwrap());
    assert!(company.has_path("locations.headquarters").unwrap());
    assert!(company.has_path("locations.branch_office").unwrap());

    println!("✅ Complex scenario test passed");
    println!(
        "   Created enterprise structure with {} top-level sections",
        3
    );
}

#[test]
fn test_set_by_path_whitespace_handling() {
    let mut data = DefaultValueFactory::create_object();

    data.set_by_path(
        "  user.name  ",
        DefaultValueFactory::create_string("trimmed"),
    )
    .unwrap();

    let retrieved = data.get_by_path("user.name").unwrap().unwrap();
    assert_eq!(retrieved.as_str().unwrap(), "trimmed");

    let retrieved2 = data.get_by_path("user.name").unwrap().unwrap();
    assert_eq!(retrieved2.as_str().unwrap(), "trimmed");

    println!("✅ Whitespace handling test passed");
}

#[test]
fn test_set_by_path_performance() {
    let mut data = DefaultValueFactory::create_object();

    let start = std::time::Instant::now();

    for i in 0..100 {
        let path = format!("level1.level2.level3.item_{}", i);
        data.set_by_path(&path, DefaultValueFactory::create_number(i as f64).unwrap())
            .unwrap();
    }

    let duration = start.elapsed();

    assert!(duration.as_millis() < 1000);

    for i in 0..100 {
        let path = format!("level1.level2.level3.item_{}", i);
        let value = data.get_by_path(&path).unwrap().unwrap();
        assert_eq!(value.as_number().unwrap(), i as f64);
    }

    println!(
        "✅ Performance test passed: {} operations in {:?}",
        100, duration
    );
}

#[test]
fn test_model_manager_get_by_path_simple() {
    let mut manager = DefaultModelManager::create();

    let mut user = DefaultValueFactory::create_object();
    user.set("name", DefaultValueFactory::create_string("Ana García"))
        .unwrap();
    user.set(
        "email",
        DefaultValueFactory::create_string("ana@empresa.com"),
    )
    .unwrap();
    user.set("age", DefaultValueFactory::create_number(28.0).unwrap())
        .unwrap();

    let mut profile = DefaultValueFactory::create_object();
    profile
        .set(
            "department",
            DefaultValueFactory::create_string("Engineering"),
        )
        .unwrap();
    profile
        .set("level", DefaultValueFactory::create_number(5.0).unwrap())
        .unwrap();
    profile
        .set("active", DefaultValueFactory::create_bool(true))
        .unwrap();
    user.set("profile", profile).unwrap();

    manager
        .insert("users".to_string(), Some("user_001".to_string()), user)
        .unwrap();

    let name_result = manager
        .get_by_path(
            "users".to_string(),
            "user_001".to_string(),
            "name".to_string(),
        )
        .unwrap();

    let department_result = manager
        .get_by_path(
            "users".to_string(),
            "user_001".to_string(),
            "profile.department".to_string(),
        )
        .unwrap();

    let level_result = manager
        .get_by_path(
            "users".to_string(),
            "user_001".to_string(),
            "profile.level".to_string(),
        )
        .unwrap();

    assert!(name_result.is_some());
    assert_eq!(name_result.unwrap().as_str().unwrap(), "Ana García");

    assert!(department_result.is_some());
    assert_eq!(department_result.unwrap().as_str().unwrap(), "Engineering");

    assert!(level_result.is_some());
    assert_eq!(level_result.unwrap().as_number().unwrap(), 5.0);

    println!("✅ ModelManager get_by_path simple test passed");
}

#[test]
fn test_model_manager_get_by_path_nonexistent_paths() {
    let mut manager = DefaultModelManager::create();

    let mut user = DefaultValueFactory::create_object();
    user.set("name", DefaultValueFactory::create_string("Test User"))
        .unwrap();
    manager
        .insert("users".to_string(), Some("user_001".to_string()), user)
        .unwrap();

    let nonexistent_field = manager
        .get_by_path(
            "users".to_string(),
            "user_001".to_string(),
            "nonexistent".to_string(),
        )
        .unwrap();

    let nonexistent_nested = manager
        .get_by_path(
            "users".to_string(),
            "user_001".to_string(),
            "name.invalid".to_string(),
        )
        .unwrap();

    assert!(nonexistent_field.is_none());
    assert!(nonexistent_nested.is_none());

    println!("✅ ModelManager get_by_path nonexistent paths test passed");
}

#[test]
fn test_model_manager_get_by_path_nonexistent_record() {
    let mut manager = DefaultModelManager::create();

    let result = manager.get_by_path(
        "users".to_string(),
        "nonexistent_user".to_string(),
        "name".to_string(),
    );

    assert!(result.is_err());
    if let Err(ModelError::NotFound(id)) = result {
        assert_eq!(id, "nonexistent_user");
    } else {
        panic!("Expected NotFound error");
    }

    println!("✅ ModelManager get_by_path nonexistent record test passed");
}

#[test]
fn test_model_manager_find_by_path_exists_basic() {
    let mut manager = DefaultModelManager::create();

    let mut user1 = DefaultValueFactory::create_object();
    user1
        .set("name", DefaultValueFactory::create_string("User 1"))
        .unwrap();
    user1
        .set(
            "email",
            DefaultValueFactory::create_string("user1@test.com"),
        )
        .unwrap();
    user1
        .set("phone", DefaultValueFactory::create_string("123-456-7890"))
        .unwrap();
    manager.insert("users".to_string(), None, user1).unwrap();

    let mut user2 = DefaultValueFactory::create_object();
    user2
        .set("name", DefaultValueFactory::create_string("User 2"))
        .unwrap();
    user2
        .set(
            "email",
            DefaultValueFactory::create_string("user2@test.com"),
        )
        .unwrap();
    manager.insert("users".to_string(), None, user2).unwrap();

    let mut user3 = DefaultValueFactory::create_object();
    user3
        .set("name", DefaultValueFactory::create_string("User 3"))
        .unwrap();
    user3
        .set("phone", DefaultValueFactory::create_string("098-765-4321"))
        .unwrap();
    manager.insert("users".to_string(), None, user3).unwrap();

    let users_with_email = manager
        .find_by_path_exists("users".to_string(), "email".to_string())
        .unwrap();

    let users_with_phone = manager
        .find_by_path_exists("users".to_string(), "phone".to_string())
        .unwrap();

    let users_with_name = manager
        .find_by_path_exists("users".to_string(), "name".to_string())
        .unwrap();

    assert_eq!(users_with_email.len(), 2);
    assert_eq!(users_with_phone.len(), 2);
    assert_eq!(users_with_name.len(), 3);

    for user in &users_with_email {
        assert!(user.get("email").unwrap().is_some());
    }

    for user in &users_with_phone {
        assert!(user.get("phone").unwrap().is_some());
    }

    println!("✅ ModelManager find_by_path_exists basic test passed");
}

#[test]
fn test_model_manager_find_by_path_exists_nested() {
    let mut manager = DefaultModelManager::create();

    let mut user1 = DefaultValueFactory::create_object();
    user1
        .set("name", DefaultValueFactory::create_string("User 1"))
        .unwrap();

    let mut profile1 = DefaultValueFactory::create_object();
    profile1
        .set(
            "department",
            DefaultValueFactory::create_string("Engineering"),
        )
        .unwrap();
    profile1
        .set("level", DefaultValueFactory::create_number(5.0).unwrap())
        .unwrap();
    user1.set("profile", profile1).unwrap();

    manager.insert("users".to_string(), None, user1).unwrap();

    let mut user2 = DefaultValueFactory::create_object();
    user2
        .set("name", DefaultValueFactory::create_string("User 2"))
        .unwrap();

    let mut profile2 = DefaultValueFactory::create_object();
    profile2
        .set("department", DefaultValueFactory::create_string("Sales"))
        .unwrap();
    user2.set("profile", profile2).unwrap();

    manager.insert("users".to_string(), None, user2).unwrap();

    let mut user3 = DefaultValueFactory::create_object();
    user3
        .set("name", DefaultValueFactory::create_string("User 3"))
        .unwrap();
    manager.insert("users".to_string(), None, user3).unwrap();

    let users_with_profile = manager
        .find_by_path_exists("users".to_string(), "profile".to_string())
        .unwrap();

    let users_with_department = manager
        .find_by_path_exists("users".to_string(), "profile.department".to_string())
        .unwrap();

    let users_with_level = manager
        .find_by_path_exists("users".to_string(), "profile.level".to_string())
        .unwrap();

    assert_eq!(users_with_profile.len(), 2);
    assert_eq!(users_with_department.len(), 2);
    assert_eq!(users_with_level.len(), 1);

    println!("✅ ModelManager find_by_path_exists nested test passed");
}

#[test]
fn test_model_manager_find_by_path_value_basic() {
    let mut manager = DefaultModelManager::create();

    let mut user1 = DefaultValueFactory::create_object();
    user1
        .set("name", DefaultValueFactory::create_string("Ana"))
        .unwrap();
    user1
        .set(
            "department",
            DefaultValueFactory::create_string("Engineering"),
        )
        .unwrap();
    user1
        .set("active", DefaultValueFactory::create_bool(true))
        .unwrap();
    manager.insert("users".to_string(), None, user1).unwrap();

    let mut user2 = DefaultValueFactory::create_object();
    user2
        .set("name", DefaultValueFactory::create_string("Carlos"))
        .unwrap();
    user2
        .set("department", DefaultValueFactory::create_string("Sales"))
        .unwrap();
    user2
        .set("active", DefaultValueFactory::create_bool(true))
        .unwrap();
    manager.insert("users".to_string(), None, user2).unwrap();

    let mut user3 = DefaultValueFactory::create_object();
    user3
        .set("name", DefaultValueFactory::create_string("Maria"))
        .unwrap();
    user3
        .set(
            "department",
            DefaultValueFactory::create_string("Engineering"),
        )
        .unwrap();
    user3
        .set("active", DefaultValueFactory::create_bool(false))
        .unwrap();
    manager.insert("users".to_string(), None, user3).unwrap();

    let engineering_users = manager
        .find_by_path_value(
            "users".to_string(),
            "department".to_string(),
            DefaultValueFactory::create_string("Engineering"),
        )
        .unwrap();

    let sales_users = manager
        .find_by_path_value(
            "users".to_string(),
            "department".to_string(),
            DefaultValueFactory::create_string("Sales"),
        )
        .unwrap();

    let active_users = manager
        .find_by_path_value(
            "users".to_string(),
            "active".to_string(),
            DefaultValueFactory::create_bool(true),
        )
        .unwrap();

    let inactive_users = manager
        .find_by_path_value(
            "users".to_string(),
            "active".to_string(),
            DefaultValueFactory::create_bool(false),
        )
        .unwrap();

    assert_eq!(engineering_users.len(), 2);
    assert_eq!(sales_users.len(), 1);
    assert_eq!(active_users.len(), 2);
    assert_eq!(inactive_users.len(), 1);

    for user in &engineering_users {
        let dept = user.get("department").unwrap().unwrap();
        assert_eq!(dept.as_str().unwrap(), "Engineering");
    }

    for user in &active_users {
        let active = user.get("active").unwrap().unwrap();
        assert_eq!(active.as_bool().unwrap(), true);
    }

    println!("✅ ModelManager find_by_path_value basic test passed");
}

#[test]
fn test_model_manager_find_by_path_value_nested() {
    let mut manager = DefaultModelManager::create();

    let mut user1 = DefaultValueFactory::create_object();
    user1
        .set("name", DefaultValueFactory::create_string("Senior Dev"))
        .unwrap();

    let mut profile1 = DefaultValueFactory::create_object();
    profile1
        .set(
            "department",
            DefaultValueFactory::create_string("Engineering"),
        )
        .unwrap();
    profile1
        .set("level", DefaultValueFactory::create_number(8.0).unwrap())
        .unwrap();
    profile1
        .set("remote", DefaultValueFactory::create_bool(true))
        .unwrap();
    user1.set("profile", profile1).unwrap();

    manager.insert("users".to_string(), None, user1).unwrap();

    let mut user2 = DefaultValueFactory::create_object();
    user2
        .set("name", DefaultValueFactory::create_string("Junior Dev"))
        .unwrap();

    let mut profile2 = DefaultValueFactory::create_object();
    profile2
        .set(
            "department",
            DefaultValueFactory::create_string("Engineering"),
        )
        .unwrap();
    profile2
        .set("level", DefaultValueFactory::create_number(3.0).unwrap())
        .unwrap();
    profile2
        .set("remote", DefaultValueFactory::create_bool(false))
        .unwrap();
    user2.set("profile", profile2).unwrap();

    manager.insert("users".to_string(), None, user2).unwrap();

    let mut user3 = DefaultValueFactory::create_object();
    user3
        .set("name", DefaultValueFactory::create_string("Sales Manager"))
        .unwrap();

    let mut profile3 = DefaultValueFactory::create_object();
    profile3
        .set("department", DefaultValueFactory::create_string("Sales"))
        .unwrap();
    profile3
        .set("level", DefaultValueFactory::create_number(7.0).unwrap())
        .unwrap();
    profile3
        .set("remote", DefaultValueFactory::create_bool(true))
        .unwrap();
    user3.set("profile", profile3).unwrap();

    manager.insert("users".to_string(), None, user3).unwrap();

    let engineering_users = manager
        .find_by_path_value(
            "users".to_string(),
            "profile.department".to_string(),
            DefaultValueFactory::create_string("Engineering"),
        )
        .unwrap();

    let remote_users = manager
        .find_by_path_value(
            "users".to_string(),
            "profile.remote".to_string(),
            DefaultValueFactory::create_bool(true),
        )
        .unwrap();

    let senior_users = manager
        .find_by_path_value(
            "users".to_string(),
            "profile.level".to_string(),
            DefaultValueFactory::create_number(8.0).unwrap(),
        )
        .unwrap();

    assert_eq!(engineering_users.len(), 2);
    assert_eq!(remote_users.len(), 2);
    assert_eq!(senior_users.len(), 1);

    for user in &engineering_users {
        let dept = user
            .get("profile")
            .unwrap()
            .unwrap()
            .get("department")
            .unwrap()
            .unwrap();
        assert_eq!(dept.as_str().unwrap(), "Engineering");
    }

    let senior_user = &senior_users[0];
    let name = senior_user.get("name").unwrap().unwrap();
    assert_eq!(name.as_str().unwrap(), "Senior Dev");

    println!("✅ ModelManager find_by_path_value nested test passed");
}

#[test]
fn test_model_manager_find_by_path_value_no_matches() {
    let mut manager = DefaultModelManager::create();

    let mut user1 = DefaultValueFactory::create_object();
    user1
        .set(
            "department",
            DefaultValueFactory::create_string("Engineering"),
        )
        .unwrap();
    manager.insert("users".to_string(), None, user1).unwrap();

    let mut user2 = DefaultValueFactory::create_object();
    user2
        .set("department", DefaultValueFactory::create_string("Sales"))
        .unwrap();
    manager.insert("users".to_string(), None, user2).unwrap();

    let marketing_users = manager
        .find_by_path_value(
            "users".to_string(),
            "department".to_string(),
            DefaultValueFactory::create_string("Marketing"),
        )
        .unwrap();

    let nonexistent_field = manager
        .find_by_path_value(
            "users".to_string(),
            "nonexistent_field".to_string(),
            DefaultValueFactory::create_string("any_value"),
        )
        .unwrap();

    assert_eq!(marketing_users.len(), 0);
    assert_eq!(nonexistent_field.len(), 0);

    println!("✅ ModelManager find_by_path_value no matches test passed");
}

#[test]
fn test_model_manager_path_methods_with_different_data_types() {
    let mut manager = DefaultModelManager::create();

    let mut record = DefaultValueFactory::create_object();
    record
        .set(
            "string_field",
            DefaultValueFactory::create_string("test_string"),
        )
        .unwrap();
    record
        .set(
            "number_field",
            DefaultValueFactory::create_number(42.5).unwrap(),
        )
        .unwrap();
    record
        .set("bool_field", DefaultValueFactory::create_bool(true))
        .unwrap();

    let mut array_field = DefaultValueFactory::create_array();
    array_field
        .push(DefaultValueFactory::create_string("item1"))
        .unwrap();
    array_field
        .push(DefaultValueFactory::create_string("item2"))
        .unwrap();
    record.set("array_field", array_field).unwrap();

    manager
        .insert("records".to_string(), Some("rec_001".to_string()), record)
        .unwrap();

    let string_result = manager
        .get_by_path(
            "records".to_string(),
            "rec_001".to_string(),
            "string_field".to_string(),
        )
        .unwrap();

    let number_result = manager
        .get_by_path(
            "records".to_string(),
            "rec_001".to_string(),
            "number_field".to_string(),
        )
        .unwrap();

    let bool_result = manager
        .get_by_path(
            "records".to_string(),
            "rec_001".to_string(),
            "bool_field".to_string(),
        )
        .unwrap();

    let records_with_string = manager
        .find_by_path_value(
            "records".to_string(),
            "string_field".to_string(),
            DefaultValueFactory::create_string("test_string"),
        )
        .unwrap();

    let records_with_number = manager
        .find_by_path_value(
            "records".to_string(),
            "number_field".to_string(),
            DefaultValueFactory::create_number(42.5).unwrap(),
        )
        .unwrap();

    let records_with_bool = manager
        .find_by_path_value(
            "records".to_string(),
            "bool_field".to_string(),
            DefaultValueFactory::create_bool(true),
        )
        .unwrap();

    assert!(string_result.is_some());
    assert_eq!(string_result.unwrap().as_str().unwrap(), "test_string");

    assert!(number_result.is_some());
    assert_eq!(number_result.unwrap().as_number().unwrap(), 42.5);

    assert!(bool_result.is_some());
    assert_eq!(bool_result.unwrap().as_bool().unwrap(), true);

    assert_eq!(records_with_string.len(), 1);
    assert_eq!(records_with_number.len(), 1);
    assert_eq!(records_with_bool.len(), 1);

    println!("✅ ModelManager path methods with different data types test passed");
}

#[test]
fn test_model_manager_path_methods_performance() {
    let mut manager = DefaultModelManager::create();

    for i in 0..100 {
        let mut user = DefaultValueFactory::create_object();
        user.set(
            "name",
            DefaultValueFactory::create_string(&format!("User {}", i)),
        )
        .unwrap();
        user.set(
            "department",
            Value::from_str(match i % 3 {
                0 => "Engineering",
                1 => "Sales",
                2 => "Marketing",
                _ => unreachable!(),
            }),
        )
        .unwrap();
        user.set(
            "level",
            DefaultValueFactory::create_number((i % 10 + 1) as f64).unwrap(),
        )
        .unwrap();

        manager.insert("users".to_string(), None, user).unwrap();
    }

    let start = std::time::Instant::now();

    let users_with_department = manager
        .find_by_path_exists("users".to_string(), "department".to_string())
        .unwrap();

    let engineering_users = manager
        .find_by_path_value(
            "users".to_string(),
            "department".to_string(),
            DefaultValueFactory::create_string("Engineering"),
        )
        .unwrap();

    let duration = start.elapsed();

    assert_eq!(users_with_department.len(), 100);
    assert!(engineering_users.len() >= 30);
    assert!(duration.as_millis() < 100);

    println!(
        "✅ Path operations on 100 records completed in {:?}",
        duration
    );
    println!(
        "   - Users with department: {}",
        users_with_department.len()
    );
    println!("   - Engineering users: {}", engineering_users.len());
}

#[test]
fn test_model_manager_path_methods_empty_model() {
    let mut manager = DefaultModelManager::create();

    let exists_results = manager
        .find_by_path_exists("empty_model".to_string(), "any_field".to_string())
        .unwrap();

    let value_results = manager
        .find_by_path_value(
            "empty_model".to_string(),
            "any_field".to_string(),
            DefaultValueFactory::create_string("any_value"),
        )
        .unwrap();

    assert_eq!(exists_results.len(), 0);
    assert_eq!(value_results.len(), 0);

    println!("✅ ModelManager path methods on empty model test passed");
}

#[test]
fn test_model_manager_path_methods_complex_scenario() {
    let mut manager = DefaultModelManager::create();

    let mut employee1 = DefaultValueFactory::create_object();
    employee1
        .set("name", DefaultValueFactory::create_string("Alice"))
        .unwrap();
    employee1
        .set(
            "department",
            DefaultValueFactory::create_string("Engineering"),
        )
        .unwrap();

    let mut projects1 = DefaultValueFactory::create_array();
    let mut project1 = DefaultValueFactory::create_object();
    project1
        .set("name", DefaultValueFactory::create_string("Project Alpha"))
        .unwrap();
    project1
        .set("status", DefaultValueFactory::create_string("active"))
        .unwrap();
    projects1.push(project1).unwrap();
    employee1.set("projects", projects1).unwrap();

    manager
        .insert(
            "employees".to_string(),
            Some("emp_001".to_string()),
            employee1,
        )
        .unwrap();

    let mut employee2 = DefaultValueFactory::create_object();
    employee2
        .set("name", DefaultValueFactory::create_string("Bob"))
        .unwrap();
    employee2
        .set("department", DefaultValueFactory::create_string("Sales"))
        .unwrap();
    manager
        .insert(
            "employees".to_string(),
            Some("emp_002".to_string()),
            employee2,
        )
        .unwrap();

    let mut employee3 = DefaultValueFactory::create_object();
    employee3
        .set("name", DefaultValueFactory::create_string("Charlie"))
        .unwrap();
    employee3
        .set(
            "department",
            DefaultValueFactory::create_string("Engineering"),
        )
        .unwrap();

    let mut projects3 = DefaultValueFactory::create_array();
    let mut project3 = DefaultValueFactory::create_object();
    project3
        .set("name", DefaultValueFactory::create_string("Project Beta"))
        .unwrap();
    project3
        .set("status", DefaultValueFactory::create_string("completed"))
        .unwrap();
    projects3.push(project3).unwrap();
    employee3.set("projects", projects3).unwrap();

    manager
        .insert(
            "employees".to_string(),
            Some("emp_003".to_string()),
            employee3,
        )
        .unwrap();

    let engineering_employees = manager
        .find_by_path_value(
            "employees".to_string(),
            "department".to_string(),
            DefaultValueFactory::create_string("Engineering"),
        )
        .unwrap();

    let employees_with_projects = manager
        .find_by_path_exists("employees".to_string(), "projects".to_string())
        .unwrap();

    let alice_department = manager
        .get_by_path(
            "employees".to_string(),
            "emp_001".to_string(),
            "department".to_string(),
        )
        .unwrap();

    assert_eq!(engineering_employees.len(), 2);
    assert_eq!(employees_with_projects.len(), 2);

    assert!(alice_department.is_some());
    assert_eq!(alice_department.unwrap().as_str().unwrap(), "Engineering");

    for emp in &engineering_employees {
        let dept = emp.get("department").unwrap().unwrap();
        assert_eq!(dept.as_str().unwrap(), "Engineering");
    }

    for emp in &employees_with_projects {
        assert!(emp.get("projects").unwrap().is_some());
    }

    println!("✅ ModelManager complex path scenario test passed");
    println!(
        "   - Engineering employees: {}",
        engineering_employees.len()
    );
    println!(
        "   - Employees with projects: {}",
        employees_with_projects.len()
    );
}
