use model_manager::{
    CoreValue, DefaultModelManager, DefaultValueFactory, ModelManager, ModelManagerFactory,
    ValueFactory,
};
use std::time::Instant;

type Value = <DefaultValueFactory as ValueFactory>::Value;

#[test]
fn test_performance_single_operations() {
    let mut manager = DefaultModelManager::create();

    println!("=== PERFORMANCE TEST: Single Operations ===");

    let mut insert_times = Vec::new();
    for i in 0..1000 {
        let mut data = DefaultValueFactory::create_object();
        data.set("id", DefaultValueFactory::create_number(i as f64).unwrap())
            .unwrap();
        data.set(
            "name",
            DefaultValueFactory::create_string(&format!("User {}", i)),
        )
        .unwrap();
        data.set(
            "email",
            DefaultValueFactory::create_string(&format!("user{}@test.com", i)),
        )
        .unwrap();

        let start = Instant::now();
        let result = manager.insert("perf_users".to_string(), None, data);
        let duration = start.elapsed();

        assert!(result.is_ok());
        insert_times.push(duration.as_micros());
    }

    let avg_insert_time = insert_times.iter().sum::<u128>() / insert_times.len() as u128;
    let max_insert_time = *insert_times.iter().max().unwrap();
    let min_insert_time = *insert_times.iter().min().unwrap();

    println!("INSERT Performance (1000 operations):");
    println!("  Average: {}μs", avg_insert_time);
    println!("  Min: {}μs", min_insert_time);
    println!("  Max: {}μs", max_insert_time);

    let mut get_times = Vec::new();
    let all_users = manager.get_all("perf_users".to_string()).unwrap();

    for _ in 0..100 {
        let start = Instant::now();
        let _users = manager.get_all("perf_users".to_string()).unwrap();
        let duration = start.elapsed();
        get_times.push(duration.as_micros());
    }

    let avg_get_time = get_times.iter().sum::<u128>() / get_times.len() as u128;
    println!(
        "GET_ALL Performance (100 operations, {} records):",
        all_users.len()
    );
    println!("  Average: {}μs", avg_get_time);

    assert!(
        avg_insert_time < 5000,
        "Insert too slow: {}μs > 5000μs",
        avg_insert_time
    );
    assert!(
        avg_get_time < 10000,
        "Get all too slow: {}μs > 10000μs",
        avg_get_time
    );
}

#[test]
fn test_performance_bulk_operations() {
    let mut manager = DefaultModelManager::create();

    println!("=== PERFORMANCE TEST: Bulk Operations ===");

    let start = Instant::now();
    for i in 0..10000 {
        let mut data = DefaultValueFactory::create_object();
        data.set("id", DefaultValueFactory::create_number(i as f64).unwrap())
            .unwrap();
        data.set(
            "name",
            DefaultValueFactory::create_string(&format!("BulkUser {}", i)),
        )
        .unwrap();
        data.set(
            "category",
            Value::from_str(match i % 5 {
                0 => "A",
                1 => "B",
                2 => "C",
                3 => "D",
                4 => "E",
                _ => unreachable!(),
            }),
        )
        .unwrap();

        let result = manager.insert("bulk_users".to_string(), None, data);
        assert!(result.is_ok());

        if i % 1000 == 0 && i > 0 {
            let elapsed = start.elapsed();
            let ops_per_sec = (i as f64) / elapsed.as_secs_f64();
            println!("  Progress: {} operations, {:.0} ops/sec", i, ops_per_sec);
        }
    }

    let total_duration = start.elapsed();
    let total_ops_per_sec = 10000.0 / total_duration.as_secs_f64();

    println!("BULK INSERT Performance (10,000 operations):");
    println!("  Total time: {:?}", total_duration);
    println!("  Throughput: {:.0} ops/sec", total_ops_per_sec);

    let all_users = manager.get_all("bulk_users".to_string()).unwrap();
    assert_eq!(all_users.len(), 10000);

    assert!(
        total_ops_per_sec > 1000.0,
        "Bulk insert too slow: {:.0} ops/sec < 1000 ops/sec",
        total_ops_per_sec
    );
    assert!(
        total_duration.as_secs() < 30,
        "Bulk insert took too long: {:?} > 30s",
        total_duration
    );
}

