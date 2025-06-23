use model_manager::{
    ArrayIterator, CoreValue, DefaultIteratorFactory, DefaultModelManager, DefaultValueFactory,
    IteratorFactory, ModelManager, ModelManagerFactory, ObjectIterator, ValueFactory,
};

#[test]
fn test_debug_simple_array_creation() {
    let empty_array = DefaultValueFactory::create_array();
    assert!(empty_array.is_array());
    assert!(empty_array.is_empty());
    let iterator_result = DefaultIteratorFactory::create_array_iterator(empty_array);
    match iterator_result {
        Ok(mut iterator) => {
            let next_result = iterator.next();
            assert!(next_result.is_none());
        }
        Err(e) => {
            panic!("Iterator creation failed: {}", e);
        }
    }
}

#[test]
fn test_debug_simple_object_creation() {
    let empty_object = DefaultValueFactory::create_object();

    assert!(empty_object.is_object());
    assert!(empty_object.is_empty());

    let iterator_result = DefaultIteratorFactory::create_object_iterator(empty_object);

    match iterator_result {
        Ok(mut iterator) => {
            let next_result = iterator.next();
            assert!(next_result.is_none());
        }
        Err(e) => {
            panic!("Iterator creation failed: {}", e);
        }
    }
}

#[test]
fn test_array_iterator_basic_next() {
    let non_array = DefaultValueFactory::create_string("not an array");

    let array_result = DefaultIteratorFactory::create_array_iterator(non_array);

    assert!(array_result.is_err());
    let error_msg = array_result.unwrap_err();
    assert!(
        error_msg.to_lowercase().contains("array") || error_msg.to_lowercase().contains("invalid")
    );

    let non_object = DefaultValueFactory::create_number(123.0).unwrap();

    let object_result = DefaultIteratorFactory::create_object_iterator(non_object);

    assert!(object_result.is_err());
    let error_msg = object_result.unwrap_err();
    assert!(
        error_msg.to_lowercase().contains("object") || error_msg.to_lowercase().contains("invalid")
    );
}

#[test]
fn test_array_iterator_size_hint() {
    let mut array = DefaultValueFactory::create_array();
    for i in 0..5 {
        array
            .push(DefaultValueFactory::create_number(i as f64).unwrap())
            .unwrap();
    }

    let mut iterator = DefaultIteratorFactory::create_array_iterator(array).unwrap();

    let initial_hint = iterator.size_hint();

    iterator.next();
    iterator.next();

    let after_consumption_hint = iterator.size_hint();

    assert_eq!(initial_hint, (5, Some(5)));
    assert_eq!(after_consumption_hint, (3, Some(3)));
}

#[test]
fn test_array_iterator_nth() {
    let mut array = DefaultValueFactory::create_array();
    for i in 0..10 {
        array
            .push(DefaultValueFactory::create_number(i as f64).unwrap())
            .unwrap();
    }

    let mut iterator = DefaultIteratorFactory::create_array_iterator(array).unwrap();

    let element_0 = iterator.nth(0);
    let element_2 = iterator.nth(2);
    let element_beyond = iterator.nth(100);

    assert!(element_0.is_some());
    assert_eq!(element_0.unwrap().as_number().unwrap(), 0.0);

    assert!(element_2.is_some());
    assert_eq!(element_2.unwrap().as_number().unwrap(), 3.0);

    assert!(element_beyond.is_none());
}

#[test]
fn test_array_iterator_count() {
    let mut array = DefaultValueFactory::create_array();
    for i in 0..7 {
        array
            .push(DefaultValueFactory::create_string(&format!("item_{}", i)))
            .unwrap();
    }

    let mut iterator = DefaultIteratorFactory::create_array_iterator(array).unwrap();
    let count = iterator.count();

    assert_eq!(count, 7);
}

#[test]
fn test_array_iterator_collect() {
    let mut array = DefaultValueFactory::create_array();
    array
        .push(DefaultValueFactory::create_string("text"))
        .unwrap();
    array
        .push(DefaultValueFactory::create_number(42.0).unwrap())
        .unwrap();
    array.push(DefaultValueFactory::create_bool(true)).unwrap();

    let mut iterator = DefaultIteratorFactory::create_array_iterator(array).unwrap();
    let collected = iterator.collect();

    assert_eq!(collected.len(), 3);
    assert_eq!(collected[0].as_str().unwrap(), "text");
    assert_eq!(collected[1].as_number().unwrap(), 42.0);
    assert_eq!(collected[2].as_bool().unwrap(), true);
}

