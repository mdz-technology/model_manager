use tokio::task::LocalSet;
use model_manager::{
    ArrayIterator, ArrayIteratorFactory, ObjectIterator, ObjectIteratorFactory,
    DefaultValue, DynamicValue, DynamicValueFactory, IteratorFactory,
    DefaultIteratorFactory, ModelManager, ModelManagerFactory,
    DefaultModelManager,
};

// Type alias para el valor concreto que usa el factory
type Value = <DefaultValue as DynamicValueFactory>::Value;

#[tokio::test]
async fn test_debug_simple_array_creation() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            let empty_array = Value::new_array();
            assert!(empty_array.is_array());
            assert!(empty_array.is_empty());
            let iterator_result =
                DefaultIteratorFactory::create_array_iterator(empty_array);
            match iterator_result {
                Ok(mut iterator) => {
                    let next_result = iterator.next().await;
                    assert!(next_result.is_none());
                }
                Err(e) => {
                    panic!("Iterator creation failed: {}", e);
                }
            }
        })
        .await;
}

#[tokio::test]
async fn test_debug_simple_object_creation() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            let empty_object = Value::new_object();

            assert!(empty_object.is_object());
            assert!(empty_object.is_empty());

            let iterator_result =
                DefaultIteratorFactory::create_object_iterator(empty_object);

            match iterator_result {
                Ok(mut iterator) => {
                    let next_result = iterator.next().await;

                    assert!(next_result.is_none());
                }
                Err(e) => {
                    panic!("Iterator creation failed: {}", e);
                }
            }
        })
        .await;
}

#[tokio::test]
async fn test_array_iterator_basic_next() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Non-array value for array iterator
            let non_array = Value::from_str("not an array");

            // When: Try to create array iterator from non-array
            let array_result = DefaultIteratorFactory::create_array_iterator(non_array);

            // Then: Error returned
            assert!(array_result.is_err());
            let error_msg = array_result.unwrap_err();
            assert!(
                error_msg.to_lowercase().contains("array")
                    || error_msg.to_lowercase().contains("invalid")
            );

            // Given: Non-object value for object iterator
            let non_object = Value::from_number(123.0).unwrap();

            // When: Try to create object iterator from non-object
            let object_result =
                DefaultIteratorFactory::create_object_iterator(non_object);

            // Then: Error returned
            assert!(object_result.is_err());
            let error_msg = object_result.unwrap_err();
            assert!(
                error_msg.to_lowercase().contains("object")
                    || error_msg.to_lowercase().contains("invalid")
            );
        })
        .await;
}

#[tokio::test]
async fn test_iterator_complex_predicate_operations() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Array con datos complejos para predicados avanzados
            let mut array = Value::new_array();
            for i in 1..=20 {
                let mut item = Value::new_object();
                item.set("id", Value::from_number(i as f64).unwrap())
                    .await
                    .unwrap();
                item.set("name", Value::from_str(&format!("Item {}", i)))
                    .await
                    .unwrap();
                item.set("active", Value::from_bool(i % 3 == 0))
                    .await
                    .unwrap(); // Cada tercer elemento activo
                item.set("score", Value::from_number((i * 5) as f64).unwrap())
                    .await
                    .unwrap();
                array.push(item).await.unwrap();
            }

            // When: Predicados complejos en find
            let mut iterator =
                DefaultIteratorFactory::create_array_iterator(array.clone()).unwrap();

            let high_score_active = iterator
                .find(|item| {
                    let item_clone = item.clone();
                    async move {
                        let score = item_clone
                            .get("score")
                            .await
                            .unwrap()
                            .unwrap()
                            .as_number()
                            .unwrap_or(0.0);
                        let active = item_clone
                            .get("active")
                            .await
                            .unwrap()
                            .unwrap()
                            .as_bool()
                            .unwrap_or(false);
                        score > 50.0 && active
                    }
                })
                .await;

            // Then: Elemento correcto encontrado
            assert!(high_score_active.is_some());
            let found_item = high_score_active.unwrap();
            let found_id = found_item
                .get("id")
                .await
                .unwrap()
                .unwrap()
                .as_number()
                .unwrap();
            assert_eq!(found_id, 12.0); // Primer elemento activo con score > 50

            // When: Filtrado complejo
            let mut iterator2 =
                DefaultIteratorFactory::create_array_iterator(array).unwrap();

            let filtered_complex = iterator2
                .filter(|item| {
                    let item_clone = item.clone();
                    async move {
                        let id = item_clone
                            .get("id")
                            .await
                            .unwrap()
                            .unwrap()
                            .as_number()
                            .unwrap_or(0.0);
                        let score = item_clone
                            .get("score")
                            .await
                            .unwrap()
                            .unwrap()
                            .as_number()
                            .unwrap_or(0.0);
                        id > 10.0 && score % 15.0 == 0.0
                    }
                })
                .await;

            assert_eq!(filtered_complex.len(), 3);
        })
        .await;
}