#[test]
fn test_performance_multiple_models() {
    let mut manager = DefaultModelManager::create();

    println!("=== PERFORMANCE TEST: Multiple Models ===");

    let models = vec!["users", "products", "orders", "logs", "configs"];
    let operations_per_model = 2000;

    let start = Instant::now();

    for (model_idx, model_name) in models.iter().enumerate() {
        for i in 0..operations_per_model {
            let mut data = DefaultValueFactory::create_object();
            data.set(
                "model_id",
                DefaultValueFactory::create_number(model_idx as f64).unwrap(),
            )
            .unwrap();
            data.set(
                "record_id",
                DefaultValueFactory::create_number(i as f64).unwrap(),
            )
            .unwrap();
            data.set(
                "name",
                DefaultValueFactory::create_string(&format!("{}_record_{}", model_name, i)),
            )
            .unwrap();
            data.set(
                "timestamp",
                DefaultValueFactory::create_number(
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as f64,
                )
                .unwrap(),
            )
            .unwrap();

            let result = manager.insert(model_name.to_string(), None, data);
            assert!(result.is_ok());
        }

        let elapsed = start.elapsed();
        let total_ops = (model_idx + 1) * operations_per_model;
        let ops_per_sec = (total_ops as f64) / elapsed.as_secs_f64();
        println!(
            "  Model '{}' completed: {} total ops, {:.0} ops/sec",
            model_name, total_ops, ops_per_sec
        );
    }

    let total_duration = start.elapsed();
    let total_operations = models.len() * operations_per_model;
    let total_ops_per_sec = (total_operations as f64) / total_duration.as_secs_f64();

    println!("MULTIPLE MODELS Performance:");
    println!("  Models: {}", models.len());
    println!("  Total operations: {}", total_operations);
    println!("  Total time: {:?}", total_duration);
    println!("  Overall throughput: {:.0} ops/sec", total_ops_per_sec);

    for model_name in &models {
        let records = manager.get_all(model_name.to_string()).unwrap();
        assert_eq!(
            records.len(),
            operations_per_model,
            "Model '{}' should have {} records, got {}",
            model_name,
            operations_per_model,
            records.len()
        );
    }

    assert!(
        total_ops_per_sec > 800.0,
        "Multi-model too slow: {:.0} ops/sec < 800 ops/sec",
        total_ops_per_sec
    );
}

#[test]
fn test_performance_complex_data_structures() -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = DefaultModelManager::create();

    println!("=== PERFORMANCE TEST: Complex Data Structures ===");

    let start = Instant::now();

    for i in 0..1000 {
        let mut complex_data = DefaultValueFactory::create_object();
        complex_data.set("id", DefaultValueFactory::create_number(i as f64).unwrap())?;
        complex_data.set(
            "title",
            DefaultValueFactory::create_string(&format!("Complex Record {}", i)),
        )?;

        let mut user = DefaultValueFactory::create_object();
        user.set(
            "name",
            DefaultValueFactory::create_string(&format!("User {}", i)),
        )?;
        user.set(
            "email",
            DefaultValueFactory::create_string(&format!("user{}@complex.com", i)),
        )?;
        user.set("active", DefaultValueFactory::create_bool(i % 2 == 0))?;

        let mut profile = DefaultValueFactory::create_object();
        profile.set(
            "age",
            DefaultValueFactory::create_number(20.0 + (i % 50) as f64)?,
        )?;
        profile.set(
            "department",
            Value::from_str(match i % 4 {
                0 => "Engineering",
                1 => "Sales",
                2 => "Marketing",
                3 => "Support",
                _ => unreachable!(),
            }),
        )?;
        user.set("profile", profile)?;

        complex_data.set("user", user)?;

        let mut items = DefaultValueFactory::create_array();
        for j in 0..10 {
            let mut item = DefaultValueFactory::create_object();
            item.set(
                "item_id",
                DefaultValueFactory::create_number(j as f64).unwrap(),
            )?;
            item.set(
                "value",
                DefaultValueFactory::create_string(&format!("Item {}-{}", i, j)),
            )?;
            item.set(
                "price",
                DefaultValueFactory::create_number(10.0 + j as f64).unwrap(),
            )?;
            items.push(item)?;
        }
        complex_data.set("items", items)?;

        let mut metadata = DefaultValueFactory::create_object();
        metadata.set(
            "created_at",
            Value::from_number(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as f64,
            )?,
        )?;
        metadata.set("version", DefaultValueFactory::create_number(1.0).unwrap())?;
        metadata.set("tags", {
            let mut tags = DefaultValueFactory::create_array();
            tags.push(DefaultValueFactory::create_string("complex"))?;
            tags.push(DefaultValueFactory::create_string("test"))?;
            tags.push(DefaultValueFactory::create_string(&format!(
                "batch_{}",
                i / 100
            )))?;
            tags
        })?;
        complex_data.set("metadata", metadata)?;

        let result = manager.insert("complex_records".to_string(), None, complex_data);
        assert!(result.is_ok());

        if i % 100 == 0 && i > 0 {
            let elapsed = start.elapsed();
            let ops_per_sec = (i as f64) / elapsed.as_secs_f64();
            println!(
                "  Complex structures: {} processed, {:.0} ops/sec",
                i, ops_per_sec
            );
        }
    }

    let total_duration = start.elapsed();
    let ops_per_sec = 1000.0 / total_duration.as_secs_f64();

    println!("COMPLEX DATA Performance (1000 nested structures):");
    println!("  Total time: {:?}", total_duration);
    println!("  Throughput: {:.0} ops/sec", ops_per_sec);

    let retrieval_start = Instant::now();
    let all_complex = manager.get_all("complex_records".to_string()).unwrap();
    let retrieval_duration = retrieval_start.elapsed();

    println!(
        "  Retrieval time: {:?} for {} records",
        retrieval_duration,
        all_complex.len()
    );
    assert_eq!(all_complex.len(), 1000);

    let access_start = Instant::now();
    let mut successful_accesses = 0;

    for record in all_complex.iter().take(100) {
        if let Some(user) = record.get("user").unwrap() {
            if let Some(profile) = user.get("profile").unwrap() {
                if let Some(_department) = profile.get("department").unwrap() {
                    successful_accesses += 1;
                }
            }
        }
    }

    let access_duration = access_start.elapsed();
    println!(
        "  Nested access: {} accesses in {:?}",
        successful_accesses, access_duration
    );

    assert!(
        ops_per_sec > 100.0,
        "Complex data too slow: {:.0} ops/sec < 100 ops/sec",
        ops_per_sec
    );
    assert!(
        retrieval_duration.as_millis() < 500,
        "Retrieval too slow: {:?} > 500ms",
        retrieval_duration
    );
    assert_eq!(
        successful_accesses, 100,
        "Should access all nested data successfully"
    );

    Ok(())
}