#[test]
fn test_array_iterator_find() {
    let mut array = DefaultValueFactory::create_array();
    for i in 1..=10 {
        array
            .push(DefaultValueFactory::create_number(i as f64).unwrap())
            .unwrap();
    }

    let mut iterator = DefaultIteratorFactory::create_array_iterator(array).unwrap();

    let found = iterator.find(|item| item.as_number().unwrap_or(0.0) == 7.0);

    assert!(found.is_some());
    assert_eq!(found.unwrap().as_number().unwrap(), 7.0);

    let mut iterator2 = DefaultIteratorFactory::create_array_iterator({
        let mut arr = DefaultValueFactory::create_array();
        for i in 1..=5 {
            arr.push(DefaultValueFactory::create_number(i as f64).unwrap())
                .unwrap();
        }
        arr
    })
    .unwrap();

    let not_found = iterator2.find(|item| item.as_number().unwrap_or(0.0) == 100.0);

    assert!(not_found.is_none());
}

#[test]
fn test_array_iterator_filter() {
    let mut array = DefaultValueFactory::create_array();
    for i in 1..=10 {
        array
            .push(DefaultValueFactory::create_number(i as f64).unwrap())
            .unwrap();
    }

    let mut iterator = DefaultIteratorFactory::create_array_iterator(array).unwrap();

    let filtered = iterator.filter(|item| {
        let num = item.as_number().unwrap_or(0.0);
        num % 2.0 == 0.0
    });

    assert_eq!(filtered.len(), 5);

    for item in &filtered {
        let num = item.as_number().unwrap();
        assert_eq!(num % 2.0, 0.0);
    }

    assert_eq!(filtered[0].as_number().unwrap(), 2.0);
    assert_eq!(filtered[1].as_number().unwrap(), 4.0);
    assert_eq!(filtered[4].as_number().unwrap(), 10.0);

    println!("✅ Array iterator filter test passed");
}

#[test]
fn test_array_iterator_map() {
    let mut array = DefaultValueFactory::create_array();
    array
        .push(DefaultValueFactory::create_string("hello"))
        .unwrap();
    array
        .push(DefaultValueFactory::create_string("world"))
        .unwrap();
    array
        .push(DefaultValueFactory::create_string("rust"))
        .unwrap();

    let mut iterator = DefaultIteratorFactory::create_array_iterator(array).unwrap();

    let mapped: Vec<usize> = iterator.map(|item| item.as_str().unwrap_or_default().len());

    assert_eq!(mapped.len(), 3);
    assert_eq!(mapped[0], 5);
    assert_eq!(mapped[1], 5);
    assert_eq!(mapped[2], 4);
}

#[test]
fn test_array_iterator_for_each_batch() {
    let mut array = DefaultValueFactory::create_array();
    for i in 1..=10 {
        array
            .push(DefaultValueFactory::create_number(i as f64).unwrap())
            .unwrap();
    }

    let mut iterator = DefaultIteratorFactory::create_array_iterator(array).unwrap();

    use std::sync::{Arc, Mutex};

    let batch_sizes = Arc::new(Mutex::new(Vec::new()));
    let total_sum = Arc::new(Mutex::new(0.0));

    let batch_sizes_clone = batch_sizes.clone();
    let total_sum_clone = total_sum.clone();

    let total_processed = iterator.for_each_batch(3, move |batch| {
        let batch_size = batch.len();
        batch_sizes_clone.lock().unwrap().push(batch_size);

        let batch_sum: f64 = batch
            .iter()
            .map(|item| item.as_number().unwrap_or(0.0))
            .sum();
        *total_sum_clone.lock().unwrap() += batch_sum;
    });

    assert_eq!(total_processed, 10);

    let final_batch_sizes = batch_sizes.lock().unwrap();
    let final_total_sum = *total_sum.lock().unwrap();

    assert_eq!(final_batch_sizes.len(), 4);
    assert_eq!(final_batch_sizes[0], 3);
    assert_eq!(final_batch_sizes[1], 3);
    assert_eq!(final_batch_sizes[2], 3);
    assert_eq!(final_batch_sizes[3], 1);

    assert_eq!(final_total_sum, 55.0);

    println!("✅ Array iterator for_each_batch test passed");
}

#[test]
fn test_array_iterator_empty_array() {
    let empty_array = DefaultValueFactory::create_array();

    let mut iterator = DefaultIteratorFactory::create_array_iterator(empty_array).unwrap();

    assert_eq!(iterator.size_hint(), (0, Some(0)));
    assert!(iterator.next().is_none());
    assert_eq!(iterator.count(), 0);
    assert_eq!(iterator.collect().len(), 0);

    let mut iterator2 =
        DefaultIteratorFactory::create_array_iterator(DefaultValueFactory::create_array()).unwrap();

    let not_found = iterator2.find(|_| true);
    assert!(not_found.is_none());

    println!("✅ Array iterator empty array test passed");
}

