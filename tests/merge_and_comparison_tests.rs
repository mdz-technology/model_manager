use std::time::Instant;
use model_manager::{DefaultValue, DynamicValue, ModelManager, ModelManagerFactory, DefaultModelManager, DynamicValueFactory};

type Value = <DefaultValue as DynamicValueFactory>::Value;

#[test]
fn test_merge_basic_objects() {

        // Given: Dos objetos para hacer merge
        let mut base_config = Value::new_object();
        base_config.set("app_name", Value::from_str("MyApp")).unwrap();
        base_config.set("version", Value::from_str("1.0.0")).unwrap();
        base_config.set("debug", Value::from_bool(false)).unwrap();

        let mut override_config = Value::new_object();
        override_config.set("version", Value::from_str("1.1.0")).unwrap(); // Override
        override_config.set("debug", Value::from_bool(true)).unwrap(); // Override
        override_config.set("api_key", Value::from_str("secret123")).unwrap(); // New

        println!("Base config: {}", base_config.to_string());
        println!("Override config: {}", override_config.to_string());

        // When: Hacer merge
        base_config.merge(&override_config).unwrap();

        // Then: Configuración merged correctamente
        assert_eq!(base_config.get("app_name").unwrap().unwrap().as_str().unwrap(), "MyApp");
        assert_eq!(base_config.get("version").unwrap().unwrap().as_str().unwrap(), "1.1.0");
        assert_eq!(base_config.get("debug").unwrap().unwrap().as_bool().unwrap(), true);
        assert_eq!(base_config.get("api_key").unwrap().unwrap().as_str().unwrap(), "secret123");

        println!("✅ Merged config: {}", base_config.to_string());
}

#[test]
fn test_merge_nested_objects() {


        // Given: Objetos anidados para merge
        let mut global_context = Value::new_object();
        global_context.set("environment", Value::from_str("production")).unwrap();

        let mut global_database = Value::new_object();
        global_database.set("host", Value::from_str("localhost")).unwrap();
        global_database.set("port", Value::from_number(5432.0).unwrap()).unwrap();
        global_database.set("ssl", Value::from_bool(false)).unwrap();
        global_context.set("database", global_database).unwrap();

        let mut global_cache = Value::new_object();
        global_cache.set("enabled", Value::from_bool(true)).unwrap();
        global_cache.set("ttl", Value::from_number(3600.0).unwrap()).unwrap();
        global_context.set("cache", global_cache).unwrap();

        let mut specific_context = Value::new_object();
        specific_context.set("environment", Value::from_str("development")).unwrap();

        let mut specific_database = Value::new_object();
        specific_database.set("host", Value::from_str("dev-db.internal")).unwrap();
        specific_database.set("ssl", Value::from_bool(true)).unwrap();
        specific_context.set("database", specific_database).unwrap();

        let mut specific_logging = Value::new_object();
        specific_logging.set("level", Value::from_str("debug")).unwrap();
        specific_logging.set("file", Value::from_str("/var/log/app.log")).unwrap();
        specific_context.set("logging", specific_logging).unwrap(); // Nueva sección

        println!("BEFORE MERGE:");
        println!("Global context: {}", global_context.to_string());
        println!("Specific context: {}", specific_context.to_string());

        // When: Merge nested
        global_context.merge(&specific_context).unwrap();

        // Then: Merge anidado correcto
        assert_eq!(global_context.get_by_path("environment").unwrap().unwrap().as_str().unwrap(), "development");
        assert_eq!(global_context.get_by_path("database.host").unwrap().unwrap().as_str().unwrap(), "dev-db.internal");
        assert_eq!(global_context.get_by_path("database.port").unwrap().unwrap().as_number().unwrap(), 5432.0); // Preserved
        assert_eq!(global_context.get_by_path("database.ssl").unwrap().unwrap().as_bool().unwrap(), true); // Overridden
        assert_eq!(global_context.get_by_path("cache.enabled").unwrap().unwrap().as_bool().unwrap(), true); // Preserved
        assert_eq!(global_context.get_by_path("logging.level").unwrap().unwrap().as_str().unwrap(), "debug"); // New

        println!("✅ AFTER MERGE: {}", global_context.to_string());
}