#[test]
fn test_performance_memory_usage() {
    let mut manager = DefaultModelManager::create();

    println!("=== PERFORMANCE TEST: Memory Usage ===");

    let record_sizes = vec![100, 500, 1000, 2000];

    for &size in &record_sizes {
        let start = Instant::now();

        for i in 0..size {
            let mut data = DefaultValueFactory::create_object();

            for field in 0..20 {
                data.set(
                    &format!("field_{}", field),
                    DefaultValueFactory::create_string(&format!("value_{}_{}", i, field)),
                )
                .unwrap();
            }

            data.set("id", DefaultValueFactory::create_number(i as f64).unwrap())
                .unwrap();
            data.set(
                "size_category",
                DefaultValueFactory::create_number(size as f64).unwrap(),
            )
            .unwrap();

            let result = manager.insert(format!("memory_test_{}", size), None, data);
            assert!(result.is_ok());
        }

        let duration = start.elapsed();
        let ops_per_sec = (size as f64) / duration.as_secs_f64();

        let records = manager.get_all(format!("memory_test_{}", size)).unwrap();
        assert_eq!(records.len(), size);

        println!(
            "  {} records: {:?}, {:.0} ops/sec",
            size, duration, ops_per_sec
        );
    }

    println!("  Testing data access after bulk inserts...");

    for &size in &record_sizes {
        let start = Instant::now();
        let records = manager.get_all(format!("memory_test_{}", size)).unwrap();
        let duration = start.elapsed();

        assert_eq!(records.len(), size);
        println!("    {} records retrieved in {:?}", size, duration);

        if !records.is_empty() {
            let first_record = &records[0];
            assert!(first_record.get("field_0").unwrap().is_some());
            assert!(first_record.get("id").unwrap().is_some());
        }
    }
}
#[test]
fn test_performance_mixed_workload() {
    let mut manager = DefaultModelManager::create();

    println!("=== PERFORMANCE TEST: Mixed Workload ===");

    for i in 0..1000 {
        let mut data = DefaultValueFactory::create_object();
        data.set("id", DefaultValueFactory::create_number(i as f64).unwrap())
            .unwrap();
        data.set(
            "name",
            DefaultValueFactory::create_string(&format!("Initial User {}", i)),
        )
        .unwrap();

        manager
            .insert("mixed_users".to_string(), Some(format!("user_{}", i)), data)
            .unwrap();
    }

    println!("  Pre-populated with 1000 records");

    let total_operations = 5000;
    let start = Instant::now();

    let mut insert_count = 0;
    let mut read_count = 0;
    let mut update_count = 0;

    for i in 0..total_operations {
        let operation_type = i % 10;

        match operation_type {
            0..=5 => {
                let _all_users = manager.get_all("mixed_users".to_string()).unwrap();
                read_count += 1;
            }
            6..=8 => {
                let mut new_data = DefaultValueFactory::create_object();
                new_data
                    .set(
                        "id",
                        DefaultValueFactory::create_number((1000 + insert_count) as f64).unwrap(),
                    )
                    .unwrap();
                new_data
                    .set(
                        "name",
                        DefaultValueFactory::create_string(&format!("New User {}", insert_count)),
                    )
                    .unwrap();
                new_data
                    .set("created_in_mixed", DefaultValueFactory::create_bool(true))
                    .unwrap();

                let result = manager.insert("mixed_users".to_string(), None, new_data);
                assert!(result.is_ok());
                insert_count += 1;
            }
            9 => {
                let user_id = format!("user_{}", update_count % 1000);
                let mut update_data = DefaultValueFactory::create_object();
                update_data
                    .set(
                        "name",
                        DefaultValueFactory::create_string(&format!(
                            "Updated User {}",
                            update_count
                        )),
                    )
                    .unwrap();
                update_data
                    .set("updated", DefaultValueFactory::create_bool(true))
                    .unwrap();
                update_data
                    .set(
                        "update_count",
                        DefaultValueFactory::create_number(update_count as f64).unwrap(),
                    )
                    .unwrap();

                let _result = manager.update("mixed_users".to_string(), user_id, update_data);
                update_count += 1;
            }
            _ => unreachable!(),
        }

        if i % 1000 == 0 && i > 0 {
            let elapsed = start.elapsed();
            let ops_per_sec = (i as f64) / elapsed.as_secs_f64();
            println!("    {} operations completed, {:.0} ops/sec", i, ops_per_sec);
        }
    }

    let total_duration = start.elapsed();
    let total_ops_per_sec = (total_operations as f64) / total_duration.as_secs_f64();

    println!("MIXED WORKLOAD Performance:");
    println!("  Total operations: {}", total_operations);
    println!(
        "  Reads: {} ({}%)",
        read_count,
        (read_count * 100) / total_operations
    );
    println!(
        "  Inserts: {} ({}%)",
        insert_count,
        (insert_count * 100) / total_operations
    );
    println!(
        "  Updates: {} ({}%)",
        update_count,
        (update_count * 100) / total_operations
    );
    println!("  Total time: {:?}", total_duration);
    println!("  Overall throughput: {:.0} ops/sec", total_ops_per_sec);

    let final_users = manager.get_all("mixed_users".to_string()).unwrap();
    let expected_final_count = 1000 + insert_count;
    assert_eq!(final_users.len(), expected_final_count);

    println!(
        "  Final record count: {} (expected: {})",
        final_users.len(),
        expected_final_count
    );

    assert!(
        total_ops_per_sec > 500.0,
        "Mixed workload too slow: {:.0} ops/sec < 500 ops/sec",
        total_ops_per_sec
    );
}