#[test]
fn test_array_iterator_large_dataset() {
    let mut large_array = DefaultValueFactory::create_array();
    for i in 0..1000 {
        large_array
            .push(DefaultValueFactory::create_number(i as f64).unwrap())
            .unwrap();
    }

    let mut iterator = DefaultIteratorFactory::create_array_iterator(large_array).unwrap();

    let start = std::time::Instant::now();
    let count = iterator.count();
    let duration = start.elapsed();

    assert_eq!(count, 1000);
    assert!(duration.as_millis() < 100);

    let mut iterator2 = DefaultIteratorFactory::create_array_iterator({
        let mut arr = DefaultValueFactory::create_array();
        for i in 0..1000 {
            arr.push(DefaultValueFactory::create_number(i as f64).unwrap())
                .unwrap();
        }
        arr
    })
    .unwrap();

    let filtered = iterator2.filter(|item| {
        let num = item.as_number().unwrap_or(0.0);
        num >= 990.0
    });

    assert_eq!(filtered.len(), 10);

    println!(
        "✅ Array iterator large dataset test passed in {:?}",
        duration
    );
}

#[test]
fn test_object_iterator_basic_next() {
    let mut object = DefaultValueFactory::create_object();
    object
        .set("name", DefaultValueFactory::create_string("John"))
        .unwrap();
    object
        .set("age", DefaultValueFactory::create_number(30.0).unwrap())
        .unwrap();
    object
        .set("active", DefaultValueFactory::create_bool(true))
        .unwrap();

    let mut iterator = DefaultIteratorFactory::create_object_iterator(object).unwrap();

    let mut pairs = Vec::new();
    while let Some((key, value)) = iterator.next() {
        pairs.push((key, value));
    }

    assert_eq!(pairs.len(), 3);

    let keys: Vec<&String> = pairs.iter().map(|(k, _)| k).collect();
    assert!(keys.contains(&&"name".to_string()));
    assert!(keys.contains(&&"age".to_string()));
    assert!(keys.contains(&&"active".to_string()));

    for (key, value) in &pairs {
        match key.as_str() {
            "name" => assert_eq!(value.as_str().unwrap(), "John"),
            "age" => assert_eq!(value.as_number().unwrap(), 30.0),
            "active" => assert_eq!(value.as_bool().unwrap(), true),
            _ => panic!("Unexpected key: {}", key),
        }
    }
}

#[test]
fn test_object_iterator_size_hint() {
    let mut object = DefaultValueFactory::create_object();
    object
        .set("prop1", DefaultValueFactory::create_string("value1"))
        .unwrap();
    object
        .set("prop2", DefaultValueFactory::create_string("value2"))
        .unwrap();
    object
        .set("prop3", DefaultValueFactory::create_string("value3"))
        .unwrap();
    object
        .set("prop4", DefaultValueFactory::create_string("value4"))
        .unwrap();

    let mut iterator = DefaultIteratorFactory::create_object_iterator(object).unwrap();

    let initial_hint = iterator.size_hint();

    iterator.next();
    iterator.next();

    let after_consumption_hint = iterator.size_hint();

    assert_eq!(initial_hint, (4, Some(4)));
    assert_eq!(after_consumption_hint, (2, Some(2)));

    println!("✅ Object iterator size hint test passed");
}

#[test]
fn test_object_iterator_find_key() {
    let mut object = DefaultValueFactory::create_object();
    object
        .set("user_name", DefaultValueFactory::create_string("Alice"))
        .unwrap();
    object
        .set(
            "user_email",
            DefaultValueFactory::create_string("alice@example.com"),
        )
        .unwrap();
    object
        .set(
            "user_score",
            DefaultValueFactory::create_number(95.5).unwrap(),
        )
        .unwrap();

    let mut iterator = DefaultIteratorFactory::create_object_iterator(object).unwrap();

    let found_name = iterator.find_key("user_name");
    let found_score = iterator.find_key("user_score");
    let not_found = iterator.find_key("non_existent");

    assert!(found_name.is_some());
    assert_eq!(found_name.unwrap().as_str().unwrap(), "Alice");

    assert!(found_score.is_some());
    assert_eq!(found_score.unwrap().as_number().unwrap(), 95.5);

    assert!(not_found.is_none());

    println!("✅ Object iterator find_key test passed");
}

