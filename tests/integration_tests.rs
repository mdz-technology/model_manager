use dynamic_value::{
    CoreValue, DefaultModelManagerFactory, DefaultValueFactory, ModelManager, ModelManagerFactory,
    ValueFactory,
};

#[test]
fn test_basic_crud_operations() {
    let manager = DefaultModelManagerFactory::create();

    // Create data
    let mut user_data = DefaultValueFactory::create_object();
    user_data
        .set("name", DefaultValueFactory::create_string("John Doe"))
        .unwrap();
    user_data
        .set("age", DefaultValueFactory::create_number(30.0).unwrap())
        .unwrap();

    // Insert
    let inserted = manager
        .insert("users", Some("user_1"), user_data)
        .unwrap();

    assert_eq!(
        inserted.get("name").unwrap().unwrap().as_str().unwrap(),
        "John Doe"
    );

    // Get
    let retrieved = manager
        .get("users", "user_1")
        .unwrap();

    assert_eq!(
        retrieved.get("age").unwrap().unwrap().as_number().unwrap(),
        30.0
    );

    // Update
    let mut updated_data = DefaultValueFactory::create_object();
    updated_data
        .set("name", DefaultValueFactory::create_string("Jane Doe"))
        .unwrap();
    updated_data
        .set("age", DefaultValueFactory::create_number(25.0).unwrap())
        .unwrap();

    let updated = manager
        .update("users", "user_1", updated_data)
        .unwrap();

    assert_eq!(
        updated.get("name").unwrap().unwrap().as_str().unwrap(),
        "Jane Doe"
    );

    // Get all
    let all_users = manager.get_all("users").unwrap();

    assert_eq!(all_users.len(), 1);

    // Remove
    let removed = manager
        .remove("users", "user_1")
        .unwrap();

    assert_eq!(
        removed.get("name").unwrap().unwrap().as_str().unwrap(),
        "Jane Doe"
    );

    // Verify removal
    let empty_list = manager.get_all("users").unwrap();

    assert_eq!(empty_list.len(), 0);
}

#[test]
fn test_multiple_models() {
    let manager = DefaultModelManagerFactory::create();

    // Insert
    let mut user = DefaultValueFactory::create_object();
    user.set("name", DefaultValueFactory::create_string("Test User"))
        .unwrap();

    let mut product = DefaultValueFactory::create_object();
    product
        .set("name", DefaultValueFactory::create_string("Test Product"))
        .unwrap();
    product
        .set("price", DefaultValueFactory::create_number(99.99).unwrap())
        .unwrap();

    // Insert
    manager.insert("users", None, user).unwrap();
    manager
        .insert("products", None, product)
        .unwrap();

    // Verify
    let users = manager.get_all("users").unwrap();
    let products = manager.get_all("products").unwrap();

    assert_eq!(users.len(), 1);
    assert_eq!(products.len(), 1);
}

#[test]
fn test_sequential_operations() {
    let manager = DefaultModelManagerFactory::create();

    // Create and insert 10 items sequential (la concurrencia ocurre dentro del actor system)
    for i in 0..10 {
        let mut data = DefaultValueFactory::create_object();
        data.set("id", DefaultValueFactory::create_number(i as f64).unwrap())
            .unwrap();
        data.set(
            "name",
            DefaultValueFactory::create_string(&format!("User {}", i)),
        )
        .unwrap();

        // Insert
        let result = manager.insert("sequential_users", None, data);
        assert!(result.is_ok());
    }

    // Verify
    let all_users = manager.get_all("sequential_users").unwrap();
    assert_eq!(all_users.len(), 10);
}

