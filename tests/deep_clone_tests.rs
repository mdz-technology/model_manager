use std::time::Instant;
use model_manager::{DefaultModelManager, DefaultValue, DynamicValue, DynamicValueFactory, ModelManager, ModelManagerFactory};

type Value = <DefaultValue as DynamicValueFactory>::Value;

#[test]
fn test_deep_clone_basic_object() {
    let mut original = DefaultValue::create();
    original.set("name", Value::from_str("Test User")).unwrap();
    original.set("age", Value::from_number(25.0).unwrap()).unwrap();
    original.set("active", Value::from_bool(true)).unwrap();

    let cloned = original.deep_clone().unwrap();

    assert_eq!(original.get("name").unwrap().unwrap().as_str().unwrap(),
               cloned.get("name").unwrap().unwrap().as_str().unwrap());
    assert_eq!(original.get("age").unwrap().unwrap().as_number().unwrap(),
               cloned.get("age").unwrap().unwrap().as_number().unwrap());
    assert_eq!(original.get("active").unwrap().unwrap().as_bool().unwrap(),
               cloned.get("active").unwrap().unwrap().as_bool().unwrap());

    println!("✅ Basic object deep clone test passed");
}

#[test]
fn test_deep_clone_basic_array() {
    let mut original = Value::new_array();
    original.push(Value::from_str("item1")).unwrap();
    original.push(Value::from_number(42.0).unwrap()).unwrap();
    original.push(Value::from_bool(true)).unwrap();

    let cloned = original.deep_clone().unwrap();

    let original_array = original.as_array().unwrap().unwrap();
    let cloned_array = cloned.as_array().unwrap().unwrap();

    assert_eq!(original_array.len(), cloned_array.len());
    assert_eq!(original_array[0].as_str().unwrap(), cloned_array[0].as_str().unwrap());
    assert_eq!(original_array[1].as_number().unwrap(), cloned_array[1].as_number().unwrap());
    assert_eq!(original_array[2].as_bool().unwrap(), cloned_array[2].as_bool().unwrap());

    println!("✅ Basic array deep clone test passed");
}

#[test]
fn test_deep_clone_primitive_values() {
    let string_val = Value::from_str("test string");
    let number_val = Value::from_number(123.45).unwrap();
    let bool_val = Value::from_bool(false);

    let cloned_string = string_val.deep_clone().unwrap();
    let cloned_number = number_val.deep_clone().unwrap();
    let cloned_bool = bool_val.deep_clone().unwrap();

    assert_eq!(string_val.as_str().unwrap(), cloned_string.as_str().unwrap());
    assert_eq!(number_val.as_number().unwrap(), cloned_number.as_number().unwrap());
    assert_eq!(bool_val.as_bool().unwrap(), cloned_bool.as_bool().unwrap());

    println!("✅ Primitive values deep clone test passed");
}