#[test]
fn test_object_iterator_count() {
    let mut object = DefaultValueFactory::create_object();
    for i in 0..6 {
        object
            .set(
                &format!("field_{}", i),
                DefaultValueFactory::create_number(i as f64).unwrap(),
            )
            .unwrap();
    }

    let mut iterator = DefaultIteratorFactory::create_object_iterator(object).unwrap();
    let count = iterator.count();

    assert_eq!(count, 6);

    println!("✅ Object iterator count test passed");
}

#[test]
fn test_object_iterator_collect() {
    let mut object = DefaultValueFactory::create_object();
    object
        .set("text_field", DefaultValueFactory::create_string("hello"))
        .unwrap();
    object
        .set(
            "number_field",
            DefaultValueFactory::create_number(123.45).unwrap(),
        )
        .unwrap();
    object
        .set("bool_field", DefaultValueFactory::create_bool(false))
        .unwrap();

    let mut iterator = DefaultIteratorFactory::create_object_iterator(object).unwrap();
    let collected = iterator.collect();

    assert_eq!(collected.len(), 3);

    let collected_map: std::collections::HashMap<String, _> = collected.into_iter().collect();

    assert!(collected_map.contains_key("text_field"));
    assert_eq!(collected_map["text_field"].as_str().unwrap(), "hello");

    assert!(collected_map.contains_key("number_field"));
    assert_eq!(collected_map["number_field"].as_number().unwrap(), 123.45);

    assert!(collected_map.contains_key("bool_field"));
    assert_eq!(collected_map["bool_field"].as_bool().unwrap(), false);

    println!("✅ Object iterator collect test passed");
}

#[test]
fn test_object_iterator_filter() {
    let mut object = DefaultValueFactory::create_object();
    object
        .set("score1", DefaultValueFactory::create_number(85.0).unwrap())
        .unwrap();
    object
        .set("name", DefaultValueFactory::create_string("Test"))
        .unwrap();
    object
        .set("score2", DefaultValueFactory::create_number(92.0).unwrap())
        .unwrap();
    object
        .set("active", DefaultValueFactory::create_bool(true))
        .unwrap();
    object
        .set("score3", DefaultValueFactory::create_number(78.0).unwrap())
        .unwrap();

    let mut iterator = DefaultIteratorFactory::create_object_iterator(object).unwrap();

    let filtered = iterator.filter(|key, _value| key.starts_with("score"));

    assert_eq!(filtered.len(), 3);

    for (key, value) in &filtered {
        assert!(key.starts_with("score"));
        assert!(value.as_number().is_some());
    }

    println!("✅ Object iterator filter test passed");
}

#[test]
fn test_object_iterator_map() {
    let mut object = DefaultValueFactory::create_object();
    object
        .set("first_name", DefaultValueFactory::create_string("John"))
        .unwrap();
    object
        .set("last_name", DefaultValueFactory::create_string("Doe"))
        .unwrap();
    object
        .set("city", DefaultValueFactory::create_string("NYC"))
        .unwrap();

    let mut iterator = DefaultIteratorFactory::create_object_iterator(object).unwrap();

    let mapped: Vec<(String, usize)> =
        iterator.map(|_key, value| value.as_str().unwrap_or_default().len());

    assert_eq!(mapped.len(), 3);

    let mapped_map: std::collections::HashMap<String, usize> = mapped.into_iter().collect();

    assert_eq!(mapped_map["first_name"], 4);
    assert_eq!(mapped_map["last_name"], 3);
    assert_eq!(mapped_map["city"], 3);

    println!("✅ Object iterator map test passed");
}

#[test]
fn test_object_iterator_empty_object() {
    let empty_object = DefaultValueFactory::create_object();

    let mut iterator = DefaultIteratorFactory::create_object_iterator(empty_object).unwrap();

    assert_eq!(iterator.size_hint(), (0, Some(0)));
    assert!(iterator.next().is_none());
    assert_eq!(iterator.count(), 0);
    assert_eq!(iterator.collect().len(), 0);
    assert!(iterator.find_key("any_key").is_none());

    println!("✅ Object iterator empty object test passed");
}

