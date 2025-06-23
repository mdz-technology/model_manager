use model_manager::{
    CoreValue, DefaultModelManager, DefaultValueFactory, ModelManager, ModelManagerFactory,
    ValueFactory, ValueIntrospection,
};

#[test]
fn test_has_property_basic() {
    // Given: Objeto con propiedades conocidas
    let mut user = DefaultValueFactory::create_object();
    user.set("name", DefaultValueFactory::create_string("Ana García"))
        .unwrap();
    user.set("age", DefaultValueFactory::create_number(28.0).unwrap())
        .unwrap();
    user.set("active", DefaultValueFactory::create_bool(true))
        .unwrap();

    // When: Verificar propiedades existentes y no existentes
    let has_name = user.has_property("name").unwrap();
    let has_age = user.has_property("age").unwrap();
    let has_active = user.has_property("active").unwrap();
    let has_nonexistent = user.has_property("nonexistent").unwrap();

    // Then: Resultados correctos
    assert!(has_name);
    assert!(has_age);
    assert!(has_active);
    assert!(!has_nonexistent);

    println!("✅ Basic has_property test passed");
}

#[test]
fn test_has_property_error_cases() {
    // Given: Valores no-objeto
    let string_value = DefaultValueFactory::create_string("not an object");
    let number_value = DefaultValueFactory::create_number(42.0).unwrap();
    let array_value = DefaultValueFactory::create_array();

    // When: Intentar verificar propiedades en valores no-objeto
    let string_result = string_value.has_property("key");
    let number_result = number_value.has_property("key");
    let array_result = array_value.has_property("key");

    // Then: Errores apropiados
    assert!(string_result.is_err());
    assert!(number_result.is_err());
    assert!(array_result.is_err());

    println!("✅ has_property error cases test passed");
}

#[test]
fn test_get_property_type_basic() {
    // Given: Objeto con diferentes tipos de propiedades
    let mut data = DefaultValueFactory::create_object();
    data.set("string_prop", DefaultValueFactory::create_string("test"))
        .unwrap();
    data.set(
        "number_prop",
        DefaultValueFactory::create_number(123.45).unwrap(),
    )
    .unwrap();
    data.set("bool_prop", DefaultValueFactory::create_bool(false))
        .unwrap();
    data.set("object_prop", DefaultValueFactory::create_object())
        .unwrap();
    data.set("array_prop", DefaultValueFactory::create_array())
        .unwrap();

    // When: Obtener tipos de propiedades
    let string_type = data.get_property_type("string_prop").unwrap();
    let number_type = data.get_property_type("number_prop").unwrap();
    let bool_type = data.get_property_type("bool_prop").unwrap();
    let object_type = data.get_property_type("object_prop").unwrap();
    let array_type = data.get_property_type("array_prop").unwrap();
    let nonexistent_type = data.get_property_type("nonexistent").unwrap();

    // Then: Tipos correctos
    assert_eq!(string_type, Some("String".to_string()));
    assert_eq!(number_type, Some("Number".to_string()));
    assert_eq!(bool_type, Some("Bool".to_string()));
    assert_eq!(object_type, Some("Object".to_string()));
    assert_eq!(array_type, Some("Array".to_string()));
    assert_eq!(nonexistent_type, None);

    println!("✅ Basic get_property_type test passed");
}

#[test]
fn test_get_property_names_basic() {
    // Given: Objeto con múltiples propiedades
    let mut employee = DefaultValueFactory::create_object();
    employee
        .set("name", DefaultValueFactory::create_string("Carlos Mendoza"))
        .unwrap();
    employee
        .set(
            "department",
            DefaultValueFactory::create_string("Engineering"),
        )
        .unwrap();
    employee
        .set(
            "salary",
            DefaultValueFactory::create_number(75000.0).unwrap(),
        )
        .unwrap();
    employee
        .set("remote", DefaultValueFactory::create_bool(true))
        .unwrap();
    employee
        .set(
            "start_date",
            DefaultValueFactory::create_string("2023-01-15"),
        )
        .unwrap();

    // When: Obtener nombres de propiedades
    let property_names = employee.get_property_names().unwrap();

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
}

#[test]
fn test_get_property_names_empty_object() {
    // Given: Objeto vacío
    let empty_object = DefaultValueFactory::create_object();

    // When: Obtener nombres de propiedades
    let property_names = empty_object.get_property_names().unwrap();

    // Then: Lista vacía
    assert_eq!(property_names.len(), 0);
    assert!(property_names.is_empty());

    println!("✅ Empty object get_property_names test passed");
}