#[test]
fn test_deep_clone_nested_structure() {
    let mut original = DefaultValue::create();
    original.set("title", Value::from_str("Root Document")).unwrap();

    let mut user = Value::new_object();
    user.set("name", Value::from_str("Ana García")).unwrap();
    user.set("email", Value::from_str("ana@empresa.com")).unwrap();

    let mut profile = Value::new_object();
    profile.set("department", Value::from_str("Engineering")).unwrap();
    profile.set("level", Value::from_number(8.0).unwrap()).unwrap();
    profile.set("remote", Value::from_bool(true)).unwrap();
    user.set("profile", profile).unwrap();

    let mut projects = Value::new_array();
    for i in 1..=5 {
        let mut project = Value::new_object();
        project.set("id", Value::from_number(i as f64).unwrap()).unwrap();
        project.set("name", Value::from_str(&format!("Project {}", i))).unwrap();
        project.set("active", Value::from_bool(i % 2 == 0)).unwrap();
        projects.push(project).unwrap();
    }
    user.set("projects", projects).unwrap();

    original.set("user", user).unwrap();

    let start = Instant::now();
    let cloned = original.deep_clone().unwrap();
    let duration = start.elapsed();

    assert_eq!(
        original.get_by_path("title").unwrap().unwrap().as_str().unwrap(),
        cloned.get_by_path("title").unwrap().unwrap().as_str().unwrap()
    );

    assert_eq!(
        original.get_by_path("user.name").unwrap().unwrap().as_str().unwrap(),
        cloned.get_by_path("user.name").unwrap().unwrap().as_str().unwrap()
    );

    assert_eq!(
        original.get_by_path("user.profile.department").unwrap().unwrap().as_str().unwrap(),
        cloned.get_by_path("user.profile.department").unwrap().unwrap().as_str().unwrap()
    );

    assert_eq!(
        original.get_by_path("user.profile.level").unwrap().unwrap().as_number().unwrap(),
        cloned.get_by_path("user.profile.level").unwrap().unwrap().as_number().unwrap()
    );

    let original_projects = original.get("user").unwrap().unwrap()
        .get("projects").unwrap().unwrap();
    let cloned_projects = cloned.get("user").unwrap().unwrap()
        .get("projects").unwrap().unwrap();

    let original_array = original_projects.as_array().unwrap().unwrap();
    let cloned_array = cloned_projects.as_array().unwrap().unwrap();

    assert_eq!(original_array.len(), cloned_array.len());
    assert_eq!(
        original_array[0].get("name").unwrap().unwrap().as_str().unwrap(),
        cloned_array[0].get("name").unwrap().unwrap().as_str().unwrap()
    );

    println!("✅ Nested structure deep clone test passed in {:?}", duration);
}

#[test]
fn test_deep_clone_independence() {
    let mut original = DefaultValue::create();
    original.set("shared_field", Value::from_str("original value")).unwrap();

    let mut nested = Value::new_object();
    nested.set("inner_field", Value::from_str("inner original")).unwrap();
    original.set("nested", nested).unwrap();

    let cloned = original.deep_clone().unwrap();

    original.set("shared_field", Value::from_str("MODIFIED")).unwrap();
    original.set("new_field", Value::from_str("added after clone")).unwrap();

    let mut modified_nested = original.get("nested").unwrap().unwrap();
    modified_nested.set("inner_field", Value::from_str("MODIFIED INNER")).unwrap();
    original.set("nested", modified_nested).unwrap();

    assert_eq!(
        cloned.get("shared_field").unwrap().unwrap().as_str().unwrap(),
        "original value"
    );

    assert!(cloned.get("new_field").unwrap().is_none());

    assert_eq!(
        cloned.get_by_path("nested.inner_field").unwrap().unwrap().as_str().unwrap(),
        "inner original"
    );

    assert_eq!(
        original.get("shared_field").unwrap().unwrap().as_str().unwrap(),
        "MODIFIED"
    );

    println!("✅ Deep clone independence test passed");
}

