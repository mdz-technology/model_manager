use tokio::task::LocalSet;
use model_manager::{DefaultValue, DynamicValue, ModelManager, DefaultModelManager, DynamicValueFactory, ModelManagerFactory};

type Value = <DefaultValue as DynamicValueFactory>::Value;

#[tokio::test]
async fn test_has_property_basic() {
    let local_set = LocalSet::new();

    local_set.run_until(async {
        // Given: Objeto con propiedades conocidas
        let mut user = DefaultValue::create();
        user.set("name", Value::from_str("Ana García")).await.unwrap();
        user.set("age", Value::from_number(28.0).unwrap()).await.unwrap();
        user.set("active", Value::from_bool(true)).await.unwrap();

        // When: Verificar propiedades existentes y no existentes
        let has_name = user.has_property("name").await.unwrap();
        let has_age = user.has_property("age").await.unwrap();
        let has_active = user.has_property("active").await.unwrap();
        let has_nonexistent = user.has_property("nonexistent").await.unwrap();

        // Then: Resultados correctos
        assert!(has_name);
        assert!(has_age);
        assert!(has_active);
        assert!(!has_nonexistent);

        println!("✅ Basic has_property test passed");
    }).await;
}

#[tokio::test]
async fn test_has_property_error_cases() {
    let local_set = LocalSet::new();

    local_set.run_until(async {
        // Given: Valores no-objeto
        let string_value = Value::from_str("not an object");
        let number_value = Value::from_number(42.0).unwrap();
        let array_value = Value::new_array();

        // When: Intentar verificar propiedades en valores no-objeto
        let string_result = string_value.has_property("key").await;
        let number_result = number_value.has_property("key").await;
        let array_result = array_value.has_property("key").await;

        // Then: Errores apropiados
        assert!(string_result.is_err());
        assert!(number_result.is_err());
        assert!(array_result.is_err());

        println!("✅ has_property error cases test passed");
    }).await;
}

#[tokio::test]
async fn test_get_property_type_basic() {
    let local_set = LocalSet::new();

    local_set.run_until(async {
        // Given: Objeto con diferentes tipos de propiedades
        let mut data = DefaultValue::create();
        data.set("string_prop", Value::from_str("test")).await.unwrap();
        data.set("number_prop", Value::from_number(123.45).unwrap()).await.unwrap();
        data.set("bool_prop", Value::from_bool(false)).await.unwrap();
        data.set("object_prop", Value::new_object()).await.unwrap();
        data.set("array_prop", Value::new_array()).await.unwrap();

        // When: Obtener tipos de propiedades
        let string_type = data.get_property_type("string_prop").await.unwrap();
        let number_type = data.get_property_type("number_prop").await.unwrap();
        let bool_type = data.get_property_type("bool_prop").await.unwrap();
        let object_type = data.get_property_type("object_prop").await.unwrap();
        let array_type = data.get_property_type("array_prop").await.unwrap();
        let nonexistent_type = data.get_property_type("nonexistent").await.unwrap();

        // Then: Tipos correctos
        assert_eq!(string_type, Some("String".to_string()));
        assert_eq!(number_type, Some("Number".to_string()));
        assert_eq!(bool_type, Some("Bool".to_string()));
        assert_eq!(object_type, Some("Object".to_string()));
        assert_eq!(array_type, Some("Array".to_string()));
        assert_eq!(nonexistent_type, None);

        println!("✅ Basic get_property_type test passed");
    }).await;
}

#[tokio::test]
async fn test_get_property_names_basic() {
    let local_set = LocalSet::new();

    local_set.run_until(async {
        // Given: Objeto con múltiples propiedades
        let mut employee = DefaultValue::create();
        employee.set("name", Value::from_str("Carlos Mendoza")).await.unwrap();
        employee.set("department", Value::from_str("Engineering")).await.unwrap();
        employee.set("salary", Value::from_number(75000.0).unwrap()).await.unwrap();
        employee.set("remote", Value::from_bool(true)).await.unwrap();
        employee.set("start_date", Value::from_str("2023-01-15")).await.unwrap();

        // When: Obtener nombres de propiedades
        let property_names = employee.get_property_names().await.unwrap();

        // Then: Todos los nombres presentes (ordenados alfabéticamente)
        assert_eq!(property_names.len(), 5);
        assert!(property_names.contains(&"name".to_string()));
        assert!(property_names.contains(&"department".to_string()));
        assert!(property_names.contains(&"salary".to_string()));
        assert!(property_names.contains(&"remote".to_string()));
        assert!(property_names.contains(&"start_date".to_string()));

        let expected_order = vec!["department", "name", "remote", "salary", "start_date"];
        assert_eq!(property_names, expected_order);

        println!("✅ Basic get_property_names test passed");
    }).await;
}