#[test]
fn test_object_iterator_nested_objects() {
    let mut nested_object = DefaultValueFactory::create_object();
    nested_object
        .set("inner_value", DefaultValueFactory::create_string("nested"))
        .unwrap();

    let mut main_object = DefaultValueFactory::create_object();
    main_object
        .set("simple", DefaultValueFactory::create_string("value"))
        .unwrap();
    main_object.set("nested", nested_object).unwrap();
    main_object
        .set("number", DefaultValueFactory::create_number(42.0).unwrap())
        .unwrap();

    let mut iterator = DefaultIteratorFactory::create_object_iterator(main_object).unwrap();
    let collected = iterator.collect();

    assert_eq!(collected.len(), 3);

    let collected_map: std::collections::HashMap<String, _> = collected.into_iter().collect();

    assert!(collected_map.contains_key("simple"));
    assert!(collected_map.contains_key("nested"));
    assert!(collected_map.contains_key("number"));

    let nested_value = &collected_map["nested"];
    assert!(nested_value.is_object());

    println!("✅ Object iterator nested objects test passed");
}

#[test]
fn test_object_iterator_large_object() {
    let mut large_object = DefaultValueFactory::create_object();
    for i in 0..100 {
        large_object
            .set(
                &format!("property_{:03}", i),
                DefaultValueFactory::create_number(i as f64).unwrap(),
            )
            .unwrap();
    }

    let mut iterator = DefaultIteratorFactory::create_object_iterator(large_object).unwrap();

    let start = std::time::Instant::now();
    let count = iterator.count();
    let duration = start.elapsed();

    assert_eq!(count, 100);
    assert!(duration.as_millis() < 100);

    let mut iterator2 = DefaultIteratorFactory::create_object_iterator({
        let mut obj = DefaultValueFactory::create_object();
        for i in 0..100 {
            obj.set(
                &format!("property_{:03}", i),
                DefaultValueFactory::create_number(i as f64).unwrap(),
            )
            .unwrap();
        }
        obj
    })
    .unwrap();

    let filtered = iterator2.filter(|key, _value| key.contains("09"));

    assert_eq!(filtered.len(), 11);

    println!(
        "✅ Object iterator large object test passed in {:?}",
        duration
    );
}

#[test]
fn test_iterator_complex_predicate_operations() {
    let mut array = DefaultValueFactory::create_array();
    for i in 1..=20 {
        let mut item = DefaultValueFactory::create_object();
        item.set("id", DefaultValueFactory::create_number(i as f64).unwrap())
            .unwrap();
        item.set(
            "name",
            DefaultValueFactory::create_string(&format!("Item {}", i)),
        )
        .unwrap();
        item.set("active", DefaultValueFactory::create_bool(i % 3 == 0))
            .unwrap();
        item.set(
            "score",
            DefaultValueFactory::create_number((i * 5) as f64).unwrap(),
        )
        .unwrap();
        array.push(item).unwrap();
    }

    let mut iterator = DefaultIteratorFactory::create_array_iterator(array.clone()).unwrap();

    let high_score_active = iterator.find(|item| {
        let score = item
            .get("score")
            .unwrap()
            .unwrap()
            .as_number()
            .unwrap_or(0.0);
        let active = item
            .get("active")
            .unwrap()
            .unwrap()
            .as_bool()
            .unwrap_or(false);
        score > 50.0 && active
    });

    assert!(high_score_active.is_some());
    let found_item = high_score_active.unwrap();
    let found_id = found_item.get("id").unwrap().unwrap().as_number().unwrap();
    assert_eq!(found_id, 12.0);

    let mut iterator2 = DefaultIteratorFactory::create_array_iterator(array).unwrap();

    let filtered_complex = iterator2.filter(|item| {
        let id = item.get("id").unwrap().unwrap().as_number().unwrap_or(0.0);
        let score = item
            .get("score")
            .unwrap()
            .unwrap()
            .as_number()
            .unwrap_or(0.0);
        id > 10.0 && score % 15.0 == 0.0
    });

    assert_eq!(filtered_complex.len(), 3);
}

#[test]
fn test_iterator_chaining_operations() {
    let mut object = DefaultValueFactory::create_object();
    for i in 1..=10 {
        object
            .set(
                &format!("item_{}", i),
                DefaultValueFactory::create_number(i as f64).unwrap(),
            )
            .unwrap();
    }

    let mut iterator1 = DefaultIteratorFactory::create_object_iterator(object).unwrap();

    let filtered = iterator1.filter(|_key, value| {
        let num = value.as_number().unwrap_or(0.0);
        num % 2.0 == 0.0
    });

    let mut array_from_filtered = DefaultValueFactory::create_array();
    for (_key, value) in filtered {
        array_from_filtered.push(value).unwrap();
    }

    let mut iterator2 = DefaultIteratorFactory::create_array_iterator(array_from_filtered).unwrap();

    let mapped = iterator2.map(|item| {
        let num = item.as_number().unwrap_or(0.0);
        format!("Number: {}", num)
    });

    assert_eq!(mapped.len(), 5);

    for mapped_value in &mapped {
        assert!(mapped_value.starts_with("Number: "));
    }

    println!("✅ Iterator chaining operations test passed");
}