#[test]
fn test_deep_clone_large_dataset() {
    let mut large_object = DefaultValue::create();
    large_object.set("metadata", Value::from_str("Large Dataset Test")).unwrap();

    let mut records = Value::new_array();
    for i in 0..5000 {
        let mut record = Value::new_object();
        record.set("id", Value::from_number(i as f64).unwrap()).unwrap();
        record.set("name", Value::from_str(&format!("Record {}", i))).unwrap();
        record.set("description", Value::from_str(&format!("Description for record {} with additional text to make it larger", i))).unwrap();
        record.set("active", Value::from_bool(i % 2 == 0)).unwrap();
        record.set("score", Value::from_number((i * 3) as f64).unwrap()).unwrap();

        let mut details = Value::new_object();
        details.set("category", Value::from_str(match i % 4 {
            0 => "A", 1 => "B", 2 => "C", _ => "D"
        })).unwrap();
        details.set("priority", Value::from_number((i % 10) as f64).unwrap()).unwrap();
        record.set("details", details).unwrap();

        records.push(record).unwrap();
    }

    large_object.set("records", records).unwrap();

    println!("Large dataset created with 5000 records");

    let start = Instant::now();
    let cloned = large_object.deep_clone().unwrap();
    let duration = start.elapsed();

    assert!(duration.as_millis() < 1000);

    assert_eq!(
        large_object.get("metadata").unwrap().unwrap().as_str().unwrap(),
        cloned.get("metadata").unwrap().unwrap().as_str().unwrap()
    );

    let original_records = large_object.get("records").unwrap().unwrap();
    let cloned_records = cloned.get("records").unwrap().unwrap();

    let original_array = original_records.as_array().unwrap().unwrap();
    let cloned_array = cloned_records.as_array().unwrap().unwrap();

    assert_eq!(original_array.len(), cloned_array.len());
    assert_eq!(original_array.len(), 5000);

    assert_eq!(
        original_array[0].get("name").unwrap().unwrap().as_str().unwrap(),
        cloned_array[0].get("name").unwrap().unwrap().as_str().unwrap()
    );

    assert_eq!(
        original_array[2500].get("description").unwrap().unwrap().as_str().unwrap(),
        cloned_array[2500].get("description").unwrap().unwrap().as_str().unwrap()
    );

    assert_eq!(
        original_array[4999].get("details").unwrap().unwrap().get("category").unwrap().unwrap().as_str().unwrap(),
        cloned_array[4999].get("details").unwrap().unwrap().get("category").unwrap().unwrap().as_str().unwrap()
    );

    println!("✅ Large dataset deep clone test passed in {:?}", duration);
    println!("   Throughput: {:.2} records/ms", 5000.0 / duration.as_millis() as f64);
}

#[test]
fn test_deep_clone_performance_threshold() {
    let mut small_object = DefaultValue::create();
    small_object.set("field1", Value::from_str("value1")).unwrap();
    small_object.set("field2", Value::from_number(123.0).unwrap()).unwrap();

    let start = Instant::now();
    let _small_clone = small_object.deep_clone().unwrap();
    let small_duration = start.elapsed();

    let mut medium_object = DefaultValue::create();
    let mut medium_array = Value::new_array();

    for i in 0..2000 {
        let mut item = Value::new_object();
        item.set("id", Value::from_number(i as f64).unwrap()).unwrap();
        item.set("large_field", Value::from_str(&"x".repeat(1000))).unwrap();
        medium_array.push(item).unwrap();
    }
    medium_object.set("data", medium_array).unwrap();

    let start = Instant::now();
    let _medium_clone = medium_object.deep_clone().unwrap();
    let medium_duration = start.elapsed();

    println!("Small structure clone time: {:?}", small_duration);
    println!("Medium structure clone time: {:?}", medium_duration);

    assert!(small_duration < medium_duration);
    assert!(small_duration.as_millis() < 10);
    assert!(medium_duration.as_millis() < 1000);

    println!("✅ Performance threshold test passed");
}

