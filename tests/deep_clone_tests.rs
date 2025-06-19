use tokio::task::LocalSet;
use model_manager::{DefaultModelManager, DefaultValue, DynamicValue, DynamicValueFactory, ModelManager, ModelManagerFactory};
use std::time::Instant;

type Value = <DefaultValue as DynamicValueFactory>::Value;

#[tokio::test]
async fn test_deep_clone_basic_object() {
    let local_set = LocalSet::new();

    local_set.run_until(async {
        // Given: Objeto básico
        let mut original = DefaultValue::create();
        original.set("name", Value::from_str("Test User")).await.unwrap();
        original.set("age", Value::from_number(25.0).unwrap()).await.unwrap();
        original.set("active", Value::from_bool(true)).await.unwrap();

        // When: Realizar deep clone
        let cloned = original.deep_clone().await.unwrap();

        // Then: Clone es idéntico pero independiente
        assert_eq!(original.get("name").await.unwrap().unwrap().as_str().unwrap(),
                   cloned.get("name").await.unwrap().unwrap().as_str().unwrap());
        assert_eq!(original.get("age").await.unwrap().unwrap().as_number().unwrap(),
                   cloned.get("age").await.unwrap().unwrap().as_number().unwrap());
        assert_eq!(original.get("active").await.unwrap().unwrap().as_bool().unwrap(),
                   cloned.get("active").await.unwrap().unwrap().as_bool().unwrap());

        println!("✅ Basic object deep clone test passed");
    }).await;
}

#[tokio::test]
async fn test_deep_clone_basic_array() {
    let local_set = LocalSet::new();

    local_set.run_until(async {
        // Given: Array básico
        let mut original = Value::new_array();
        original.push(Value::from_str("item1")).await.unwrap();
        original.push(Value::from_number(42.0).unwrap()).await.unwrap();
        original.push(Value::from_bool(true)).await.unwrap();

        // When: Realizar deep clone
        let cloned = original.deep_clone().await.unwrap();

        // Then: Clone es idéntico
        let original_array = original.as_array().await.unwrap().unwrap();
        let cloned_array = cloned.as_array().await.unwrap().unwrap();

        assert_eq!(original_array.len(), cloned_array.len());
        assert_eq!(original_array[0].as_str().unwrap(), cloned_array[0].as_str().unwrap());
        assert_eq!(original_array[1].as_number().unwrap(), cloned_array[1].as_number().unwrap());
        assert_eq!(original_array[2].as_bool().unwrap(), cloned_array[2].as_bool().unwrap());

        println!("✅ Basic array deep clone test passed");
    }).await;
}

#[tokio::test]
async fn test_deep_clone_primitive_values() {
    let local_set = LocalSet::new();

    local_set.run_until(async {
        // Given: Valores primitivos
        let string_val = Value::from_str("test string");
        let number_val = Value::from_number(123.45).unwrap();
        let bool_val = Value::from_bool(false);

        // When: Realizar deep clone
        let cloned_string = string_val.deep_clone().await.unwrap();
        let cloned_number = number_val.deep_clone().await.unwrap();
        let cloned_bool = bool_val.deep_clone().await.unwrap();

        // Then: Valores primitivos clonados correctamente
        assert_eq!(string_val.as_str().unwrap(), cloned_string.as_str().unwrap());
        assert_eq!(number_val.as_number().unwrap(), cloned_number.as_number().unwrap());
        assert_eq!(bool_val.as_bool().unwrap(), cloned_bool.as_bool().unwrap());

        println!("✅ Primitive values deep clone test passed");
    }).await;
}

