use dynamic_value::{CoreValue, DefaultModelManagerFactory, DefaultValueFactory, ModelManager, ModelManagerFactory, PathNavigation, ValueAnalysis, ValueFactory};
use std::time::Instant;

#[test]
fn test_merge_basic_objects() {
    // Given: Dos objetos para hacer merge
    let mut base_config = DefaultValueFactory::create_object();
    base_config
        .set("app_name", DefaultValueFactory::create_string("MyApp"))
        .unwrap();
    base_config
        .set("version", DefaultValueFactory::create_string("1.0.0"))
        .unwrap();
    base_config
        .set("debug", DefaultValueFactory::create_bool(false))
        .unwrap();

    let mut override_config = DefaultValueFactory::create_object();
    override_config
        .set("version", DefaultValueFactory::create_string("1.1.0"))
        .unwrap(); // Override
    override_config
        .set("debug", DefaultValueFactory::create_bool(true))
        .unwrap(); // Override
    override_config
        .set("api_key", DefaultValueFactory::create_string("secret123"))
        .unwrap(); // New

    println!("Base config: {}", base_config.to_string());
    println!("Override config: {}", override_config.to_string());

    // When: Hacer merge
    base_config.merge(&override_config).unwrap();

    // Then: Configuración merged correctamente
    assert_eq!(
        base_config
            .get("app_name")
            .unwrap()
            .unwrap()
            .as_str()
            .unwrap(),
        "MyApp"
    );
    assert_eq!(
        base_config
            .get("version")
            .unwrap()
            .unwrap()
            .as_str()
            .unwrap(),
        "1.1.0"
    );
    assert_eq!(
        base_config
            .get("debug")
            .unwrap()
            .unwrap()
            .as_bool()
            .unwrap(),
        true
    );
    assert_eq!(
        base_config
            .get("api_key")
            .unwrap()
            .unwrap()
            .as_str()
            .unwrap(),
        "secret123"
    );

    println!("✅ Merged config: {}", base_config.to_string());
}

#[test]
fn test_merge_nested_objects() {
    // Given: Objetos anidados para merge
    let mut global_context = DefaultValueFactory::create_object();
    global_context
        .set(
            "environment",
            DefaultValueFactory::create_string("production"),
        )
        .unwrap();

    let mut global_database = DefaultValueFactory::create_object();
    global_database
        .set("host", DefaultValueFactory::create_string("localhost"))
        .unwrap();
    global_database
        .set("port", DefaultValueFactory::create_number(5432.0).unwrap())
        .unwrap();
    global_database
        .set("ssl", DefaultValueFactory::create_bool(false))
        .unwrap();
    global_context.set("database", global_database).unwrap();

    let mut global_cache = DefaultValueFactory::create_object();
    global_cache
        .set("enabled", DefaultValueFactory::create_bool(true))
        .unwrap();
    global_cache
        .set("ttl", DefaultValueFactory::create_number(3600.0).unwrap())
        .unwrap();
    global_context.set("cache", global_cache).unwrap();

    let mut specific_context = DefaultValueFactory::create_object();
    specific_context
        .set(
            "environment",
            DefaultValueFactory::create_string("development"),
        )
        .unwrap();

    let mut specific_database = DefaultValueFactory::create_object();
    specific_database
        .set(
            "host",
            DefaultValueFactory::create_string("dev-db.internal"),
        )
        .unwrap();
    specific_database
        .set("ssl", DefaultValueFactory::create_bool(true))
        .unwrap();
    specific_context.set("database", specific_database).unwrap();

    let mut specific_logging = DefaultValueFactory::create_object();
    specific_logging
        .set("level", DefaultValueFactory::create_string("debug"))
        .unwrap();
    specific_logging
        .set(
            "file",
            DefaultValueFactory::create_string("/var/log/app.log"),
        )
        .unwrap();
    specific_context.set("logging", specific_logging).unwrap(); // Nueva sección

    println!("BEFORE MERGE:");
    println!("Global context: {}", global_context.to_string());
    println!("Specific context: {}", specific_context.to_string());

    // When: Merge nested
    global_context.merge(&specific_context).unwrap();

    // Then: Merge anidado correcto
    assert_eq!(
        global_context
            .get_by_path("environment")
            .unwrap()
            .unwrap()
            .as_str()
            .unwrap(),
        "development"
    );
    assert_eq!(
        global_context
            .get_by_path("database.host")
            .unwrap()
            .unwrap()
            .as_str()
            .unwrap(),
        "dev-db.internal"
    );
    assert_eq!(
        global_context
            .get_by_path("database.port")
            .unwrap()
            .unwrap()
            .as_number()
            .unwrap(),
        5432.0
    ); // Preserved
    assert_eq!(
        global_context
            .get_by_path("database.ssl")
            .unwrap()
            .unwrap()
            .as_bool()
            .unwrap(),
        true
    ); // Overridden
    assert_eq!(
        global_context
            .get_by_path("cache.enabled")
            .unwrap()
            .unwrap()
            .as_bool()
            .unwrap(),
        true
    ); // Preserved
    assert_eq!(
        global_context
            .get_by_path("logging.level")
            .unwrap()
            .unwrap()
            .as_str()
            .unwrap(),
        "debug"
    ); // New

    println!("✅ AFTER MERGE: {}", global_context.to_string());
}

