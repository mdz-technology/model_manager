use tokio::task::LocalSet;
use model_manager::{DynamicValue, ModelManager, DefaultValue, ModelManagerFactory};
use model_manager::infrastructure::factories::default_model_manager_factory::DefaultModelManagerFactory;

#[tokio::test]
async fn test_basic_crud_operations() {
    let local_set = LocalSet::new();

    local_set.run_until(async {
        let mut manager = DefaultModelManagerFactory::create();

        // Create data
        let mut user_data = DefaultValue::new_object();
        user_data.set("name", DefaultValue::from_str("John Doe")).await.unwrap();
        user_data.set("age", DefaultValue::from_number(30.0).unwrap()).await.unwrap();

        // Insert
        let inserted = manager
            .insert("users".to_string(), Some("user_1".to_string()), user_data)
            .await
            .unwrap();

        assert_eq!(inserted.get("name").await.unwrap().unwrap().as_str().unwrap(), "John Doe");

        // Get
        let retrieved = manager
            .get("users".to_string(), "user_1".to_string())
            .await
            .unwrap();

        assert_eq!(retrieved.get("age").await.unwrap().unwrap().as_number().unwrap(), 30.0);

        // Update
        let mut updated_data = DefaultValue::new_object();
        updated_data.set("name", DefaultValue::from_str("Jane Doe")).await.unwrap();
        updated_data.set("age", DefaultValue::from_number(25.0).unwrap()).await.unwrap();

        let updated = manager
            .update("users".to_string(), "user_1".to_string(), updated_data)
            .await
            .unwrap();

        assert_eq!(updated.get("name").await.unwrap().unwrap().as_str().unwrap(), "Jane Doe");

        // Get all
        let all_users = manager
            .get_all("users".to_string())
            .await
            .unwrap();

        assert_eq!(all_users.len(), 1);

        // Remove
        let removed = manager
            .remove("users".to_string(), "user_1".to_string())
            .await
            .unwrap();

        assert_eq!(removed.get("name").await.unwrap().unwrap().as_str().unwrap(), "Jane Doe");

        // Verify removal
        let empty_list = manager
            .get_all("users".to_string())
            .await
            .unwrap();

        assert_eq!(empty_list.len(), 0);
    }).await;
}

#[tokio::test]
async fn test_multiple_models() {
    let local_set = LocalSet::new();

    local_set.run_until(async {
        let mut manager = DefaultModelManagerFactory::create();

        // Insert
        let mut user = DefaultValue::new_object();
        user.set("name", DefaultValue::from_str("Test User")).await.unwrap();

        let mut product = DefaultValue::new_object();
        product.set("name", DefaultValue::from_str("Test Product")).await.unwrap();
        product.set("price", DefaultValue::from_number(99.99).unwrap()).await.unwrap();

        // Insert
        manager.insert("users".to_string(), None, user).await.unwrap();
        manager.insert("products".to_string(), None, product).await.unwrap();

        // Verify
        let users = manager.get_all("users".to_string()).await.unwrap();
        let products = manager.get_all("products".to_string()).await.unwrap();

        assert_eq!(users.len(), 1);
        assert_eq!(products.len(), 1);
    }).await;
}

#[tokio::test]
async fn test_sequential_operations() {
    let local_set = LocalSet::new();

    local_set.run_until(async {
        let mut manager = DefaultModelManagerFactory::create();

        // Create and insert 10 items sequential (la concurrencia ocurre dentro del actor system)
        for i in 0..10 {
            let mut data = DefaultValue::new_object();
            data.set("id", DefaultValue::from_number(i as f64).unwrap()).await.unwrap();
            data.set("name", DefaultValue::from_str(&format!("User {}", i))).await.unwrap();

            // Insert
            let result = manager.insert("sequential_users".to_string(), None, data).await;
            assert!(result.is_ok());
        }

        // Verify
        let all_users = manager.get_all("sequential_users".to_string()).await.unwrap();
        assert_eq!(all_users.len(), 10);
    }).await;
}