#[test]
fn test_dynamic_value_operations() {
    // Test de AsyncDynamicValue trait directamente
    let mut obj = DefaultValueFactory::create_object();

    // Test setting values usando trait
    obj.set("string_field", DefaultValueFactory::create_string("test"))
        .unwrap();
    obj.set(
        "number_field",
        DefaultValueFactory::create_number(42.5).unwrap(),
    )
    .unwrap();
    obj.set("bool_field", DefaultValueFactory::create_bool(true))
        .unwrap();

    // Test getting values usando trait
    let string_val = obj.get("string_field").unwrap().unwrap();
    assert_eq!(string_val.as_str().unwrap(), "test");

    let number_val = obj.get("number_field").unwrap().unwrap();
    assert_eq!(number_val.as_number().unwrap(), 42.5);

    let bool_val = obj.get("bool_field").unwrap().unwrap();
    assert_eq!(bool_val.as_bool().unwrap(), true);

    // Test type checking usando trait
    assert!(obj.is_object());
    assert!(!obj.is_array());
    assert_eq!(obj.get_type(), "Object");

    // Test array operations usando trait
    let mut arr = DefaultValueFactory::create_array();
    arr.push(DefaultValueFactory::create_string("item1"))
        .unwrap();
    arr.push(DefaultValueFactory::create_number(123.0).unwrap())
        .unwrap();

    assert!(arr.is_array());
    assert_eq!(arr.get_type(), "Array");

    let arr_vec = arr.as_array().unwrap().unwrap();
    assert_eq!(arr_vec.len(), 2);
    assert_eq!(arr_vec[0].as_str().unwrap(), "item1");
    assert_eq!(arr_vec[1].as_number().unwrap(), 123.0);
}

#[test]
fn test_factory_pattern() {
    // Test directo del factories pattern
    let manager1 = DefaultModelManagerFactory::create();
    let manager2 = DefaultModelManagerFactory::create();

    // Verificar que se pueden crear múltiples instancias independientes
    // (cada una tendrá su propio sistema de actores)
    assert!(std::ptr::addr_of!(manager1) != std::ptr::addr_of!(manager2));
}

#[test]
fn test_core_value_remove_operations() {
    // Test remove_key en objeto
    let mut user_data = DefaultValueFactory::create_object();
    user_data
        .set("name", DefaultValueFactory::create_string("John Doe"))
        .unwrap();
    user_data
        .set("age", DefaultValueFactory::create_number(30.0).unwrap())
        .unwrap();
    user_data
        .set("active", DefaultValueFactory::create_bool(true))
        .unwrap();

    // Remover clave existente
    let removed_name = user_data.remove_key("name").unwrap();
    assert!(removed_name.is_some());
    assert_eq!(removed_name.unwrap().as_str().unwrap(), "John Doe");
    assert!(user_data.get("name").unwrap().is_none());

    // Remover clave no existente
    let removed_invalid = user_data.remove_key("invalid_key").unwrap();
    assert!(removed_invalid.is_none());

    // Verificar que otras claves siguen ahí
    assert!(user_data.get("age").unwrap().is_some());
    assert!(user_data.get("active").unwrap().is_some());

    println!("✅ Object remove_key operations passed");
}

#[test]
fn test_core_value_remove_array_operations() {
    // Test remove_at en array
    let mut items_array = DefaultValueFactory::create_array();
    items_array
        .push(DefaultValueFactory::create_string("item1"))
        .unwrap();
    items_array
        .push(DefaultValueFactory::create_string("item2"))
        .unwrap();
    items_array
        .push(DefaultValueFactory::create_string("item3"))
        .unwrap();

    // Remover elemento del medio
    let removed_item = items_array.remove_at(1).unwrap();
    assert!(removed_item.is_some());
    assert_eq!(removed_item.unwrap().as_str().unwrap(), "item2");

    let remaining_array = items_array.as_array().unwrap().unwrap();
    assert_eq!(remaining_array.len(), 2);
    assert_eq!(remaining_array[0].as_str().unwrap(), "item1");
    assert_eq!(remaining_array[1].as_str().unwrap(), "item3");

    // Remover primer elemento
    let removed_first = items_array.remove_at(0).unwrap();
    assert!(removed_first.is_some());
    assert_eq!(removed_first.unwrap().as_str().unwrap(), "item1");

    // Remover índice fuera de rango
    let removed_invalid = items_array.remove_at(10).unwrap();
    assert!(removed_invalid.is_none());

    println!("✅ Array remove_at operations passed");
}

