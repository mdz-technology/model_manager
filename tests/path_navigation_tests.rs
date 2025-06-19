use model_manager::{
    DefaultModelManager, DefaultValue, DynamicValue, DynamicValueFactory, ModelError, ModelManager,
    ModelManagerFactory, ModelResult,
};
use tokio::task::LocalSet;

type Value = <DefaultValue as DynamicValueFactory>::Value;
type Manager = <DefaultModelManager as ModelManagerFactory<Value>>::Manager;

#[tokio::test]
async fn test_get_by_path_single_level() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // ✅ CAMBIO: Usar Value::new_object() en lugar de Value::new_object()
            let mut data = Value::new_object();
            data.set("name", Value::from_str("John")).await.unwrap();
            data.set("age", Value::from_number(30.0).unwrap())
                .await
                .unwrap();
            data.set("active", Value::from_bool(true)).await.unwrap();

            // When: Acceder por path de un nivel
            let name_result = data.get_by_path("name").await.unwrap();
            let age_result = data.get_by_path("age").await.unwrap();
            let active_result = data.get_by_path("active").await.unwrap();

            // Then: Valores correctos retornados
            assert!(name_result.is_some());
            assert_eq!(name_result.unwrap().as_str().unwrap(), "John");

            assert!(age_result.is_some());
            assert_eq!(age_result.unwrap().as_number().unwrap(), 30.0);

            assert!(active_result.is_some());
            assert_eq!(active_result.unwrap().as_bool().unwrap(), true);
        })
        .await;
}

#[tokio::test]
async fn test_get_by_path_nested_levels() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // ✅ CAMBIO: Usar Value en lugar de DefaultValue
            let mut root = Value::new_object();

            let mut level1 = Value::new_object();
            let mut level2 = Value::new_object();
            level2
                .set("value", Value::from_str("deep_value"))
                .await
                .unwrap();
            level1.set("level2", level2).await.unwrap();
            root.set("level1", level1).await.unwrap();

            // When: Acceder por path anidado
            let result = root.get_by_path("level1.level2.value").await.unwrap();

            // Then: Valor correcto retornado
            assert!(result.is_some());
            assert_eq!(result.unwrap().as_str().unwrap(), "deep_value");
        })
        .await;
}

#[tokio::test]
async fn test_get_by_path_nonexistent_paths() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Objeto simple
            let mut data = Value::new_object();
            data.set("existing", Value::from_str("value"))
                .await
                .unwrap();

            // When: Acceder paths que no existen
            let nonexistent_field = data.get_by_path("nonexistent").await.unwrap();
            let nonexistent_nested = data.get_by_path("existing.nonexistent").await.unwrap();
            let completely_invalid = data.get_by_path("a.b.c.d.e").await.unwrap();

            // Then: None retornado para todos
            assert!(nonexistent_field.is_none());
            assert!(nonexistent_nested.is_none());
            assert!(completely_invalid.is_none());
        })
        .await;
}

#[tokio::test]
async fn test_get_by_path_empty_path() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Objeto con datos
            let mut data = Value::new_object();
            data.set("field", Value::from_str("value")).await.unwrap();

            // When: Path vacío
            let result = data.get_by_path("").await.unwrap();

            // Then: Objeto completo retornado
            assert!(result.is_some());
            let returned_obj = result.unwrap();
            assert!(returned_obj.is_object());

            // Verificar que contiene los datos originales
            let field_value = returned_obj.get("field").await.unwrap().unwrap();
            assert_eq!(field_value.as_str().unwrap(), "value");
        })
        .await;
}

#[tokio::test]
async fn test_has_path_existing_paths() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Estructura con múltiples niveles
            let mut root = Value::new_object();
            root.set("simple", Value::from_str("value")).await.unwrap();

            let mut nested = Value::new_object();
            nested
                .set("inner", Value::from_number(42.0).unwrap())
                .await
                .unwrap();
            root.set("nested", nested).await.unwrap();

            // When: Verificar paths existentes
            let has_simple = root.has_path("simple").await.unwrap();
            let has_nested = root.has_path("nested").await.unwrap();
            let has_deep = root.has_path("nested.inner").await.unwrap();

            // Then: Todos deben existir
            assert!(has_simple);
            assert!(has_nested);
            assert!(has_deep);
        })
        .await;
}

#[tokio::test]
async fn test_has_path_nonexistent_paths() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Objeto simple
            let mut data = Value::new_object();
            data.set("exists", Value::from_str("value")).await.unwrap();

            // When: Verificar paths que no existen
            let has_nonexistent = data.has_path("nonexistent").await.unwrap();
            let has_partial = data.has_path("exists.nonexistent").await.unwrap();
            let has_deep_invalid = data.has_path("a.b.c").await.unwrap();

            // Then: Todos deben ser false
            assert!(!has_nonexistent);
            assert!(!has_partial);
            assert!(!has_deep_invalid);
        })
        .await;
}

#[tokio::test]
async fn test_set_by_path_empty_path_error() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Objeto cualquiera
            let mut data = Value::new_object();

            // When: Intentar establecer con path vacío
            let result = data.set_by_path("", Value::from_str("value")).await;

            // Then: Error retornado
            assert!(result.is_err());
            if let Err(ModelError::InvalidData(msg)) = result {
                assert!(msg.contains("Empty path"));
            } else {
                panic!("Expected InvalidData error");
            }
        })
        .await;
}

#[tokio::test]
async fn test_path_navigation_with_different_types() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Objeto con diferentes tipos de datos
            let mut data = Value::new_object();
            data.set("string_val", Value::from_str("text"))
                .await
                .unwrap();
            data.set("number_val", Value::from_number(123.45).unwrap())
                .await
                .unwrap();
            data.set("bool_val", Value::from_bool(false)).await.unwrap();

            let mut array_val = Value::new_array();
            array_val.push(Value::from_str("item1")).await.unwrap();
            array_val.push(Value::from_str("item2")).await.unwrap();
            data.set("array_val", array_val).await.unwrap();

            // When: Acceder cada tipo por path
            let string_result = data.get_by_path("string_val").await.unwrap().unwrap();
            let number_result = data.get_by_path("number_val").await.unwrap().unwrap();
            let bool_result = data.get_by_path("bool_val").await.unwrap().unwrap();
            let array_result = data.get_by_path("array_val").await.unwrap().unwrap();

            // Then: Tipos correctos retornados
            assert_eq!(string_result.as_str().unwrap(), "text");
            assert_eq!(number_result.as_number().unwrap(), 123.45);
            assert_eq!(bool_result.as_bool().unwrap(), false);
            assert!(array_result.is_array());
        })
        .await;
}