#[tokio::test]
async fn test_deep_clone_nested_structure() {
    let local_set = LocalSet::new();

    local_set.run_until(async {
        // Given: Estructura anidada compleja
        let mut original = DefaultValue::create();
        original.set("title", Value::from_str("Root Document")).await.unwrap();

        let mut user = Value::new_object();
        user.set("name", Value::from_str("Ana García")).await.unwrap();
        user.set("email", Value::from_str("ana@empresa.com")).await.unwrap();

        let mut profile = Value::new_object();
        profile.set("department", Value::from_str("Engineering")).await.unwrap();
        profile.set("level", Value::from_number(8.0).unwrap()).await.unwrap();
        profile.set("remote", Value::from_bool(true)).await.unwrap();
        user.set("profile", profile).await.unwrap();

        let mut projects = Value::new_array();
        for i in 1..=5 {
            let mut project = Value::new_object();
            project.set("id", Value::from_number(i as f64).unwrap()).await.unwrap();
            project.set("name", Value::from_str(&format!("Project {}", i))).await.unwrap();
            project.set("active", Value::from_bool(i % 2 == 0)).await.unwrap();
            projects.push(project).await.unwrap();
        }
        user.set("projects", projects).await.unwrap();

        original.set("user", user).await.unwrap();

        // When: Realizar deep clone
        let start = Instant::now();
        let cloned = original.deep_clone().await.unwrap();
        let duration = start.elapsed();

        // Then: Verificar que toda la estructura fue clonada correctamente
        assert_eq!(
            original.get_by_path("title").await.unwrap().unwrap().as_str().unwrap(),
            cloned.get_by_path("title").await.unwrap().unwrap().as_str().unwrap()
        );

        assert_eq!(
            original.get_by_path("user.name").await.unwrap().unwrap().as_str().unwrap(),
            cloned.get_by_path("user.name").await.unwrap().unwrap().as_str().unwrap()
        );

        assert_eq!(
            original.get_by_path("user.profile.department").await.unwrap().unwrap().as_str().unwrap(),
            cloned.get_by_path("user.profile.department").await.unwrap().unwrap().as_str().unwrap()
        );

        assert_eq!(
            original.get_by_path("user.profile.level").await.unwrap().unwrap().as_number().unwrap(),
            cloned.get_by_path("user.profile.level").await.unwrap().unwrap().as_number().unwrap()
        );

        let original_projects = original.get("user").await.unwrap().unwrap()
            .get("projects").await.unwrap().unwrap();
        let cloned_projects = cloned.get("user").await.unwrap().unwrap()
            .get("projects").await.unwrap().unwrap();

        let original_array = original_projects.as_array().await.unwrap().unwrap();
        let cloned_array = cloned_projects.as_array().await.unwrap().unwrap();

        assert_eq!(original_array.len(), cloned_array.len());
        assert_eq!(
            original_array[0].get("name").await.unwrap().unwrap().as_str().unwrap(),
            cloned_array[0].get("name").await.unwrap().unwrap().as_str().unwrap()
        );

        println!("✅ Nested structure deep clone test passed in {:?}", duration);
    }).await;
}

#[tokio::test]
async fn test_deep_clone_independence() {
    let local_set = LocalSet::new();

    local_set.run_until(async {
        // Given: Objeto original
        let mut original = DefaultValue::create();
        original.set("shared_field", Value::from_str("original value")).await.unwrap();

        let mut nested = Value::new_object();
        nested.set("inner_field", Value::from_str("inner original")).await.unwrap();
        original.set("nested", nested).await.unwrap();

        // When: Realizar deep clone y modificar original
        let cloned = original.deep_clone().await.unwrap();

        original.set("shared_field", Value::from_str("MODIFIED")).await.unwrap();
        original.set("new_field", Value::from_str("added after clone")).await.unwrap();

        let mut modified_nested = original.get("nested").await.unwrap().unwrap();
        modified_nested.set("inner_field", Value::from_str("MODIFIED INNER")).await.unwrap();
        original.set("nested", modified_nested).await.unwrap();

        // Then: El clone no debe haber cambiado
        assert_eq!(
            cloned.get("shared_field").await.unwrap().unwrap().as_str().unwrap(),
            "original value"
        );

        assert!(cloned.get("new_field").await.unwrap().is_none());

        assert_eq!(
            cloned.get_by_path("nested.inner_field").await.unwrap().unwrap().as_str().unwrap(),
            "inner original"
        );

        assert_eq!(
            original.get("shared_field").await.unwrap().unwrap().as_str().unwrap(),
            "MODIFIED"
        );

        println!("✅ Deep clone independence test passed");
    }).await;
}