#[test]
fn test_deep_clone_with_model_manager() {
    let mut manager = DefaultModelManager::create();

    let unique_id = format!("tech_corp_{}", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos());

    let mut company = DefaultValue::create();
    company.set("name", Value::from_str("TechCorp")).unwrap();
    company.set("id", Value::from_str(&unique_id)).unwrap();

    let mut employees = Value::new_array();
    for i in 1..=20 {
        let mut employee = Value::new_object();
        employee.set("id", Value::from_number(i as f64).unwrap()).unwrap();
        employee.set("name", Value::from_str(&format!("Employee {}", i))).unwrap();
        employee.set("department", Value::from_str(match i % 3 {
            0 => "Engineering",
            1 => "Sales",
            _ => "Marketing"
        })).unwrap();

        let mut profile = Value::new_object();
        profile.set("level", Value::from_number((i % 10) as f64).unwrap()).unwrap();
        profile.set("remote", Value::from_bool(i % 2 == 0)).unwrap();
        employee.set("profile", profile).unwrap();

        employees.push(employee).unwrap();
    }
    company.set("employees", employees).unwrap();

    let _inserted = manager.insert(
        "companies".to_string(),
        Some(unique_id.clone()),
        company
    ).unwrap();

    println!("Inserted company with ID: {}", unique_id);

    let retrieved = manager.get(
        "companies".to_string(),
        unique_id.clone()
    ).unwrap();

    let start = Instant::now();
    let cloned_company = retrieved.deep_clone().unwrap();
    let duration = start.elapsed();

    assert_eq!(
        retrieved.get("name").unwrap().unwrap().as_str().unwrap(),
        cloned_company.get("name").unwrap().unwrap().as_str().unwrap()
    );

    assert_eq!(
        retrieved.get("id").unwrap().unwrap().as_str().unwrap(),
        cloned_company.get("id").unwrap().unwrap().as_str().unwrap()
    );

    let retrieved_employees = retrieved.get("employees").unwrap().unwrap();
    let cloned_employees = cloned_company.get("employees").unwrap().unwrap();

    let retrieved_array = retrieved_employees.as_array().unwrap().unwrap();
    let cloned_array = cloned_employees.as_array().unwrap().unwrap();

    assert_eq!(retrieved_array.len(), cloned_array.len());
    assert_eq!(retrieved_array.len(), 20);

    assert_eq!(
        retrieved_array[10].get("name").unwrap().unwrap().as_str().unwrap(),
        cloned_array[10].get("name").unwrap().unwrap().as_str().unwrap()
    );

    assert_eq!(
        retrieved_array[19].get("profile").unwrap().unwrap().get("level").unwrap().unwrap().as_number().unwrap(),
        cloned_array[19].get("profile").unwrap().unwrap().get("level").unwrap().unwrap().as_number().unwrap()
    );

    println!("✅ Deep clone with model manager test passed in {:?}", duration);
}

#[test]
fn test_deep_clone_edge_cases() {
    let empty_object = DefaultValue::create();
    let cloned_empty = empty_object.deep_clone().unwrap();
    assert!(cloned_empty.is_object());
    assert!(cloned_empty.is_empty());

    let empty_array = Value::new_array();
    let cloned_empty_array = empty_array.deep_clone().unwrap();
    assert!(cloned_empty_array.is_array());
    assert!(cloned_empty_array.is_empty());

    let mut deeply_nested = DefaultValue::create();
    let mut current = Value::new_object();
    current.set("value", Value::from_str("deep")).unwrap();

    for i in (1..=20).rev() {
        let mut parent = Value::new_object();
        parent.set(&format!("level{}", i), current).unwrap();
        current = parent;
    }
    deeply_nested.set("root", current).unwrap();

    let cloned_deep = deeply_nested.deep_clone().unwrap();

    let deep_path = (1..=20).map(|i| format!("level{}", i)).collect::<Vec<_>>().join(".");
    let full_path = format!("root.{}.value", deep_path);

    assert_eq!(
        deeply_nested.get_by_path(&full_path).unwrap().unwrap().as_str().unwrap(),
        cloned_deep.get_by_path(&full_path).unwrap().unwrap().as_str().unwrap()
    );

    let mut mixed_array = Value::new_array();
    mixed_array.push(Value::from_str("string")).unwrap();
    mixed_array.push(Value::from_number(42.0).unwrap()).unwrap();
    mixed_array.push(Value::from_bool(true)).unwrap();

    let mut nested_obj = Value::new_object();
    nested_obj.set("nested", Value::from_str("value")).unwrap();
    mixed_array.push(nested_obj).unwrap();

    let cloned_mixed = mixed_array.deep_clone().unwrap();
    let mixed_clone_array = cloned_mixed.as_array().unwrap().unwrap();

    assert_eq!(mixed_clone_array.len(), 4);
    assert_eq!(mixed_clone_array[0].as_str().unwrap(), "string");
    assert_eq!(mixed_clone_array[1].as_number().unwrap(), 42.0);
    assert_eq!(mixed_clone_array[2].as_bool().unwrap(), true);
    assert_eq!(
        mixed_clone_array[3].get("nested").unwrap().unwrap().as_str().unwrap(),
        "value"
    );

    println!("✅ Deep clone edge cases test passed");
}