#[tokio::test]
async fn test_path_navigation_max_depth_by_levels() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // ✅ CONSTRUCCIÓN CORRECTA: Crear estructura explícitamente
            let mut root = Value::new_object();

            // Crear cada nivel explícitamente
            let mut level10 = Value::new_object();
            level10
                .set("bottom", Value::from_str("bottom"))
                .await
                .unwrap();

            let mut level9 = Value::new_object();
            level9.set("level10", level10).await.unwrap();

            let mut level8 = Value::new_object();
            level8.set("level9", level9).await.unwrap();

            let mut level7 = Value::new_object();
            level7.set("level8", level8).await.unwrap();

            let mut level6 = Value::new_object();
            level6.set("level7", level7).await.unwrap();

            let mut level5 = Value::new_object();
            level5.set("level6", level6).await.unwrap();

            let mut level4 = Value::new_object();
            level4.set("level5", level5).await.unwrap();

            let mut level3 = Value::new_object();
            level3.set("level4", level4).await.unwrap();

            let mut level2 = Value::new_object();
            level2.set("level3", level3).await.unwrap();

            let mut level1 = Value::new_object();
            level1.set("level2", level2).await.unwrap();

            root.set("level1", level1).await.unwrap();

            // When: Navegar al nivel más profundo
            let path =
                "level1.level2.level3.level4.level5.level6.level7.level8.level9.level10.bottom";
            let result = root.get_by_path(&path).await.unwrap();

            // Then: Valor correcto en el fondo
            assert!(result.is_some());
            assert_eq!(result.unwrap().as_str().unwrap(), "bottom");
        })
        .await;
}

// ================================================================================
// UNIT TESTS - SerdeDynamicValue Specific Extensions
// ================================================================================

#[tokio::test]
async fn test_serde_dynamic_value_equals() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Dos valores idénticos
            let value1 = Value::from_str("test_value");
            let value2 = Value::from_str("test_value");
            let value3 = Value::from_str("different_value");

            // When: Comparar valores
            let are_equal = value1.equals(&value2).await.unwrap();
            let are_different = value1.equals(&value3).await.unwrap();

            // Then: Comparación correcta
            assert!(are_equal);
            assert!(!are_different);
        })
        .await;
}

#[tokio::test]
async fn test_serde_get_path_parts() {
    // Given: Diferentes tipos de paths
    let simple_path = "field";
    let nested_path = "level1.level2.level3";
    let complex_path = "user.profile.settings.theme";

    // When: Extraer partes del path
    let simple_parts = Value::get_path_parts(simple_path);
    let nested_parts = Value::get_path_parts(nested_path);
    let complex_parts = Value::get_path_parts(complex_path);

    // Then: Partes correctas extraídas
    assert_eq!(simple_parts, vec!["field"]);
    assert_eq!(nested_parts, vec!["level1", "level2", "level3"]);
    assert_eq!(complex_parts, vec!["user", "profile", "settings", "theme"]);
}

#[tokio::test]
async fn test_serde_is_valid_path() {
    // Given: Diferentes tipos de paths
    let valid_simple = "field";
    let valid_nested = "level1.level2";
    let invalid_empty = "";
    let invalid_empty_part = "level1..level3";
    let invalid_start_dot = ".level1";
    let invalid_end_dot = "level1.";

    // When: Validar paths
    let simple_valid = Value::is_valid_path(valid_simple);
    let nested_valid = Value::is_valid_path(valid_nested);
    let empty_invalid = Value::is_valid_path(invalid_empty);
    let empty_part_invalid = Value::is_valid_path(invalid_empty_part);
    let start_dot_invalid = Value::is_valid_path(invalid_start_dot);
    let end_dot_invalid = Value::is_valid_path(invalid_end_dot);

    // Then: Validación correcta
    assert!(simple_valid);
    assert!(nested_valid);
    assert!(!empty_invalid);
    assert!(!empty_part_invalid);
    assert!(!start_dot_invalid);
    assert!(!end_dot_invalid);
}

// ================================================================================
// UNIT TESTS - Edge Cases y Error Handling
// ================================================================================

#[tokio::test]
async fn test_path_with_special_characters() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Objeto con campos que tienen caracteres especiales
            let mut data = Value::new_object();
            data.set("field-with-dash", Value::from_str("dash_value"))
                .await
                .unwrap();
            data.set("field_with_underscore", Value::from_str("underscore_value"))
                .await
                .unwrap();
            data.set("field with spaces", Value::from_str("spaces_value"))
                .await
                .unwrap();

            // When: Acceder campos con caracteres especiales
            let dash_result = data.get_by_path("field-with-dash").await.unwrap();
            let underscore_result = data.get_by_path("field_with_underscore").await.unwrap();
            let spaces_result = data.get_by_path("field with spaces").await.unwrap();

            // Then: Valores correctos retornados
            assert!(dash_result.is_some());
            assert_eq!(dash_result.unwrap().as_str().unwrap(), "dash_value");

            assert!(underscore_result.is_some());
            assert_eq!(
                underscore_result.unwrap().as_str().unwrap(),
                "underscore_value"
            );

            assert!(spaces_result.is_some());
            assert_eq!(spaces_result.unwrap().as_str().unwrap(), "spaces_value");
        })
        .await;
}

#[tokio::test]
async fn test_path_navigation_through_array() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Estructura con array en el path
            let mut root = Value::new_object();

            let mut array = Value::new_array();
            array.push(Value::from_str("item")).await.unwrap();
            root.set("array_field", array).await.unwrap();

            // When: Intentar navegar a través del array
            let result = root.get_by_path("array_field.0").await.unwrap();

            // Then: None retornado (arrays no soportan navegación por index como path)
            // Nota: Esta es una limitación conocida de la implementación actual
            assert!(result.is_none());
        })
        .await;
}

#[tokio::test]
async fn test_path_navigation_through_non_object() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Estructura donde intentamos navegar a través de un valor primitivo
            let mut data = Value::new_object();
            data.set("string_field", Value::from_str("just_a_string"))
                .await
                .unwrap();

            // When: Intentar navegar a través del string
            let result = data.get_by_path("string_field.nonexistent").await.unwrap();

            // Then: None retornado
            assert!(result.is_none());
        })
        .await;
}