#[tokio::test]
async fn test_get_property_names_empty_object() {
    let local_set = LocalSet::new();

    local_set.run_until(async {
        // Given: Objeto vacío
        let empty_object = DefaultValue::create();

        // When: Obtener nombres de propiedades
        let property_names = empty_object.get_property_names().await.unwrap();

        // Then: Lista vacía
        assert_eq!(property_names.len(), 0);
        assert!(property_names.is_empty());

        println!("✅ Empty object get_property_names test passed");
    }).await;
}

#[tokio::test]
async fn test_count_properties_basic() {
    let local_set = LocalSet::new();

    local_set.run_until(async {
        // Given: Objeto con diferentes cantidades de propiedades
        let mut small_object = DefaultValue::create();
        small_object.set("prop1", Value::from_str("value1")).await.unwrap();
        small_object.set("prop2", Value::from_str("value2")).await.unwrap();

        let mut large_object = DefaultValue::create();
        for i in 0..100 {
            large_object.set(
                &format!("property_{}", i),
                Value::from_number(i as f64).unwrap()
            ).await.unwrap();
        }

        let empty_object = DefaultValue::create();

        // When: Contar propiedades
        let small_count = small_object.count_properties().await.unwrap();
        let large_count = large_object.count_properties().await.unwrap();
        let empty_count = empty_object.count_properties().await.unwrap();

        // Then: Conteos correctos
        assert_eq!(small_count, 2);
        assert_eq!(large_count, 100);
        assert_eq!(empty_count, 0);

        println!("✅ Basic count_properties test passed");
    }).await;
}

#[tokio::test]
async fn test_introspection_error_cases() {
    let local_set = LocalSet::new();

    local_set.run_until(async {
        // Given: Valores que no son objetos
        let string_val = Value::from_str("not an object");
        let number_val = Value::from_number(42.0).unwrap();
        let bool_val = Value::from_bool(true);
        let array_val = Value::new_array();

        let test_values = vec![string_val, number_val, bool_val, array_val];

        for (i, value) in test_values.iter().enumerate() {
            // When: Intentar operaciones de introspección en valores no-objeto
            let property_type_result = value.get_property_type("any_key").await;
            let property_names_result = value.get_property_names().await;
            let count_result = value.count_properties().await;

            // Then: Todos deben fallar con error apropiado
            assert!(property_type_result.is_err(), "Test {} should fail for get_property_type", i);
            assert!(property_names_result.is_err(), "Test {} should fail for get_property_names", i);
            assert!(count_result.is_err(), "Test {} should fail for count_properties", i);

            // Verificar que los mensajes de error sean apropiados
            assert!(property_type_result.unwrap_err().to_string().contains("non-object"));
            assert!(property_names_result.unwrap_err().to_string().contains("non-object"));
            assert!(count_result.unwrap_err().to_string().contains("non-object"));
        }

        println!("✅ Introspection error cases test passed");
    }).await;
}

#[tokio::test]
async fn test_introspection_with_nested_objects() {
    let local_set = LocalSet::new();

    local_set.run_until(async {
        // Given: Objeto con estructura anidada
        let mut root = DefaultValue::create();
        root.set("id", Value::from_number(1.0).unwrap()).await.unwrap();
        root.set("name", Value::from_str("Root Object")).await.unwrap();

        let mut nested = Value::new_object();
        nested.set("nested_prop", Value::from_str("nested_value")).await.unwrap();
        nested.set("nested_number", Value::from_number(99.9).unwrap()).await.unwrap();
        root.set("nested", nested).await.unwrap();

        let mut array = Value::new_array();
        array.push(Value::from_str("item1")).await.unwrap();
        array.push(Value::from_str("item2")).await.unwrap();
        root.set("items", array).await.unwrap();

        // When: Introspección en objeto raíz
        let root_properties = root.get_property_names().await.unwrap();
        let root_count = root.count_properties().await.unwrap();
        let nested_type = root.get_property_type("nested").await.unwrap();
        let items_type = root.get_property_type("items").await.unwrap();

        // Then: Resultados correctos para objeto raíz
        assert_eq!(root_count, 4); // id, name, nested, items
        assert_eq!(root_properties, vec!["id", "items", "name", "nested"]);
        assert_eq!(nested_type, Some("Object".to_string()));
        assert_eq!(items_type, Some("Array".to_string()));

        // When: Introspección en objeto anidado
        let nested_obj = root.get("nested").await.unwrap().unwrap();
        let nested_properties = nested_obj.get_property_names().await.unwrap();
        let nested_count = nested_obj.count_properties().await.unwrap();

        // Then: Resultados correctos para objeto anidado
        assert_eq!(nested_count, 2);
        assert_eq!(nested_properties, vec!["nested_number", "nested_prop"]);

        println!("✅ Introspection with nested objects test passed");
    }).await;
}