#[tokio::test]
async fn test_iterator_performance_stress() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Dataset muy grande para stress test
            let mut large_array = Value::new_array();
            for i in 0..5000 {
                large_array
                    .push(Value::from_number(i as f64).unwrap())
                    .await
                    .unwrap();
            }

            // When: Operaciones intensivas de rendimiento
            let start = std::time::Instant::now();

            let mut iterator =
                DefaultIteratorFactory::create_array_iterator(large_array).unwrap();

            let filtered = iterator
                .filter(|item| {
                    let item_clone = item.clone();
                    async move {
                        let num = item_clone.as_number().unwrap_or(0.0) as i32;
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
                    }
                })
                .await;

            let duration = start.elapsed();

            // Then: Performance aceptable incluso con operaciones complejas
            assert!(filtered.len() > 0);
            assert!(duration.as_millis() < 5000);

            println!(
                "✅ Iterator performance stress test passed in {:?}",
                duration
            );
            println!(
                "   Processed 5000 elements, found {} primes",
                filtered.len()
            );
        })
        .await;
}

#[tokio::test]
async fn test_iterator_chaining_operations() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Object con datos para operaciones encadenadas
            let mut object = Value::new_object();
            for i in 1..=10 {
                object
                    .set(
                        &format!("item_{}", i),
                        Value::from_number(i as f64).unwrap(),
                    )
                    .await
                    .unwrap();
            }

            // When: Simular operaciones encadenadas usando múltiples iteradores
            let mut iterator1 =
                DefaultIteratorFactory::create_object_iterator(object).unwrap();

            let filtered = iterator1
                .filter(|_key, value| {
                    let value_clone = value.clone();
                    async move {
                        let num = value_clone.as_number().unwrap_or(0.0);
                        num % 2.0 == 0.0
                    }
                })
                .await;

            let mut array_from_filtered = Value::new_array();
            for (_key, value) in filtered {
                array_from_filtered.push(value).await.unwrap();
            }

            let mut iterator2 =
                DefaultIteratorFactory::create_array_iterator(array_from_filtered)
                    .unwrap();

            let mapped = iterator2
                .map(|item| async move {
                    let num = item.as_number().unwrap_or(0.0);
                    format!("Number: {}", num)
                })
                .await;

            // Then: Operaciones encadenadas correctas
            assert_eq!(mapped.len(), 5); // 2, 4, 6, 8, 10

            for mapped_value in &mapped {
                assert!(mapped_value.starts_with("Number: "));
            }

            println!("✅ Iterator chaining operations test passed");
        })
        .await;
}

#[tokio::test]
async fn test_iterator_memory_efficiency() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Test de eficiencia de memoria con procesamiento por lotes
            let mut large_array = Value::new_array();
            for i in 0..1000 {
                let mut complex_item = Value::new_object();
                complex_item
                    .set("id", Value::from_number(i as f64).unwrap())
                    .await
                    .unwrap();
                complex_item
                    .set("data", Value::from_str(&"x".repeat(100)))
                    .await
                    .unwrap(); // 100 chars por item
                large_array.push(complex_item).await.unwrap();
            }

            // When: Procesamiento por lotes para memoria eficiente
            let mut iterator =
                DefaultIteratorFactory::create_array_iterator(large_array).unwrap();

            use std::sync::{Arc, Mutex};

            let batch_count = Arc::new(Mutex::new(0));
            let total_processed = Arc::new(Mutex::new(0));

            let batch_count_clone = batch_count.clone();
            let total_processed_clone = total_processed.clone();

            let processed_count = iterator
                .for_each_batch(50, move |batch| {
                    let batch_count = batch_count_clone.clone();
                    let total_processed = total_processed_clone.clone();
                    async move {
                        *batch_count.lock().unwrap() += 1;
                        let batch_size = batch.len();

                        for item in batch {
                            let _id = item.get("id").await.unwrap().unwrap().as_number().unwrap();
                        }

                        *total_processed.lock().unwrap() += batch_size;
                    }
                })
                .await;

            // Then: Procesamiento eficiente por lotes
            assert_eq!(processed_count, 1000);
            assert_eq!(*total_processed.lock().unwrap(), 1000);
            assert_eq!(*batch_count.lock().unwrap(), 20); // 1000 / 50 = 20 lotes

            println!("✅ Iterator memory efficiency test passed");
            println!(
                "   Processed {} items in {} batches",
                processed_count,
                *batch_count.lock().unwrap()
            );
        })
        .await;
}

#[tokio::test]
async fn test_iterator_concurrent_safe_operations() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Múltiples iteradores operando independientemente
            let mut array1 = Value::new_array();
            let mut array2 = Value::new_array();

            for i in 0..100 {
                array1
                    .push(Value::from_number(i as f64).unwrap())
                    .await
                    .unwrap();
                array2
                    .push(Value::from_number((i * 2) as f64).unwrap())
                    .await
                    .unwrap();
            }

            // When: Operaciones "concurrentes" usando tokio::join!
            let (result1, result2) = tokio::join!(
                async {
                    let mut iter1 =
                        DefaultIteratorFactory::create_array_iterator(array1).unwrap();
                    iter1
                        .filter(|item| {
                            let item_clone = item.clone();
                            async move { item_clone.as_number().unwrap_or(0.0) % 10.0 == 0.0 }
                        })
                        .await
                },
                async {
                    let mut iter2 =
                        DefaultIteratorFactory::create_array_iterator(array2).unwrap();
                    iter2
                        .map(|item| async move { item.as_number().unwrap_or(0.0) / 2.0 })
                        .await
                }
            );

            // Then: Ambas operaciones completadas independientemente
            assert_eq!(result1.len(), 10);
            assert_eq!(result2.len(), 100);

            assert_eq!(result1[0].as_number().unwrap(), 0.0);
            assert_eq!(result1[1].as_number().unwrap(), 10.0);
            assert_eq!(result2[0], 0.0);
            assert_eq!(result2[1], 1.0);

            println!("✅ Iterator concurrent safe operations test passed");
        })
        .await;
}