#[tokio::test]
async fn test_empty_values_in_path_navigation() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Objeto con valores vacíos
            let mut data = Value::new_object();
            data.set("empty_string", Value::from_str("")).await.unwrap();
            data.set("zero_number", Value::from_number(0.0).unwrap())
                .await
                .unwrap();
            data.set("false_bool", Value::from_bool(false))
                .await
                .unwrap();

            // When: Acceder valores "vacíos" por path
            let empty_str = data.get_by_path("empty_string").await.unwrap();
            let zero_num = data.get_by_path("zero_number").await.unwrap();
            let false_bool = data.get_by_path("false_bool").await.unwrap();

            // Then: Valores "vacíos" siguen siendo valores válidos
            assert!(empty_str.is_some());
            assert_eq!(empty_str.unwrap().as_str().unwrap(), "");

            assert!(zero_num.is_some());
            assert_eq!(zero_num.unwrap().as_number().unwrap(), 0.0);

            assert!(false_bool.is_some());
            assert_eq!(false_bool.unwrap().as_bool().unwrap(), false);

            // When: Verificar que has_path también funciona con valores "vacíos"
            let has_empty = data.has_path("empty_string").await.unwrap();
            let has_zero = data.has_path("zero_number").await.unwrap();
            let has_false = data.has_path("false_bool").await.unwrap();

            // Then: has_path retorna true para valores vacíos válidos
            assert!(has_empty);
            assert!(has_zero);
            assert!(has_false);
        })
        .await;
}

// ================================================================================
// UNIT TESTS - Performance y Stress
// ================================================================================

#[tokio::test]
async fn test_path_navigation_performance_simple() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Objeto simple para test de performance
            let mut data = Value::new_object();
            for i in 0..100 {
                data.set(
                    &format!("field_{}", i),
                    Value::from_number(i as f64).unwrap(),
                )
                .await
                .unwrap();
            }

            // When: Acceso rápido múltiple
            let start = std::time::Instant::now();
            for i in 0..100 {
                let result = data.get_by_path(&format!("field_{}", i)).await.unwrap();
                assert!(result.is_some());
                assert_eq!(result.unwrap().as_number().unwrap(), i as f64);
            }
            let duration = start.elapsed();

            // Then: Performance aceptable (< 10ms para 100 accesos)
            assert!(
                duration.as_millis() < 10,
                "Path navigation too slow: {:?}",
                duration
            );
            println!("✅ 100 path accesses completed in {:?}", duration);
        })
        .await;
}

#[tokio::test]
async fn test_deep_path_performance() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Estructura profunda (20 niveles)
            let mut current = Value::new_object();
            current.set("value", Value::from_str("deep")).await.unwrap();

            for i in (1..=20).rev() {
                let mut parent = Value::new_object();
                parent.set(&format!("level{}", i), current).await.unwrap();
                current = parent;
            }

            // When: Navegación profunda múltiple
            let start = std::time::Instant::now();
            let path = (1..=20)
                .map(|i| format!("level{}", i))
                .collect::<Vec<_>>()
                .join(".");

            for _ in 0..10 {
                let result = current.get_by_path(&path).await.unwrap();
                assert!(result.is_some());
            }
            let duration = start.elapsed();

            // Then: Performance aceptable para navegación profunda
            assert!(
                duration.as_millis() < 50,
                "Deep path navigation too slow: {:?}",
                duration
            );
            println!(
                "✅ 10 deep path accesses (20 levels) completed in {:?}",
                duration
            );
        })
        .await;
}

// ================================================================================
// UNIT TESTS - Memory Safety
// ================================================================================

#[tokio::test]
async fn test_path_navigation_memory_safety() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Estructura que será clonada múltiples veces durante navegación
            let mut data = Value::new_object();

            // Crear estructura con contenido que se clonará durante navegación
            let large_string = "x".repeat(1000); // 1KB string
            data.set("large_field", Value::from_str(&large_string))
                .await
                .unwrap();

            let mut nested = Value::new_object();
            nested
                .set("inner_large", Value::from_str(&large_string))
                .await
                .unwrap();
            data.set("nested", nested).await.unwrap();

            // When: Múltiples accesos que causan clonación
            for _ in 0..100 {
                let result1 = data.get_by_path("large_field").await.unwrap();
                let result2 = data.get_by_path("nested.inner_large").await.unwrap();

                assert!(result1.is_some());
                assert!(result2.is_some());

                // Verificar que los valores son correctos después de clonación
                assert_eq!(result1.unwrap().as_str().unwrap().len(), 1000);
                assert_eq!(result2.unwrap().as_str().unwrap().len(), 1000);
            }

            // Then: No memory leaks (verificado implícitamente por Rust)
            println!("✅ Memory safety maintained during cloning operations");
        })
        .await;
}

// ================================================================================
// UNIT TESTS - Trait Contract Verification
// ================================================================================

#[tokio::test]
async fn test_trait_contract_path_methods() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Función genérica que usa el trait
            async fn test_path_contract<T: DynamicValue>(mut value: T) -> ModelResult<bool> {
                // Set a value
                value.set("test", T::from_str("contract_test")).await?;

                // Test path navigation through trait
                let exists = value.has_path("test").await?;
                let retrieved = value.get_by_path("test").await?;

                // Verify results
                if let Some(val) = retrieved {
                    Ok(exists && val.as_str().unwrap() == "contract_test")
                } else {
                    Ok(false)
                }
            }

            // When: Usar la función con diferentes implementaciones del trait
            let serde_value = Value::new_object();
            let result = test_path_contract(serde_value).await.unwrap();

            // Then: Contrato del trait respetado
            assert!(result);
            println!("✅ Trait contract for path methods verified");
        })
        .await;
}

#[tokio::test]
async fn test_set_by_path_simple() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Empty object
            let mut data = Value::new_object();

            // When: Set simple path
            let result = data.set_by_path("name", Value::from_str("John Doe")).await;

            // Then: Success and value is set
            assert!(result.is_ok());

            let retrieved = data.get("name").await.unwrap().unwrap();
            assert_eq!(retrieved.as_str().unwrap(), "John Doe");

            println!("✅ Simple set_by_path test passed");
        })
        .await;
}

#[tokio::test]
async fn test_set_by_path_nested_creation() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Empty object
            let mut data = Value::new_object();

            // When: Set nested path that doesn't exist
            let result = data
                .set_by_path("user.profile.name", Value::from_str("Jane Smith"))
                .await;

            // Then: Success and nested structure is created
            assert!(result.is_ok());

            // Verify nested structure was created
            let user = data.get("user").await.unwrap().unwrap();
            assert!(user.is_object());

            let profile = user.get("profile").await.unwrap().unwrap();
            assert!(profile.is_object());

            let name = profile.get("name").await.unwrap().unwrap();
            assert_eq!(name.as_str().unwrap(), "Jane Smith");

            // Verify using get_by_path
            let retrieved = data
                .get_by_path("user.profile.name")
                .await
                .unwrap()
                .unwrap();
            assert_eq!(retrieved.as_str().unwrap(), "Jane Smith");

            println!("✅ Nested creation test passed");
        })
        .await;
}