#[tokio::test]
async fn test_deep_clone_large_dataset() {
    let local_set = LocalSet::new();

    local_set.run_until(async {
        // Given: Dataset grande para probar optimización
        let mut large_object = DefaultValue::create();
        large_object.set("metadata", Value::from_str("Large Dataset Test")).await.unwrap();

        let mut records = Value::new_array();
        for i in 0..5000 {
            let mut record = Value::new_object();
            record.set("id", Value::from_number(i as f64).unwrap()).await.unwrap();
            record.set("name", Value::from_str(&format!("Record {}", i))).await.unwrap();
            record.set("description", Value::from_str(&format!("Description for record {} with additional text to make it larger", i))).await.unwrap();
            record.set("active", Value::from_bool(i % 2 == 0)).await.unwrap();
            record.set("score", Value::from_number((i * 3) as f64).unwrap()).await.unwrap();

            // Agregar sub-objeto para cada record
            let mut details = Value::new_object();
            details.set("category", Value::from_str(match i % 4 {
                0 => "A", 1 => "B", 2 => "C", _ => "D"
            })).await.unwrap();
            details.set("priority", Value::from_number((i % 10) as f64).unwrap()).await.unwrap();
            record.set("details", details).await.unwrap();

            records.push(record).await.unwrap();
        }

        large_object.set("records", records).await.unwrap();

        println!("Large dataset created with 5000 records");

        // When: Realizar deep clone de dataset grande
        let start = Instant::now();
        let cloned = large_object.deep_clone().await.unwrap();
        let duration = start.elapsed();

        // Then: Verificar performance y correctitud
        assert!(duration.as_millis() < 5000); // Debe completarse en menos de 5 segundos

        assert_eq!(
            large_object.get("metadata").await.unwrap().unwrap().as_str().unwrap(),
            cloned.get("metadata").await.unwrap().unwrap().as_str().unwrap()
        );

        let original_records = large_object.get("records").await.unwrap().unwrap();
        let cloned_records = cloned.get("records").await.unwrap().unwrap();

        let original_array = original_records.as_array().await.unwrap().unwrap();
        let cloned_array = cloned_records.as_array().await.unwrap().unwrap();

        assert_eq!(original_array.len(), cloned_array.len());
        assert_eq!(original_array.len(), 5000);

        assert_eq!(
            original_array[0].get("name").await.unwrap().unwrap().as_str().unwrap(),
            cloned_array[0].get("name").await.unwrap().unwrap().as_str().unwrap()
        );

        assert_eq!(
            original_array[2500].get("description").await.unwrap().unwrap().as_str().unwrap(),
            cloned_array[2500].get("description").await.unwrap().unwrap().as_str().unwrap()
        );

        assert_eq!(
            original_array[4999].get("details").await.unwrap().unwrap().get("category").await.unwrap().unwrap().as_str().unwrap(),
            cloned_array[4999].get("details").await.unwrap().unwrap().get("category").await.unwrap().unwrap().as_str().unwrap()
        );

        println!("✅ Large dataset deep clone test passed in {:?}", duration);
        println!("   Throughput: {:.2} records/ms", 5000.0 / duration.as_millis() as f64);
    }).await;
}