#[test]
fn test_count_properties_basic() {
    // Given: Objeto con diferentes cantidades de propiedades
    let mut small_object = DefaultValueFactory::create_object();
    small_object
        .set("prop1", DefaultValueFactory::create_string("value1"))
        .unwrap();
    small_object
        .set("prop2", DefaultValueFactory::create_string("value2"))
        .unwrap();

    let mut large_object = DefaultValueFactory::create_object();
    for i in 0..100 {
        large_object
            .set(
                &format!("property_{}", i),
                DefaultValueFactory::create_number(i as f64).unwrap(),
            )
            .unwrap();
    }

    let empty_object = DefaultValueFactory::create_object();

    // When: Contar propiedades
    let small_count = small_object.count_properties().unwrap();
    let large_count = large_object.count_properties().unwrap();
    let empty_count = empty_object.count_properties().unwrap();

    // Then: Conteos correctos
    assert_eq!(small_count, 2);
    assert_eq!(large_count, 100);
    assert_eq!(empty_count, 0);

    println!("✅ Basic count_properties test passed");
}

#[test]
fn test_introspection_error_cases() {
    // Given: Valores que no son objetos
    let string_val = DefaultValueFactory::create_string("not an object");
    let number_val = DefaultValueFactory::create_number(42.0).unwrap();
    let bool_val = DefaultValueFactory::create_bool(true);
    let array_val = DefaultValueFactory::create_array();

    let test_values = vec![string_val, number_val, bool_val, array_val];

    for (i, value) in test_values.iter().enumerate() {
        // When: Intentar operaciones de introspección en valores no-objeto
        let property_type_result = value.get_property_type("any_key");
        let property_names_result = value.get_property_names();
        let count_result = value.count_properties();

        // Then: Todos deben fallar con error apropiado
        assert!(
            property_type_result.is_err(),
            "Test {} should fail for get_property_type",
            i
        );
        assert!(
            property_names_result.is_err(),
            "Test {} should fail for get_property_names",
            i
        );
        assert!(
            count_result.is_err(),
            "Test {} should fail for count_properties",
            i
        );

        // Verificar que los mensajes de error sean apropiados
        assert!(property_type_result
            .unwrap_err()
            .to_string()
            .contains("non-object"));
        assert!(property_names_result
            .unwrap_err()
            .to_string()
            .contains("non-object"));
        assert!(count_result.unwrap_err().to_string().contains("non-object"));
    }

    println!("✅ Introspection error cases test passed");
}

#[test]
fn test_introspection_with_nested_objects() {
    // Given: Objeto con estructura anidada
    let mut root = DefaultValueFactory::create_object();
    root.set("id", DefaultValueFactory::create_number(1.0).unwrap())
        .unwrap();
    root.set("name", DefaultValueFactory::create_string("Root Object"))
        .unwrap();

    let mut nested = DefaultValueFactory::create_object();
    nested
        .set(
            "nested_prop",
            DefaultValueFactory::create_string("nested_value"),
        )
        .unwrap();
    nested
        .set(
            "nested_number",
            DefaultValueFactory::create_number(99.9).unwrap(),
        )
        .unwrap();
    root.set("nested", nested).unwrap();

    let mut array = DefaultValueFactory::create_array();
    array
        .push(DefaultValueFactory::create_string("item1"))
        .unwrap();
    array
        .push(DefaultValueFactory::create_string("item2"))
        .unwrap();
    root.set("items", array).unwrap();

    // When: Introspección en objeto raíz
    let root_properties = root.get_property_names().unwrap();
    let root_count = root.count_properties().unwrap();
    let nested_type = root.get_property_type("nested").unwrap();
    let items_type = root.get_property_type("items").unwrap();

    // Then: Resultados correctos para objeto raíz
    assert_eq!(root_count, 4); // id, name, nested, items
    assert_eq!(root_properties, vec!["id", "items", "name", "nested"]);
    assert_eq!(nested_type, Some("Object".to_string()));
    assert_eq!(items_type, Some("Array".to_string()));

    // When: Introspección en objeto anidado
    let nested_obj = root.get("nested").unwrap().unwrap();
    let nested_properties = nested_obj.get_property_names().unwrap();
    let nested_count = nested_obj.count_properties().unwrap();

    // Then: Resultados correctos para objeto anidado
    assert_eq!(nested_count, 2);
    assert_eq!(nested_properties, vec!["nested_number", "nested_prop"]);

    println!("✅ Introspection with nested objects test passed");
}