#[tokio::test]
async fn test_set_by_path_deep_nesting() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Empty object
            let mut data = Value::new_object();

            // When: Set very deep nested path
            let deep_path = "level1.level2.level3.level4.level5.value";
            let result = data
                .set_by_path(deep_path, Value::from_str("deep_value"))
                .await;

            // Then: Success and deep structure is created
            assert!(result.is_ok());

            // Verify deep access works
            let retrieved = data.get_by_path(deep_path).await.unwrap().unwrap();
            assert_eq!(retrieved.as_str().unwrap(), "deep_value");

            // Verify intermediate objects exist
            assert!(data.has_path("level1").await.unwrap());
            assert!(data.has_path("level1.level2").await.unwrap());
            assert!(data.has_path("level1.level2.level3").await.unwrap());
            assert!(data.has_path("level1.level2.level3.level4").await.unwrap());
            assert!(data
                .has_path("level1.level2.level3.level4.level5")
                .await
                .unwrap());

            println!("✅ Deep nesting test passed");
        })
        .await;
}

#[tokio::test]
async fn test_set_by_path_overwrite_existing() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Object with existing nested structure
            let mut data = Value::new_object();

            // Create initial structure
            data.set_by_path("user.name", Value::from_str("Old Name"))
                .await
                .unwrap();
            data.set_by_path("user.age", Value::from_number(25.0).unwrap())
                .await
                .unwrap();

            // When: Overwrite existing value
            let result = data
                .set_by_path("user.name", Value::from_str("New Name"))
                .await;

            // Then: Success and value is updated
            assert!(result.is_ok());

            let name = data.get_by_path("user.name").await.unwrap().unwrap();
            assert_eq!(name.as_str().unwrap(), "New Name");

            // Verify other values are preserved
            let age = data.get_by_path("user.age").await.unwrap().unwrap();
            assert_eq!(age.as_number().unwrap(), 25.0);

            println!("✅ Overwrite existing test passed");
        })
        .await;
}

#[tokio::test]
async fn test_set_by_path_replace_non_object() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Object with non-object value
            let mut data = Value::new_object();
            data.set("user", Value::from_str("not an object"))
                .await
                .unwrap();

            // When: Try to set nested path on non-object
            let result = data.set_by_path("user.name", Value::from_str("John")).await;

            // Then: Success - non-object is replaced with object
            assert!(result.is_ok());

            let user = data.get("user").await.unwrap().unwrap();
            assert!(user.is_object());

            let name = user.get("name").await.unwrap().unwrap();
            assert_eq!(name.as_str().unwrap(), "John");

            println!("✅ Replace non-object test passed");
        })
        .await;
}

#[tokio::test]
async fn test_set_by_path_different_data_types() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Empty object
            let mut data = Value::new_object();

            // When: Set different data types via paths
            data.set_by_path("config.string_val", Value::from_str("test"))
                .await
                .unwrap();
            data.set_by_path("config.number_val", Value::from_number(42.5).unwrap())
                .await
                .unwrap();
            data.set_by_path("config.bool_val", Value::from_bool(true))
                .await
                .unwrap();

            // Then: All values are set correctly
            let string_val = data
                .get_by_path("config.string_val")
                .await
                .unwrap()
                .unwrap();
            assert_eq!(string_val.as_str().unwrap(), "test");

            let number_val = data
                .get_by_path("config.number_val")
                .await
                .unwrap()
                .unwrap();
            assert_eq!(number_val.as_number().unwrap(), 42.5);

            let bool_val = data.get_by_path("config.bool_val").await.unwrap().unwrap();
            assert_eq!(bool_val.as_bool().unwrap(), true);

            println!("✅ Different data types test passed");
        })
        .await;
}

#[tokio::test]
async fn test_set_by_path_error_cases() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            let mut data = Value::new_object();

            // Test empty path
            let result = data.set_by_path("", Value::from_str("value")).await;
            assert!(result.is_err());
            if let Err(ModelError::InvalidData(msg)) = result {
                assert!(msg.contains("Empty path"));
            }

            // Test invalid path with empty segments
            let result = data
                .set_by_path("level1..level3", Value::from_str("value"))
                .await;
            assert!(result.is_err());
            if let Err(ModelError::InvalidData(msg)) = result {
                assert!(msg.contains("empty segment"));
            }

            // Test path starting with dot
            let result = data.set_by_path(".level1", Value::from_str("value")).await;
            assert!(result.is_err());

            // Test path ending with dot
            let result = data.set_by_path("level1.", Value::from_str("value")).await;
            assert!(result.is_err());

            // Test setting on non-object root
            let mut non_object = Value::from_str("not an object");
            let result = non_object
                .set_by_path("some.path", Value::from_str("value"))
                .await;
            assert!(result.is_err());
            if let Err(ModelError::InvalidData(msg)) = result {
                assert!(msg.contains("non-object root"));
            }

            println!("✅ Error cases test passed");
        })
        .await;
}

#[tokio::test]
async fn test_set_by_path_complex_scenario() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Complex enterprise data structure
            let mut company = Value::new_object();

            // When: Build complex structure using set_by_path
            company
                .set_by_path("info.name", Value::from_str("TechCorp Inc"))
                .await
                .unwrap();
            company
                .set_by_path("info.founded", Value::from_number(2020.0).unwrap())
                .await
                .unwrap();

            company
                .set_by_path(
                    "departments.engineering.head",
                    Value::from_str("Alice Johnson"),
                )
                .await
                .unwrap();
            company
                .set_by_path(
                    "departments.engineering.budget",
                    Value::from_number(500000.0).unwrap(),
                )
                .await
                .unwrap();

            company
                .set_by_path("departments.sales.head", Value::from_str("Bob Smith"))
                .await
                .unwrap();
            company
                .set_by_path(
                    "departments.sales.budget",
                    Value::from_number(300000.0).unwrap(),
                )
                .await
                .unwrap();

            company
                .set_by_path(
                    "locations.headquarters.city",
                    Value::from_str("San Francisco"),
                )
                .await
                .unwrap();
            company
                .set_by_path("locations.headquarters.country", Value::from_str("USA"))
                .await
                .unwrap();

            company
                .set_by_path("locations.branch_office.city", Value::from_str("London"))
                .await
                .unwrap();
            company
                .set_by_path("locations.branch_office.country", Value::from_str("UK"))
                .await
                .unwrap();

            // Then: All data is accessible via get_by_path
            assert_eq!(
                company
                    .get_by_path("info.name")
                    .await
                    .unwrap()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "TechCorp Inc"
            );
            assert_eq!(
                company
                    .get_by_path("departments.engineering.head")
                    .await
                    .unwrap()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "Alice Johnson"
            );
            assert_eq!(
                company
                    .get_by_path("departments.sales.budget")
                    .await
                    .unwrap()
                    .unwrap()
                    .as_number()
                    .unwrap(),
                300000.0
            );
            assert_eq!(
                company
                    .get_by_path("locations.headquarters.city")
                    .await
                    .unwrap()
                    .unwrap()
                    .as_str()
                    .unwrap(),
                "San Francisco"
            );

            // Verify structure integrity
            assert!(company.has_path("info").await.unwrap());
            assert!(company.has_path("departments").await.unwrap());
            assert!(company.has_path("departments.engineering").await.unwrap());
            assert!(company.has_path("departments.sales").await.unwrap());
            assert!(company.has_path("locations").await.unwrap());
            assert!(company.has_path("locations.headquarters").await.unwrap());
            assert!(company.has_path("locations.branch_office").await.unwrap());

            println!("✅ Complex scenario test passed");
            println!(
                "   Created enterprise structure with {} top-level sections",
                3
            );
        })
        .await;
}

