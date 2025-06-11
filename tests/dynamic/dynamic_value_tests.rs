#[cfg(test)]
mod tests {
    use model_manager::dynamic::application::dynamic_value::{DynamicValue};
    use model_manager::dynamic::infrastructure::dynamic_value_impl::DynamicValueImpl;

    fn setup() -> DynamicValueImpl {
        let mut dynamic_data = DynamicValueImpl::new_object();

        let mut internal_obj = DynamicValueImpl::new_object();
        internal_obj.set("columnBA", DynamicValueImpl::from_str("valueBA1")).unwrap();
        internal_obj.set("columnBB", DynamicValueImpl::from_bool(false)).unwrap();

        let mut row01 = DynamicValueImpl::new_object();
        row01.set("columnA", DynamicValueImpl::from_str("valueA1")).unwrap();
        row01.set("columnB", internal_obj).unwrap();
        row01.set("columnC", DynamicValueImpl::from_number(12.5).unwrap()).unwrap();
        dynamic_data.set("row01", row01).unwrap();

        dynamic_data
    }

    #[test]
    fn test_new_creates_empty_dynamic_data() {
        let dynamic_data = DynamicValueImpl::new_object();
        assert!(dynamic_data.is_empty());
    }

    #[test]
    fn test_insert_string_value() {
        let mut dynamic_data = DynamicValueImpl::new_object();
        dynamic_data.set("key1", DynamicValueImpl::from_str("value1")).unwrap();
        let dynamic_value = dynamic_data.get("key1").unwrap();
        let other = DynamicValueImpl::from_str("value1");
        assert_eq!(dynamic_value.as_str(), other.as_str());
    }

    #[test]
    fn test_insert_object_value() {
        let dynamic_data = setup();
        let row01 = dynamic_data.get("row01").unwrap().as_map().unwrap();
        let column_b = row01.get("columnB").unwrap().as_map().unwrap();
        let column_ba = column_b.get("columnBA").unwrap();
        let other = DynamicValueImpl::from_str("valueBA1");
        assert_eq!(column_ba.as_str(), other.as_str());
    }

    #[test]
    fn test_remove_string_value() {
        let mut dynamic_data = DynamicValueImpl::new_object();
        dynamic_data.set("key1", DynamicValueImpl::from_str("value1")).unwrap();
        dynamic_data.remove("key1").unwrap();
        assert!(dynamic_data.is_empty());
    }

    #[test]
    fn test_remove_object_value() {
        let mut dynamic_data = setup();
        dynamic_data.remove("row01").unwrap();
        assert!(dynamic_data.is_empty());
    }

    #[test]
    fn test_update_string_value() {
        let mut dynamic_data = DynamicValueImpl::new_object();
        dynamic_data.set("key1", DynamicValueImpl::from_str("value1")).unwrap();
        dynamic_data.set("key1", DynamicValueImpl::from_str("value2")).unwrap();
        let updated = dynamic_data.get("key1").unwrap();
        assert_eq!(updated.as_str(), DynamicValueImpl::from_str("value2").as_str());
    }

    #[test]
    fn test_update_object_value() {
        let mut dynamic_data = setup();

        let mut internal_obj = DynamicValueImpl::new_object();
        internal_obj.set("columnBA", DynamicValueImpl::from_str("valueBA2")).unwrap();
        internal_obj.set("columnBB", DynamicValueImpl::from_bool(true)).unwrap();

        let mut new_row = DynamicValueImpl::new_object();
        new_row.set("columnA", DynamicValueImpl::from_str("valueA2")).unwrap();
        new_row.set("columnB", internal_obj).unwrap();
        new_row.set("columnC", DynamicValueImpl::from_number(25.5).unwrap()).unwrap();

        dynamic_data.set("row01", new_row).unwrap();

        let row01 = dynamic_data.get("row01").unwrap().as_map().unwrap();
        let column_b = row01.get("columnB").unwrap().as_map().unwrap();
        let column_ba = column_b.get("columnBA").unwrap();
        let other = DynamicValueImpl::from_str("valueBA2");
        assert_eq!(column_ba.as_str(), other.as_str());
    }

    #[test]
    fn test_insert_array() {
        let mut dynamic_data = DynamicValueImpl::new_object();
        let mut array = DynamicValueImpl::new_array();
        array.push(DynamicValueImpl::from_str("value1")).unwrap();
        array.push(DynamicValueImpl::from_number(12.5).unwrap()).unwrap();
        dynamic_data.set("key1", array).unwrap();
        let dynamic_value = dynamic_data.get("key1").unwrap();
        let mut other = DynamicValueImpl::new_array();
        other.push(DynamicValueImpl::from_str("value1")).unwrap();
        other.push(DynamicValueImpl::from_number(12.5).unwrap()).unwrap();
        assert_eq!(dynamic_value.as_array().unwrap().len(), other.as_array().unwrap().len());
        assert_eq!(dynamic_value.as_array().unwrap()[0].as_str(), other.as_array().unwrap()[0].as_str());
    }

    #[test]
    fn test_iter_object() {
        let mut obj = DynamicValueImpl::new_object();
        obj.set("name", DynamicValueImpl::from_str("Juan")).unwrap();
        obj.set("age", DynamicValueImpl::from_number(25.0).unwrap()).unwrap();

        let mut items = Vec::new();
        if let Some(iter) = obj.iter_object() {
            for (key, value) in iter {
                items.push((key, value.to_string()));
            }
        }

        assert_eq!(items.len(), 2);
        assert!(items.iter().any(|(k, _)| k == "name"));
        assert!(items.iter().any(|(k, _)| k == "age"));
    }