#[tokio::test]
async fn test_deep_clone_performance_threshold() {
    let local_set = LocalSet::new();

    local_set.run_until(async {
        // Given: Estructura pequeña (debe usar clone normal)
        let mut small_object = DefaultValue::create();
        small_object.set("field1", Value::from_str("value1")).await.unwrap();
        small_object.set("field2", Value::from_number(123.0).unwrap()).await.unwrap();

        // When: Clone estructura pequeña
        let start = Instant::now();
        let _small_clone = small_object.deep_clone().await.unwrap();
        let small_duration = start.elapsed();

        // Given: Estructura mediana (debe usar streaming)
        let mut medium_object = DefaultValue::create();
        let mut medium_array = Value::new_array();

        for i in 0..2000 {
            let mut item = Value::new_object();
            item.set("id", Value::from_number(i as f64).unwrap()).await.unwrap();
            // String grande para superar threshold
            item.set("large_field", Value::from_str(&"x".repeat(1000))).await.unwrap();
            medium_array.push(item).await.unwrap();
        }
        medium_object.set("data", medium_array).await.unwrap();

        // When: Clone estructura mediana
        let start = Instant::now();
        let _medium_clone = medium_object.deep_clone().await.unwrap();
        let medium_duration = start.elapsed();

        // Then: Verificar que ambos funcionan pero con tiempos diferentes
        println!("Small structure clone time: {:?}", small_duration);
        println!("Medium structure clone time: {:?}", medium_duration);

        assert!(small_duration < medium_duration);

        assert!(small_duration.as_millis() < 10);
        assert!(medium_duration.as_millis() < 3000);

        println!("✅ Performance threshold test passed");
    }).await;
}