#[tokio::test]
async fn test_iterator_edge_cases_and_boundaries() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Test Case 1: Array con un solo elemento
            let mut single_item_array = Value::new_array();
            single_item_array
                .push(Value::from_str("only_one"))
                .await
                .unwrap();

            let mut iterator =
                DefaultIteratorFactory::create_array_iterator(single_item_array).unwrap();
            assert_eq!(iterator.size_hint(), (1, Some(1)));
            assert!(iterator.next().await.is_some());
            assert!(iterator.next().await.is_none());

            // Test Case 2: Object con una sola propiedad
            let mut single_prop_object = Value::new_object();
            single_prop_object
                .set("only_prop", Value::from_str("only_value"))
                .await
                .unwrap();

            let mut obj_iterator =
                DefaultIteratorFactory::create_object_iterator(single_prop_object)
                    .unwrap();
            assert_eq!(obj_iterator.size_hint(), (1, Some(1)));
            let pair = obj_iterator.next().await;
            assert!(pair.is_some());
            let (key, value) = pair.unwrap();
            assert_eq!(key, "only_prop");
            assert_eq!(value.as_str().unwrap(), "only_value");

            // Test Case 3: nth con índice 0
            let mut test_array = Value::new_array();
            test_array
                .push(Value::from_str("first"))
                .await
                .unwrap();
            test_array
                .push(Value::from_str("second"))
                .await
                .unwrap();

            let mut nth_iterator =
                DefaultIteratorFactory::create_array_iterator(test_array).unwrap();
            let zeroth_element = nth_iterator.nth(0).await;
            assert!(zeroth_element.is_some());
            assert_eq!(zeroth_element.unwrap().as_str().unwrap(), "first");

            // Test Case 4: Map con función que retorna diferentes tipos
            let mut mixed_array = Value::new_array();
            mixed_array
                .push(Value::from_number(5.0).unwrap())
                .await
                .unwrap();
            mixed_array
                .push(Value::from_number(10.0).unwrap())
                .await
                .unwrap();

            let mut map_iterator =
                DefaultIteratorFactory::create_array_iterator(mixed_array).unwrap();
            let mapped_bools: Vec<bool> = map_iterator
                .map(|item| async move { item.as_number().unwrap_or(0.0) > 7.0 })
                .await;

            assert_eq!(mapped_bools.len(), 2);
            assert_eq!(mapped_bools[0], false);
            assert_eq!(mapped_bools[1], true);

            println!("✅ Iterator edge cases and boundaries test passed");
        })
        .await;
}

#[tokio::test]
async fn test_iterator_error_resilience() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Array con datos que pueden causar errores en el procesamiento
            let mut array_with_issues = Value::new_array();
            array_with_issues
                .push(Value::from_number(10.0).unwrap())
                .await
                .unwrap();
            array_with_issues
                .push(Value::from_str("not_a_number"))
                .await
                .unwrap();
            array_with_issues
                .push(Value::from_number(20.0).unwrap())
                .await
                .unwrap();
            array_with_issues
                .push(Value::from_bool(true))
                .await
                .unwrap();
            array_with_issues
                .push(Value::from_number(30.0).unwrap())
                .await
                .unwrap();

            // When: Filtrar solo números válidos de manera resiliente
            let mut iterator =
                DefaultIteratorFactory::create_array_iterator(array_with_issues).unwrap();

            let valid_numbers = iterator
                .filter(|item| {
                    let item_clone = item.clone();
                    async move { item_clone.as_number().is_some() }
                })
                .await;

            // Then: Solo números válidos extraídos
            assert_eq!(valid_numbers.len(), 3);
            assert_eq!(valid_numbers[0].as_number().unwrap(), 10.0);
            assert_eq!(valid_numbers[1].as_number().unwrap(), 20.0);
            assert_eq!(valid_numbers[2].as_number().unwrap(), 30.0);

            let mut array_for_map = Value::new_array();
            array_for_map
                .push(Value::from_str("123"))
                .await
                .unwrap();
            array_for_map
                .push(Value::from_str("not_numeric"))
                .await
                .unwrap();
            array_for_map
                .push(Value::from_str("456"))
                .await
                .unwrap();

            let mut map_iterator =
                DefaultIteratorFactory::create_array_iterator(array_for_map).unwrap();

            let parsed_numbers: Vec<f64> = map_iterator
                .map(|item| async move {
                    item.as_str()
                        .and_then(|s| s.parse::<f64>().ok())
                        .unwrap_or(0.0) // Valor por defecto para casos de error
                })
                .await;

            assert_eq!(parsed_numbers.len(), 3);
            assert_eq!(parsed_numbers[0], 123.0);
            assert_eq!(parsed_numbers[1], 0.0); // Error case -> default value
            assert_eq!(parsed_numbers[2], 456.0);

            println!("✅ Iterator error resilience test passed");
        })
        .await;
}