#[test]
fn test_introspection_performance() {
    // Given: Objeto grande para test de performance
    let mut large_object = DefaultValueFactory::create_object();
    for i in 0..10000 {
        large_object
            .set(
                &format!("prop_{:04}", i),
                DefaultValueFactory::create_string(&format!("value_{}", i)),
            )
            .unwrap();
    }

    // When: Operaciones de introspección en objeto grande
    let start = std::time::Instant::now();

    let count = large_object.count_properties().unwrap();
    let count_duration = start.elapsed();

    let start = std::time::Instant::now();
    let property_names = large_object.get_property_names().unwrap();
    let names_duration = start.elapsed();

    let start = std::time::Instant::now();
    let has_first = large_object.has_property("prop_0000").unwrap();
    let has_middle = large_object.has_property("prop_5000").unwrap();
    let has_last = large_object.has_property("prop_9999").unwrap();
    let has_nonexistent = large_object.has_property("nonexistent").unwrap();
    let has_duration = start.elapsed();

    let start = std::time::Instant::now();
    let type_first = large_object.get_property_type("prop_0000").unwrap();
    let type_middle = large_object.get_property_type("prop_5000").unwrap();
    let type_last = large_object.get_property_type("prop_9999").unwrap();
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
    assert!(
        count_duration.as_millis() < 100,
        "count_properties too slow: {:?}",
        count_duration
    );
    assert!(
        names_duration.as_millis() < 200,
        "get_property_names too slow: {:?}",
        names_duration
    );
    assert!(
        has_duration.as_millis() < 50,
        "has_property too slow: {:?}",
        has_duration
    );
    assert!(
        type_duration.as_millis() < 50,
        "get_property_type too slow: {:?}",
        type_duration
    );

    println!("✅ Introspection performance test passed");
    println!("   count_properties: {:?}", count_duration);
    println!("   get_property_names: {:?}", names_duration);
    println!("   has_property (4 calls): {:?}", has_duration);
    println!("   get_property_type (3 calls): {:?}", type_duration);
}

#[test]
fn test_introspection_with_model_manager() {
    // Given: Model manager con datos empresariales
    let manager = DefaultModelManager::create();

    let mut company = DefaultValueFactory::create_object();
    company
        .set("name", DefaultValueFactory::create_string("TechCorp"))
        .unwrap();
    company
        .set(
            "founded",
            DefaultValueFactory::create_number(2020.0).unwrap(),
        )
        .unwrap();
    company
        .set("active", DefaultValueFactory::create_bool(true))
        .unwrap();

    let mut address = DefaultValueFactory::create_object();
    address
        .set(
            "street",
            DefaultValueFactory::create_string("123 Tech Street"),
        )
        .unwrap();
    address
        .set("city", DefaultValueFactory::create_string("San Francisco"))
        .unwrap();
    address
        .set("country", DefaultValueFactory::create_string("USA"))
        .unwrap();
    company.set("address", address).unwrap();

    manager
        .insert(
            "companies",
            Some("company_001"),
            company,
        )
        .unwrap();

    // When: Usar introspección a través del model manager
    let retrieved = manager
        .get("companies", "company_001")
        .unwrap();

    let property_names = retrieved.get_property_names().unwrap();
    let property_count = retrieved.count_properties().unwrap();
    let has_name = retrieved.has_property("name").unwrap();
    let has_invalid = retrieved.has_property("invalid_prop").unwrap();
    let address_type = retrieved.get_property_type("address").unwrap();

    // Then: Introspección funciona correctamente con datos del model manager
    assert_eq!(property_count, 4); // name, founded, active, address
    assert_eq!(property_names, vec!["active", "address", "founded", "name"]);
    assert!(has_name);
    assert!(!has_invalid);
    assert_eq!(address_type, Some("Object".to_string()));

    // When: Introspección en objeto anidado
    let address_obj = retrieved.get("address").unwrap().unwrap();
    let address_properties = address_obj.get_property_names().unwrap();
    let address_count = address_obj.count_properties().unwrap();

    // Then: Objeto anidado también funciona
    assert_eq!(address_count, 3);
    assert_eq!(address_properties, vec!["city", "country", "street"]);

    println!("✅ Introspection with model manager test passed");
}