#[tokio::test]
async fn test_introspection_performance() {
    let local_set = LocalSet::new();

    local_set.run_until(async {
        // Given: Objeto grande para test de performance
        let mut large_object = DefaultValue::create();
        for i in 0..10000 {
            large_object.set(
                &format!("prop_{:04}", i),
                Value::from_str(&format!("value_{}", i))
            ).await.unwrap();
        }

        // When: Operaciones de introspección en objeto grande
        let start = std::time::Instant::now();

        let count = large_object.count_properties().await.unwrap();
        let count_duration = start.elapsed();

        let start = std::time::Instant::now();
        let property_names = large_object.get_property_names().await.unwrap();
        let names_duration = start.elapsed();

        let start = std::time::Instant::now();
        let has_first = large_object.has_property("prop_0000").await.unwrap();
        let has_middle = large_object.has_property("prop_5000").await.unwrap();
        let has_last = large_object.has_property("prop_9999").await.unwrap();
        let has_nonexistent = large_object.has_property("nonexistent").await.unwrap();
        let has_duration = start.elapsed();

        let start = std::time::Instant::now();
        let type_first = large_object.get_property_type("prop_0000").await.unwrap();
        let type_middle = large_object.get_property_type("prop_5000").await.unwrap();
        let type_last = large_object.get_property_type("prop_9999").await.unwrap();
        let type_duration = start.elapsed();

        // Then: Performance aceptable y resultados correctos
        assert_eq!(count, 10000);
        assert_eq!(property_names.len(), 10000);
        assert!(has_first);
        assert!(has_middle);
        assert!(has_last);
        assert!(!has_nonexistent);
        assert_eq!(type_first, Some("String".to_string()));
        assert_eq!(type_middle, Some("String".to_string()));
        assert_eq!(type_last, Some("String".to_string()));

        // Verificar performance
        assert!(count_duration.as_millis() < 100, "count_properties too slow: {:?}", count_duration);
        assert!(names_duration.as_millis() < 200, "get_property_names too slow: {:?}", names_duration);
        assert!(has_duration.as_millis() < 50, "has_property too slow: {:?}", has_duration);
        assert!(type_duration.as_millis() < 50, "get_property_type too slow: {:?}", type_duration);

        println!("✅ Introspection performance test passed");
        println!("   count_properties: {:?}", count_duration);
        println!("   get_property_names: {:?}", names_duration);
        println!("   has_property (4 calls): {:?}", has_duration);
        println!("   get_property_type (3 calls): {:?}", type_duration);
    }).await;
}

#[tokio::test]
async fn test_introspection_with_model_manager() {
    let local_set = LocalSet::new();

    local_set.run_until(async {
        // Given: Model manager con datos empresariales
        let mut manager = DefaultModelManager::create();

        let mut company = DefaultValue::create();
        company.set("name", Value::from_str("TechCorp")).await.unwrap();
        company.set("founded", Value::from_number(2020.0).unwrap()).await.unwrap();
        company.set("active", Value::from_bool(true)).await.unwrap();

        let mut address = Value::new_object();
        address.set("street", Value::from_str("123 Tech Street")).await.unwrap();
        address.set("city", Value::from_str("San Francisco")).await.unwrap();
        address.set("country", Value::from_str("USA")).await.unwrap();
        company.set("address", address).await.unwrap();

        manager.insert(
            "companies".to_string(),
            Some("company_001".to_string()),
            company
        ).await.unwrap();

        // When: Usar introspección a través del model manager
        let retrieved = manager.get(
            "companies".to_string(),
            "company_001".to_string()
        ).await.unwrap();

        let property_names = retrieved.get_property_names().await.unwrap();
        let property_count = retrieved.count_properties().await.unwrap();
        let has_name = retrieved.has_property("name").await.unwrap();
        let has_invalid = retrieved.has_property("invalid_prop").await.unwrap();
        let address_type = retrieved.get_property_type("address").await.unwrap();

        // Then: Introspección funciona correctamente con datos del model manager
        assert_eq!(property_count, 4); // name, founded, active, address
        assert_eq!(property_names, vec!["active", "address", "founded", "name"]);
        assert!(has_name);
        assert!(!has_invalid);
        assert_eq!(address_type, Some("Object".to_string()));

        // When: Introspección en objeto anidado
        let address_obj = retrieved.get("address").await.unwrap().unwrap();
        let address_properties = address_obj.get_property_names().await.unwrap();
        let address_count = address_obj.count_properties().await.unwrap();

        // Then: Objeto anidado también funciona
        assert_eq!(address_count, 3);
        assert_eq!(address_properties, vec!["city", "country", "street"]);

        println!("✅ Introspection with model manager test passed");
    }).await;
}