#[tokio::test]
async fn test_iterators_with_model_manager_integration() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Model manager con datos estructurados
            let mut manager = DefaultModelManager::create();

            for i in 1..=5 {
                let mut user = Value::new_object();
                user.set("id", Value::from_number(i as f64).unwrap())
                    .await
                    .unwrap();
                user.set("name", Value::from_str(&format!("User {}", i)))
                    .await
                    .unwrap();
                user.set("active", Value::from_bool(i % 2 == 0))
                    .await
                    .unwrap();

                let mut tags = Value::new_array();
                tags.push(Value::from_str("tag1")).await.unwrap();
                tags.push(Value::from_str(&format!("tag_{}", i)))
                    .await
                    .unwrap();
                user.set("tags", tags).await.unwrap();

                manager
                    .insert("users".to_string(), None, user)
                    .await
                    .unwrap();
            }

            // When: Obtener datos y usar iteradores
            let users = manager.get_all("users".to_string()).await.unwrap();

            let mut users_array = Value::new_array();
            for user in users {
                users_array.push(user).await.unwrap();
            }

            let mut array_iterator =
                DefaultIteratorFactory::create_array_iterator(users_array).unwrap();

            let active_users = array_iterator
                .filter(|user| {
                    let user_clone = user.clone();
                    async move {
                        user_clone
                            .get("active")
                            .await
                            .unwrap()
                            .unwrap()
                            .as_bool()
                            .unwrap_or(false)
                    }
                })
                .await;

            // Then: Filtrado correcto en integración
            assert_eq!(active_users.len(), 2); // Users 2 y 4

            let first_user = &active_users[0];
            let mut object_iterator =
                DefaultIteratorFactory::create_object_iterator(first_user.clone())
                    .unwrap();

            let properties = object_iterator.collect().await;
            assert!(properties.len() >= 4); // id, name, active, tags

            let prop_keys: Vec<&String> = properties.iter().map(|(k, _)| k).collect();
            assert!(prop_keys.contains(&&"id".to_string()));
            assert!(prop_keys.contains(&&"name".to_string()));
            assert!(prop_keys.contains(&&"active".to_string()));
            assert!(prop_keys.contains(&&"tags".to_string()));

            println!("✅ Iterators with model manager integration test passed");
        })
        .await;
}

#[tokio::test]
async fn test_comprehensive_iterator_benchmarks() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            println!("=== COMPREHENSIVE ITERATOR PERFORMANCE BENCHMARKS ===");

            // Benchmark 1: Array Iterator Operations
            let mut benchmark_array = Value::new_array();
            for i in 0..1000 {
                benchmark_array
                    .push(Value::from_number(i as f64).unwrap())
                    .await
                    .unwrap();
            }

            let start = std::time::Instant::now();
            let mut iter1 =
                DefaultIteratorFactory::create_array_iterator(benchmark_array.clone())
                    .unwrap();
            let count_result = iter1.count().await;
            let count_duration = start.elapsed();

            let start = std::time::Instant::now();
            let mut iter2 =
                DefaultIteratorFactory::create_array_iterator(benchmark_array.clone())
                    .unwrap();
            let collect_result = iter2.collect().await;
            let collect_duration = start.elapsed();

            let start = std::time::Instant::now();
            let mut iter3 =
                DefaultIteratorFactory::create_array_iterator(benchmark_array).unwrap();
            let filter_result = iter3
                .filter(|item| {
                    let item_clone = item.clone();
                    async move { item_clone.as_number().unwrap_or(0.0) % 10.0 == 0.0 }
                })
                .await;
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
            let mut benchmark_object = Value::new_object();
            for i in 0..500 {
                benchmark_object
                    .set(
                        &format!("property_{:03}", i),
                        Value::from_str(&format!("value_{}", i)),
                    )
                    .await
                    .unwrap();
            }

            let start = std::time::Instant::now();
            let mut obj_iter1 =
                DefaultIteratorFactory::create_object_iterator(benchmark_object.clone())
                    .unwrap();
            let obj_count_result = obj_iter1.count().await;
            let obj_count_duration = start.elapsed();

            let start = std::time::Instant::now();
            let mut obj_iter2 =
                DefaultIteratorFactory::create_object_iterator(benchmark_object)
                    .unwrap();
            let obj_filter_result = obj_iter2
                .filter(|key, _value| {
                    let key_owned = key.to_string();
                    async move { key_owned.contains("0") }
                })
                .await;
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
        })
        .await;
}

#[tokio::test]
async fn test_array_iterator_size_hint() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Array con 5 elementos
            let mut array = Value::new_array();
            for i in 0..5 {
                array
                    .push(Value::from_number(i as f64).unwrap())
                    .await
                    .unwrap();
            }

            // When: Crear iterador y verificar size_hint
            let mut iterator =
                DefaultIteratorFactory::create_array_iterator(array).unwrap();

            let initial_hint = iterator.size_hint();

            iterator.next().await;
            iterator.next().await;

            let after_consumption_hint = iterator.size_hint();

            // Then: Size hints correctos
            assert_eq!(initial_hint, (5, Some(5)));
            assert_eq!(after_consumption_hint, (3, Some(3)));
        })
        .await;
}