    #[test]
    fn test_iter_array() {
        let mut arr = DynamicValueImpl::new_array();
        arr.push(DynamicValueImpl::from_str("item1")).unwrap();
        arr.push(DynamicValueImpl::from_str("item2")).unwrap();

        let mut items = Vec::new();
        if let Some(iter) = arr.iter_array() {
            for value in iter {
                items.push(value.as_str().unwrap());
            }
        }

        assert_eq!(items, vec!["item1", "item2"]);
    }

    #[test]
    fn test_get_by_path() {
        let mut obj = DynamicValueImpl::new_object();

        let mut user = DynamicValueImpl::new_object();
        let mut profile = DynamicValueImpl::new_object();
        profile.set("name", DynamicValueImpl::from_str("Juan")).unwrap();
        user.set("profile", profile).unwrap();
        obj.set("user", user).unwrap();

        let result = obj.get_by_path("user.profile.name").unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().as_str().unwrap(), "Juan");

        let result = obj.get_by_path("user.profile.email").unwrap();
        assert!(result.is_none());

        let result = obj.get_by_path("user..profile");
        assert!(result.is_err());
    }

    #[test]
    fn test_set_by_path() {
        let mut obj = DynamicValueImpl::new_object();

        obj.set_by_path("user.profile.name", DynamicValueImpl::from_str("Juan")).unwrap();

        let result = obj.get_by_path("user.profile.name").unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().as_str().unwrap(), "Juan");

        obj.set_by_path("user.profile.age", DynamicValueImpl::from_number(25.0).unwrap()).unwrap();

        let result = obj.get_by_path("user.profile.age").unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().as_number().unwrap(), 25.0);

        let result = obj.get_by_path("user.profile.name").unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().as_str().unwrap(), "Juan");
    }

    #[test]
    fn test_deep_clone() {
        let mut original = DynamicValueImpl::new_object();
        original.set("name", DynamicValueImpl::from_str("Original")).unwrap();

        let mut cloned = original.deep_clone();
        cloned.set("name", DynamicValueImpl::from_str("Cloned")).unwrap();

        assert_eq!(original.get("name").unwrap().as_str().unwrap(), "Original");
        assert_eq!(cloned.get("name").unwrap().as_str().unwrap(), "Cloned");
    }

    #[test]
    fn test_merge() {
        let mut obj1 = DynamicValueImpl::new_object();
        obj1.set("a", DynamicValueImpl::from_number(1.0).unwrap()).unwrap();
        obj1.set("b", DynamicValueImpl::from_number(2.0).unwrap()).unwrap();

        let mut obj2 = DynamicValueImpl::new_object();
        obj2.set("b", DynamicValueImpl::from_number(3.0).unwrap()).unwrap();
        obj2.set("c", DynamicValueImpl::from_number(4.0).unwrap()).unwrap();

        obj1.merge(&obj2).unwrap();

        assert_eq!(obj1.get("a").unwrap().as_number().unwrap(), 1.0); // Original
        assert_eq!(obj1.get("b").unwrap().as_number().unwrap(), 3.0); // Sobrescrito
        assert_eq!(obj1.get("c").unwrap().as_number().unwrap(), 4.0); // Nuevo
    }

    #[test]
    fn test_keys() {
        let mut obj = DynamicValueImpl::new_object();
        obj.set("name", DynamicValueImpl::from_str("Juan")).unwrap();
        obj.set("age", DynamicValueImpl::from_number(25.0).unwrap()).unwrap();

        let keys = obj.keys();
        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&"name".to_string()));
        assert!(keys.contains(&"age".to_string()));
    }

    #[test]
    fn test_matches_schema() {
        let mut user = DynamicValueImpl::new_object();
        user.set("name", DynamicValueImpl::from_str("Juan")).unwrap();
        user.set("age", DynamicValueImpl::from_number(25.0).unwrap()).unwrap();

        let mut schema = DynamicValueImpl::new_object();
        schema.set("name", DynamicValueImpl::from_str("string")).unwrap();
        schema.set("age", DynamicValueImpl::from_str("number")).unwrap();

        assert!(user.matches_schema(&schema));

        user.set("age", DynamicValueImpl::from_str("not_a_number")).unwrap();
        assert!(!user.matches_schema(&schema));
    }

    #[test]
    fn test_path_validation() {
        assert!(DynamicValueImpl::is_valid_path("user.profile.name"));
        assert!(DynamicValueImpl::is_valid_path("simple"));

        assert!(!DynamicValueImpl::is_valid_path(""));
        assert!(!DynamicValueImpl::is_valid_path(".user"));
        assert!(!DynamicValueImpl::is_valid_path("user."));
        assert!(!DynamicValueImpl::is_valid_path("user..profile"));
    }

    #[test]
    fn test_split_path() {
        let parts = DynamicValueImpl::split_path("user.profile.name");
        assert_eq!(parts, vec!["user", "profile", "name"]);

        let parts = DynamicValueImpl::split_path("simple");
        assert_eq!(parts, vec!["simple"]);
    }
}