#[test]
fn test_merge_arrays() {


        // Given: Arrays para concatenar
        let mut base_features = Value::new_array();
        base_features.push(Value::from_str("authentication")).unwrap();
        base_features.push(Value::from_str("authorization")).unwrap();
        base_features.push(Value::from_str("logging")).unwrap();

        let mut additional_features = Value::new_array();
        additional_features.push(Value::from_str("caching")).unwrap();
        additional_features.push(Value::from_str("monitoring")).unwrap();

        println!("Base features: {}", base_features.to_string());
        println!("Additional features: {}", additional_features.to_string());

        // When: Merge arrays
        base_features.merge(&additional_features).unwrap();

        // Then: Arrays concatenados
        let merged_array = base_features.as_array().unwrap().unwrap();
        assert_eq!(merged_array.len(), 5);
        assert_eq!(merged_array[0].as_str().unwrap(), "authentication");
        assert_eq!(merged_array[3].as_str().unwrap(), "caching");
        assert_eq!(merged_array[4].as_str().unwrap(), "monitoring");

        println!("✅ Merged features: {}", base_features.to_string());
}

#[test]
fn test_calculate_hash_consistency() {


        // Given: Objetos idénticos
        let mut object1 = Value::new_object();
        object1.set("name", Value::from_str("Test")).unwrap();
        object1.set("value", Value::from_number(42.0).unwrap()).unwrap();
        object1.set("active", Value::from_bool(true)).unwrap();

        let mut object2 = Value::new_object();
        object2.set("name", Value::from_str("Test")).unwrap();
        object2.set("value", Value::from_number(42.0).unwrap()).unwrap();
        object2.set("active", Value::from_bool(true)).unwrap();

        // When: Calcular hashes
        let hash1 = object1.calculate_hash().unwrap();
        let hash2 = object2.calculate_hash().unwrap();

        // Then: Hashes idénticos
        assert_eq!(hash1, hash2);
        println!("✅ Consistent hash: {} = {}", hash1, hash2);

        // When: Modificar uno
        object2.set("active", Value::from_bool(false)).unwrap();
        let hash3 = object2.calculate_hash().unwrap();

        // Then: Hashes diferentes
        assert_ne!(hash1, hash3);
        println!("✅ Different hash after modification: {} != {}", hash1, hash3);
}

#[test]
fn test_calculate_hash_performance() {


        // Given: Objeto grande
        let mut large_object = Value::new_object();
        large_object.set("metadata", Value::from_str("Large object test")).unwrap();

        let mut large_array = Value::new_array();
        for i in 0..10000 {
            let mut item = Value::new_object();
            item.set("id", Value::from_number(i as f64).unwrap()).unwrap();
            item.set("name", Value::from_str(&format!("Item {}", i))).unwrap();
            item.set("description", Value::from_str(&format!("Description for item {} with some extra content", i))).unwrap();
            large_array.push(item).unwrap();
        }
        large_object.set("items", large_array).unwrap();

        println!("Large object created with 10,000 items");

        // When: Calcular hash de objeto grande
        let start = Instant::now();
        let hash = large_object.calculate_hash().unwrap();
        let duration = start.elapsed();

        // Then: Performance aceptable
        assert!(duration.as_millis() < 1000); // Debe completarse en menos de 1 segundo
        assert_ne!(hash, 0); // Hash válido

        println!("✅ Large object hash calculated in {:?}: {}", duration, hash);

        // When: Calcular hash nuevamente
        let start2 = Instant::now();
        let hash2 = large_object.calculate_hash().unwrap();
        let duration2 = start2.elapsed();

        // Then: Resultado consistente
        assert_eq!(hash, hash2);
        println!("✅ Second hash calculation in {:?}: {} (consistent)", duration2, hash2);
}