#[tokio::test]
async fn test_set_by_path_whitespace_handling() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Object with whitespace in paths
            let mut data = Value::new_object();

            // When: Set paths with various whitespace scenarios
            data.set_by_path("  user.name  ", Value::from_str("trimmed"))
                .await
                .unwrap();

            // Then: Whitespace is handled correctly
            let retrieved = data.get_by_path("user.name").await.unwrap().unwrap();
            assert_eq!(retrieved.as_str().unwrap(), "trimmed");

            // Verify normalized path works
            let retrieved2 = data.get_by_path("user.name").await.unwrap().unwrap();
            assert_eq!(retrieved2.as_str().unwrap(), "trimmed");

            println!("✅ Whitespace handling test passed");
        })
        .await;
}

#[tokio::test]
async fn test_set_by_path_performance() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            // Given: Performance test with many nested paths
            let mut data = Value::new_object();

            let start = std::time::Instant::now();

            // When: Set many nested paths
            for i in 0..100 {
                let path = format!("level1.level2.level3.item_{}", i);
                data.set_by_path(&path, Value::from_number(i as f64).unwrap())
                    .await
                    .unwrap();
            }

            let duration = start.elapsed();

            // Then: Performance is acceptable
            assert!(duration.as_millis() < 1000); // Should complete in < 1 second

            // Verify all paths were set correctly
            for i in 0..100 {
                let path = format!("level1.level2.level3.item_{}", i);
                let value = data.get_by_path(&path).await.unwrap().unwrap();
                assert_eq!(value.as_number().unwrap(), i as f64);
            }

            println!(
                "✅ Performance test passed: {} operations in {:?}",
                100, duration
            );
        })
        .await;
}

// ================================================================================
// MODEL MANAGER PATH NAVIGATION TESTS
// ================================================================================

#[tokio::test]
async fn test_model_manager_get_by_path_simple() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            let mut manager = DefaultModelManager::create();

            // Given: Insert user with nested data
            let mut user = Value::new_object();
            user.set("name", Value::from_str("Ana García"))
                .await
                .unwrap();
            user.set("email", Value::from_str("ana@empresa.com"))
                .await
                .unwrap();
            user.set("age", Value::from_number(28.0).unwrap())
                .await
                .unwrap();

            let mut profile = Value::new_object();
            profile
                .set("department", Value::from_str("Engineering"))
                .await
                .unwrap();
            profile
                .set("level", Value::from_number(5.0).unwrap())
                .await
                .unwrap();
            profile.set("active", Value::from_bool(true)).await.unwrap();
            user.set("profile", profile).await.unwrap();

            // Insert user
            manager
                .insert("users".to_string(), Some("user_001".to_string()), user)
                .await
                .unwrap();

            // When: Get values by different paths
            let name_result = manager
                .get_by_path(
                    "users".to_string(),
                    "user_001".to_string(),
                    "name".to_string(),
                )
                .await
                .unwrap();

            let department_result = manager
                .get_by_path(
                    "users".to_string(),
                    "user_001".to_string(),
                    "profile.department".to_string(),
                )
                .await
                .unwrap();

            let level_result = manager
                .get_by_path(
                    "users".to_string(),
                    "user_001".to_string(),
                    "profile.level".to_string(),
                )
                .await
                .unwrap();

            // Then: Correct values returned
            assert!(name_result.is_some());
            assert_eq!(name_result.unwrap().as_str().unwrap(), "Ana García");

            assert!(department_result.is_some());
            assert_eq!(department_result.unwrap().as_str().unwrap(), "Engineering");

            assert!(level_result.is_some());
            assert_eq!(level_result.unwrap().as_number().unwrap(), 5.0);

            println!("✅ ModelManager get_by_path simple test passed");
        })
        .await;
}

#[tokio::test]
async fn test_model_manager_get_by_path_nonexistent_paths() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            let mut manager = DefaultModelManager::create();

            // Given: Insert simple user
            let mut user = Value::new_object();
            user.set("name", Value::from_str("Test User"))
                .await
                .unwrap();
            manager
                .insert("users".to_string(), Some("user_001".to_string()), user)
                .await
                .unwrap();

            // When: Try to get nonexistent paths
            let nonexistent_field = manager
                .get_by_path(
                    "users".to_string(),
                    "user_001".to_string(),
                    "nonexistent".to_string(),
                )
                .await
                .unwrap();

            let nonexistent_nested = manager
                .get_by_path(
                    "users".to_string(),
                    "user_001".to_string(),
                    "name.invalid".to_string(),
                )
                .await
                .unwrap();

            // Then: None returned for nonexistent paths
            assert!(nonexistent_field.is_none());
            assert!(nonexistent_nested.is_none());

            println!("✅ ModelManager get_by_path nonexistent paths test passed");
        })
        .await;
}

#[tokio::test]
async fn test_model_manager_get_by_path_nonexistent_record() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            let mut manager = DefaultModelManager::create();

            // When: Try to get path from nonexistent record
            let result = manager
                .get_by_path(
                    "users".to_string(),
                    "nonexistent_user".to_string(),
                    "name".to_string(),
                )
                .await;

            // Then: NotFound error returned
            assert!(result.is_err());
            if let Err(ModelError::NotFound(id)) = result {
                assert_eq!(id, "nonexistent_user");
            } else {
                panic!("Expected NotFound error");
            }

            println!("✅ ModelManager get_by_path nonexistent record test passed");
        })
        .await;
}