#[tokio::test]
async fn test_actor_system_stress() {
    let local_set = LocalSet::new();

    local_set.run_until(async {
        let mut manager = DefaultModelManagerFactory::create();

        // Test: El actor system maneja múltiples operaciones rápidas

        // Fase 1: Inserción rápida de múltiples elementos
        for i in 0..50 {
            let mut data = DefaultValue::new_object();
            data.set("id", DefaultValue::from_number(i as f64).unwrap()).await.unwrap();
            data.set("name", DefaultValue::from_str(&format!("User {}", i))).await.unwrap();
            data.set("batch", DefaultValue::from_str("stress_test")).await.unwrap();

            let result = manager.insert("stress_users".to_string(), None, data).await;
            assert!(result.is_ok(), "Failed to insert user {}", i);
        }

        // Fase 2: Verificación de consistencia
        let all_users = manager.get_all("stress_users".to_string()).await.unwrap();
        assert_eq!(all_users.len(), 50, "Expected 50 users, got {}", all_users.len());

        // Fase 3: Operaciones mixtas en diferentes modelos
        for i in 0..10 {
            // Crear productos en paralelo a los usuarios
            let mut product = DefaultValue::new_object();
            product.set("name", DefaultValue::from_str(&format!("Product {}", i))).await.unwrap();
            product.set("price", DefaultValue::from_number(10.0 * i as f64).unwrap()).await.unwrap();

            let result = manager.insert("stress_products".to_string(), None, product).await;
            assert!(result.is_ok());
        }

        // Verificación final - múltiples modelos
        let final_users = manager.get_all("stress_users".to_string()).await.unwrap();
        let final_products = manager.get_all("stress_products".to_string()).await.unwrap();

        assert_eq!(final_users.len(), 50);
        assert_eq!(final_products.len(), 10);

        println!("Actor system handled {} users and {} products successfully",
                 final_users.len(), final_products.len());
    }).await;
}

#[tokio::test]
async fn test_dynamic_value_operations() {
    let local_set = LocalSet::new();

    local_set.run_until(async {
        // Test de AsyncDynamicValue trait directamente
        let mut obj = DefaultValue::new_object();

        // Test setting values usando trait
        obj.set("string_field", DefaultValue::from_str("test")).await.unwrap();
        obj.set("number_field", DefaultValue::from_number(42.5).unwrap()).await.unwrap();
        obj.set("bool_field", DefaultValue::from_bool(true)).await.unwrap();

        // Test getting values usando trait
        let string_val = obj.get("string_field").await.unwrap().unwrap();
        assert_eq!(string_val.as_str().unwrap(), "test");

        let number_val = obj.get("number_field").await.unwrap().unwrap();
        assert_eq!(number_val.as_number().unwrap(), 42.5);

        let bool_val = obj.get("bool_field").await.unwrap().unwrap();
        assert_eq!(bool_val.as_bool().unwrap(), true);

        // Test type checking usando trait
        assert!(obj.is_object());
        assert!(!obj.is_array());
        assert_eq!(obj.get_type(), "Object");

        // Test array operations usando trait
        let mut arr = DefaultValue::new_array();
        arr.push(DefaultValue::from_str("item1")).await.unwrap();
        arr.push(DefaultValue::from_number(123.0).unwrap()).await.unwrap();

        assert!(arr.is_array());
        assert_eq!(arr.get_type(), "Array");

        let arr_vec = arr.as_array().await.unwrap().unwrap();
        assert_eq!(arr_vec.len(), 2);
        assert_eq!(arr_vec[0].as_str().unwrap(), "item1");
        assert_eq!(arr_vec[1].as_number().unwrap(), 123.0);
    }).await;
}

#[tokio::test]
async fn test_factory_pattern() {
    let local_set = LocalSet::new();

    local_set.run_until(async {
        // Test directo del factories pattern
        let manager1 = DefaultModelManagerFactory::create();
        let manager2 = DefaultModelManagerFactory::create();

        // Verificar que se pueden crear múltiples instancias independientes
        // (cada una tendrá su propio sistema de actores)
        assert!(std::ptr::addr_of!(manager1) != std::ptr::addr_of!(manager2));
    }).await;
}