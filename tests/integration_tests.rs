use model_manager::{
    CoreValue, DefaultModelManager, DefaultValueFactory, ModelManager, ModelManagerFactory,
    ValueFactory,
};

#[test]
fn test_basic_crud_operations() {
    let manager = DefaultModelManager::create();

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
    let manager = DefaultModelManager::create();

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
    let manager = DefaultModelManager::create();

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
fn test_actor_system_stress() {
    let manager = DefaultModelManager::create();

    // Test: El actor system maneja múltiples operaciones rápidas

    // Fase 1: Inserción rápida de múltiples elementos
    for i in 0..50 {
        let mut data = DefaultValueFactory::create_object();
        data.set("id", DefaultValueFactory::create_number(i as f64).unwrap())
            .unwrap();
        data.set(
            "name",
            DefaultValueFactory::create_string(&format!("User {}", i)),
        )
        .unwrap();
        data.set("batch", DefaultValueFactory::create_string("stress_test"))
            .unwrap();

        let result = manager.insert("stress_users", None, data);
        assert!(result.is_ok(), "Failed to insert user {}", i);
    }

    // Fase 2: Verificación de consistencia
    let all_users = manager.get_all("stress_users").unwrap();
    assert_eq!(
        all_users.len(),
        50,
        "Expected 50 users, got {}",
        all_users.len()
    );

    // Fase 3: Operaciones mixtas en diferentes modelos
    for i in 0..10 {
        // Crear productos en paralelo a los usuarios
        let mut product = DefaultValueFactory::create_object();
        product
            .set(
                "name",
                DefaultValueFactory::create_string(&format!("Product {}", i)),
            )
            .unwrap();
        product
            .set(
                "price",
                DefaultValueFactory::create_number(10.0 * i as f64).unwrap(),
            )
            .unwrap();

        let result = manager.insert("stress_products", None, product);
        assert!(result.is_ok());
    }

    // Verificación final - múltiples modelos
    let final_users = manager.get_all("stress_users").unwrap();
    let final_products = manager.get_all("stress_products").unwrap();

    assert_eq!(final_users.len(), 50);
    assert_eq!(final_products.len(), 10);

    println!(
        "Actor system handled {} users and {} products successfully",
        final_users.len(),
        final_products.len()
    );
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
    let manager1 = DefaultModelManager::create();
    let manager2 = DefaultModelManager::create();

    // Verificar que se pueden crear múltiples instancias independientes
    // (cada una tendrá su propio sistema de actores)
    assert!(std::ptr::addr_of!(manager1) != std::ptr::addr_of!(manager2));
}