#[test]
fn test_performance_detailed_benchmarks() {
    let mut manager = DefaultModelManager::create();

    println!("=== DETAILED PERFORMANCE BENCHMARKS ===");

    let mut test_data = DefaultValueFactory::create_object();
    test_data
        .set("name", DefaultValueFactory::create_string("Benchmark User"))
        .unwrap();
    test_data
        .set(
            "email",
            DefaultValueFactory::create_string("bench@test.com"),
        )
        .unwrap();
    test_data
        .set("active", DefaultValueFactory::create_bool(true))
        .unwrap();

    let mut insert_times = Vec::new();
    let start = Instant::now();

    for _ in 0..100 {
        let operation_start = Instant::now();
        let data = test_data.clone();
        let result = manager.insert("benchmark_users".to_string(), None, data);
        let operation_duration = operation_start.elapsed();

        assert!(result.is_ok());
        insert_times.push(operation_duration.as_micros());
    }

    let total_duration = start.elapsed();
    let avg_insert_time = insert_times.iter().sum::<u128>() / insert_times.len() as u128;

    println!("BENCHMARK: Single Insert (100 iterations)");
    println!("  Total time: {:?}", total_duration);
    println!("  Average: {}μs", avg_insert_time);
    println!("  Min: {}μs", insert_times.iter().min().unwrap());
    println!("  Max: {}μs", insert_times.iter().max().unwrap());

    let users = manager.get_all("benchmark_users".to_string()).unwrap();
    let record_count = users.len();

    let mut get_times = Vec::new();
    let get_start = Instant::now();

    for _ in 0..50 {
        let operation_start = Instant::now();
        let _result = manager.get_all("benchmark_users".to_string()).unwrap();
        let operation_duration = operation_start.elapsed();
        get_times.push(operation_duration.as_micros());
    }

    let get_total_duration = get_start.elapsed();
    let avg_get_time = get_times.iter().sum::<u128>() / get_times.len() as u128;

    println!(
        "BENCHMARK: Get All ({} records, 50 iterations)",
        record_count
    );
    println!("  Total time: {:?}", get_total_duration);
    println!("  Average: {}μs", avg_get_time);
    println!("  Min: {}μs", get_times.iter().min().unwrap());
    println!("  Max: {}μs", get_times.iter().max().unwrap());

    assert!(
        avg_insert_time < 10000,
        "Detailed insert benchmark too slow: {}μs > 10000μs",
        avg_insert_time
    );
    assert!(
        avg_get_time < 50000,
        "Detailed get benchmark too slow: {}μs > 50000μs",
        avg_get_time
    );

    println!("=== BENCHMARKS COMPLETED ===");
}