#[test]
fn test_equals_basic() {


        // Given: Objetos para comparación
        let mut obj1 = Value::new_object();
        obj1.set("name", Value::from_str("Alice")).unwrap();
        obj1.set("age", Value::from_number(30.0).unwrap()).unwrap();

        let mut obj2 = Value::new_object();
        obj2.set("name", Value::from_str("Alice")).unwrap();
        obj2.set("age", Value::from_number(30.0).unwrap()).unwrap();

        let mut obj3 = Value::new_object();
        obj3.set("name", Value::from_str("Bob")).unwrap();
        obj3.set("age", Value::from_number(25.0).unwrap()).unwrap();

        // When: Comparar objetos
        let equal_result = obj1.equals(&obj2).unwrap();
        let different_result = obj1.equals(&obj3).unwrap();

        // Then: Comparaciones correctas
        assert!(equal_result);
        assert!(!different_result);

        println!("✅ Equal objects: {}", equal_result);
        println!("✅ Different objects: {}", different_result);
}

#[test]
fn test_equals_performance_large_data() {


        // Given: Dos objetos grandes idénticos
        let mut obj1 = Value::new_object();
        let mut obj2 = Value::new_object();

        for i in 0..5000 {
            let value = Value::from_str(&format!("value_{}", i));
            obj1.set(&format!("key_{}", i), value.clone()).unwrap();
            obj2.set(&format!("key_{}", i), value).unwrap();
        }

        println!("Large objects created with 5,000 properties each");

        // When: Comparar objetos grandes usando hash optimization
        let start = Instant::now();
        let are_equal = obj1.equals(&obj2).unwrap();
        let duration = start.elapsed();

        // Then: Performance eficiente y resultado correcto
        assert!(are_equal);
        assert!(duration.as_millis() < 500); // Debe ser rápido gracias al hash

        println!("✅ Large objects comparison in {:?}: equal = {}", duration, are_equal);

        // When: Modificar ligeramente y comparar
        obj2.set("key_2500", Value::from_str("modified_value")).unwrap();

        let start2 = Instant::now();
        let are_different = obj1.equals(&obj2).unwrap();
        let duration2 = start2.elapsed();

        // Then: Detecta diferencia eficientemente
        assert!(!are_different);
        assert!(duration2.as_millis() < 200); // Hash debería detectar diferencia rápido

        println!("✅ Modified objects comparison in {:?}: equal = {}", duration2, are_different);
}