#[test]
fn test_merge_arrays() {
    // Given: Arrays para concatenar
    let mut base_features = DefaultValueFactory::create_array();
    base_features
        .push(DefaultValueFactory::create_string("authentication"))
        .unwrap();
    base_features
        .push(DefaultValueFactory::create_string("authorization"))
        .unwrap();
    base_features
        .push(DefaultValueFactory::create_string("logging"))
        .unwrap();

    let mut additional_features = DefaultValueFactory::create_array();
    additional_features
        .push(DefaultValueFactory::create_string("caching"))
        .unwrap();
    additional_features
        .push(DefaultValueFactory::create_string("monitoring"))
        .unwrap();

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
    let mut object1 = DefaultValueFactory::create_object();
    object1
        .set("name", DefaultValueFactory::create_string("Test"))
        .unwrap();
    object1
        .set("value", DefaultValueFactory::create_number(42.0).unwrap())
        .unwrap();
    object1
        .set("active", DefaultValueFactory::create_bool(true))
        .unwrap();

    let mut object2 = DefaultValueFactory::create_object();
    object2
        .set("name", DefaultValueFactory::create_string("Test"))
        .unwrap();
    object2
        .set("value", DefaultValueFactory::create_number(42.0).unwrap())
        .unwrap();
    object2
        .set("active", DefaultValueFactory::create_bool(true))
        .unwrap();

    // When: Calcular hashes
    let hash1 = object1.calculate_hash().unwrap();
    let hash2 = object2.calculate_hash().unwrap();

    // Then: Hashes idénticos
    assert_eq!(hash1, hash2);
    println!("✅ Consistent hash: {} = {}", hash1, hash2);

    // When: Modificar uno
    object2
        .set("active", DefaultValueFactory::create_bool(false))
        .unwrap();
    let hash3 = object2.calculate_hash().unwrap();

    // Then: Hashes diferentes
    assert_ne!(hash1, hash3);
    println!(
        "✅ Different hash after modification: {} != {}",
        hash1, hash3
    );
}