#[test]
fn test_iterator_memory_efficiency() {
    let mut large_array = DefaultValueFactory::create_array();
    for i in 0..1000 {
        let mut complex_item = DefaultValueFactory::create_object();
        complex_item
            .set("id", DefaultValueFactory::create_number(i as f64).unwrap())
            .unwrap();
        complex_item
            .set("data", DefaultValueFactory::create_string(&"x".repeat(100)))
            .unwrap();
        large_array.push(complex_item).unwrap();
    }

    let mut iterator = DefaultIteratorFactory::create_array_iterator(large_array).unwrap();

    use std::sync::{Arc, Mutex};

    let batch_count = Arc::new(Mutex::new(0));
    let total_processed = Arc::new(Mutex::new(0));

    let batch_count_clone = batch_count.clone();
    let total_processed_clone = total_processed.clone();

    let processed_count = iterator.for_each_batch(50, move |batch| {
        *batch_count_clone.lock().unwrap() += 1;
        let batch_size = batch.len();

        for item in batch {
            let _id = item.get("id").unwrap().unwrap().as_number().unwrap();
        }

        *total_processed_clone.lock().unwrap() += batch_size;
    });

    assert_eq!(processed_count, 1000);
    assert_eq!(*total_processed.lock().unwrap(), 1000);
    assert_eq!(*batch_count.lock().unwrap(), 20);

    println!("✅ Iterator memory efficiency test passed");
    println!(
        "   Processed {} items in {} batches",
        processed_count,
        *batch_count.lock().unwrap()
    );
}

#[test]
fn test_iterator_performance_stress() {
    let mut large_array = DefaultValueFactory::create_array();
    for i in 0..5000 {
        large_array
            .push(DefaultValueFactory::create_number(i as f64).unwrap())
            .unwrap();
    }

    let start = std::time::Instant::now();

    let mut iterator = DefaultIteratorFactory::create_array_iterator(large_array).unwrap();

    let filtered = iterator.filter(|item| {
        let num = item.as_number().unwrap_or(0.0) as i32;
        if num < 2 {
            return false;
        }
        if num == 2 {
            return true;
        }
        if num % 2 == 0 {
            return false;
        }

        for i in (3..).step_by(2) {
            if i * i > num {
                break;
            }
            if num % i == 0 {
                return false;
            }
        }
        true
    });

    let duration = start.elapsed();

    assert!(filtered.len() > 0);
    assert!(duration.as_millis() < 1000);

    println!(
        "✅ Iterator performance stress test passed in {:?}",
        duration
    );
    println!(
        "   Processed 5000 elements, found {} primes",
        filtered.len()
    );
}

#[test]
fn test_iterator_concurrent_safety() {
    let mut array1 = DefaultValueFactory::create_array();
    let mut array2 = DefaultValueFactory::create_array();

    for i in 0..100 {
        array1
            .push(DefaultValueFactory::create_number(i as f64).unwrap())
            .unwrap();
        array2
            .push(DefaultValueFactory::create_number((i * 2) as f64).unwrap())
            .unwrap();
    }

    // Sequential processing (simulating concurrent safety)
    let mut iter1 = DefaultIteratorFactory::create_array_iterator(array1).unwrap();
    let result1 = iter1.filter(|item| item.as_number().unwrap_or(0.0) % 10.0 == 0.0);

    let mut iter2 = DefaultIteratorFactory::create_array_iterator(array2).unwrap();
    let result2 = iter2.map(|item| item.as_number().unwrap_or(0.0) / 2.0);

    assert_eq!(result1.len(), 10);
    assert_eq!(result2.len(), 100);

    assert_eq!(result1[0].as_number().unwrap(), 0.0);
    assert_eq!(result1[1].as_number().unwrap(), 10.0);
    assert_eq!(result2[0], 0.0);
    assert_eq!(result2[1], 1.0);

    println!("✅ Iterator concurrent safety test passed");
}

