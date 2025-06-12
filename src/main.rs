use model_manager::{
    AsyncModelManager,
    AsyncDynamicValue,
    ModelManagerFactory,
    DefaultFactory,
    DefaultValue,
};

use tokio::task::LocalSet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Model Manager - Ejemplo Básico de Uso");
    println!("=========================================\n");

    let local_set = LocalSet::new();

    local_set.run_until(async {
        let mut manager = DefaultFactory::create();
        println!("✅ Manager creado exitosamente\n");

        // === 1
        println!("=== EJEMPLO 1: OPERACIONES CRUD BÁSICAS ===");

        // INPUT
        let mut user = DefaultValue::new_object();
        user.set_async("name", DefaultValue::from_str("Ana García")).await?;
        user.set_async("email", DefaultValue::from_str("ana@empresa.com")).await?;
        user.set_async("department", DefaultValue::from_str("Ventas")).await?;
        user.set_async("active", DefaultValue::from_bool(true)).await?;
        user.set_async("age", DefaultValue::from_number(28.0)?).await?;

        println!("INPUT: {}", user.to_string());

        // Insert
        let inserted = manager.insert(
            "users".to_string(),
            Some("usr_001".to_string()),
            user
        ).await?;

        println!("OUTPUT INSERT: {}", inserted.to_string());

        // Get
        let retrieved = manager.get(
            "users".to_string(),
            "usr_001".to_string()
        ).await?;

        println!("OUTPUT GET: {}", retrieved.to_string());

        // Update
        let mut updated_user = DefaultValue::new_object();
        updated_user.set_async("name", DefaultValue::from_str("Ana García Pérez")).await?;
        updated_user.set_async("email", DefaultValue::from_str("ana.garcia@empresa.com")).await?;
        updated_user.set_async("department", DefaultValue::from_str("Ingeniería")).await?;
        updated_user.set_async("active", DefaultValue::from_bool(true)).await?;
        updated_user.set_async("age", DefaultValue::from_number(29.0)?).await?;

        let updated = manager.update(
            "users".to_string(),
            "usr_001".to_string(),
            updated_user
        ).await?;

        println!("OUTPUT UPDATE: {}", updated.to_string());

        // Get All
        let all_users = manager.get_all("users".to_string()).await?;
        println!("OUTPUT GET_ALL: {} usuarios encontrados", all_users.len());
        for (i, user) in all_users.iter().enumerate() {
            println!("  Usuario {}: {}", i + 1, user.to_string());
        }

        // === 2
        println!("\n=== EJEMPLO 2: MÚLTIPLES MODELOS ===");

        for i in 1..=3 {
            let mut product = DefaultValue::new_object();
            product.set_async("name", DefaultValue::from_str(&format!("Producto {}", i))).await?;
            product.set_async("price", DefaultValue::from_number(99.99 * i as f64)?).await?;
            product.set_async("stock", DefaultValue::from_number(100.0 - i as f64 * 10.0)?).await?;

            let inserted_product = manager.insert(
                "products".to_string(),
                None, // ID automático
                product
            ).await?;

            println!("Producto {} creado: {}", i, inserted_product.to_string());
        }

        let users = manager.get_all("users".to_string()).await?;
        let products = manager.get_all("products".to_string()).await?;

        println!("OUTPUT MÚLTIPLES MODELOS:");
        println!("  Usuarios: {}", users.len());
        println!("  Productos: {}", products.len());

        // ===  3
        println!("\n=== EJEMPLO 3: OPERACIONES ASYNC EN DATOS ===");

        let mut complex_data = DefaultValue::new_object();
        complex_data.set_async("title", DefaultValue::from_str("Datos Complejos")).await?;

        let mut items = DefaultValue::new_array();
        for i in 1..=5 {
            let mut item = DefaultValue::new_object();
            item.set_async("id", DefaultValue::from_number(i as f64)?).await?;
            item.set_async("value", DefaultValue::from_str(&format!("Item {}", i))).await?;
            items.push_async(item).await?;
        }

        complex_data.set_async("items", items).await?;

        let complex_inserted = manager.insert(
            "complex".to_string(),
            Some("complex_001".to_string()),
            complex_data
        ).await?;

        println!("OUTPUT COMPLEX DATA: {}", complex_inserted.to_string());

        let title = complex_inserted.get_async("title").await?.unwrap();
        println!("Título extraído: {}", title.as_str().unwrap());

        let items_array = complex_inserted.get_async("items").await?.unwrap();
        if let Some(array) = items_array.as_array_async().await? {
            println!("Items en el array: {}", array.len());
            for (i, item) in array.iter().enumerate() {
                if let Some(value) = item.get_async("value").await? {
                    println!("  Item {}: {}", i + 1, value.as_str().unwrap());
                }
            }
        }

        // === 4
        println!("\n=== EJEMPLO 4: REMOVE ===");

        let removed_user = manager.remove(
            "users".to_string(),
            "usr_001".to_string()
        ).await?;

        println!("OUTPUT REMOVE: {}", removed_user.to_string());

        let users_after_remove = manager.get_all("users".to_string()).await?;
        println!("Usuarios restantes: {}", users_after_remove.len());

        println!("\n🎉 Todos los ejemplos completados exitosamente!");

        Ok::<(), Box<dyn std::error::Error>>(())
    }).await
}