#[test]
fn test_introspection_comprehensive_scenario() {
    // Given: Escenario empresarial complejo
    let mut employee = DefaultValueFactory::create_object();
    employee
        .set("id", DefaultValueFactory::create_number(12345.0).unwrap())
        .unwrap();
    employee
        .set("name", DefaultValueFactory::create_string("María González"))
        .unwrap();
    employee
        .set(
            "email",
            DefaultValueFactory::create_string("maria@techcorp.com"),
        )
        .unwrap();
    employee
        .set("active", DefaultValueFactory::create_bool(true))
        .unwrap();

    let mut profile = DefaultValueFactory::create_object();
    profile
        .set(
            "department",
            DefaultValueFactory::create_string("Engineering"),
        )
        .unwrap();
    profile
        .set("level", DefaultValueFactory::create_number(8.0).unwrap())
        .unwrap();
    profile
        .set("remote", DefaultValueFactory::create_bool(true))
        .unwrap();
    profile
        .set(
            "start_date",
            DefaultValueFactory::create_string("2022-03-15"),
        )
        .unwrap();
    employee.set("profile", profile).unwrap();

    let mut skills = DefaultValueFactory::create_array();
    skills
        .push(DefaultValueFactory::create_string("Rust"))
        .unwrap();
    skills
        .push(DefaultValueFactory::create_string("JavaScript"))
        .unwrap();
    skills
        .push(DefaultValueFactory::create_string("Database Design"))
        .unwrap();
    employee.set("skills", skills).unwrap();

    let mut projects = DefaultValueFactory::create_array();
    for i in 1..=3 {
        let mut project = DefaultValueFactory::create_object();
        project
            .set("id", DefaultValueFactory::create_number(i as f64).unwrap())
            .unwrap();
        project
            .set(
                "name",
                DefaultValueFactory::create_string(&format!("Project {}", i)),
            )
            .unwrap();
        project
            .set("status", DefaultValueFactory::create_string("active"))
            .unwrap();
        projects.push(project).unwrap();
    }
    employee.set("projects", projects).unwrap();

    // When: Análisis completo de introspección
    println!("=== COMPREHENSIVE INTROSPECTION ANALYSIS ===");

    let top_level_props = employee.get_property_names().unwrap();
    let top_level_count = employee.count_properties().unwrap();

    println!("Employee top-level properties: {} total", top_level_count);
    for prop in &top_level_props {
        let prop_type = employee.get_property_type(prop).unwrap().unwrap();
        println!("  {}: {}", prop, prop_type);
    }

    let profile_obj = employee.get("profile").unwrap().unwrap();
    let profile_props = profile_obj.get_property_names().unwrap();
    let profile_count = profile_obj.count_properties().unwrap();

    println!("Profile properties: {} total", profile_count);
    for prop in &profile_props {
        let prop_type = profile_obj.get_property_type(prop).unwrap().unwrap();
        println!("  profile.{}: {}", prop, prop_type);
    }

    // Análisis de arrays
    let skills_array = employee.get("skills").unwrap().unwrap();
    println!("Skills type: {}", skills_array.get_type());

    let projects_array = employee.get("projects").unwrap().unwrap();
    println!("Projects type: {}", projects_array.get_type());

    // Then: Verificaciones específicas - CORREGIDO
    assert_eq!(top_level_count, 7); // id, name, email, active, profile, skills, projects (7 en total)
    assert_eq!(
        top_level_props,
        vec!["active", "email", "id", "name", "profile", "projects", "skills"]
    );

    assert_eq!(profile_count, 4); // department, level, remote, start_date
    assert_eq!(
        profile_props,
        vec!["department", "level", "remote", "start_date"]
    );

    assert_eq!(
        employee.get_property_type("id").unwrap(),
        Some("Number".to_string())
    );
    assert_eq!(
        employee.get_property_type("name").unwrap(),
        Some("String".to_string())
    );
    assert_eq!(
        employee.get_property_type("email").unwrap(),
        Some("String".to_string())
    );
    assert_eq!(
        employee.get_property_type("active").unwrap(),
        Some("Bool".to_string())
    );
    assert_eq!(
        employee.get_property_type("profile").unwrap(),
        Some("Object".to_string())
    );
    assert_eq!(
        employee.get_property_type("skills").unwrap(),
        Some("Array".to_string())
    );
    assert_eq!(
        employee.get_property_type("projects").unwrap(),
        Some("Array".to_string())
    );

    // Verificar propiedades específicas
    assert!(employee.has_property("email").unwrap());
    assert!(employee.has_property("profile").unwrap());
    assert!(!employee.has_property("salary").unwrap()); // No existe

    assert!(profile_obj.has_property("department").unwrap());
    assert!(profile_obj.has_property("level").unwrap());
    assert!(!profile_obj.has_property("bonus").unwrap()); // No existe

    println!("✅ Comprehensive introspection scenario test passed");
}