#[test]
fn test_iterator_edge_cases_and_boundaries() {
    // Test Case 1: Array con un solo elemento
    let mut single_item_array = DefaultValueFactory::create_array();
    single_item_array
        .push(DefaultValueFactory::create_string("only_one"))
        .unwrap();

    let mut iterator = DefaultIteratorFactory::create_array_iterator(single_item_array).unwrap();
    assert_eq!(iterator.size_hint(), (1, Some(1)));
    assert!(iterator.next().is_some());
    assert!(iterator.next().is_none());

    // Test Case 2: Object con una sola propiedad
    let mut single_prop_object = DefaultValueFactory::create_object();
    single_prop_object
        .set(
            "only_prop",
            DefaultValueFactory::create_string("only_value"),
        )
        .unwrap();

    let mut obj_iterator =
        DefaultIteratorFactory::create_object_iterator(single_prop_object).unwrap();
    assert_eq!(obj_iterator.size_hint(), (1, Some(1)));
    let pair = obj_iterator.next();
    assert!(pair.is_some());
    let (key, value) = pair.unwrap();
    assert_eq!(key, "only_prop");
    assert_eq!(value.as_str().unwrap(), "only_value");

    // Test Case 3: nth con índice 0
    let mut test_array = DefaultValueFactory::create_array();
    test_array
        .push(DefaultValueFactory::create_string("first"))
        .unwrap();
    test_array
        .push(DefaultValueFactory::create_string("second"))
        .unwrap();

    let mut nth_iterator = DefaultIteratorFactory::create_array_iterator(test_array).unwrap();
    let zeroth_element = nth_iterator.nth(0);
    assert!(zeroth_element.is_some());
    assert_eq!(zeroth_element.unwrap().as_str().unwrap(), "first");

    // Test Case 4: Map con función que retorna diferentes tipos
    let mut mixed_array = DefaultValueFactory::create_array();
    mixed_array
        .push(DefaultValueFactory::create_number(5.0).unwrap())
        .unwrap();
    mixed_array
        .push(DefaultValueFactory::create_number(10.0).unwrap())
        .unwrap();

    let mut map_iterator = DefaultIteratorFactory::create_array_iterator(mixed_array).unwrap();
    let mapped_bools: Vec<bool> = map_iterator.map(|item| item.as_number().unwrap_or(0.0) > 7.0);

    assert_eq!(mapped_bools.len(), 2);
    assert_eq!(mapped_bools[0], false);
    assert_eq!(mapped_bools[1], true);

    println!("✅ Iterator edge cases and boundaries test passed");
}

#[test]
fn test_iterator_error_resilience() {
    let mut array_with_issues = DefaultValueFactory::create_array();
    array_with_issues
        .push(DefaultValueFactory::create_number(10.0).unwrap())
        .unwrap();
    array_with_issues
        .push(DefaultValueFactory::create_string("not_a_number"))
        .unwrap();
    array_with_issues
        .push(DefaultValueFactory::create_number(20.0).unwrap())
        .unwrap();
    array_with_issues
        .push(DefaultValueFactory::create_bool(true))
        .unwrap();
    array_with_issues
        .push(DefaultValueFactory::create_number(30.0).unwrap())
        .unwrap();

    let mut iterator = DefaultIteratorFactory::create_array_iterator(array_with_issues).unwrap();

    let valid_numbers = iterator.filter(|item| item.as_number().is_some());

    assert_eq!(valid_numbers.len(), 3);
    assert_eq!(valid_numbers[0].as_number().unwrap(), 10.0);
    assert_eq!(valid_numbers[1].as_number().unwrap(), 20.0);
    assert_eq!(valid_numbers[2].as_number().unwrap(), 30.0);

    let mut array_for_map = DefaultValueFactory::create_array();
    array_for_map
        .push(DefaultValueFactory::create_string("123"))
        .unwrap();
    array_for_map
        .push(DefaultValueFactory::create_string("not_numeric"))
        .unwrap();
    array_for_map
        .push(DefaultValueFactory::create_string("456"))
        .unwrap();

    let mut map_iterator = DefaultIteratorFactory::create_array_iterator(array_for_map).unwrap();

    let parsed_numbers: Vec<f64> = map_iterator.map(|item| {
        item.as_str()
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0)
    });

    assert_eq!(parsed_numbers.len(), 3);
    assert_eq!(parsed_numbers[0], 123.0);
    assert_eq!(parsed_numbers[1], 0.0);
    assert_eq!(parsed_numbers[2], 456.0);

    println!("✅ Iterator error resilience test passed");
}