#[tokio::test]
async fn test_deep_clone_with_model_manager() {
    let local_set = LocalSet::new();

    local_set.run_until(async {
        // Given: Model manager con datos complejos
        let mut manager = DefaultModelManager::create();

        let unique_id = format!("tech_corp_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos());

        let mut company = DefaultValue::create();
        company.set("name", Value::from_str("TechCorp")).await.unwrap();
        company.set("id", Value::from_str(&unique_id)).await.unwrap();

        let mut employees = Value::new_array();
        for i in 1..=20 { // Reducido para performance en tests masivos
            let mut employee = Value::new_object();
            employee.set("id", Value::from_number(i as f64).unwrap()).await.unwrap();
            employee.set("name", Value::from_str(&format!("Employee {}", i))).await.unwrap();
            employee.set("department", Value::from_str(match i % 3 {
                0 => "Engineering",
                1 => "Sales",
                _ => "Marketing"
            })).await.unwrap();

            let mut profile = Value::new_object();
            profile.set("level", Value::from_number((i % 10) as f64).unwrap()).await.unwrap();
            profile.set("remote", Value::from_bool(i % 2 == 0)).await.unwrap();
            employee.set("profile", profile).await.unwrap();

            employees.push(employee).await.unwrap();
        }
        company.set("employees", employees).await.unwrap();

        let inserted = manager.insert(
            "companies".to_string(),
            Some(unique_id.clone()),
            company
        ).await.unwrap();

        println!("Inserted company with ID: {}", unique_id);

        // When: Obtener y clonar
        let retrieved = manager.get(
            "companies".to_string(),
            unique_id.clone()
        ).await.unwrap();

        let start = Instant::now();
        let cloned_company = retrieved.deep_clone().await.unwrap();
        let duration = start.elapsed();

        // Then: Verificar que la clonación funciona con datos del model manager
        assert_eq!(
            retrieved.get("name").await.unwrap().unwrap().as_str().unwrap(),
            cloned_company.get("name").await.unwrap().unwrap().as_str().unwrap()
        );

        assert_eq!(
            retrieved.get("id").await.unwrap().unwrap().as_str().unwrap(),
            cloned_company.get("id").await.unwrap().unwrap().as_str().unwrap()
        );

        let retrieved_employees = retrieved.get("employees").await.unwrap().unwrap();
        let cloned_employees = cloned_company.get("employees").await.unwrap().unwrap();

        let retrieved_array = retrieved_employees.as_array().await.unwrap().unwrap();
        let cloned_array = cloned_employees.as_array().await.unwrap().unwrap();

        assert_eq!(retrieved_array.len(), cloned_array.len());
        assert_eq!(retrieved_array.len(), 20);

        assert_eq!(
            retrieved_array[10].get("name").await.unwrap().unwrap().as_str().unwrap(),
            cloned_array[10].get("name").await.unwrap().unwrap().as_str().unwrap()
        );

        assert_eq!(
            retrieved_array[19].get("profile").await.unwrap().unwrap().get("level").await.unwrap().unwrap().as_number().unwrap(),
            cloned_array[19].get("profile").await.unwrap().unwrap().get("level").await.unwrap().unwrap().as_number().unwrap()
        );

        println!("✅ Deep clone with model manager test passed in {:?}", duration);
    }).await;
}


#[tokio::test]
async fn test_deep_clone_edge_cases() {
    let local_set = LocalSet::new();

    local_set.run_until(async {
        // Test Case 1: Objeto vacío
        let empty_object = DefaultValue::create();
        let cloned_empty = empty_object.deep_clone().await.unwrap();
        assert!(cloned_empty.is_object());
        assert!(cloned_empty.is_empty());

        // Test Case 2: Array vacío
        let empty_array = Value::new_array();
        let cloned_empty_array = empty_array.deep_clone().await.unwrap();
        assert!(cloned_empty_array.is_array());
        assert!(cloned_empty_array.is_empty());

        // Test Case 3: Estructura muy anidada (20 niveles)
        let mut deeply_nested = DefaultValue::create();
        let mut current = Value::new_object();
        current.set("value", Value::from_str("deep")).await.unwrap();

        for i in (1..=20).rev() {
            let mut parent = Value::new_object();
            parent.set(&format!("level{}", i), current).await.unwrap();
            current = parent;
        }
        deeply_nested.set("root", current).await.unwrap();

        let cloned_deep = deeply_nested.deep_clone().await.unwrap();

        let deep_path = (1..=20).map(|i| format!("level{}", i)).collect::<Vec<_>>().join(".");
        let full_path = format!("root.{}.value", deep_path);

        assert_eq!(
            deeply_nested.get_by_path(&full_path).await.unwrap().unwrap().as_str().unwrap(),
            cloned_deep.get_by_path(&full_path).await.unwrap().unwrap().as_str().unwrap()
        );

        // Test Case 4: Array con tipos mixtos
        let mut mixed_array = Value::new_array();
        mixed_array.push(Value::from_str("string")).await.unwrap();
        mixed_array.push(Value::from_number(42.0).unwrap()).await.unwrap();
        mixed_array.push(Value::from_bool(true)).await.unwrap();

        let mut nested_obj = Value::new_object();
        nested_obj.set("nested", Value::from_str("value")).await.unwrap();
        mixed_array.push(nested_obj).await.unwrap();

        let cloned_mixed = mixed_array.deep_clone().await.unwrap();
        let mixed_clone_array = cloned_mixed.as_array().await.unwrap().unwrap();

        assert_eq!(mixed_clone_array.len(), 4);
        assert_eq!(mixed_clone_array[0].as_str().unwrap(), "string");
        assert_eq!(mixed_clone_array[1].as_number().unwrap(), 42.0);
        assert_eq!(mixed_clone_array[2].as_bool().unwrap(), true);
        assert_eq!(
            mixed_clone_array[3].get("nested").await.unwrap().unwrap().as_str().unwrap(),
            "value"
        );

        println!("✅ Deep clone edge cases test passed");
    }).await;
}

#[tokio::test]
async fn test_deep_clone_memory_efficiency() {
    let local_set = LocalSet::new();

    local_set.run_until(async {
        // Given: Crear estructura que simule uso intensivo de memoria
        let mut memory_test = DefaultValue::create();
        memory_test.set("metadata", Value::from_str("Memory efficiency test")).await.unwrap();

        for array_idx in 0..5 {
            let mut large_array = Value::new_array();

            for i in 0..1000 {
                let mut item = Value::new_object();
                item.set("array_id", Value::from_number(array_idx as f64).unwrap()).await.unwrap();
                item.set("item_id", Value::from_number(i as f64).unwrap()).await.unwrap();
                item.set("data", Value::from_str(&format!("Data for array {} item {}", array_idx, i))).await.unwrap();

                // Sub-estructura para cada item
                let mut sub_data = Value::new_object();
                sub_data.set("timestamp", Value::from_str("2025-01-15T10:00:00Z")).await.unwrap();
                sub_data.set("processed", Value::from_bool(i % 2 == 0)).await.unwrap();
                item.set("metadata", sub_data).await.unwrap();

                large_array.push(item).await.unwrap();
            }

            memory_test.set(&format!("array_{}", array_idx), large_array).await.unwrap();
        }

        println!("Memory test structure created: 5 arrays × 1000 items = 5000 total items");

        // When: Realizar deep clone con monitoreo de tiempo
        let start = Instant::now();
        let cloned = memory_test.deep_clone().await.unwrap();
        let duration = start.elapsed();

        // Then: Verificar eficiencia y correctitud
        println!("Memory efficiency test completed in: {:?}", duration);
        assert!(duration.as_millis() < 10000); // Max 10 segundos

        for array_idx in 0..5 {
            let original_array = memory_test.get(&format!("array_{}", array_idx)).await.unwrap().unwrap();
            let cloned_array = cloned.get(&format!("array_{}", array_idx)).await.unwrap().unwrap();

            let orig_arr = original_array.as_array().await.unwrap().unwrap();
            let cloned_arr = cloned_array.as_array().await.unwrap().unwrap();

            assert_eq!(orig_arr.len(), cloned_arr.len());
            assert_eq!(orig_arr.len(), 1000);

            let test_indices = [0, 250, 500, 750, 999];
            for &idx in &test_indices {
                assert_eq!(
                    orig_arr[idx].get("item_id").await.unwrap().unwrap().as_number().unwrap(),
                    cloned_arr[idx].get("item_id").await.unwrap().unwrap().as_number().unwrap()
                );
            }
        }

        println!("✅ Memory efficiency test passed - 5000 items cloned successfully");
    }).await;
}

#[tokio::test]
async fn test_deep_clone_concurrent_safety() {
    let local_set = LocalSet::new();

    local_set.run_until(async {
        // Given: Estructura que será clonada múltiples veces concurrentemente
        let mut shared_data = DefaultValue::create();
        shared_data.set("shared_field", Value::from_str("shared value")).await.unwrap();

        let mut array_data = Value::new_array();
        for i in 0..100 {
            let mut item = Value::new_object();
            item.set("id", Value::from_number(i as f64).unwrap()).await.unwrap();
            item.set("value", Value::from_str(&format!("value {}", i))).await.unwrap();
            array_data.push(item).await.unwrap();
        }
        shared_data.set("array", array_data).await.unwrap();

        // When: Realizar múltiples clones concurrentemente
        let clone_tasks = (0..10).map(|task_id| {
            let data = shared_data.clone();
            async move {
                let start = Instant::now();
                let cloned = data.deep_clone().await.unwrap();
                let duration = start.elapsed();

                let shared_value = cloned.get("shared_field").await.unwrap().unwrap().as_str().unwrap();
                let array = cloned.get("array").await.unwrap().unwrap();
                let array_items = array.as_array().await.unwrap().unwrap();

                (task_id, duration, shared_value == "shared value", array_items.len() == 100)
            }
        });

        let results = futures::future::join_all(clone_tasks).await;

        // Then: Todos los clones deben ser exitosos
        for (task_id, duration, shared_correct, array_correct) in results {
            assert!(shared_correct, "Task {} failed shared field verification", task_id);
            assert!(array_correct, "Task {} failed array verification", task_id);
            assert!(duration.as_millis() < 1000, "Task {} took too long: {:?}", task_id, duration);
            println!("Task {} completed in {:?}", task_id, duration);
        }

        println!("✅ Concurrent safety test passed - 10 concurrent clones successful");
    }).await;
}