#[tokio::test]
async fn test_array_iterator_nth() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Array con elementos numerados
            let mut array = Value::new_array();
            for i in 0..10 {
                array
                    .push(Value::from_number(i as f64).unwrap())
                    .await
                    .unwrap();
            }

            // When: Obtener elementos específicos por posición
            let mut iterator =
                DefaultIteratorFactory::create_array_iterator(array).unwrap();

            let element_0 = iterator.nth(0).await;
            let element_2 = iterator.nth(2).await; // Debería ser el elemento 3 (saltó 1 y 2 más)
            let element_beyond = iterator.nth(100).await;

            // Then: Elementos correctos por posición
            assert!(element_0.is_some());
            assert_eq!(element_0.unwrap().as_number().unwrap(), 0.0);

            assert!(element_2.is_some());
            assert_eq!(element_2.unwrap().as_number().unwrap(), 3.0);

            assert!(element_beyond.is_none());
        })
        .await;
}

#[tokio::test]
async fn test_array_iterator_count() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Array con elementos conocidos
            let mut array = Value::new_array();
            for i in 0..7 {
                array
                    .push(Value::from_str(&format!("item_{}", i)))
                    .await
                    .unwrap();
            }

            // When: Contar elementos
            let mut iterator =
                DefaultIteratorFactory::create_array_iterator(array).unwrap();
            let count = iterator.count().await;

            // Then: Conteo correcto
            assert_eq!(count, 7);
        })
        .await;
}

#[tokio::test]
async fn test_array_iterator_collect() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Array con elementos mixed
            let mut array = Value::new_array();
            array.push(Value::from_str("text")).await.unwrap();
            array
                .push(Value::from_number(42.0).unwrap())
                .await
                .unwrap();
            array.push(Value::from_bool(true)).await.unwrap();

            // When: Collect todos los elementos
            let mut iterator =
                DefaultIteratorFactory::create_array_iterator(array).unwrap();
            let collected = iterator.collect().await;

            // Then: Todos los elementos collectados correctamente
            assert_eq!(collected.len(), 3);
            assert_eq!(collected[0].as_str().unwrap(), "text");
            assert_eq!(collected[1].as_number().unwrap(), 42.0);
            assert_eq!(collected[2].as_bool().unwrap(), true);
        })
        .await;
}

#[tokio::test]
async fn test_array_iterator_find() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Array con números
            let mut array = Value::new_array();
            for i in 1..=10 {
                array
                    .push(Value::from_number(i as f64).unwrap())
                    .await
                    .unwrap();
            }

            // When: Buscar elemento específico
            let mut iterator =
                DefaultIteratorFactory::create_array_iterator(array).unwrap();

            let found = iterator
                .find(|item| {
                    let item_clone = item.clone();
                    async move { item_clone.as_number().unwrap_or(0.0) == 7.0 }
                })
                .await;

            // Then: Elemento encontrado
            assert!(found.is_some());
            assert_eq!(found.unwrap().as_number().unwrap(), 7.0);

            let mut iterator2 = DefaultIteratorFactory::create_array_iterator({
                let mut arr = Value::new_array();
                for i in 1..=5 {
                    arr.push(Value::from_number(i as f64).unwrap())
                        .await
                        .unwrap();
                }
                arr
            })
                .unwrap();

            let not_found = iterator2
                .find(|item| {
                    let item_clone = item.clone();
                    async move { item_clone.as_number().unwrap_or(0.0) == 100.0 }
                })
                .await;

            assert!(not_found.is_none());
        })
        .await;
}

#[tokio::test]
async fn test_array_iterator_filter() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Array con números del 1 al 10
            let mut array = Value::new_array();
            for i in 1..=10 {
                array
                    .push(Value::from_number(i as f64).unwrap())
                    .await
                    .unwrap();
            }

            // When: Filtrar números pares
            let mut iterator =
                DefaultIteratorFactory::create_array_iterator(array).unwrap();

            let filtered = iterator
                .filter(|item| {
                    let item_clone = item.clone();
                    async move {
                        let num = item_clone.as_number().unwrap_or(0.0);
                        num % 2.0 == 0.0
                    }
                })
                .await;

            // Then: Solo números pares
            assert_eq!(filtered.len(), 5); // 2, 4, 6, 8, 10

            for item in &filtered {
                let num = item.as_number().unwrap();
                assert_eq!(num % 2.0, 0.0);
            }

            assert_eq!(filtered[0].as_number().unwrap(), 2.0);
            assert_eq!(filtered[1].as_number().unwrap(), 4.0);
            assert_eq!(filtered[4].as_number().unwrap(), 10.0);

            println!("✅ Async array iterator filter test passed");
        })
        .await;
}

