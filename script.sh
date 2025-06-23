#!/bin/bash

# Script para corregir las pruebas del model_manager después de la refactorización

set -e

echo "🔧 Iniciando corrección de pruebas..."

# Función para hacer backup
make_backup() {
    local file=$1
    if [ -f "$file" ]; then
        cp "$file" "$file.backup.$(date +%s)"
        echo "✅ Backup creado para $file"
    fi
}

# Función para aplicar correcciones
apply_corrections() {
    local file=$1
    echo "🔄 Corrigiendo $file..."
    
    make_backup "$file"
    
    # Reemplazar imports obsoletos
    sed -i 's/use model_manager::{DefaultValue, DynamicValue, ModelManager, DefaultModelManager, DynamicValueFactory, ModelManagerFactory};/use model_manager::{DefaultValueFactory, CoreValue, ModelManager, DefaultModelManager, ValueFactory, ModelManagerFactory};/g' "$file"
    
    # Reemplazar DefaultValue::create() por DefaultValueFactory::create_object()
    sed -i 's/DefaultValue::create()/DefaultValueFactory::create_object()/g' "$file"
    
    # Reemplazar Value::from_str por DefaultValueFactory::create_string
    sed -i 's/Value::from_str(\([^)]*\))/DefaultValueFactory::create_string(\1)/g' "$file"
    
    # Reemplazar Value::from_number por DefaultValueFactory::create_number
    sed -i 's/Value::from_number(\([^)]*\))/DefaultValueFactory::create_number(\1)/g' "$file"
    
    # Reemplazar Value::from_bool por DefaultValueFactory::create_bool
    sed -i 's/Value::from_bool(\([^)]*\))/DefaultValueFactory::create_bool(\1)/g' "$file"
    
    # Reemplazar Value::new_object por DefaultValueFactory::create_object
    sed -i 's/Value::new_object()/DefaultValueFactory::create_object()/g' "$file"
    
    # Reemplazar Value::new_array por DefaultValueFactory::create_array
    sed -i 's/Value::new_array()/DefaultValueFactory::create_array()/g' "$file"
    
    # Corregir type aliases
    sed -i 's/type Value = <DefaultValue as DynamicValueFactory>::Value;/type Value = <DefaultValueFactory as ValueFactory>::Value;/g' "$file"
    
    # Agregar manejo de errores para create_number
    sed -i 's/DefaultValueFactory::create_number(\([^)]*\))/DefaultValueFactory::create_number(\1).unwrap()/g' "$file"
    
    echo "✅ Correcciones aplicadas a $file"
}

# Función para corregir un test específico
fix_specific_test() {
    local file=$1
    echo "🎯 Aplicando correcciones específicas a $file..."
    
    # Correcciones específicas por archivo
    case "$file" in
        *converter_tests.rs)
            # Corregir imports específicos para converter tests
            sed -i 's/use model_manager::{DynamicValue, DefaultValue, DefaultConverter, ConverterFactory, JsonConverter, DataConverter, JsonConverterConfig, ConverterConfig, DynamicValueFactory};/use model_manager::{CoreValue, DefaultValueFactory, DefaultConverter, ConverterFactory, JsonConverter, DataConverter, JsonConverterConfig, ConverterConfig, ValueFactory};/g' "$file"
            ;;
        *integration_tests.rs)
            # Corregir imports para integration tests
            sed -i 's/use model_manager::{DynamicValue, ModelManager, DefaultValue, DefaultModelManager, DynamicValueFactory, ModelManagerFactory};/use model_manager::{CoreValue, ModelManager, DefaultValueFactory, DefaultModelManager, ValueFactory, ModelManagerFactory};/g' "$file"
            ;;
        *iterator_tests.rs)
            # Corregir imports para iterator tests
            sed -i 's/use model_manager::{ArrayIterator, ArrayIteratorFactory, ObjectIterator, ObjectIteratorFactory, DefaultValue, DynamicValue, DynamicValueFactory, IteratorFactory, DefaultIteratorFactory, ModelManager, ModelManagerFactory, DefaultModelManager,};/use model_manager::{ArrayIterator, ArrayIteratorFactory, ObjectIterator, ObjectIteratorFactory, DefaultValueFactory, CoreValue, ValueFactory, IteratorFactory, DefaultIteratorFactory, ModelManager, ModelManagerFactory, DefaultModelManager,};/g' "$file"
            ;;
    esac
}

