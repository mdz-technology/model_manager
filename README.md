# Model Manager

[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://www.rust-lang.org)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://github.com/spreadapp/model-manager)

> **Gestor asíncrono de múltiples modelos de datos para aplicaciones offline-first**

Model Manager es un componente central del framework SpreadApp que proporciona gestión de datos completamente asíncrona con alta concurrencia, diseñado específicamente para aplicaciones empresariales que requieren rendimiento extremo y operación offline.

## 🛠️ Instalación

Agrega al `Cargo.toml`:

```toml
[dependencies]
model_manager = { path = "." }
tokio = { version = "1.44", features = ["full"] }
```

## 🎯 Ejemplo Básico

```rust
use model_manager::{
    AsyncModelManager, AsyncDynamicValue, ModelManagerFactory,
    DefaultFactory, DefaultValue
};
use tokio::task::LocalSet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let local_set = LocalSet::new();
    
    local_set.run_until(async {
        // === CREAR MANAGER ===
        let mut manager = DefaultFactory::create();
        
        // === CREAR DATOS EMPRESARIALES ===
        let mut user = DefaultValue::new_object();
        user.set("name", DefaultValue::from_str("Ana García")).await?;
        user.set("email", DefaultValue::from_str("ana@empresa.com")).await?;
        user.set("department", DefaultValue::from_str("Ventas")).await?;
        user.set("active", DefaultValue::from_bool(true)).await?;
        
        // === OPERACIONES CRUD ASÍNCRONAS ===
        
        // Insert asíncrono
        let inserted = manager.insert(
            "users".to_string(),
            Some("usr_001".to_string()),
            user
        ).await?;
        println!("Usuario insertado: {}", inserted.to_string());
        
        // Get asíncrono
        let retrieved = manager.get(
            "users".to_string(),
            "usr_001".to_string()
        ).await?;
        println!("Usuario recuperado: {}", retrieved.to_string());
        
        // Update asíncrono
        let mut updated_user = DefaultValue::new_object();
        updated_user.set("name", DefaultValue::from_str("Ana García Pérez")).await?;
        updated_user.set("department", DefaultValue::from_str("Ingeniería")).await?;
        
        let updated = manager.update(
            "users".to_string(),
            "usr_001".to_string(),
            updated_user
        ).await?;
        println!("Usuario actualizado: {}", updated.to_string());
        
        // Get All asíncrono
        let all_users = manager.get_all("users".to_string()).await?;
        println!("Total usuarios: {}", all_users.len());
        
        // Remove asíncrono
        let removed = manager.remove(
            "users".to_string(),
            "usr_001".to_string()
        ).await?;
        println!("Usuario eliminado: {}", removed.to_string());
        
        Ok::<(), Box<dyn std::error::Error>>(())
    }).await
}
```

**Output esperado:**
```
Usuario insertado: {"name":"Ana García","email":"ana@empresa.com","department":"Ventas","active":true}
Usuario recuperado: {"name":"Ana García","email":"ana@empresa.com","department":"Ventas","active":true}
Usuario actualizado: {"name":"Ana García Pérez","department":"Ingeniería"}
Total usuarios: 1
Usuario eliminado: {"name":"Ana García Pérez","department":"Ingeniería"}
```

## 🏢 Ejemplo Avanzado

### Operaciones Masivas Concurrentes

```rust
use futures::future::join_all;

async fn enterprise_workload() -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = DefaultFactory::create();
    
    // === 10,000 OPERACIONES CONCURRENTES ===
    let tasks: Vec<_> = (0..10_000).map(|i| {
        async {
            let mut data = DefaultValue::new_object();
            data.set("id", DefaultValue::from_number(i as f64)?).await?;
            data.set("type", DefaultValue::from_str(match i % 4 {
                0 => "user",
                1 => "product", 
                2 => "order",
                3 => "audit",
                _ => unreachable!(),
            })).await?;
            
            let model = match i % 4 {
                0 => "users",
                1 => "products",
                2 => "orders", 
                3 => "audit_logs",
                _ => unreachable!(),
            };
            
            manager.insert(model.to_string(), None, data).await
        }
    }).collect();
    
    println!("Iniciando {} operaciones concurrentes...", tasks.len());
    
    let start = std::time::Instant::now();
    let results = join_all(tasks).await;
    let duration = start.elapsed();
    
    let successful = results.iter().filter(|r| r.is_ok()).count();
    let throughput = successful as f64 / duration.as_secs_f64();
    
    println!("✅ Operaciones exitosas: {}", successful);
    println!("⏱️  Tiempo total: {:?}", duration);
    println!("🚀 Throughput: {:.0} ops/seg", throughput);
    
    Ok(())
}
```

### Datos Complejos Anidados

```rust
async fn complex_enterprise_data() -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = DefaultFactory::create();
    
    // === ESTRUCTURA EMPRESARIAL COMPLEJA ===
    let mut company = DefaultValue::new_object();
    company.set("name", DefaultValue::from_str("TechCorp SA")).await?;
    
    // Departamentos
    let mut departments = DefaultValue::new_array();
    for i in 0..5 {
        let mut dept = DefaultValue::new_object();
        dept.set("id", DefaultValue::from_number(i as f64)?).await?;
        dept.set("name", DefaultValue::from_str(&format!("Departamento {}", i))).await?;
        
        // Empleados por departamento
        let mut employees = DefaultValue::new_array();
        for j in 0..10 {
            let mut emp = DefaultValue::new_object();
            emp.set("id", DefaultValue::from_number(j as f64)?).await?;
            emp.set("name", DefaultValue::from_str(&format!("Empleado {}-{}", i, j))).await?;
            emp.set("salary", DefaultValue::from_number(50000.0 + j as f64 * 1000.0)?).await?;
            employees.push(emp).await?;
        }
        
        dept.set("employees", employees).await?;
        departments.push(dept).await?;
    }
    
    company.set("departments", departments).await?;
    
    // === INSERTAR ESTRUCTURA COMPLETA ===
    let inserted = manager.insert(
        "companies".to_string(),
        Some("company_001".to_string()),
        company
    ).await?;
    
    println!("Empresa creada con estructura compleja");
    println!("Tipo: {}", inserted.get_type());
    
    // === ACCESO A DATOS ANIDADOS ===
    let departments = inserted.get("departments").await?.unwrap();
    if let Some(dept_array) = departments.as_array().await? {
        println!("Departamentos encontrados: {}", dept_array.len());
        
        for (i, dept) in dept_array.iter().enumerate() {
            let dept_name = dept.get("name").await?.unwrap().as_str().unwrap();
            let employees = dept.get("employees").await?.unwrap();
            let emp_count = employees.as_array().await?.unwrap().len();
            
            println!("  {}: {} empleados", dept_name, emp_count);
        }
    }
    
    Ok(())
}
```

## 🧪 Tests y Benchmarks

### Ejecutar Tests de Integración

```bash
cargo test --test integration_tests
```

### Ejecutar Tests de Performance

```bash
cargo test --test performance_tests --release
```

## 🏗️ Arquitectura del Sistema

### Estructura de Capas

```
┌─────────────────────────────────────────┐
│              APPLICATION                │
│  ┌─────────────┐  ┌─────────────────┐   │
│  │   Traits    │  │    Services     │   │
│  │             │  │                 │   │
│  │ ┌─────────┐ │  │ ┌─────────────┐ │   │
│  │ │ Model   │ │  │ │  Factory    │ │   │
│  │ │Manager  │ │  │ │  Pattern    │ │   │
│  │ └─────────┘ │  │ └─────────────┘ │   │
│  │             │  │                 │   │
│  │ ┌─────────┐ │  │ ┌─────────────┐ │   │
│  │ │Dynamic  │ │  │ │   Models    │ │   │
│  │ │ Value   │ │  │ │  & Errors   │ │   │
│  │ └─────────┘ │  │ └─────────────┘ │   │
│  └─────────────┘  └─────────────────┘   │
└─────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────┐
│            INFRASTRUCTURE              │
│  ┌─────────────┐  ┌─────────────────┐   │
│  │Implementations│  │     Actors     │   │
│  │             │  │                 │   │
│  │ ┌─────────┐ │  │ ┌─────────────┐ │   │
│  │ │ Actix   │ │  │ │   Model     │ │   │
│  │ │Manager  │ │  │ │   Actor     │ │   │
│  │ └─────────┘ │  │ └─────────────┘ │   │
│  │             │  │                 │   │
│  │ ┌─────────┐ │  │ ┌─────────────┐ │   │
│  │ │ Serde   │ │  │ │ Messages    │ │   │
│  │ │ Value   │ │  │ │ & Protocol  │ │   │
│  │ └─────────┘ │  │ └─────────────┘ │   │
│  └─────────────┘  └─────────────────┘   │
└─────────────────────────────────────────┘
```

### Principios de Diseño

1. **Dependency Inversion**: Las interfaces son definidas por la capa de aplicación
2. **Single Responsibility**: Cada módulo tiene una responsabilidad única
3. **Async-First**: Todas las operaciones críticas son asíncronas
4. **Actor Isolation**: Cada modelo tiene su propio actor pool independiente
5. **Memory Efficiency**: Streaming automático para datasets grandes

## 🤝 Contribuciones

¡Las contribuciones son bienvenidas! Por favor:

1. **Fork** el proyecto
2. **Crea** una feature branch (`git checkout -b feature/amazing-feature`)
3. **Commit** tus cambios (`git commit -m 'Add amazing feature'`)
4. **Push** a la branch (`git push origin feature/amazing-feature`)
5. **Abre** un Pull Request

### Guías de Contribución

- Seguir los **principios de arquitectura** definidos
- **Tests obligatorios** para nuevas features
- **Performance benchmarks** para cambios críticos
- **Documentación actualizada** en README

## 📄 License

🆓 **Not making money with it?** → Free noncommercial license  
💰 **Making money with it?** → One-time commercial license

This project is dual-licensed:
- [Noncommercial License](LICENSE-NONCOMMERCIAL) for free use
- [Commercial License](LICENSE-COMMERCIAL-TERMS) for business use

See [LICENSE](LICENSE) for details.

### 🔍 Which License Do I Need?

```
Are you making money using this software?
├── 🚫 NO → Free license
└── 💰 YES → Commercial license (one-time payment)
```

### 💼 Commercial License
- **One-time payment** - no recurring fees
- **Unlimited projects** with the license
- **Full commercial rights**
- **30-day evaluation** available

**Get quote:** licensing@your-domain.com

### 🆓 Free License (Noncommercial)
- ✅ Personal projects
- ✅ Learning and education
- ✅ Open source projects
- ✅ Internal tools (not sold)
- ✅ Research and experiments

**[Read full terms →](LICENSE-NONCOMMERCIAL)**

### 📞 License Questions?
Email: rodframeh@gmail.com 

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md).

**By contributing, you agree to assign copyright to enable commercial licensing and project sustainability.**

## 💬 Community

- 🐛 [**Issue Tracker**](https://github.com/mdz-technology/model_manager/issues)
- 💡 [**Feature Requests**](https://github.com/mdz-technology/model_manager/discussions)
- 📧 [**Email Support**](mailto:rodframeh@gmail.com)