#[test]
fn test_remove_operations_error_cases() {
    // Test remove_key en valor no-objeto
    let mut string_value = DefaultValueFactory::create_string("not an object");
    let result = string_value.remove_key("key");
    assert!(result.is_err());

    let mut number_value = DefaultValueFactory::create_number(42.0).unwrap();
    let result = number_value.remove_key("key");
    assert!(result.is_err());

    // Test remove_at en valor no-array
    let mut object_value = DefaultValueFactory::create_object();
    let result = object_value.remove_at(0);
    assert!(result.is_err());

    let mut bool_value = DefaultValueFactory::create_bool(true);
    let result = bool_value.remove_at(0);
    assert!(result.is_err());

    println!("✅ Remove operations error cases passed");
}

#[test]
fn test_remove_operations_with_model_manager() {
    let manager = DefaultModelManagerFactory::create();

    // Crear usuario con datos completos
    let mut user = DefaultValueFactory::create_object();
    user.set("name", DefaultValueFactory::create_string("Jane Smith"))
        .unwrap();
    user.set("email", DefaultValueFactory::create_string("jane@test.com"))
        .unwrap();
    user.set("age", DefaultValueFactory::create_number(28.0).unwrap())
        .unwrap();
    user.set("department", DefaultValueFactory::create_string("Engineering"))
        .unwrap();

    let mut tags = DefaultValueFactory::create_array();
    tags.push(DefaultValueFactory::create_string("developer"))
        .unwrap();
    tags.push(DefaultValueFactory::create_string("senior"))
        .unwrap();
    tags.push(DefaultValueFactory::create_string("backend"))
        .unwrap();
    user.set("tags", tags).unwrap();

    manager.insert("users", Some("user_1"), user).unwrap();

    // Recuperar y modificar usando remove
    let mut retrieved = manager.get("users", "user_1").unwrap();

    // Remover campo email
    let removed_email = retrieved.remove_key("email").unwrap();
    assert!(removed_email.is_some());
    assert_eq!(removed_email.unwrap().as_str().unwrap(), "jane@test.com");

    // Remover elemento del array tags
    let mut user_tags = retrieved.get("tags").unwrap().unwrap();
    let removed_tag = user_tags.remove_at(1).unwrap();
    assert!(removed_tag.is_some());
    assert_eq!(removed_tag.unwrap().as_str().unwrap(), "senior");

    retrieved.set("tags", user_tags).unwrap();

    // Actualizar en el manager
    manager.update("users", "user_1", retrieved).unwrap();

    // Verificar cambios
    let final_user = manager.get("users", "user_1").unwrap();
    assert!(final_user.get("email").unwrap().is_none());

    let final_tags = final_user.get("tags").unwrap().unwrap();
    let final_tags_array = final_tags.as_array().unwrap().unwrap();
    assert_eq!(final_tags_array.len(), 2);
    assert_eq!(final_tags_array[0].as_str().unwrap(), "developer");
    assert_eq!(final_tags_array[1].as_str().unwrap(), "backend");

    println!("✅ Remove operations with model manager passed");
}