#[tokio::test]
async fn test_array_iterator_map() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Array con strings
            let mut array = Value::new_array();
            array.push(Value::from_str("hello")).await.unwrap();
            array.push(Value::from_str("world")).await.unwrap();
            array.push(Value::from_str("rust")).await.unwrap();

            // When: Mapear a longitudes de strings
            let mut iterator =
                DefaultIteratorFactory::create_array_iterator(array).unwrap();

            let mapped: Vec<usize> = iterator
                .map(|item| async move { item.as_str().unwrap_or_default().len() })
                .await;

            // Then: Longitudes correctas
            assert_eq!(mapped.len(), 3);
            assert_eq!(mapped[0], 5); // "hello".len()
            assert_eq!(mapped[1], 5); // "world".len()
            assert_eq!(mapped[2], 4); // "rust".len()
        })
        .await;
}

#[tokio::test]
async fn test_array_iterator_for_each_batch() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Array con 10 elementos
            let mut array = Value::new_array();
            for i in 1..=10 {
                array
                    .push(Value::from_number(i as f64).unwrap())
                    .await
                    .unwrap();
            }

            // When: Procesar en lotes de 3
            let mut iterator =
                DefaultIteratorFactory::create_array_iterator(array).unwrap();

            use std::sync::{Arc, Mutex};

            let batch_sizes = Arc::new(Mutex::new(Vec::new()));
            let total_sum = Arc::new(Mutex::new(0.0));

            let batch_sizes_clone = batch_sizes.clone();
            let total_sum_clone = total_sum.clone();

            let total_processed = iterator
                .for_each_batch(3, move |batch| {
                    let batch_sizes = batch_sizes_clone.clone();
                    let total_sum = total_sum_clone.clone();
                    async move {
                        let batch_size = batch.len();
                        batch_sizes.lock().unwrap().push(batch_size);

                        let batch_sum: f64 = batch
                            .iter()
                            .map(|item| item.as_number().unwrap_or(0.0))
                            .sum();
                        *total_sum.lock().unwrap() += batch_sum;
                    }
                })
                .await;

            // Then: Procesamiento por lotes correcto
            assert_eq!(total_processed, 10);

            let final_batch_sizes = batch_sizes.lock().unwrap();
            let final_total_sum = *total_sum.lock().unwrap();

            assert_eq!(final_batch_sizes.len(), 4);
            assert_eq!(final_batch_sizes[0], 3);
            assert_eq!(final_batch_sizes[1], 3);
            assert_eq!(final_batch_sizes[2], 3);
            assert_eq!(final_batch_sizes[3], 1);

            assert_eq!(final_total_sum, 55.0);

            println!("✅ Async array iterator for_each_batch test passed");
        })
        .await;
}

#[tokio::test]
async fn test_array_iterator_empty_array() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Array vacío
            let empty_array = Value::new_array();

            // When: Crear iterador de array vacío
            let mut iterator =
                DefaultIteratorFactory::create_array_iterator(empty_array).unwrap();

            // Then: Operaciones en array vacío
            assert_eq!(iterator.size_hint(), (0, Some(0)));
            assert!(iterator.next().await.is_none());
            assert_eq!(iterator.count().await, 0);
            assert_eq!(iterator.collect().await.len(), 0);

            let mut iterator2 =
                DefaultIteratorFactory::create_array_iterator(Value::new_array())
                    .unwrap();

            let not_found = iterator2.find(|_| async { true }).await;
            assert!(not_found.is_none());

            println!("✅ Async array iterator empty array test passed");
        })
        .await;
}

#[tokio::test]
async fn test_array_iterator_large_dataset() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Array grande con 1000 elementos
            let mut large_array = Value::new_array();
            for i in 0..1000 {
                large_array
                    .push(Value::from_number(i as f64).unwrap())
                    .await
                    .unwrap();
            }

            // When: Operaciones en dataset grande
            let mut iterator =
                DefaultIteratorFactory::create_array_iterator(large_array).unwrap();

            let start = std::time::Instant::now();
            let count = iterator.count().await;
            let duration = start.elapsed();

            assert_eq!(count, 1000);
            assert!(duration.as_millis() < 100);

            // Test filtrado en dataset grande
            let mut iterator2 = DefaultIteratorFactory::create_array_iterator({
                let mut arr = Value::new_array();
                for i in 0..1000 {
                    arr.push(Value::from_number(i as f64).unwrap())
                        .await
                        .unwrap();
                }
                arr
            })
                .unwrap();

            let filtered = iterator2
                .filter(|item| {
                    let item_clone = item.clone();
                    async move {
                        let num = item_clone.as_number().unwrap_or(0.0);
                        num >= 990.0
                    }
                })
                .await;

            assert_eq!(filtered.len(), 10);

            println!(
                "✅ Async array iterator large dataset test passed in {:?}",
                duration
            );
        })
        .await;
}