#[test]
fn test_performance_update_operations() {
    let mut manager = DefaultModelManager::create();

    println!("=== PERFORMANCE TEST: Update Operations ===");

    for i in 0..1000 {
        let mut data = DefaultValueFactory::create_object();
        data.set("id", DefaultValueFactory::create_number(i as f64).unwrap())
            .unwrap();
        data.set(
            "name",
            DefaultValueFactory::create_string(&format!("User {}", i)),
        )
        .unwrap();
        data.set("version", DefaultValueFactory::create_number(1.0).unwrap())
            .unwrap();

        manager
            .insert(
                "update_users".to_string(),
                Some(format!("user_{}", i)),
                data,
            )
            .unwrap();
    }

    let mut update_times = Vec::new();

    for i in 0..1000 {
        let mut updated_data = DefaultValueFactory::create_object();
        updated_data
            .set("id", DefaultValueFactory::create_number(i as f64).unwrap())
            .unwrap();
        updated_data
            .set(
                "name",
                DefaultValueFactory::create_string(&format!("Updated User {}", i)),
            )
            .unwrap();
        updated_data
            .set("version", DefaultValueFactory::create_number(2.0).unwrap())
            .unwrap();
        updated_data
            .set(
                "last_modified",
                Value::from_number(
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as f64,
                )
                .unwrap(),
            )
            .unwrap();

        let start = Instant::now();
        let result = manager.update(
            "update_users".to_string(),
            format!("user_{}", i),
            updated_data,
        );
        let duration = start.elapsed();

        assert!(result.is_ok());
        update_times.push(duration.as_micros());
    }

    let avg_update_time = update_times.iter().sum::<u128>() / update_times.len() as u128;
    let max_update_time = *update_times.iter().max().unwrap();
    let min_update_time = *update_times.iter().min().unwrap();

    println!("UPDATE Performance (1000 operations):");
    println!("  Average: {}μs", avg_update_time);
    println!("  Min: {}μs", min_update_time);
    println!("  Max: {}μs", max_update_time);

    assert!(
        avg_update_time < 5000,
        "Update too slow: {}μs > 5000μs",
        avg_update_time
    );
}

#[test]
fn test_performance_delete_operations() {
    let mut manager = DefaultModelManager::create();

    println!("=== PERFORMANCE TEST: Delete Operations ===");

    for i in 0..1000 {
        let mut data = DefaultValueFactory::create_object();
        data.set("id", DefaultValueFactory::create_number(i as f64).unwrap())
            .unwrap();
        data.set(
            "name",
            DefaultValueFactory::create_string(&format!("User {}", i)),
        )
        .unwrap();

        manager
            .insert(
                "delete_users".to_string(),
                Some(format!("user_{}", i)),
                data,
            )
            .unwrap();
    }

    let mut delete_times = Vec::new();

    for i in 0..1000 {
        let start = Instant::now();
        let result = manager.remove("delete_users".to_string(), format!("user_{}", i));
        let duration = start.elapsed();

        assert!(result.is_ok());
        delete_times.push(duration.as_micros());
    }

    let avg_delete_time = delete_times.iter().sum::<u128>() / delete_times.len() as u128;
    let max_delete_time = *delete_times.iter().max().unwrap();
    let min_delete_time = *delete_times.iter().min().unwrap();

    println!("DELETE Performance (1000 operations):");
    println!("  Average: {}μs", avg_delete_time);
    println!("  Min: {}μs", min_delete_time);
    println!("  Max: {}μs", max_delete_time);

    let remaining_users = manager.get_all("delete_users".to_string()).unwrap();
    assert_eq!(remaining_users.len(), 0);

    assert!(
        avg_delete_time < 5000,
        "Delete too slow: {}μs > 5000μs",
        avg_delete_time
    );
}