#[test]
fn test_context_merge_scenario() {


        // Given: Escenario empresarial realista con múltiples contextos
        let mut global_context = Value::new_object();
        global_context.set("company", Value::from_str("TechCorp Inc")).unwrap();
        global_context.set("environment", Value::from_str("production")).unwrap();

        let mut global_config = Value::new_object();
        global_config.set("timeout", Value::from_number(30000.0).unwrap()).unwrap();
        global_config.set("retry_attempts", Value::from_number(3.0).unwrap()).unwrap();
        global_config.set("debug_mode", Value::from_bool(false)).unwrap();
        global_context.set("config", global_config).unwrap();

        let mut global_endpoints = Value::new_array();
        global_endpoints.push(Value::from_str("https://api.techcorp.com/v1")).unwrap();
        global_endpoints.push(Value::from_str("https://backup-api.techcorp.com/v1")).unwrap();
        global_context.set("endpoints", global_endpoints).unwrap();

        // Regional context
        let mut regional_context = Value::new_object();
        regional_context.set("region", Value::from_str("us-west")).unwrap();

        let mut regional_config = Value::new_object();
        regional_config.set("timeout", Value::from_number(45000.0).unwrap()).unwrap(); // Override
        regional_config.set("currency", Value::from_str("USD")).unwrap(); // New
        regional_context.set("config", regional_config).unwrap();

        let mut regional_endpoints = Value::new_array();
        regional_endpoints.push(Value::from_str("https://us-west-api.techcorp.com/v1")).unwrap();
        regional_context.set("endpoints", regional_endpoints).unwrap();

        // User-specific context
        let mut user_context = Value::new_object();
        user_context.set("user_id", Value::from_str("user_12345")).unwrap();

        let mut user_config = Value::new_object();
        user_config.set("debug_mode", Value::from_bool(true)).unwrap(); // User override
        user_config.set("language", Value::from_str("en-US")).unwrap(); // New
        user_context.set("config", user_config).unwrap();

        println!("=== CONTEXT MERGING SCENARIO ===");
        println!("Global context: {}", global_context.to_string());
        println!("Regional context: {}", regional_context.to_string());
        println!("User context: {}", user_context.to_string());

        // When: Merge contexts in priority order (global -> regional -> user)
        let mut final_context = global_context.deep_clone().unwrap();
        final_context.merge(&regional_context).unwrap();
        final_context.merge(&user_context).unwrap();

        // Then: Verificar merge correcto con prioridades
        assert_eq!(final_context.get_by_path("company").unwrap().unwrap().as_str().unwrap(), "TechCorp Inc");
        assert_eq!(final_context.get_by_path("environment").unwrap().unwrap().as_str().unwrap(), "production");
        assert_eq!(final_context.get_by_path("region").unwrap().unwrap().as_str().unwrap(), "us-west");
        assert_eq!(final_context.get_by_path("user_id").unwrap().unwrap().as_str().unwrap(), "user_12345");

        // Config merging
        assert_eq!(final_context.get_by_path("config.timeout").unwrap().unwrap().as_number().unwrap(), 45000.0); // Regional override
        assert_eq!(final_context.get_by_path("config.retry_attempts").unwrap().unwrap().as_number().unwrap(), 3.0); // Global preserved
        assert_eq!(final_context.get_by_path("config.debug_mode").unwrap().unwrap().as_bool().unwrap(), true); // User override
        assert_eq!(final_context.get_by_path("config.currency").unwrap().unwrap().as_str().unwrap(), "USD"); // Regional new
        assert_eq!(final_context.get_by_path("config.language").unwrap().unwrap().as_str().unwrap(), "en-US"); // User new

        let final_endpoints = final_context.get("endpoints").unwrap().unwrap();
        let endpoints_array = final_endpoints.as_array().unwrap().unwrap();

        println!("DEBUG - Final endpoints count: {}", endpoints_array.len());
        for (i, endpoint) in endpoints_array.iter().enumerate() {
            println!("  Endpoint {}: {}", i, endpoint.as_str().unwrap_or(String::from("INVALID")));
        }

        assert_eq!(endpoints_array.len(), 3); // 2 global + 1 regional

        // Verificar contenido específico
        let endpoint_strings: Vec<String> = endpoints_array.iter()
            .map(|e| e.as_str().unwrap_or(String::from("")).to_string())
            .collect();

        assert!(endpoint_strings.contains(&"https://api.techcorp.com/v1".to_string()));
        assert!(endpoint_strings.contains(&"https://backup-api.techcorp.com/v1".to_string()));
        assert!(endpoint_strings.contains(&"https://us-west-api.techcorp.com/v1".to_string()));

        println!("✅ FINAL MERGED CONTEXT: {}", final_context.to_string());
}