#[test]
fn test_iterators_with_model_manager_integration() {
    let mut manager = DefaultModelManager::create();

    for i in 1..=5 {
        let mut user = DefaultValueFactory::create_object();
        user.set("id", DefaultValueFactory::create_number(i as f64).unwrap())
            .unwrap();
        user.set(
            "name",
            DefaultValueFactory::create_string(&format!("User {}", i)),
        )
        .unwrap();
        user.set("active", DefaultValueFactory::create_bool(i % 2 == 0))
            .unwrap();

        let mut tags = DefaultValueFactory::create_array();
        tags.push(DefaultValueFactory::create_string("tag1"))
            .unwrap();
        tags.push(DefaultValueFactory::create_string(&format!("tag_{}", i)))
            .unwrap();
        user.set("tags", tags).unwrap();

        manager.insert("users".to_string(), None, user).unwrap();
    }

    let users = manager.get_all("users".to_string()).unwrap();

    let mut users_array = DefaultValueFactory::create_array();
    for user in users {
        users_array.push(user).unwrap();
    }

    let mut array_iterator = DefaultIteratorFactory::create_array_iterator(users_array).unwrap();

    let active_users = array_iterator.filter(|user| {
        user.get("active")
            .unwrap()
            .unwrap()
            .as_bool()
            .unwrap_or(false)
    });

    assert_eq!(active_users.len(), 2);

    let first_user = &active_users[0];
    let mut object_iterator =
        DefaultIteratorFactory::create_object_iterator(first_user.clone()).unwrap();

    let properties = object_iterator.collect();
    assert!(properties.len() >= 4);

    let prop_keys: Vec<&String> = properties.iter().map(|(k, _)| k).collect();
    assert!(prop_keys.contains(&&"id".to_string()));
    assert!(prop_keys.contains(&&"name".to_string()));
    assert!(prop_keys.contains(&&"active".to_string()));
    assert!(prop_keys.contains(&&"tags".to_string()));

    println!("✅ Iterators with model manager integration test passed");
}

#[test]
fn test_comprehensive_iterator_benchmarks() {
    println!("=== COMPREHENSIVE ITERATOR PERFORMANCE BENCHMARKS ===");

    // Benchmark 1: Array Iterator Operations
    let mut benchmark_array = DefaultValueFactory::create_array();
    for i in 0..1000 {
        benchmark_array
            .push(DefaultValueFactory::create_number(i as f64).unwrap())
            .unwrap();
    }

    let start = std::time::Instant::now();
    let mut iter1 = DefaultIteratorFactory::create_array_iterator(benchmark_array.clone()).unwrap();
    let count_result = iter1.count();
    let count_duration = start.elapsed();

    let start = std::time::Instant::now();
    let mut iter2 = DefaultIteratorFactory::create_array_iterator(benchmark_array.clone()).unwrap();
    let collect_result = iter2.collect();
    let collect_duration = start.elapsed();

    let start = std::time::Instant::now();
    let mut iter3 = DefaultIteratorFactory::create_array_iterator(benchmark_array).unwrap();
    let filter_result = iter3.filter(|item| item.as_number().unwrap_or(0.0) % 10.0 == 0.0);
    let filter_duration = start.elapsed();

    println!("Array Iterator Benchmarks (1000 elements):");
    println!("  Count: {} items in {:?}", count_result, count_duration);
    println!(
        "  Collect: {} items in {:?}",
        collect_result.len(),
        collect_duration
    );
    println!(
        "  Filter: {} items in {:?}",
        filter_result.len(),
        filter_duration
    );

    // Benchmark 2: Object Iterator Operations
    let mut benchmark_object = DefaultValueFactory::create_object();
    for i in 0..500 {
        benchmark_object
            .set(
                &format!("property_{:03}", i),
                DefaultValueFactory::create_string(&format!("value_{}", i)),
            )
            .unwrap();
    }

    let start = std::time::Instant::now();
    let mut obj_iter1 =
        DefaultIteratorFactory::create_object_iterator(benchmark_object.clone()).unwrap();
    let obj_count_result = obj_iter1.count();
    let obj_count_duration = start.elapsed();

    let start = std::time::Instant::now();
    let mut obj_iter2 = DefaultIteratorFactory::create_object_iterator(benchmark_object).unwrap();
    let obj_filter_result = obj_iter2.filter(|key, _value| key.contains("0"));
    let obj_filter_duration = start.elapsed();

    println!("Object Iterator Benchmarks (500 properties):");
    println!(
        "  Count: {} properties in {:?}",
        obj_count_result, obj_count_duration
    );
    println!(
        "  Filter: {} properties in {:?}",
        obj_filter_result.len(),
        obj_filter_duration
    );

    // Performance assertions
    assert!(count_duration.as_millis() < 100);
    assert!(collect_duration.as_millis() < 100);
    assert!(filter_duration.as_millis() < 200);
    assert!(obj_count_duration.as_millis() < 100);
    assert!(obj_filter_duration.as_millis() < 200);
}