#[test]
fn test_performance_concurrent_model_access() {
    let mut manager = DefaultModelManager::create();

    println!("=== PERFORMANCE TEST: Concurrent Model Access ===");

    let models = vec!["model_a", "model_b", "model_c", "model_d", "model_e"];
    let operations_per_model = 500;

    for model_name in &models {
        for i in 0..operations_per_model {
            let mut data = DefaultValueFactory::create_object();
            data.set("id", DefaultValueFactory::create_number(i as f64).unwrap())
                .unwrap();
            data.set("model", DefaultValueFactory::create_string(model_name))
                .unwrap();
            data.set(
                "data",
                DefaultValueFactory::create_string(&format!("Data for {} item {}", model_name, i)),
            )
            .unwrap();

            manager.insert(model_name.to_string(), None, data).unwrap();
        }
    }

    let start = Instant::now();
    let mut operation_count = 0;

    for _ in 0..1000 {
        for model_name in &models {
            let _records = manager.get_all(model_name.to_string()).unwrap();
            operation_count += 1;
        }
    }

    let duration = start.elapsed();
    let ops_per_sec = (operation_count as f64) / duration.as_secs_f64();

    println!("CONCURRENT MODEL ACCESS Performance:");
    println!("  Models accessed: {}", models.len());
    println!("  Total operations: {}", operation_count);
    println!("  Total time: {:?}", duration);
    println!("  Throughput: {:.0} ops/sec", ops_per_sec);

    for model_name in &models {
        let records = manager.get_all(model_name.to_string()).unwrap();
        assert_eq!(records.len(), operations_per_model);
    }

    assert!(
        ops_per_sec > 1000.0,
        "Concurrent access too slow: {:.0} ops/sec < 1000 ops/sec",
        ops_per_sec
    );
}

#[test]
fn test_performance_large_record_operations() {
    let mut manager = DefaultModelManager::create();

    println!("=== PERFORMANCE TEST: Large Record Operations ===");

    let large_text = "x".repeat(10000);
    let mut large_records = Vec::new();

    for i in 0..100 {
        let mut data = DefaultValueFactory::create_object();
        data.set("id", DefaultValueFactory::create_number(i as f64).unwrap())
            .unwrap();
        data.set(
            "large_field",
            DefaultValueFactory::create_string(&large_text),
        )
        .unwrap();
        data.set(
            "metadata",
            DefaultValueFactory::create_string(&format!("Metadata for record {}", i)),
        )
        .unwrap();

        let mut nested_data = DefaultValueFactory::create_object();
        for j in 0..50 {
            nested_data
                .set(
                    &format!("field_{}", j),
                    DefaultValueFactory::create_string(&format!("value_{}_{}", i, j)),
                )
                .unwrap();
        }
        data.set("nested", nested_data).unwrap();

        large_records.push(data);
    }

    let start = Instant::now();
    for (i, record) in large_records.into_iter().enumerate() {
        let result = manager.insert(
            "large_records".to_string(),
            Some(format!("large_{}", i)),
            record,
        );
        assert!(result.is_ok());
    }
    let insert_duration = start.elapsed();

    let start = Instant::now();
    let all_records = manager.get_all("large_records".to_string()).unwrap();
    let retrieval_duration = start.elapsed();

    println!("LARGE RECORD Performance:");
    println!("  Insert 100 large records: {:?}", insert_duration);
    println!("  Retrieve 100 large records: {:?}", retrieval_duration);
    println!("  Average insert time: {:?}", insert_duration / 100);
    println!("  Average retrieval time: {:?}", retrieval_duration / 100);

    assert_eq!(all_records.len(), 100);
    assert!(
        insert_duration.as_millis() < 5000,
        "Large record insert too slow"
    );
    assert!(
        retrieval_duration.as_millis() < 1000,
        "Large record retrieval too slow"
    );
}