#[tokio::test]
async fn test_introspection_comprehensive_scenario() {
    let local_set = LocalSet::new();

    local_set.run_until(async {
        // Given: Escenario empresarial complejo
        let mut employee = DefaultValue::create();
        employee.set("id", Value::from_number(12345.0).unwrap()).await.unwrap();
        employee.set("name", Value::from_str("María González")).await.unwrap();
        employee.set("email", Value::from_str("maria@techcorp.com")).await.unwrap();
        employee.set("active", Value::from_bool(true)).await.unwrap();

        let mut profile = Value::new_object();
        profile.set("department", Value::from_str("Engineering")).await.unwrap();
        profile.set("level", Value::from_number(8.0).unwrap()).await.unwrap();
        profile.set("remote", Value::from_bool(true)).await.unwrap();
        profile.set("start_date", Value::from_str("2022-03-15")).await.unwrap();
        employee.set("profile", profile).await.unwrap();

        let mut skills = Value::new_array();
        skills.push(Value::from_str("Rust")).await.unwrap();
        skills.push(Value::from_str("JavaScript")).await.unwrap();
        skills.push(Value::from_str("Database Design")).await.unwrap();
        employee.set("skills", skills).await.unwrap();

        let mut projects = Value::new_array();
        for i in 1..=3 {
            let mut project = Value::new_object();
            project.set("id", Value::from_number(i as f64).unwrap()).await.unwrap();
            project.set("name", Value::from_str(&format!("Project {}", i))).await.unwrap();
            project.set("status", Value::from_str("active")).await.unwrap();
            projects.push(project).await.unwrap();
        }
        employee.set("projects", projects).await.unwrap();

        // When: Análisis completo de introspección
        println!("=== COMPREHENSIVE INTROSPECTION ANALYSIS ===");

        let top_level_props = employee.get_property_names().await.unwrap();
        let top_level_count = employee.count_properties().await.unwrap();

        println!("Employee top-level properties: {} total", top_level_count);
        for prop in &top_level_props {
            let prop_type = employee.get_property_type(prop).await.unwrap().unwrap();
            println!("  {}: {}", prop, prop_type);
        }

        let profile_obj = employee.get("profile").await.unwrap().unwrap();
        let profile_props = profile_obj.get_property_names().await.unwrap();
        let profile_count = profile_obj.count_properties().await.unwrap();

        println!("Profile properties: {} total", profile_count);
        for prop in &profile_props {
            let prop_type = profile_obj.get_property_type(prop).await.unwrap().unwrap();
            println!("  profile.{}: {}", prop, prop_type);
        }

        // Análisis de arrays
        let skills_array = employee.get("skills").await.unwrap().unwrap();
        println!("Skills type: {}", skills_array.get_type());

        let projects_array = employee.get("projects").await.unwrap().unwrap();
        println!("Projects type: {}", projects_array.get_type());

        // Then: Verificaciones específicas - CORREGIDO
        assert_eq!(top_level_count, 7); // id, name, email, active, profile, skills, projects (7 en total)
        assert_eq!(top_level_props, vec!["active", "email", "id", "name", "profile", "projects", "skills"]);

        assert_eq!(profile_count, 4); // department, level, remote, start_date
        assert_eq!(profile_props, vec!["department", "level", "remote", "start_date"]);

        assert_eq!(employee.get_property_type("id").await.unwrap(), Some("Number".to_string()));
        assert_eq!(employee.get_property_type("name").await.unwrap(), Some("String".to_string()));
        assert_eq!(employee.get_property_type("email").await.unwrap(), Some("String".to_string()));
        assert_eq!(employee.get_property_type("active").await.unwrap(), Some("Bool".to_string()));
        assert_eq!(employee.get_property_type("profile").await.unwrap(), Some("Object".to_string()));
        assert_eq!(employee.get_property_type("skills").await.unwrap(), Some("Array".to_string()));
        assert_eq!(employee.get_property_type("projects").await.unwrap(), Some("Array".to_string()));

        // Verificar propiedades específicas
        assert!(employee.has_property("email").await.unwrap());
        assert!(employee.has_property("profile").await.unwrap());
        assert!(!employee.has_property("salary").await.unwrap()); // No existe

        assert!(profile_obj.has_property("department").await.unwrap());
        assert!(profile_obj.has_property("level").await.unwrap());
        assert!(!profile_obj.has_property("bonus").await.unwrap()); // No existe

        println!("✅ Comprehensive introspection scenario test passed");
    }).await;
}