#[tokio::test]
async fn test_model_manager_find_by_path_exists_basic() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            let mut manager = DefaultModelManager::create();

            // Given: Insert multiple users with different structures
            let mut user1 = Value::new_object();
            user1.set("name", Value::from_str("User 1")).await.unwrap();
            user1
                .set("email", Value::from_str("user1@test.com"))
                .await
                .unwrap();
            user1
                .set("phone", Value::from_str("123-456-7890"))
                .await
                .unwrap();
            manager
                .insert("users".to_string(), None, user1)
                .await
                .unwrap();

            let mut user2 = Value::new_object();
            user2.set("name", Value::from_str("User 2")).await.unwrap();
            user2
                .set("email", Value::from_str("user2@test.com"))
                .await
                .unwrap();
            // No phone field
            manager
                .insert("users".to_string(), None, user2)
                .await
                .unwrap();

            let mut user3 = Value::new_object();
            user3.set("name", Value::from_str("User 3")).await.unwrap();
            user3
                .set("phone", Value::from_str("098-765-4321"))
                .await
                .unwrap();
            // No email field
            manager
                .insert("users".to_string(), None, user3)
                .await
                .unwrap();

            // When: Find records that have specific paths
            let users_with_email = manager
                .find_by_path_exists("users".to_string(), "email".to_string())
                .await
                .unwrap();

            let users_with_phone = manager
                .find_by_path_exists("users".to_string(), "phone".to_string())
                .await
                .unwrap();

            let users_with_name = manager
                .find_by_path_exists("users".to_string(), "name".to_string())
                .await
                .unwrap();

            // Then: Correct number of users found
            assert_eq!(users_with_email.len(), 2); // user1 and user2
            assert_eq!(users_with_phone.len(), 2); // user1 and user3
            assert_eq!(users_with_name.len(), 3); // all users

            // Verify email users
            for user in &users_with_email {
                assert!(user.get("email").await.unwrap().is_some());
            }

            // Verify phone users
            for user in &users_with_phone {
                assert!(user.get("phone").await.unwrap().is_some());
            }

            println!("✅ ModelManager find_by_path_exists basic test passed");
        })
        .await;
}

#[tokio::test]
async fn test_model_manager_find_by_path_exists_nested() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            let mut manager = DefaultModelManager::create();

            // Given: Insert users with nested profile data
            let mut user1 = Value::new_object();
            user1.set("name", Value::from_str("User 1")).await.unwrap();

            let mut profile1 = Value::new_object();
            profile1
                .set("department", Value::from_str("Engineering"))
                .await
                .unwrap();
            profile1
                .set("level", Value::from_number(5.0).unwrap())
                .await
                .unwrap();
            user1.set("profile", profile1).await.unwrap();

            manager
                .insert("users".to_string(), None, user1)
                .await
                .unwrap();

            let mut user2 = Value::new_object();
            user2.set("name", Value::from_str("User 2")).await.unwrap();

            let mut profile2 = Value::new_object();
            profile2
                .set("department", Value::from_str("Sales"))
                .await
                .unwrap();
            // No level field
            user2.set("profile", profile2).await.unwrap();

            manager
                .insert("users".to_string(), None, user2)
                .await
                .unwrap();

            let mut user3 = Value::new_object();
            user3.set("name", Value::from_str("User 3")).await.unwrap();
            // No profile at all
            manager
                .insert("users".to_string(), None, user3)
                .await
                .unwrap();

            // When: Find users with nested paths
            let users_with_profile = manager
                .find_by_path_exists("users".to_string(), "profile".to_string())
                .await
                .unwrap();

            let users_with_department = manager
                .find_by_path_exists("users".to_string(), "profile.department".to_string())
                .await
                .unwrap();

            let users_with_level = manager
                .find_by_path_exists("users".to_string(), "profile.level".to_string())
                .await
                .unwrap();

            // Then: Correct filtering by nested paths
            assert_eq!(users_with_profile.len(), 2); // user1 and user2
            assert_eq!(users_with_department.len(), 2); // user1 and user2
            assert_eq!(users_with_level.len(), 1); // only user1

            println!("✅ ModelManager find_by_path_exists nested test passed");
        })
        .await;
}

#[tokio::test]
async fn test_model_manager_find_by_path_value_basic() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            let mut manager = DefaultModelManager::create();

            // Given: Insert users with different departments
            let mut user1 = Value::new_object();
            user1.set("name", Value::from_str("Ana")).await.unwrap();
            user1
                .set("department", Value::from_str("Engineering"))
                .await
                .unwrap();
            user1.set("active", Value::from_bool(true)).await.unwrap();
            manager
                .insert("users".to_string(), None, user1)
                .await
                .unwrap();

            let mut user2 = Value::new_object();
            user2.set("name", Value::from_str("Carlos")).await.unwrap();
            user2
                .set("department", Value::from_str("Sales"))
                .await
                .unwrap();
            user2.set("active", Value::from_bool(true)).await.unwrap();
            manager
                .insert("users".to_string(), None, user2)
                .await
                .unwrap();

            let mut user3 = Value::new_object();
            user3.set("name", Value::from_str("Maria")).await.unwrap();
            user3
                .set("department", Value::from_str("Engineering"))
                .await
                .unwrap();
            user3.set("active", Value::from_bool(false)).await.unwrap();
            manager
                .insert("users".to_string(), None, user3)
                .await
                .unwrap();

            // When: Find users by specific field values
            let engineering_users = manager
                .find_by_path_value(
                    "users".to_string(),
                    "department".to_string(),
                    Value::from_str("Engineering"),
                )
                .await
                .unwrap();

            let sales_users = manager
                .find_by_path_value(
                    "users".to_string(),
                    "department".to_string(),
                    Value::from_str("Sales"),
                )
                .await
                .unwrap();

            let active_users = manager
                .find_by_path_value(
                    "users".to_string(),
                    "active".to_string(),
                    Value::from_bool(true),
                )
                .await
                .unwrap();

            let inactive_users = manager
                .find_by_path_value(
                    "users".to_string(),
                    "active".to_string(),
                    Value::from_bool(false),
                )
                .await
                .unwrap();

            // Then: Correct users found by value
            assert_eq!(engineering_users.len(), 2); // Ana and Maria
            assert_eq!(sales_users.len(), 1); // Carlos
            assert_eq!(active_users.len(), 2); // Ana and Carlos
            assert_eq!(inactive_users.len(), 1); // Maria

            // Verify engineering users
            for user in &engineering_users {
                let dept = user.get("department").await.unwrap().unwrap();
                assert_eq!(dept.as_str().unwrap(), "Engineering");
            }

            // Verify active users
            for user in &active_users {
                let active = user.get("active").await.unwrap().unwrap();
                assert_eq!(active.as_bool().unwrap(), true);
            }

            println!("✅ ModelManager find_by_path_value basic test passed");
        })
        .await;
}

