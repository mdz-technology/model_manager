use std::any::Any;
use std::collections::HashMap;
use serde_json::{Map, Value};
use crate::dynamic::application::dynamic_value::DynamicValue;

#[derive(Debug, Clone)]
pub struct DynamicValueImpl {
    value: Value
}

impl DynamicValue for DynamicValueImpl {


    /// Crea un nuevo JsonWrapper desde un mapa clave-valor de Rust
    fn from_map(map: HashMap<&str, Box<dyn DynamicValue>>) -> Box<dyn DynamicValue> {
        /*Box::new(
            DynamicValueImpl {
                value: serde_json::Value::Object(map.into_iter()
                    .map(|(k, v)| (k.to_string(), v.to_json()))
                    .collect())
            })

*/

        let json_map: Map<String, Value> = map.into_iter()
            .map(|(k, v)| {
                // Downcast to get the concrete inner value.
                if let Some(inner) = v.as_any().downcast_ref::<DynamicValueImpl>() {
                    (k.to_string(), inner.value.clone())
                } else {
                    panic!("Expected DynamicValueImpl")
                }
            })
            .collect();
        Box::new( DynamicValueImpl{ value: Value::Object(json_map) })
    }

    /// Crea un nuevo JsonWrapper desde un vector de JsonWrapper
    fn from_array(array: Vec<Box<dyn DynamicValue>>) -> Box<dyn DynamicValue> {
        /*Box::new(DynamicValueImpl { value: serde_json::Value::Array(array.into_iter().map(|v| v.to_json()).collect()) })*/
        let json_array: Vec<Value> = array.into_iter().map(|v| {
            if let Some(inner) = v.as_any().downcast_ref::<DynamicValueImpl>() {
                inner.value.clone()
            } else {
                panic!("Expected DynamicValueImpl")
            }
        }).collect();
        Box::new(DynamicValueImpl { value: Value::Array(json_array) })
        //Box::new(DynamicValueImpl { })
    }

    /// Crea un JsonWrapper desde un string
    fn from_str(s: &str) -> Box<dyn DynamicValue> {
        Box::new(DynamicValueImpl { value: serde_json::Value::String(s.to_string()) })
    }

    /// Crea un JsonWrapper desde un número
    fn from_number(n: f64) -> Box<dyn DynamicValue> {
        // Usar Number::from_f64 que devuelve un Option
        let num = serde_json::Number::from_f64(n)
            .expect("Número no válido (NaN o infinito)"); // O manejar el error adecuadamente

        Box::new(DynamicValueImpl { value: serde_json::Value::Number(num) })
    }

    /// Crea un JsonWrapper desde un booleano
    fn from_bool(b: bool) -> Box<dyn DynamicValue> {
        Box::new(DynamicValueImpl {value: Value::Bool(b)  })
    }

    /// Verifica si es un objeto
    fn is_object(&self) -> bool {
        self.value.is_object()
    }

    /// Verifica si es un array
    fn is_array(&self) -> bool {
        self.value.is_array()
    }

    /// Obtiene un string si es posible
    fn as_str(&self) -> Option<String> {
        self.value.as_str().map(|s| s.to_string())
    }

    /// Obtiene un número si es posible
    fn as_number(&self) -> Option<f64> {
        self.value.as_f64()
    }

    /// Obtiene un booleano si es posible
    fn as_bool(&self) -> Option<bool> {
        self.value.as_bool()
    }

    /// Obtiene un HashMap si es un objeto JSON
    fn as_map(&self) -> Option<HashMap<String, Box<dyn DynamicValue>>> {
        self.value.as_object().map(|map| {
            map.iter().map(|(k, v)| {
                (k.clone(), Box::new(DynamicValueImpl { value: v.clone() }) as Box<dyn DynamicValue>)
            }).collect()
        })
    }

    /// Obtiene un vector si es un array JSON
    fn as_array(&self) -> Option<Vec<Box<dyn DynamicValue>>> {
        self.value.as_array().map(|arr| {
            arr.iter().map(|v| Box::new(DynamicValueImpl { value: v.clone() }) as Box<dyn DynamicValue>).collect()
        })
    }

    /// Obtiene un campo dentro del objeto JSON
    fn get(&self, key: &str) -> Option<Box<dyn DynamicValue>> {
        self.value.get(key).map(|v| Box::new(DynamicValueImpl { value: v.clone() }) as Box<dyn DynamicValue>)
    }

    /// Establece un valor dentro del objeto JSON
    fn set(&mut self, key: &str, value: Box<dyn DynamicValue>) {
        if let Value::Object(ref mut map) = self.value {
            if let Some(inner) = value.as_any().downcast_ref::<DynamicValueImpl>() {
                map.insert(key.to_string(), inner.value.clone());
            } else {
                panic!("Expected DynamicValueImpl")
            }
        }
    }

    fn remove(&mut self, key: &str) {
        if let Value::Object(ref mut map) = self.value {
            map.remove(key);
        }
    }

    /// Agrega un elemento a un array JSON
    fn push(&mut self, value: Box<dyn DynamicValue>) {
        if let Value::Array(ref mut arr) = self.value {
            if let Some(inner) = value.as_any().downcast_ref::<DynamicValueImpl>() {
                arr.push(inner.value.clone());
            } else {
                panic!("Expected DynamicValueImpl")
            }
        }
    }

    /// Convierte el JSON en una cadena de texto
    fn to_string(&self) -> String {
        self.value.to_string()
    }

    fn is_empty(&self) -> bool {
        match &self.value {
            Value::Null => true,
            Value::String(s) if s.is_empty() => true,
            Value::Array(arr) if arr.is_empty() => true,
            Value::Object(obj) if obj.is_empty() => true,
            _ => false,
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn DynamicValue> {
        Box::new(self.clone()) // Use the derived `Clone`
    }
}
