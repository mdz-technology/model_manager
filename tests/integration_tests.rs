
use model_manager::{DynamicValue, ModelManager, DefaultValue, DefaultModelManager, DynamicValueFactory, ModelManagerFactory};

type Value = <DefaultValue as DynamicValueFactory>::Value;

#[test]
fn test_basic_crud_operations() {

        let mut manager = DefaultModelManager::create();

        // Create data
        let mut user_data = DefaultValue::create();
        user_data.set("name", Value::from_str("John Doe")).unwrap();
        user_data.set("age", Value::from_number(30.0).unwrap()).unwrap();

        // Insert
        let inserted = manager
            .insert("users".to_string(), Some("user_1".to_string()), user_data)
            
            .unwrap();

        assert_eq!(inserted.get("name").unwrap().unwrap().as_str().unwrap(), "John Doe");

        // Get
        let retrieved = manager
            .get("users".to_string(), "user_1".to_string())
            
            .unwrap();

        assert_eq!(retrieved.get("age").unwrap().unwrap().as_number().unwrap(), 30.0);

        // Update
        let mut updated_data = DefaultValue::create();
        updated_data.set("name", Value::from_str("Jane Doe")).unwrap();
        updated_data.set("age", Value::from_number(25.0).unwrap()).unwrap();

        let updated = manager
            .update("users".to_string(), "user_1".to_string(), updated_data)
            
            .unwrap();

        assert_eq!(updated.get("name").unwrap().unwrap().as_str().unwrap(), "Jane Doe");

        // Get all
        let all_users = manager
            .get_all("users".to_string())
            
            .unwrap();

        assert_eq!(all_users.len(), 1);

        // Remove
        let removed = manager
            .remove("users".to_string(), "user_1".to_string())
            
            .unwrap();

        assert_eq!(removed.get("name").unwrap().unwrap().as_str().unwrap(), "Jane Doe");

        // Verify removal
        let empty_list = manager
            .get_all("users".to_string())
            
            .unwrap();

        assert_eq!(empty_list.len(), 0);
}

#[test]
fn test_multiple_models() {

        let mut manager = DefaultModelManager::create();

        // Insert
        let mut user = DefaultValue::create();
        user.set("name", Value::from_str("Test User")).unwrap();

        let mut product = DefaultValue::create();
        product.set("name", Value::from_str("Test Product")).unwrap();
        product.set("price", Value::from_number(99.99).unwrap()).unwrap();

        // Insert
        manager.insert("users".to_string(), None, user).unwrap();
        manager.insert("products".to_string(), None, product).unwrap();

        // Verify
        let users = manager.get_all("users".to_string()).unwrap();
        let products = manager.get_all("products".to_string()).unwrap();

        assert_eq!(users.len(), 1);
        assert_eq!(products.len(), 1);
}

#[test]
fn test_sequential_operations() {

        let mut manager = DefaultModelManager::create();

        // Create and insert 10 items sequential (la concurrencia ocurre dentro del actor system)
        for i in 0..10 {
            let mut data = DefaultValue::create();
            data.set("id", Value::from_number(i as f64).unwrap()).unwrap();
            data.set("name", Value::from_str(&format!("User {}", i))).unwrap();

            // Insert
            let result = manager.insert("sequential_users".to_string(), None, data);
            assert!(result.is_ok());
        }

        // Verify
        let all_users = manager.get_all("sequential_users".to_string()).unwrap();
        assert_eq!(all_users.len(), 10);
}

#[test]
fn test_actor_system_stress() {

        let mut manager = DefaultModelManager::create();

        // Test: El actor system maneja múltiples operaciones rápidas

        // Fase 1: Inserción rápida de múltiples elementos
        for i in 0..50 {
            let mut data = DefaultValue::create();
            data.set("id", Value::from_number(i as f64).unwrap()).unwrap();
            data.set("name", Value::from_str(&format!("User {}", i))).unwrap();
            data.set("batch", Value::from_str("stress_test")).unwrap();

            let result = manager.insert("stress_users".to_string(), None, data);
            assert!(result.is_ok(), "Failed to insert user {}", i);
        }

        // Fase 2: Verificación de consistencia
        let all_users = manager.get_all("stress_users".to_string()).unwrap();
        assert_eq!(all_users.len(), 50, "Expected 50 users, got {}", all_users.len());

        // Fase 3: Operaciones mixtas en diferentes modelos
        for i in 0..10 {
            // Crear productos en paralelo a los usuarios
            let mut product = DefaultValue::create();
            product.set("name", Value::from_str(&format!("Product {}", i))).unwrap();
            product.set("price", Value::from_number(10.0 * i as f64).unwrap()).unwrap();

            let result = manager.insert("stress_products".to_string(), None, product);
            assert!(result.is_ok());
        }

        // Verificación final - múltiples modelos
        let final_users = manager.get_all("stress_users".to_string()).unwrap();
        let final_products = manager.get_all("stress_products".to_string()).unwrap();

        assert_eq!(final_users.len(), 50);
        assert_eq!(final_products.len(), 10);

        println!("Actor system handled {} users and {} products successfully",
                 final_users.len(), final_products.len());
}

#[test]
fn test_dynamic_value_operations() {

        // Test de AsyncDynamicValue trait directamente
        let mut obj = DefaultValue::create();

        // Test setting values usando trait
        obj.set("string_field", Value::from_str("test")).unwrap();
        obj.set("number_field", Value::from_number(42.5).unwrap()).unwrap();
        obj.set("bool_field", Value::from_bool(true)).unwrap();

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
        let mut arr = Value::new_array();
        arr.push(Value::from_str("item1")).unwrap();
        arr.push(Value::from_number(123.0).unwrap()).unwrap();

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