#[tokio::test]
async fn test_object_iterator_basic_next() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Object con 3 propiedades
            let mut object = Value::new_object();
            object
                .set("name", Value::from_str("John"))
                .await
                .unwrap();
            object
                .set("age", Value::from_number(30.0).unwrap())
                .await
                .unwrap();
            object
                .set("active", Value::from_bool(true))
                .await
                .unwrap();

            // When: Crear iterador y obtener pares clave-valor
            let mut iterator =
                DefaultIteratorFactory::create_object_iterator(object).unwrap();

            let mut pairs = Vec::new();
            while let Some((key, value)) = iterator.next().await {
                pairs.push((key, value));
            }

            // Then: Todas las propiedades iteradas
            assert_eq!(pairs.len(), 3);

            // Verificar que todas las claves están presentes (orden puede variar en BTreeMap)
            let keys: Vec<&String> = pairs.iter().map(|(k, _)| k).collect();
            assert!(keys.contains(&&"name".to_string()));
            assert!(keys.contains(&&"age".to_string()));
            assert!(keys.contains(&&"active".to_string()));

            // Verificar valores
            for (key, value) in &pairs {
                match key.as_str() {
                    "name" => assert_eq!(value.as_str().unwrap(), "John"),
                    "age" => assert_eq!(value.as_number().unwrap(), 30.0),
                    "active" => assert_eq!(value.as_bool().unwrap(), true),
                    _ => panic!("Unexpected key: {}", key),
                }
            }
        })
        .await;
}

#[tokio::test]
async fn test_object_iterator_size_hint() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Object con 4 propiedades
            let mut object = Value::new_object();
            object
                .set("prop1", Value::from_str("value1"))
                .await
                .unwrap();
            object
                .set("prop2", Value::from_str("value2"))
                .await
                .unwrap();
            object
                .set("prop3", Value::from_str("value3"))
                .await
                .unwrap();
            object
                .set("prop4", Value::from_str("value4"))
                .await
                .unwrap();

            // When: Crear iterador y verificar size_hint
            let mut iterator =
                DefaultIteratorFactory::create_object_iterator(object).unwrap();

            let initial_hint = iterator.size_hint();

            // Consumir 2 elementos
            iterator.next().await;
            iterator.next().await;

            let after_consumption_hint = iterator.size_hint();

            // Then: Size hints correctos
            assert_eq!(initial_hint, (4, Some(4)));
            assert_eq!(after_consumption_hint, (2, Some(2)));

            println!("✅ Async object iterator size hint test passed");
        })
        .await;
}

#[tokio::test]
async fn test_object_iterator_find_key() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Object con propiedades específicas
            let mut object = Value::new_object();
            object
                .set("user_name", Value::from_str("Alice"))
                .await
                .unwrap();
            object
                .set("user_email", Value::from_str("alice@example.com"))
                .await
                .unwrap();
            object
                .set("user_score", Value::from_number(95.5).unwrap())
                .await
                .unwrap();

            // When: Buscar claves específicas
            let mut iterator =
                DefaultIteratorFactory::create_object_iterator(object).unwrap();

            let found_name = iterator.find_key("user_name").await;
            let found_score = iterator.find_key("user_score").await;
            let not_found = iterator.find_key("non_existent").await;

            // Then: Búsquedas correctas
            assert!(found_name.is_some());
            assert_eq!(found_name.unwrap().as_str().unwrap(), "Alice");

            assert!(found_score.is_some());
            assert_eq!(found_score.unwrap().as_number().unwrap(), 95.5);

            assert!(not_found.is_none());

            println!("✅ Async object iterator find_key test passed");
        })
        .await;
}

#[tokio::test]
async fn test_object_iterator_count() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Object con propiedades conocidas
            let mut object = Value::new_object();
            for i in 0..6 {
                object
                    .set(
                        &format!("field_{}", i),
                        Value::from_number(i as f64).unwrap(),
                    )
                    .await
                    .unwrap();
            }

            // When: Contar propiedades
            let mut iterator =
                DefaultIteratorFactory::create_object_iterator(object).unwrap();
            let count = iterator.count().await;

            // Then: Conteo correcto
            assert_eq!(count, 6);

            println!("✅ Async object iterator count test passed");
        })
        .await;
}

#[tokio::test]
async fn test_object_iterator_collect() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Object con tipos mixtos
            let mut object = Value::new_object();
            object
                .set("text_field", Value::from_str("hello"))
                .await
                .unwrap();
            object
                .set("number_field", Value::from_number(123.45).unwrap())
                .await
                .unwrap();
            object
                .set("bool_field", Value::from_bool(false))
                .await
                .unwrap();

            // When: Collect todos los pares clave-valor
            let mut iterator =
                DefaultIteratorFactory::create_object_iterator(object).unwrap();
            let collected = iterator.collect().await;

            // Then: Todas las propiedades collectadas
            assert_eq!(collected.len(), 3);

            let collected_map: std::collections::HashMap<String, _> =
                collected.into_iter().collect();

            assert!(collected_map.contains_key("text_field"));
            assert_eq!(collected_map["text_field"].as_str().unwrap(), "hello");

            assert!(collected_map.contains_key("number_field"));
            assert_eq!(collected_map["number_field"].as_number().unwrap(), 123.45);

            assert!(collected_map.contains_key("bool_field"));
            assert_eq!(collected_map["bool_field"].as_bool().unwrap(), false);

            println!("✅ Async object iterator collect test passed");
        })
        .await;
}