#[test]
fn test_deep_clone_memory_efficiency() {
    let mut memory_test = DefaultValue::create();
    memory_test.set("metadata", Value::from_str("Memory efficiency test")).unwrap();

    for array_idx in 0..5 {
        let mut large_array = Value::new_array();

        for i in 0..1000 {
            let mut item = Value::new_object();
            item.set("array_id", Value::from_number(array_idx as f64).unwrap()).unwrap();
            item.set("item_id", Value::from_number(i as f64).unwrap()).unwrap();
            item.set("data", Value::from_str(&format!("Data for array {} item {}", array_idx, i))).unwrap();

            let mut sub_data = Value::new_object();
            sub_data.set("timestamp", Value::from_str("2025-01-15T10:00:00Z")).unwrap();
            sub_data.set("processed", Value::from_bool(i % 2 == 0)).unwrap();
            item.set("metadata", sub_data).unwrap();

            large_array.push(item).unwrap();
        }

        memory_test.set(&format!("array_{}", array_idx), large_array).unwrap();
    }

    println!("Memory test structure created: 5 arrays × 1000 items = 5000 total items");

    let start = Instant::now();
    let cloned = memory_test.deep_clone().unwrap();
    let duration = start.elapsed();

    println!("Memory efficiency test completed in: {:?}", duration);
    assert!(duration.as_millis() < 5000);

    for array_idx in 0..5 {
        let original_array = memory_test.get(&format!("array_{}", array_idx)).unwrap().unwrap();
        let cloned_array = cloned.get(&format!("array_{}", array_idx)).unwrap().unwrap();

        let orig_arr = original_array.as_array().unwrap().unwrap();
        let cloned_arr = cloned_array.as_array().unwrap().unwrap();

        assert_eq!(orig_arr.len(), cloned_arr.len());
        assert_eq!(orig_arr.len(), 1000);

        let test_indices = [0, 250, 500, 750, 999];
        for &idx in &test_indices {
            assert_eq!(
                orig_arr[idx].get("item_id").unwrap().unwrap().as_number().unwrap(),
                cloned_arr[idx].get("item_id").unwrap().unwrap().as_number().unwrap()
            );
        }
    }

    println!("✅ Memory efficiency test passed - 5000 items cloned successfully");
}

#[test]
fn test_deep_clone_concurrent_safety() {
    let mut shared_data = DefaultValue::create();
    shared_data.set("shared_field", Value::from_str("shared value")).unwrap();

    let mut array_data = Value::new_array();
    for i in 0..100 {
        let mut item = Value::new_object();
        item.set("id", Value::from_number(i as f64).unwrap()).unwrap();
        item.set("value", Value::from_str(&format!("value {}", i))).unwrap();
        array_data.push(item).unwrap();
    }
    shared_data.set("array", array_data).unwrap();

    // Simulate multiple clones (sequential in sync version)
    let mut results = Vec::new();
    for task_id in 0..10 {
        let start = Instant::now();
        let cloned = shared_data.deep_clone().unwrap();
        let duration = start.elapsed();

        let shared_value = cloned.get("shared_field").unwrap().unwrap().as_str().unwrap();
        let array = cloned.get("array").unwrap().unwrap();
        let array_items = array.as_array().unwrap().unwrap();

        results.push((task_id, duration, shared_value == "shared value", array_items.len() == 100));
    }

    for (task_id, duration, shared_correct, array_correct) in results {
        assert!(shared_correct, "Task {} failed shared field verification", task_id);
        assert!(array_correct, "Task {} failed array verification", task_id);
        assert!(duration.as_millis() < 100, "Task {} took too long: {:?}", task_id, duration);
        println!("Task {} completed in {:?}", task_id, duration);
    }

    println!("✅ Concurrent safety test passed - 10 sequential clones successful");
}