#[tokio::test]
async fn test_model_manager_find_by_path_value_nested() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            let mut manager = DefaultModelManager::create();

            // Given: Insert users with nested profile data
            let mut user1 = Value::new_object();
            user1
                .set("name", Value::from_str("Senior Dev"))
                .await
                .unwrap();

            let mut profile1 = Value::new_object();
            profile1
                .set("department", Value::from_str("Engineering"))
                .await
                .unwrap();
            profile1
                .set("level", Value::from_number(8.0).unwrap())
                .await
                .unwrap();
            profile1
                .set("remote", Value::from_bool(true))
                .await
                .unwrap();
            user1.set("profile", profile1).await.unwrap();

            manager
                .insert("users".to_string(), None, user1)
                .await
                .unwrap();

            let mut user2 = Value::new_object();
            user2
                .set("name", Value::from_str("Junior Dev"))
                .await
                .unwrap();

            let mut profile2 = Value::new_object();
            profile2
                .set("department", Value::from_str("Engineering"))
                .await
                .unwrap();
            profile2
                .set("level", Value::from_number(3.0).unwrap())
                .await
                .unwrap();
            profile2
                .set("remote", Value::from_bool(false))
                .await
                .unwrap();
            user2.set("profile", profile2).await.unwrap();

            manager
                .insert("users".to_string(), None, user2)
                .await
                .unwrap();

            let mut user3 = Value::new_object();
            user3
                .set("name", Value::from_str("Sales Manager"))
                .await
                .unwrap();

            let mut profile3 = Value::new_object();
            profile3
                .set("department", Value::from_str("Sales"))
                .await
                .unwrap();
            profile3
                .set("level", Value::from_number(7.0).unwrap())
                .await
                .unwrap();
            profile3
                .set("remote", Value::from_bool(true))
                .await
                .unwrap();
            user3.set("profile", profile3).await.unwrap();

            manager
                .insert("users".to_string(), None, user3)
                .await
                .unwrap();

            // When: Find users by nested field values
            let engineering_users = manager
                .find_by_path_value(
                    "users".to_string(),
                    "profile.department".to_string(),
                    Value::from_str("Engineering"),
                )
                .await
                .unwrap();

            let remote_users = manager
                .find_by_path_value(
                    "users".to_string(),
                    "profile.remote".to_string(),
                    Value::from_bool(true),
                )
                .await
                .unwrap();

            let senior_users = manager
                .find_by_path_value(
                    "users".to_string(),
                    "profile.level".to_string(),
                    Value::from_number(8.0).unwrap(),
                )
                .await
                .unwrap();

            // Then: Correct filtering by nested values
            assert_eq!(engineering_users.len(), 2); // Senior Dev and Junior Dev
            assert_eq!(remote_users.len(), 2); // Senior Dev and Sales Manager
            assert_eq!(senior_users.len(), 1); // Only Senior Dev

            // Verify engineering users
            for user in &engineering_users {
                let dept = user
                    .get("profile")
                    .await
                    .unwrap()
                    .unwrap()
                    .get("department")
                    .await
                    .unwrap()
                    .unwrap();
                assert_eq!(dept.as_str().unwrap(), "Engineering");
            }

            // Verify senior user
            let senior_user = &senior_users[0];
            let name = senior_user.get("name").await.unwrap().unwrap();
            assert_eq!(name.as_str().unwrap(), "Senior Dev");

            println!("✅ ModelManager find_by_path_value nested test passed");
        })
        .await;
}

#[tokio::test]
async fn test_model_manager_find_by_path_value_no_matches() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            let mut manager = DefaultModelManager::create();

            // Given: Insert users
            let mut user1 = Value::new_object();
            user1
                .set("department", Value::from_str("Engineering"))
                .await
                .unwrap();
            manager
                .insert("users".to_string(), None, user1)
                .await
                .unwrap();

            let mut user2 = Value::new_object();
            user2
                .set("department", Value::from_str("Sales"))
                .await
                .unwrap();
            manager
                .insert("users".to_string(), None, user2)
                .await
                .unwrap();

            // When: Search for value that doesn't exist
            let marketing_users = manager
                .find_by_path_value(
                    "users".to_string(),
                    "department".to_string(),
                    Value::from_str("Marketing"),
                )
                .await
                .unwrap();

            let nonexistent_field = manager
                .find_by_path_value(
                    "users".to_string(),
                    "nonexistent_field".to_string(),
                    Value::from_str("any_value"),
                )
                .await
                .unwrap();

            // Then: Empty results
            assert_eq!(marketing_users.len(), 0);
            assert_eq!(nonexistent_field.len(), 0);

            println!("✅ ModelManager find_by_path_value no matches test passed");
        })
        .await;
}

#[tokio::test]
async fn test_model_manager_path_methods_with_different_data_types() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            let mut manager = DefaultModelManager::create();

            // Given: Insert record with various data types
            let mut record = Value::new_object();
            record
                .set("string_field", Value::from_str("test_string"))
                .await
                .unwrap();
            record
                .set("number_field", Value::from_number(42.5).unwrap())
                .await
                .unwrap();
            record
                .set("bool_field", Value::from_bool(true))
                .await
                .unwrap();

            let mut array_field = Value::new_array();
            array_field.push(Value::from_str("item1")).await.unwrap();
            array_field.push(Value::from_str("item2")).await.unwrap();
            record.set("array_field", array_field).await.unwrap();

            manager
                .insert("records".to_string(), Some("rec_001".to_string()), record)
                .await
                .unwrap();

            // When: Test path operations with different data types
            let string_result = manager
                .get_by_path(
                    "records".to_string(),
                    "rec_001".to_string(),
                    "string_field".to_string(),
                )
                .await
                .unwrap();

            let number_result = manager
                .get_by_path(
                    "records".to_string(),
                    "rec_001".to_string(),
                    "number_field".to_string(),
                )
                .await
                .unwrap();

            let bool_result = manager
                .get_by_path(
                    "records".to_string(),
                    "rec_001".to_string(),
                    "bool_field".to_string(),
                )
                .await
                .unwrap();

            // Test find by different value types
            let records_with_string = manager
                .find_by_path_value(
                    "records".to_string(),
                    "string_field".to_string(),
                    Value::from_str("test_string"),
                )
                .await
                .unwrap();

            let records_with_number = manager
                .find_by_path_value(
                    "records".to_string(),
                    "number_field".to_string(),
                    Value::from_number(42.5).unwrap(),
                )
                .await
                .unwrap();

            let records_with_bool = manager
                .find_by_path_value(
                    "records".to_string(),
                    "bool_field".to_string(),
                    Value::from_bool(true),
                )
                .await
                .unwrap();

            // Then: All data types handled correctly
            assert!(string_result.is_some());
            assert_eq!(string_result.unwrap().as_str().unwrap(), "test_string");

            assert!(number_result.is_some());
            assert_eq!(number_result.unwrap().as_number().unwrap(), 42.5);

            assert!(bool_result.is_some());
            assert_eq!(bool_result.unwrap().as_bool().unwrap(), true);

            assert_eq!(records_with_string.len(), 1);
            assert_eq!(records_with_number.len(), 1);
            assert_eq!(records_with_bool.len(), 1);

            println!("✅ ModelManager path methods with different data types test passed");
        })
        .await;
}