#[test]
fn test_calculate_hash_performance() {
    // Given: Objeto grande
    let mut large_object = DefaultValueFactory::create_object();
    large_object
        .set(
            "metadata",
            DefaultValueFactory::create_string("Large object test"),
        )
        .unwrap();

    let mut large_array = DefaultValueFactory::create_array();
    for i in 0..10000 {
        let mut item = DefaultValueFactory::create_object();
        item.set("id", DefaultValueFactory::create_number(i as f64).unwrap())
            .unwrap();
        item.set(
            "name",
            DefaultValueFactory::create_string(&format!("Item {}", i)),
        )
        .unwrap();
        item.set(
            "description",
            DefaultValueFactory::create_string(&format!(
                "Description for item {} with some extra content",
                i
            )),
        )
        .unwrap();
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

    println!(
        "✅ Large object hash calculated in {:?}: {}",
        duration, hash
    );

    // When: Calcular hash nuevamente
    let start2 = Instant::now();
    let hash2 = large_object.calculate_hash().unwrap();
    let duration2 = start2.elapsed();

    // Then: Resultado consistente
    assert_eq!(hash, hash2);
    println!(
        "✅ Second hash calculation in {:?}: {} (consistent)",
        duration2, hash2
    );
}

#[test]
fn test_equals_basic() {
    // Given: Objetos para comparación
    let mut obj1 = DefaultValueFactory::create_object();
    obj1.set("name", DefaultValueFactory::create_string("Alice"))
        .unwrap();
    obj1.set("age", DefaultValueFactory::create_number(30.0).unwrap())
        .unwrap();

    let mut obj2 = DefaultValueFactory::create_object();
    obj2.set("name", DefaultValueFactory::create_string("Alice"))
        .unwrap();
    obj2.set("age", DefaultValueFactory::create_number(30.0).unwrap())
        .unwrap();

    let mut obj3 = DefaultValueFactory::create_object();
    obj3.set("name", DefaultValueFactory::create_string("Bob"))
        .unwrap();
    obj3.set("age", DefaultValueFactory::create_number(25.0).unwrap())
        .unwrap();

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
    let mut obj1 = DefaultValueFactory::create_object();
    let mut obj2 = DefaultValueFactory::create_object();

    for i in 0..5000 {
        let value = DefaultValueFactory::create_string(&format!("value_{}", i));
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

    println!(
        "✅ Large objects comparison in {:?}: equal = {}",
        duration, are_equal
    );

    // When: Modificar ligeramente y comparar
    obj2.set(
        "key_2500",
        DefaultValueFactory::create_string("modified_value"),
    )
    .unwrap();

    let start2 = Instant::now();
    let are_different = obj1.equals(&obj2).unwrap();
    let duration2 = start2.elapsed();

    // Then: Detecta diferencia eficientemente
    assert!(!are_different);
    assert!(duration2.as_millis() < 200); // Hash debería detectar diferencia rápido

    println!(
        "✅ Modified objects comparison in {:?}: equal = {}",
        duration2, are_different
    );
}

#[test]
fn test_context_merge_scenario() {
    // Given: Escenario empresarial realista con múltiples contextos
    let mut global_context = DefaultValueFactory::create_object();
    global_context
        .set(
            "company",
            DefaultValueFactory::create_string("TechCorp Inc"),
        )
        .unwrap();
    global_context
        .set(
            "environment",
            DefaultValueFactory::create_string("production"),
        )
        .unwrap();

    let mut global_config = DefaultValueFactory::create_object();
    global_config
        .set(
            "timeout",
            DefaultValueFactory::create_number(30000.0).unwrap(),
        )
        .unwrap();
    global_config
        .set(
            "retry_attempts",
            DefaultValueFactory::create_number(3.0).unwrap(),
        )
        .unwrap();
    global_config
        .set("debug_mode", DefaultValueFactory::create_bool(false))
        .unwrap();
    global_context.set("config", global_config).unwrap();

    let mut global_endpoints = DefaultValueFactory::create_array();
    global_endpoints
        .push(DefaultValueFactory::create_string(
            "https://api.techcorp.com/v1",
        ))
        .unwrap();
    global_endpoints
        .push(DefaultValueFactory::create_string(
            "https://backup-api.techcorp.com/v1",
        ))
        .unwrap();
    global_context.set("endpoints", global_endpoints).unwrap();

    // Regional context
    let mut regional_context = DefaultValueFactory::create_object();
    regional_context
        .set("region", DefaultValueFactory::create_string("us-west"))
        .unwrap();

    let mut regional_config = DefaultValueFactory::create_object();
    regional_config
        .set(
            "timeout",
            DefaultValueFactory::create_number(45000.0).unwrap(),
        )
        .unwrap(); // Override
    regional_config
        .set("currency", DefaultValueFactory::create_string("USD"))
        .unwrap(); // New
    regional_context.set("config", regional_config).unwrap();

    let mut regional_endpoints = DefaultValueFactory::create_array();
    regional_endpoints
        .push(DefaultValueFactory::create_string(
            "https://us-west-api.techcorp.com/v1",
        ))
        .unwrap();
    regional_context
        .set("endpoints", regional_endpoints)
        .unwrap();

    // User-specific context
    let mut user_context = DefaultValueFactory::create_object();
    user_context
        .set("user_id", DefaultValueFactory::create_string("user_12345"))
        .unwrap();

    let mut user_config = DefaultValueFactory::create_object();
    user_config
        .set("debug_mode", DefaultValueFactory::create_bool(true))
        .unwrap(); // User override
    user_config
        .set("language", DefaultValueFactory::create_string("en-US"))
        .unwrap(); // New
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
    assert_eq!(
        final_context
            .get_by_path("company")
            .unwrap()
            .unwrap()
            .as_str()
            .unwrap(),
        "TechCorp Inc"
    );
    assert_eq!(
        final_context
            .get_by_path("environment")
            .unwrap()
            .unwrap()
            .as_str()
            .unwrap(),
        "production"
    );
    assert_eq!(
        final_context
            .get_by_path("region")
            .unwrap()
            .unwrap()
            .as_str()
            .unwrap(),
        "us-west"
    );
    assert_eq!(
        final_context
            .get_by_path("user_id")
            .unwrap()
            .unwrap()
            .as_str()
            .unwrap(),
        "user_12345"
    );

    // Config merging
    assert_eq!(
        final_context
            .get_by_path("config.timeout")
            .unwrap()
            .unwrap()
            .as_number()
            .unwrap(),
        45000.0
    ); // Regional override
    assert_eq!(
        final_context
            .get_by_path("config.retry_attempts")
            .unwrap()
            .unwrap()
            .as_number()
            .unwrap(),
        3.0
    ); // Global preserved
    assert_eq!(
        final_context
            .get_by_path("config.debug_mode")
            .unwrap()
            .unwrap()
            .as_bool()
            .unwrap(),
        true
    ); // User override
    assert_eq!(
        final_context
            .get_by_path("config.currency")
            .unwrap()
            .unwrap()
            .as_str()
            .unwrap(),
        "USD"
    ); // Regional new
    assert_eq!(
        final_context
            .get_by_path("config.language")
            .unwrap()
            .unwrap()
            .as_str()
            .unwrap(),
        "en-US"
    ); // User new

    let final_endpoints = final_context.get("endpoints").unwrap().unwrap();
    let endpoints_array = final_endpoints.as_array().unwrap().unwrap();

    println!("DEBUG - Final endpoints count: {}", endpoints_array.len());
    for (i, endpoint) in endpoints_array.iter().enumerate() {
        println!(
            "  Endpoint {}: {}",
            i,
            endpoint.as_str().unwrap_or(String::from("INVALID"))
        );
    }

    assert_eq!(endpoints_array.len(), 3); // 2 global + 1 regional

    // Verificar contenido específico
    let endpoint_strings: Vec<String> = endpoints_array
        .iter()
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
    let manager = DefaultModelManagerFactory::create();

    // Template base
    let mut base_template = DefaultValueFactory::create_object();
    base_template
        .set("name", DefaultValueFactory::create_string("Base Template"))
        .unwrap();
    base_template
        .set("version", DefaultValueFactory::create_string("1.0.0"))
        .unwrap();

    let mut base_styles = DefaultValueFactory::create_object();
    base_styles
        .set("color", DefaultValueFactory::create_string("#000000"))
        .unwrap();
    base_styles
        .set(
            "font_size",
            DefaultValueFactory::create_number(14.0).unwrap(),
        )
        .unwrap();
    base_template.set("styles", base_styles).unwrap();

    manager
        .insert(
            "templates",
            Some("base_template"),
            base_template,
        )
        .unwrap();

    // Template override
    let mut override_template = DefaultValueFactory::create_object();
    override_template
        .set("version", DefaultValueFactory::create_string("1.1.0"))
        .unwrap();

    let mut override_styles = DefaultValueFactory::create_object();
    override_styles
        .set("color", DefaultValueFactory::create_string("#FF0000"))
        .unwrap();
    override_styles
        .set("background", DefaultValueFactory::create_string("#FFFFFF"))
        .unwrap();
    override_template.set("styles", override_styles).unwrap();

    manager
        .insert(
            "templates",
            Some("override_template"),
            override_template,
        )
        .unwrap();

    // When: Recuperar y hacer merge
    let mut base = manager
        .get("templates", "base_template")
        .unwrap();
    let override_data = manager
        .get("templates", "override_template")
        .unwrap();

    println!("Base before merge: {}", base.to_string());
    println!("Override data: {}", override_data.to_string());

    base.merge(&override_data).unwrap();

    // Then: Template merged correctamente
    assert_eq!(
        base.get_by_path("name").unwrap().unwrap().as_str().unwrap(),
        "Base Template"
    );
    assert_eq!(
        base.get_by_path("version")
            .unwrap()
            .unwrap()
            .as_str()
            .unwrap(),
        "1.1.0"
    );
    assert_eq!(
        base.get_by_path("styles.color")
            .unwrap()
            .unwrap()
            .as_str()
            .unwrap(),
        "#FF0000"
    );
    assert_eq!(
        base.get_by_path("styles.font_size")
            .unwrap()
            .unwrap()
            .as_number()
            .unwrap(),
        14.0
    );
    assert_eq!(
        base.get_by_path("styles.background")
            .unwrap()
            .unwrap()
            .as_str()
            .unwrap(),
        "#FFFFFF"
    );

    // When: Guardar template merged
    manager
        .update(
            "templates",
            "base_template",
            base.clone(),
        )
        .unwrap();

    // When: Comparar con original
    let original = manager
        .get("templates", "override_template")
        .unwrap();
    let merged = manager
        .get("templates", "base_template")
        .unwrap();

    let are_equal = merged.equals(&original).unwrap();
    let merged_hash = merged.calculate_hash().unwrap();
    let original_hash = original.calculate_hash().unwrap();

    // Then: Templates son diferentes
    assert!(!are_equal);
    assert_ne!(merged_hash, original_hash);

    println!("✅ Merged template: {}", merged.to_string());
    println!(
        "✅ Templates are different: equal = {}, hash1 = {}, hash2 = {}",
        are_equal, merged_hash, original_hash
    );
}

#[test]
fn test_edge_cases_merge_and_comparison() {
    // Test Case 1: Merge con objetos vacíos
    let mut empty1 = DefaultValueFactory::create_object();
    let empty2 = DefaultValueFactory::create_object();

    empty1.merge(&empty2).unwrap();
    assert!(empty1.is_empty());

    // Test Case 2: Merge primitivo sobre objeto
    let mut object = DefaultValueFactory::create_object();
    object
        .set("key", DefaultValueFactory::create_string("value"))
        .unwrap();

    let primitive = DefaultValueFactory::create_string("replacement");
    object.merge(&primitive).unwrap();
    assert_eq!(object.as_str().unwrap(), "replacement");

    // Test Case 3: Hash de valores null
    let null_val = DefaultValueFactory::create_string("null");
    let hash = null_val.calculate_hash().unwrap();
    assert_ne!(hash, 0);

    // Test Case 4: Equals con tipos diferentes
    let string_val = DefaultValueFactory::create_string("123");
    let number_val = DefaultValueFactory::create_number(123.0).unwrap();
    let are_equal = string_val.equals(&number_val).unwrap();
    assert!(!are_equal);

    // Test Case 5: Arrays vacíos
    let mut empty_arr1 = DefaultValueFactory::create_array();
    let empty_arr2 = DefaultValueFactory::create_array();

    empty_arr1.merge(&empty_arr2).unwrap();
    assert!(empty_arr1.is_empty());

    let are_equal = empty_arr1.equals(&empty_arr2).unwrap();
    assert!(are_equal);

    println!("✅ All edge cases passed");
}