#[tokio::test]
async fn test_object_iterator_filter() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Object con propiedades numéricas y no numéricas
            let mut object = Value::new_object();
            object
                .set("score1", Value::from_number(85.0).unwrap())
                .await
                .unwrap();
            object
                .set("name", Value::from_str("Test"))
                .await
                .unwrap();
            object
                .set("score2", Value::from_number(92.0).unwrap())
                .await
                .unwrap();
            object
                .set("active", Value::from_bool(true))
                .await
                .unwrap();
            object
                .set("score3", Value::from_number(78.0).unwrap())
                .await
                .unwrap();

            // When: Filtrar solo propiedades que empiecen con "score"
            let mut iterator =
                DefaultIteratorFactory::create_object_iterator(object).unwrap();

            let filtered = iterator
                .filter(|key, _value| {
                    let key_owned = key.to_string();
                    async move { key_owned.starts_with("score") }
                })
                .await;

            // Then: Solo propiedades de score
            assert_eq!(filtered.len(), 3);

            for (key, value) in &filtered {
                assert!(key.starts_with("score"));
                assert!(value.as_number().is_some());
            }

            println!("✅ Async object iterator filter test passed");
        })
        .await;
}

#[tokio::test]
async fn test_object_iterator_map() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Object con strings
            let mut object = Value::new_object();
            object
                .set("first_name", Value::from_str("John"))
                .await
                .unwrap();
            object
                .set("last_name", Value::from_str("Doe"))
                .await
                .unwrap();
            object
                .set("city", Value::from_str("NYC"))
                .await
                .unwrap();

            // When: Mapear a longitudes de valores string
            let mut iterator =
                DefaultIteratorFactory::create_object_iterator(object).unwrap();

            let mapped: Vec<(String, usize)> = iterator
                .map(|_key, value| async move {
                    let length = value.as_str().unwrap_or_default().len();
                    length
                })
                .await;

            // Then: Mappeo correcto
            assert_eq!(mapped.len(), 3);

            let mapped_map: std::collections::HashMap<String, usize> = mapped.into_iter().collect();

            assert_eq!(mapped_map["first_name"], 4); // "John".len()
            assert_eq!(mapped_map["last_name"], 3); // "Doe".len()
            assert_eq!(mapped_map["city"], 3); // "NYC".len()

            println!("✅ Async object iterator map test passed");
        })
        .await;
}

#[tokio::test]
async fn test_object_iterator_empty_object() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Object vacío
            let empty_object = Value::new_object();

            // When: Crear iterador de object vacío
            let mut iterator =
                DefaultIteratorFactory::create_object_iterator(empty_object).unwrap();

            // Then: Operaciones en object vacío
            assert_eq!(iterator.size_hint(), (0, Some(0)));
            assert!(iterator.next().await.is_none());
            assert_eq!(iterator.count().await, 0);
            assert_eq!(iterator.collect().await.len(), 0);
            assert!(iterator.find_key("any_key").await.is_none());

            println!("✅ Async object iterator empty object test passed");
        })
        .await;
}

#[tokio::test]
async fn test_object_iterator_nested_objects() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Object con objetos anidados
            let mut nested_object = Value::new_object();
            nested_object
                .set("inner_value", Value::from_str("nested"))
                .await
                .unwrap();

            let mut main_object = Value::new_object();
            main_object
                .set("simple", Value::from_str("value"))
                .await
                .unwrap();
            main_object.set("nested", nested_object).await.unwrap();
            main_object
                .set("number", Value::from_number(42.0).unwrap())
                .await
                .unwrap();

            // When: Iterar object con contenido anidado
            let mut iterator =
                DefaultIteratorFactory::create_object_iterator(main_object).unwrap();
            let collected = iterator.collect().await;

            // Then: Propiedades correctas incluyendo anidadas
            assert_eq!(collected.len(), 3);

            let collected_map: std::collections::HashMap<String, _> =
                collected.into_iter().collect();

            assert!(collected_map.contains_key("simple"));
            assert!(collected_map.contains_key("nested"));
            assert!(collected_map.contains_key("number"));

            let nested_value = &collected_map["nested"];
            assert!(nested_value.is_object());

            println!("✅ Async object iterator nested objects test passed");
        })
        .await;
}

#[tokio::test]
async fn test_object_iterator_large_object() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Object grande con 100 propiedades
            let mut large_object = Value::new_object();
            for i in 0..100 {
                large_object
                    .set(
                        &format!("property_{:03}", i),
                        Value::from_number(i as f64).unwrap(),
                    )
                    .await
                    .unwrap();
            }

            // When: Operaciones en object grande
            let mut iterator =
                DefaultIteratorFactory::create_object_iterator(large_object).unwrap();

            let start = std::time::Instant::now();
            let count = iterator.count().await;
            let duration = start.elapsed();

            // Then: Performance aceptable y resultado correcto
            assert_eq!(count, 100);
            assert!(duration.as_millis() < 100);

            let mut iterator2 = DefaultIteratorFactory::create_object_iterator({
                let mut obj = Value::new_object();
                for i in 0..100 {
                    obj.set(
                        &format!("property_{:03}", i),
                        Value::from_number(i as f64).unwrap(),
                    )
                        .await
                        .unwrap();
                }
                obj
            })
                .unwrap();

            let filtered = iterator2
                .filter(|key, _value| {
                    let key_owned = key.to_string();
                    async move { key_owned.contains("09") }
                })
                .await;

            assert_eq!(filtered.len(), 11);

            println!(
                "✅ Async object iterator large object test passed in {:?}",
                duration
            );
        })
        .await;
}