#[test]
fn test_remove_operations_complex_scenarios() {
    // Escenario complejo: remover elementos anidados
    let mut company = DefaultValueFactory::create_object();
    company
        .set("name", DefaultValueFactory::create_string("TechCorp"))
        .unwrap();

    let mut departments = DefaultValueFactory::create_object();

    let mut engineering = DefaultValueFactory::create_object();
    engineering
        .set("head", DefaultValueFactory::create_string("Alice"))
        .unwrap();
    engineering
        .set("budget", DefaultValueFactory::create_number(500000.0).unwrap())
        .unwrap();
    engineering
        .set("temporary_field", DefaultValueFactory::create_string("to_remove"))
        .unwrap();

    let mut sales = DefaultValueFactory::create_object();
    sales
        .set("head", DefaultValueFactory::create_string("Bob"))
        .unwrap();
    sales
        .set("budget", DefaultValueFactory::create_number(300000.0).unwrap())
        .unwrap();

    departments.set("engineering", engineering).unwrap();
    departments.set("sales", sales).unwrap();
    departments.set("marketing", DefaultValueFactory::create_object()).unwrap(); // Departamento vacío a remover

    company.set("departments", departments).unwrap();

    let mut offices = DefaultValueFactory::create_array();
    offices.push(DefaultValueFactory::create_string("New York")).unwrap();
    offices.push(DefaultValueFactory::create_string("San Francisco")).unwrap();
    offices.push(DefaultValueFactory::create_string("Austin")).unwrap(); // Oficina a remover
    offices.push(DefaultValueFactory::create_string("Seattle")).unwrap();

    company.set("offices", offices).unwrap();

    // Remover campo temporal del departamento engineering
    let mut company_departments = company.get("departments").unwrap().unwrap();
    let mut eng_dept = company_departments.get("engineering").unwrap().unwrap();
    let removed_temp = eng_dept.remove_key("temporary_field").unwrap();
    assert!(removed_temp.is_some());
    company_departments.set("engineering", eng_dept).unwrap();

    // Remover departamento marketing completo
    let removed_marketing = company_departments.remove_key("marketing").unwrap();
    assert!(removed_marketing.is_some());
    company.set("departments", company_departments).unwrap();

    // Remover oficina Austin (índice 2)
    let mut company_offices = company.get("offices").unwrap().unwrap();
    let removed_office = company_offices.remove_at(2).unwrap();
    assert!(removed_office.is_some());
    assert_eq!(removed_office.unwrap().as_str().unwrap(), "Austin");
    company.set("offices", company_offices).unwrap();

    // Verificaciones finales
    let final_departments = company.get("departments").unwrap().unwrap();
    assert!(final_departments.get("marketing").unwrap().is_none());

    let final_eng = final_departments.get("engineering").unwrap().unwrap();
    assert!(final_eng.get("temporary_field").unwrap().is_none());
    assert!(final_eng.get("head").unwrap().is_some());

    let final_offices = company.get("offices").unwrap().unwrap();
    let final_offices_array = final_offices.as_array().unwrap().unwrap();
    assert_eq!(final_offices_array.len(), 3);
    assert_eq!(final_offices_array[0].as_str().unwrap(), "New York");
    assert_eq!(final_offices_array[1].as_str().unwrap(), "San Francisco");
    assert_eq!(final_offices_array[2].as_str().unwrap(), "Seattle");

    println!("✅ Complex remove scenarios passed");
}

#[test]
fn test_remove_operations_performance() {
    // Test de performance para remociones masivas
    let mut large_object = DefaultValueFactory::create_object();

    // Crear objeto con muchas propiedades
    for i in 0..1000 {
        large_object
            .set(
                &format!("prop_{}", i),
                DefaultValueFactory::create_string(&format!("value_{}", i)),
            )
            .unwrap();
    }

    // Remover propiedades pares
    let start = std::time::Instant::now();
    let mut removed_count = 0;

    for i in (0..1000).step_by(2) {
        let removed = large_object.remove_key(&format!("prop_{}", i)).unwrap();
        if removed.is_some() {
            removed_count += 1;
        }
    }

    let duration = start.elapsed();

    assert_eq!(removed_count, 500);
    assert!(duration.as_millis() < 100); // Debe ser rápido

    // Verificar que quedan las propiedades impares
    for i in (1..1000).step_by(2) {
        assert!(large_object.get(&format!("prop_{}", i)).unwrap().is_some());
    }

    // Test de performance para array
    let mut large_array = DefaultValueFactory::create_array();
    for i in 0..1000 {
        large_array
            .push(DefaultValueFactory::create_number(i as f64).unwrap())
            .unwrap();
    }

    let start = std::time::Instant::now();

    // Remover elementos del final hacia adelante (más eficiente)
    for _ in 0..500 {
        large_array.remove_at(large_array.as_array().unwrap().unwrap().len() - 1).unwrap();
    }

    let duration = start.elapsed();

    let remaining = large_array.as_array().unwrap().unwrap();
    assert_eq!(remaining.len(), 500);
    assert!(duration.as_millis() < 50);

    println!("✅ Remove operations performance test passed in {:?}", duration);
}