use dynamic_value::{ModelManager, ModelManagerFactory, DefaultModelManagerFactory, DefaultValueFactory, ValueFactory, CoreValue};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Model Manager - Ejemplo Básico de Uso");
    println!("=========================================\n");

    let manager = DefaultModelManagerFactory::create();
    println!("✅ Manager creado exitosamente\n");

    println!("=== EJEMPLO 1: OPERACIONES CRUD BÁSICAS ===");

    let mut user = DefaultValueFactory::create_object();
    user.set("name", DefaultValueFactory::create_string("Ana García"))?;
    user.set("email", DefaultValueFactory::create_string("ana@empresa.com"))?;
    user.set("department", DefaultValueFactory::create_string("Ventas"))?;
    user.set("active", DefaultValueFactory::create_bool(true))?;
    user.set("age", DefaultValueFactory::create_number(28.0)?)?;

    println!("INPUT: {}", user.to_string());

    let inserted = manager.insert(
        "users",
        Some("usr_001"),
        user
    )?;

    println!("OUTPUT INSERT: {}", inserted.to_string());

    let retrieved = manager.get(
        "users",
        "usr_001"
    )?;

    println!("OUTPUT GET: {}", retrieved.to_string());

    let mut updated_user = DefaultValueFactory::create_object();
    updated_user.set("name", DefaultValueFactory::create_string("Ana García Pérez"))?;
    updated_user.set("email", DefaultValueFactory::create_string("ana.garcia@empresa.com"))?;
    updated_user.set("department", DefaultValueFactory::create_string("Ingeniería"))?;
    updated_user.set("active", DefaultValueFactory::create_bool(true))?;
    updated_user.set("age", DefaultValueFactory::create_number(29.0)?)?;

    let updated = manager.update(
        "users",
        "usr_001",
        updated_user
    )?;

    println!("OUTPUT UPDATE: {}", updated.to_string());

    let all_users = manager.get_all("users")?;
    println!("OUTPUT GET_ALL: {} usuarios encontrados", all_users.len());
    for (i, user) in all_users.iter().enumerate() {
        println!("  Usuario {}: {}", i + 1, user.to_string());
    }

    println!("\n=== EJEMPLO 2: MÚLTIPLES MODELOS ===");

    for i in 1..=3 {
        let mut product = DefaultValueFactory::create_object();
        product.set("name", DefaultValueFactory::create_string(&format!("Producto {}", i)))?;
        product.set("price", DefaultValueFactory::create_number(99.99 * i as f64)?)?;
        product.set("stock", DefaultValueFactory::create_number(100.0 - i as f64 * 10.0)?)?;

        let inserted_product = manager.insert(
            "products",
            None,
            product
        )?;

        println!("Producto {} creado: {}", i, inserted_product.to_string());
    }

    let users = manager.get_all("users")?;
    let products = manager.get_all("products")?;

    println!("OUTPUT MÚLTIPLES MODELOS:");
    println!("  Usuarios: {}", users.len());
    println!("  Productos: {}", products.len());

    println!("\n=== EJEMPLO 3: OPERACIONES EN DATOS ===");

    let mut complex_data = DefaultValueFactory::create_object();
    complex_data.set("title", DefaultValueFactory::create_string("Datos Complejos"))?;

    let mut items = DefaultValueFactory::create_array();
    for i in 1..=5 {
        let mut item = DefaultValueFactory::create_object();
        item.set("id", DefaultValueFactory::create_number(i as f64)?)?;
        item.set("value", DefaultValueFactory::create_string(&format!("Item {}", i)))?;
        items.push(item)?;
    }

    complex_data.set("items", items)?;

    let complex_inserted = manager.insert(
        "complex",
        Some("complex_001"),
        complex_data
    )?;

    println!("OUTPUT COMPLEX DATA: {}", complex_inserted.to_string());

    let title = complex_inserted.get("title")?.unwrap();
    println!("Título extraído: {}", title.as_str().unwrap());

    let items_array = complex_inserted.get("items")?.unwrap();
    if let Some(array) = items_array.as_array()? {
        println!("Items en el array: {}", array.len());
        for (i, item) in array.iter().enumerate() {
            if let Some(value) = item.get("value")? {
                println!("  Item {}: {}", i + 1, value.as_str().unwrap());
            }
        }
    }

    println!("\n=== EJEMPLO 4: REMOVE ===");

    let removed_user = manager.remove(
        "users",
        "usr_001"
    )?;

    println!("OUTPUT REMOVE: {}", removed_user.to_string());

    let users_after_remove = manager.get_all("users")?;
    println!("Usuarios restantes: {}", users_after_remove.len());

    println!("\n🎉 Todos los ejemplos completados exitosamente!");

    Ok(())
}