# Función para verificar errores comunes
check_common_errors() {
    local file=$1
    echo "🔍 Verificando errores comunes en $file..."
    
    # Verificar si hay referencias a DynamicValue sin import
    if grep -q "DynamicValue" "$file" && ! grep -q "use.*DynamicValue" "$file"; then
        echo "⚠️  ADVERTENCIA: $file usa DynamicValue pero no lo importa"
    fi
    
    # Verificar si hay referencias a DefaultValue
    if grep -q "DefaultValue::" "$file"; then
        echo "⚠️  ADVERTENCIA: $file aún usa DefaultValue:: (debería ser DefaultValueFactory::)"
    fi
    
    # Verificar unwrap() faltantes para create_number
    if grep -q "create_number(" "$file" && ! grep -q "create_number.*unwrap" "$file"; then
        echo "⚠️  ADVERTENCIA: $file usa create_number sin unwrap()"
    fi
}

# Función principal
main() {
    echo "🚀 Script de corrección de pruebas para model_manager"
    echo "=================================================="
    
    # Buscar todos los archivos de test
    test_files=$(find tests/ -name "*.rs" 2>/dev/null || echo "")
    
    if [ -z "$test_files" ]; then
        echo "❌ No se encontraron archivos de test en el directorio tests/"
        exit 1
    fi
    
    echo "📁 Archivos de test encontrados:"
    echo "$test_files" | sed 's/^/  - /'
    echo ""
    
    # Preguntar si proceder
    read -p "¿Proceder con las correcciones? (y/N): " confirm
    if [[ ! "$confirm" =~ ^[Yy]$ ]]; then
        echo "❌ Operación cancelada"
        exit 0
    fi
    
    # Aplicar correcciones a cada archivo
    for file in $test_files; do
        echo ""
        echo "🔧 Procesando: $file"
        apply_corrections "$file"
        fix_specific_test "$file"
        check_common_errors "$file"
    done
    
    echo ""
    echo "🎉 Correcciones completadas!"
    echo ""
    echo "📋 Pasos siguientes:"
    echo "  1. Revisar los cambios: git diff"
    echo "  2. Ejecutar las pruebas: cargo test"
    echo "  3. Si hay errores, revisar manualmente los archivos marcados con ⚠️"
    echo "  4. Los backups están disponibles como *.backup.timestamp"
}

# Función para mostrar ayuda
show_help() {
    echo "Script de corrección de pruebas para model_manager"
    echo ""
    echo "USO:"
    echo "  $0                    - Corregir todos los tests automáticamente"
    echo "  $0 --file <archivo>   - Corregir un archivo específico"
    echo "  $0 --check           - Solo verificar errores sin modificar"
    echo "  $0 --restore         - Restaurar desde backups"
    echo "  $0 --help            - Mostrar esta ayuda"
    echo ""
    echo "EJEMPLOS:"
    echo "  $0 --file tests/integration_tests.rs"
    echo "  $0 --check"
    echo "  $0 --restore"
}

# Función para corregir un archivo específico
fix_single_file() {
    local file=$1
    if [ ! -f "$file" ]; then
        echo "❌ Archivo no encontrado: $file"
        exit 1
    fi
    
    echo "🔧 Corrigiendo archivo específico: $file"
    apply_corrections "$file"
    fix_specific_test "$file"
    check_common_errors "$file"
    echo "✅ Corrección completada para $file"
}

# Función para solo verificar errores
check_only() {
    echo "🔍 Verificando errores en archivos de test..."
    
    test_files=$(find tests/ -name "*.rs" 2>/dev/null || echo "")
    
    if [ -z "$test_files" ]; then
        echo "❌ No se encontraron archivos de test"
        exit 1
    fi
    
    for file in $test_files; do
        echo ""
        echo "📄 Verificando: $file"
        check_common_errors "$file"
    done
}

# Función para restaurar backups
restore_backups() {
    echo "🔄 Buscando backups..."
    
    backups=$(find tests/ -name "*.backup.*" 2>/dev/null || echo "")
    
    if [ -z "$backups" ]; then
        echo "❌ No se encontraron backups"
        exit 1
    fi
    
    echo "📁 Backups encontrados:"
    echo "$backups" | sed 's/^/  - /'
    echo ""
    
    read -p "¿Restaurar todos los backups? (y/N): " confirm
    if [[ ! "$confirm" =~ ^[Yy]$ ]]; then
        echo "❌ Restauración cancelada"
        exit 0
    fi
    
    for backup in $backups; do
        original=$(echo "$backup" | sed 's/\.backup\.[0-9]*$//')
        echo "🔄 Restaurando: $original"
        cp "$backup" "$original"
    done
    
    echo "✅ Restauración completada"
}

# Parsear argumentos
case "${1:-}" in
    --help|-h)
        show_help
        ;;
    --file)
        if [ -z "$2" ]; then
            echo "❌ Error: --file requiere un nombre de archivo"
            show_help
            exit 1
        fi
        fix_single_file "$2"
        ;;
    --check)
        check_only
        ;;
    --restore)
        restore_backups
        ;;
    "")
        main
        ;;
    *)
        echo "❌ Opción desconocida: $1"
        show_help
        exit 1
        ;;
esac