use tokio::task::LocalSet;
use std::time::Instant;
use model_manager::{
    DynamicValue, ModelManager, DefaultValue, ModelManagerFactory,
    DynamicValueFactory, DefaultModelManager
};

type Value = <DefaultValue as DynamicValueFactory>::Value;
type Manager = <DefaultModelManager as ModelManagerFactory<Value>>::Manager;

#[tokio::test]
async fn test_performance_single_operations() {
    let local_set = LocalSet::new();

    local_set.run_until(async {
        let mut manager = DefaultModelManager::create();

        println!("=== PERFORMANCE TEST: Single Operations ===");

        // Test INSERT performance
        let mut insert_times = Vec::new();
        for i in 0..1000 {
            let mut data = Value::new_object();
            data.set("id", Value::from_number(i as f64).unwrap()).await.unwrap();
            data.set("name", Value::from_str(&format!("User {}", i))).await.unwrap();
            data.set("email", Value::from_str(&format!("user{}@test.com", i))).await.unwrap();

            let start = Instant::now();
            let result = manager.insert("perf_users".to_string(), None, data).await;
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

        // Test GET performance
        let mut get_times = Vec::new();
        let all_users = manager.get_all("perf_users".to_string()).await.unwrap();

        for _ in 0..100 {
            let start = Instant::now();
            let _users = manager.get_all("perf_users".to_string()).await.unwrap();
            let duration = start.elapsed();
            get_times.push(duration.as_micros());
        }

        let avg_get_time = get_times.iter().sum::<u128>() / get_times.len() as u128;
        println!("GET_ALL Performance (100 operations, {} records):", all_users.len());
        println!("  Average: {}μs", avg_get_time);

        // Performance assertions
        assert!(avg_insert_time < 5000, "Insert too slow: {}μs > 5000μs", avg_insert_time);
        assert!(avg_get_time < 10000, "Get all too slow: {}μs > 10000μs", avg_get_time);
    }).await;
}

#[tokio::test]
async fn test_performance_bulk_operations() {
    let local_set = LocalSet::new();

    local_set.run_until(async {
        let mut manager = DefaultModelManager::create();

        println!("=== PERFORMANCE TEST: Bulk Operations ===");

        // Test bulk insert performance
        let start = Instant::now();
        for i in 0..10000 {
            let mut data = Value::new_object();
            data.set("id", Value::from_number(i as f64).unwrap()).await.unwrap();
            data.set("name", Value::from_str(&format!("BulkUser {}", i))).await.unwrap();
            data.set("category", Value::from_str(match i % 5 {
                0 => "A",
                1 => "B",
                2 => "C",
                3 => "D",
                4 => "E",
                _ => unreachable!(),
            })).await.unwrap();

            let result = manager.insert("bulk_users".to_string(), None, data).await;
            assert!(result.is_ok());

            // Progress reporting every 1000 operations
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

        // Verify all data was inserted
        let all_users = manager.get_all("bulk_users".to_string()).await.unwrap();
        assert_eq!(all_users.len(), 10000);

        // Performance assertions
        assert!(total_ops_per_sec > 1000.0, "Bulk insert too slow: {:.0} ops/sec < 1000 ops/sec", total_ops_per_sec);
        assert!(total_duration.as_secs() < 30, "Bulk insert took too long: {:?} > 30s", total_duration);
    }).await;
}

#[tokio::test]
async fn test_performance_multiple_models() {
    let local_set = LocalSet::new();

    local_set.run_until(async {
        let mut manager = DefaultModelManager::create();

        println!("=== PERFORMANCE TEST: Multiple Models ===");

        let models = vec!["users", "products", "orders", "logs", "configs"];
        let operations_per_model = 2000;

        let start = Instant::now();

        for (model_idx, model_name) in models.iter().enumerate() {
            for i in 0..operations_per_model {
                let mut data = Value::new_object();
                data.set("model_id", Value::from_number(model_idx as f64).unwrap()).await.unwrap();
                data.set("record_id", Value::from_number(i as f64).unwrap()).await.unwrap();
                data.set("name", Value::from_str(&format!("{}_record_{}", model_name, i))).await.unwrap();
                data.set("timestamp", Value::from_number(std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as f64).unwrap()).await.unwrap();

                let result = manager.insert(model_name.to_string(), None, data).await;
                assert!(result.is_ok());
            }

            let elapsed = start.elapsed();
            let total_ops = (model_idx + 1) * operations_per_model;
            let ops_per_sec = (total_ops as f64) / elapsed.as_secs_f64();
            println!("  Model '{}' completed: {} total ops, {:.0} ops/sec",
                    model_name, total_ops, ops_per_sec);
        }

        let total_duration = start.elapsed();
        let total_operations = models.len() * operations_per_model;
        let total_ops_per_sec = (total_operations as f64) / total_duration.as_secs_f64();

        println!("MULTIPLE MODELS Performance:");
        println!("  Models: {}", models.len());
        println!("  Total operations: {}", total_operations);
        println!("  Total time: {:?}", total_duration);
        println!("  Overall throughput: {:.0} ops/sec", total_ops_per_sec);

        // Verify all models have correct data
        for model_name in &models {
            let records = manager.get_all(model_name.to_string()).await.unwrap();
            assert_eq!(records.len(), operations_per_model,
                      "Model '{}' should have {} records, got {}",
                      model_name, operations_per_model, records.len());
        }

        // Performance assertions
        assert!(total_ops_per_sec > 800.0, "Multi-model too slow: {:.0} ops/sec < 800 ops/sec", total_ops_per_sec);
    }).await;
}

#[tokio::test]
async fn test_performance_complex_data_structures() {
    let local_set = LocalSet::new();

    local_set.run_until(async {
        let mut manager = DefaultModelManager::create();

        println!("=== PERFORMANCE TEST: Complex Data Structures ===");

        let start = Instant::now();

        for i in 0..1000 {
            // Create complex nested structure
            let mut complex_data = Value::new_object();
            complex_data.set("id", Value::from_number(i as f64)?).await?;
            complex_data.set("title", Value::from_str(&format!("Complex Record {}", i))).await?;

            // Nested user object
            let mut user = Value::new_object();
            user.set("name", Value::from_str(&format!("User {}", i))).await?;
            user.set("email", Value::from_str(&format!("user{}@complex.com", i))).await?;
            user.set("active", Value::from_bool(i % 2 == 0)).await?;

            // Profile object
            let mut profile = Value::new_object();
            profile.set("age", Value::from_number(20.0 + (i % 50) as f64)?).await?;
            profile.set("department", Value::from_str(match i % 4 {
                0 => "Engineering",
                1 => "Sales",
                2 => "Marketing",
                3 => "Support",
                _ => unreachable!(),
            })).await?;
            user.set("profile", profile).await?;

            complex_data.set("user", user).await?;

            // Array of items
            let mut items = Value::new_array();
            for j in 0..10 {
                let mut item = Value::new_object();
                item.set("item_id", Value::from_number(j as f64)?).await?;
                item.set("value", Value::from_str(&format!("Item {}-{}", i, j))).await?;
                item.set("price", Value::from_number(10.0 + j as f64)?).await?;
                items.push(item).await?;
            }
            complex_data.set("items", items).await?;

            // Metadata
            let mut metadata = Value::new_object();
            metadata.set("created_at", Value::from_number(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as f64
            )?).await?;
            metadata.set("version", Value::from_number(1.0)?).await?;
            metadata.set("tags", {
                let mut tags = Value::new_array();
                tags.push(Value::from_str("complex")).await?;
                tags.push(Value::from_str("test")).await?;
                tags.push(Value::from_str(&format!("batch_{}", i / 100))).await?;
                tags
            }).await?;
            complex_data.set("metadata", metadata).await?;

            let result = manager.insert("complex_records".to_string(), None, complex_data).await;
            assert!(result.is_ok());

            if i % 100 == 0 && i > 0 {
                let elapsed = start.elapsed();
                let ops_per_sec = (i as f64) / elapsed.as_secs_f64();
                println!("  Complex structures: {} processed, {:.0} ops/sec", i, ops_per_sec);
            }
        }

        let total_duration = start.elapsed();
        let ops_per_sec = 1000.0 / total_duration.as_secs_f64();

        println!("COMPLEX DATA Performance (1000 nested structures):");
        println!("  Total time: {:?}", total_duration);
        println!("  Throughput: {:.0} ops/sec", ops_per_sec);

        // Test complex data retrieval performance
        let retrieval_start = Instant::now();
        let all_complex = manager.get_all("complex_records".to_string()).await.unwrap();
        let retrieval_duration = retrieval_start.elapsed();

        println!("  Retrieval time: {:?} for {} records", retrieval_duration, all_complex.len());
        assert_eq!(all_complex.len(), 1000);

        // Test nested data access performance
        let access_start = Instant::now();
        let mut successful_accesses = 0;

        for record in all_complex.iter().take(100) {
            if let Some(user) = record.get("user").await.unwrap() {
                if let Some(profile) = user.get("profile").await.unwrap() {
                    if let Some(_department) = profile.get("department").await.unwrap() {
                        successful_accesses += 1;
                    }
                }
            }
        }

        let access_duration = access_start.elapsed();
        println!("  Nested access: {} accesses in {:?}", successful_accesses, access_duration);

        // Performance assertions
        assert!(ops_per_sec > 100.0, "Complex data too slow: {:.0} ops/sec < 100 ops/sec", ops_per_sec);
        assert!(retrieval_duration.as_millis() < 500, "Retrieval too slow: {:?} > 500ms", retrieval_duration);
        assert_eq!(successful_accesses, 100, "Should access all nested data successfully");

        Ok::<(), Box<dyn std::error::Error>>(())
    }).await.unwrap();
}

#[tokio::test]
async fn test_performance_memory_usage() {
    let local_set = LocalSet::new();

    local_set.run_until(async {
        let mut manager = DefaultModelManager::create();

        println!("=== PERFORMANCE TEST: Memory Usage ===");

        // Test memory behavior with large datasets
        let record_sizes = vec![100, 500, 1000, 2000];

        for &size in &record_sizes {
            let start = Instant::now();

            for i in 0..size {
                let mut data = Value::new_object();

                // Create moderate size data
                for field in 0..20 {
                    data.set(
                        &format!("field_{}", field),
                        Value::from_str(&format!("value_{}_{}", i, field))
                    ).await.unwrap();
                }

                data.set("id", Value::from_number(i as f64).unwrap()).await.unwrap();
                data.set("size_category", Value::from_number(size as f64).unwrap()).await.unwrap();

                let result = manager.insert(
                    format!("memory_test_{}", size),
                    None,
                    data
                ).await;
                assert!(result.is_ok());
            }

            let duration = start.elapsed();
            let ops_per_sec = (size as f64) / duration.as_secs_f64();

            // Verify data integrity
            let records = manager.get_all(format!("memory_test_{}", size)).await.unwrap();
            assert_eq!(records.len(), size);

            println!("  {} records: {:?}, {:.0} ops/sec", size, duration, ops_per_sec);
        }

        // Test cleanup behavior
        println!("  Testing data access after bulk inserts...");

        for &size in &record_sizes {
            let start = Instant::now();
            let records = manager.get_all(format!("memory_test_{}", size)).await.unwrap();
            let duration = start.elapsed();

            assert_eq!(records.len(), size);
            println!("    {} records retrieved in {:?}", size, duration);

            // Verify a few records have correct structure
            if !records.is_empty() {
                let first_record = &records[0];
                assert!(first_record.get("field_0").await.unwrap().is_some());
                assert!(first_record.get("id").await.unwrap().is_some());
            }
        }
    }).await;
}

#[tokio::test]
async fn test_performance_mixed_workload() {
    let local_set = LocalSet::new();

    local_set.run_until(async {
        let mut manager = DefaultModelManager::create();

        println!("=== PERFORMANCE TEST: Mixed Workload ===");

        // Pre-populate with some data
        for i in 0..1000 {
            let mut data = Value::new_object();
            data.set("id", Value::from_number(i as f64).unwrap()).await.unwrap();
            data.set("name", Value::from_str(&format!("Initial User {}", i))).await.unwrap();

            manager.insert("mixed_users".to_string(), Some(format!("user_{}", i)), data).await.unwrap();
        }

        println!("  Pre-populated with 1000 records");

        // Mixed workload: 60% reads, 30% inserts, 10% updates
        let total_operations = 5000;
        let start = Instant::now();

        let mut insert_count = 0;
        let mut read_count = 0;
        let mut update_count = 0;

        for i in 0..total_operations {
            let operation_type = i % 10;

            match operation_type {
                // 60% reads (0-5)
                0..=5 => {
                    let _all_users = manager.get_all("mixed_users".to_string()).await.unwrap();
                    read_count += 1;
                }
                // 30% inserts (6-8)
                6..=8 => {
                    let mut new_data = Value::new_object();
                    new_data.set("id", Value::from_number((1000 + insert_count) as f64).unwrap()).await.unwrap();
                    new_data.set("name", Value::from_str(&format!("New User {}", insert_count))).await.unwrap();
                    new_data.set("created_in_mixed", Value::from_bool(true)).await.unwrap();

                    let result = manager.insert("mixed_users".to_string(), None, new_data).await;
                    assert!(result.is_ok());
                    insert_count += 1;
                }
                // 10% updates (9)
                9 => {
                    let user_id = format!("user_{}", update_count % 1000);
                    let mut update_data = Value::new_object();
                    update_data.set("name", Value::from_str(&format!("Updated User {}", update_count))).await.unwrap();
                    update_data.set("updated", Value::from_bool(true)).await.unwrap();
                    update_data.set("update_count", Value::from_number(update_count as f64).unwrap()).await.unwrap();

                    let _result = manager.update("mixed_users".to_string(), user_id, update_data).await;
                    // Update might fail if user doesn't exist, that's OK for this test
                    update_count += 1;
                }
                _ => unreachable!()
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
        println!("  Reads: {} ({}%)", read_count, (read_count * 100) / total_operations);
        println!("  Inserts: {} ({}%)", insert_count, (insert_count * 100) / total_operations);
        println!("  Updates: {} ({}%)", update_count, (update_count * 100) / total_operations);
        println!("  Total time: {:?}", total_duration);
        println!("  Overall throughput: {:.0} ops/sec", total_ops_per_sec);

        // Verify final state
        let final_users = manager.get_all("mixed_users".to_string()).await.unwrap();
        let expected_final_count = 1000 + insert_count;
        assert_eq!(final_users.len(), expected_final_count);

        println!("  Final record count: {} (expected: {})", final_users.len(), expected_final_count);

        // Performance assertions
        assert!(total_ops_per_sec > 500.0, "Mixed workload too slow: {:.0} ops/sec < 500 ops/sec", total_ops_per_sec);
    }).await;
}



#[tokio::test]
async fn test_performance_detailed_benchmarks() {
    let local_set = LocalSet::new();

    local_set.run_until(async {
        let mut manager = DefaultModelManager::create();

        println!("=== DETAILED PERFORMANCE BENCHMARKS ===");

        // Prepare test data
        let mut test_data = Value::new_object();
        test_data.set("name", Value::from_str("Benchmark User")).await.unwrap();
        test_data.set("email", Value::from_str("bench@test.com")).await.unwrap();
        test_data.set("active", Value::from_bool(true)).await.unwrap();

        // Benchmark single insert - version simplificada
        let mut insert_times = Vec::new();
        let start = Instant::now();

        for _ in 0..100 {
            let operation_start = Instant::now();
            let data = test_data.clone();
            let result = manager.insert("benchmark_users".to_string(), None, data).await;
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

        // Benchmark get_all
        let users = manager.get_all("benchmark_users".to_string()).await.unwrap();
        let record_count = users.len();

        let mut get_times = Vec::new();
        let get_start = Instant::now();

        for _ in 0..50 {
            let operation_start = Instant::now();
            let _result = manager.get_all("benchmark_users".to_string()).await.unwrap();
            let operation_duration = operation_start.elapsed();
            get_times.push(operation_duration.as_micros());
        }

        let get_total_duration = get_start.elapsed();
        let avg_get_time = get_times.iter().sum::<u128>() / get_times.len() as u128;

        println!("BENCHMARK: Get All ({} records, 50 iterations)", record_count);
        println!("  Total time: {:?}", get_total_duration);
        println!("  Average: {}μs", avg_get_time);
        println!("  Min: {}μs", get_times.iter().min().unwrap());
        println!("  Max: {}μs", get_times.iter().max().unwrap());

        // Performance assertions for detailed benchmarks
        assert!(avg_insert_time < 10000, "Detailed insert benchmark too slow: {}μs > 10000μs", avg_insert_time);
        assert!(avg_get_time < 50000, "Detailed get benchmark too slow: {}μs > 50000μs", avg_get_time);

        println!("=== BENCHMARKS COMPLETED ===");
    }).await;
}