#[test]
fn test_model_manager_with_merge_and_comparison() {


        // Given: Model manager con datos para merge y comparación
        let mut manager = DefaultModelManager::create();

        // Template base
        let mut base_template = Value::new_object();
        base_template.set("name", Value::from_str("Base Template")).unwrap();
        base_template.set("version", Value::from_str("1.0.0")).unwrap();

        let mut base_styles = Value::new_object();
        base_styles.set("color", Value::from_str("#000000")).unwrap();
        base_styles.set("font_size", Value::from_number(14.0).unwrap()).unwrap();
        base_template.set("styles", base_styles).unwrap();

        manager.insert(
            "templates".to_string(),
            Some("base_template".to_string()),
            base_template
        ).unwrap();

        // Template override
        let mut override_template = Value::new_object();
        override_template.set("version", Value::from_str("1.1.0")).unwrap();

        let mut override_styles = Value::new_object();
        override_styles.set("color", Value::from_str("#FF0000")).unwrap();
        override_styles.set("background", Value::from_str("#FFFFFF")).unwrap();
        override_template.set("styles", override_styles).unwrap();

        manager.insert(
            "templates".to_string(),
            Some("override_template".to_string()),
            override_template
        ).unwrap();

        // When: Recuperar y hacer merge
        let mut base = manager.get("templates".to_string(), "base_template".to_string()).unwrap();
        let override_data = manager.get("templates".to_string(), "override_template".to_string()).unwrap();

        println!("Base before merge: {}", base.to_string());
        println!("Override data: {}", override_data.to_string());

        base.merge(&override_data).unwrap();

        // Then: Template merged correctamente
        assert_eq!(base.get_by_path("name").unwrap().unwrap().as_str().unwrap(), "Base Template");
        assert_eq!(base.get_by_path("version").unwrap().unwrap().as_str().unwrap(), "1.1.0");
        assert_eq!(base.get_by_path("styles.color").unwrap().unwrap().as_str().unwrap(), "#FF0000");
        assert_eq!(base.get_by_path("styles.font_size").unwrap().unwrap().as_number().unwrap(), 14.0);
        assert_eq!(base.get_by_path("styles.background").unwrap().unwrap().as_str().unwrap(), "#FFFFFF");

        // When: Guardar template merged
        manager.update(
            "templates".to_string(),
            "base_template".to_string(),
            base.clone()
        ).unwrap();

        // When: Comparar con original
        let original = manager.get("templates".to_string(), "override_template".to_string()).unwrap();
        let merged = manager.get("templates".to_string(), "base_template".to_string()).unwrap();

        let are_equal = merged.equals(&original).unwrap();
        let merged_hash = merged.calculate_hash().unwrap();
        let original_hash = original.calculate_hash().unwrap();

        // Then: Templates son diferentes
        assert!(!are_equal);
        assert_ne!(merged_hash, original_hash);

        println!("✅ Merged template: {}", merged.to_string());
        println!("✅ Templates are different: equal = {}, hash1 = {}, hash2 = {}",
                 are_equal, merged_hash, original_hash);
}

#[test]
fn test_edge_cases_merge_and_comparison() {


        // Test Case 1: Merge con objetos vacíos
        let mut empty1 = Value::new_object();
        let empty2 = Value::new_object();

        empty1.merge(&empty2).unwrap();
        assert!(empty1.is_empty());

        // Test Case 2: Merge primitivo sobre objeto
        let mut object = Value::new_object();
        object.set("key", Value::from_str("value")).unwrap();

        let primitive = Value::from_str("replacement");
        object.merge(&primitive).unwrap();
        assert_eq!(object.as_str().unwrap(), "replacement");

        // Test Case 3: Hash de valores null
        let null_val = Value::from_str("null");
        let hash = null_val.calculate_hash().unwrap();
        assert_ne!(hash, 0);

        // Test Case 4: Equals con tipos diferentes
        let string_val = Value::from_str("123");
        let number_val = Value::from_number(123.0).unwrap();
        let are_equal = string_val.equals(&number_val).unwrap();
        assert!(!are_equal);

        // Test Case 5: Arrays vacíos
        let mut empty_arr1 = Value::new_array();
        let empty_arr2 = Value::new_array();

        empty_arr1.merge(&empty_arr2).unwrap();
        assert!(empty_arr1.is_empty());

        let are_equal = empty_arr1.equals(&empty_arr2).unwrap();
        assert!(are_equal);

        println!("✅ All edge cases passed");
}