#[tokio::test]
async fn test_model_manager_path_methods_performance() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            let mut manager = DefaultModelManager::create();

            // Given: Insert many records for performance testing
            for i in 0..100 {
                let mut user = Value::new_object();
                user.set("name", Value::from_str(&format!("User {}", i)))
                    .await
                    .unwrap();
                user.set(
                    "department",
                    Value::from_str(match i % 3 {
                        0 => "Engineering",
                        1 => "Sales",
                        2 => "Marketing",
                        _ => unreachable!(),
                    }),
                )
                .await
                .unwrap();
                user.set("level", Value::from_number((i % 10 + 1) as f64).unwrap())
                    .await
                    .unwrap();

                manager
                    .insert("users".to_string(), None, user)
                    .await
                    .unwrap();
            }

            // When: Test performance of path operations
            let start = std::time::Instant::now();

            // Test find_by_path_exists performance
            let users_with_department = manager
                .find_by_path_exists("users".to_string(), "department".to_string())
                .await
                .unwrap();

            // Test find_by_path_value performance
            let engineering_users = manager
                .find_by_path_value(
                    "users".to_string(),
                    "department".to_string(),
                    Value::from_str("Engineering"),
                )
                .await
                .unwrap();

            let duration = start.elapsed();

            // Then: Reasonable performance and correct results
            assert_eq!(users_with_department.len(), 100); // All users have department
            assert!(engineering_users.len() >= 30); // ~33% should be Engineering
            assert!(duration.as_millis() < 100); // Should be fast

            println!(
                "✅ Path operations on 100 records completed in {:?}",
                duration
            );
            println!(
                "   - Users with department: {}",
                users_with_department.len()
            );
            println!("   - Engineering users: {}", engineering_users.len());
        })
        .await;
}

#[tokio::test]
async fn test_model_manager_path_methods_empty_model() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            let mut manager = DefaultModelManager::create();

            // When: Test path operations on empty model
            let exists_results = manager
                .find_by_path_exists("empty_model".to_string(), "any_field".to_string())
                .await
                .unwrap();

            let value_results = manager
                .find_by_path_value(
                    "empty_model".to_string(),
                    "any_field".to_string(),
                    Value::from_str("any_value"),
                )
                .await
                .unwrap();

            // Then: Empty results for empty model
            assert_eq!(exists_results.len(), 0);
            assert_eq!(value_results.len(), 0);

            println!("✅ ModelManager path methods on empty model test passed");
        })
        .await;
}

#[tokio::test]
async fn test_model_manager_path_methods_complex_scenario() {
    let local_set = LocalSet::new();

    local_set
        .run_until(async {
            let mut manager = DefaultModelManager::create();

            // Given: Complex scenario with employees and projects
            let mut employee1 = Value::new_object();
            employee1
                .set("name", Value::from_str("Alice"))
                .await
                .unwrap();
            employee1
                .set("department", Value::from_str("Engineering"))
                .await
                .unwrap();

            let mut projects1 = Value::new_array();
            let mut project1 = Value::new_object();
            project1
                .set("name", Value::from_str("Project Alpha"))
                .await
                .unwrap();
            project1
                .set("status", Value::from_str("active"))
                .await
                .unwrap();
            projects1.push(project1).await.unwrap();
            employee1.set("projects", projects1).await.unwrap();

            manager
                .insert(
                    "employees".to_string(),
                    Some("emp_001".to_string()),
                    employee1,
                )
                .await
                .unwrap();

            let mut employee2 = Value::new_object();
            employee2.set("name", Value::from_str("Bob")).await.unwrap();
            employee2
                .set("department", Value::from_str("Sales"))
                .await
                .unwrap();
            // No projects
            manager
                .insert(
                    "employees".to_string(),
                    Some("emp_002".to_string()),
                    employee2,
                )
                .await
                .unwrap();

            let mut employee3 = Value::new_object();
            employee3
                .set("name", Value::from_str("Charlie"))
                .await
                .unwrap();
            employee3
                .set("department", Value::from_str("Engineering"))
                .await
                .unwrap();

            let mut projects3 = Value::new_array();
            let mut project3 = Value::new_object();
            project3
                .set("name", Value::from_str("Project Beta"))
                .await
                .unwrap();
            project3
                .set("status", Value::from_str("completed"))
                .await
                .unwrap();
            projects3.push(project3).await.unwrap();
            employee3.set("projects", projects3).await.unwrap();

            manager
                .insert(
                    "employees".to_string(),
                    Some("emp_003".to_string()),
                    employee3,
                )
                .await
                .unwrap();

            // When: Complex path queries
            let engineering_employees = manager
                .find_by_path_value(
                    "employees".to_string(),
                    "department".to_string(),
                    Value::from_str("Engineering"),
                )
                .await
                .unwrap();

            let employees_with_projects = manager
                .find_by_path_exists("employees".to_string(), "projects".to_string())
                .await
                .unwrap();

            let alice_department = manager
                .get_by_path(
                    "employees".to_string(),
                    "emp_001".to_string(),
                    "department".to_string(),
                )
                .await
                .unwrap();

            // Then: Complex queries work correctly
            assert_eq!(engineering_employees.len(), 2); // Alice and Charlie
            assert_eq!(employees_with_projects.len(), 2); // Alice and Charlie (Bob has no projects)

            assert!(alice_department.is_some());
            assert_eq!(alice_department.unwrap().as_str().unwrap(), "Engineering");

            // Verify engineering employees are correct
            for emp in &engineering_employees {
                let dept = emp.get("department").await.unwrap().unwrap();
                assert_eq!(dept.as_str().unwrap(), "Engineering");
            }

            // Verify employees with projects
            for emp in &employees_with_projects {
                assert!(emp.get("projects").await.unwrap().is_some());
            }

            println!("✅ ModelManager complex path scenario test passed");
            println!(
                "   - Engineering employees: {}",
                engineering_employees.len()
            );
            println!(
                "   - Employees with projects: {}",
                employees_with_projects.len()
            );
        })
        .await;
}
