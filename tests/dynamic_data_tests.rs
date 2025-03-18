#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use model_manager::dynamic::application::dynamic_data::DynamicData;
    use model_manager::dynamic::application::dynamic_value::DynamicValue;
    use model_manager::dynamic::infrastructure::dynamic_value_impl::DynamicValueImpl;

    fn setup() -> DynamicData {
        let mut dynamic_data = DynamicData::new();

        let mut internal_obj = HashMap::new();
        internal_obj.insert("columnBA", DynamicValueImpl::from_str("valueBA1"));
        internal_obj.insert("columnBB", DynamicValueImpl::from_bool(false));

        let mut obj = HashMap::new();
        obj.insert("columnA", DynamicValueImpl::from_str("valueA1"));
        obj.insert("columnB", DynamicValueImpl::from_map(internal_obj));
        obj.insert("columnC", DynamicValueImpl::from_number(12.5));
        dynamic_data.insert("row01", DynamicValueImpl::from_map(obj));

        dynamic_data
    }

    #[test]
    fn new_creates_empty_dynamic_data() {
        let dynamic_data = DynamicData::new();
        assert!(dynamic_data.is_empty());
    }

    #[test]
    fn insert_string_value() {
        let mut dynamic_data = DynamicData::new();
        dynamic_data.insert("key1", DynamicValueImpl::from_str("value1"));
        let dynamic_value = dynamic_data.get("key1").unwrap();
        let other = DynamicValueImpl::from_str("value1");
        assert_eq!(dynamic_value.as_str(), other.as_str());
    }

    #[test]
    fn insert_object_value() {
        let dynamic_data = setup();
        let row01 = dynamic_data.get("row01").unwrap().as_map().unwrap();
        let columnB = row01.get("columnB").unwrap().as_map().unwrap();
        let columnBA = columnB.get("columnBA").unwrap();

        let mut other = DynamicValueImpl::from_str("valueBA1");

        assert_eq!(columnBA.as_str(), other.as_str());
    }

    #[test]
    fn remove_string_value() {
        let mut dynamic_data = DynamicData::new();
        dynamic_data.insert("key1", DynamicValueImpl::from_str("value1"));
        let removed = dynamic_data.remove("key1");
        assert!(dynamic_data.is_empty());
    }

    #[test]
    fn remove_object_value() {
        let mut dynamic_data = setup();
        let removed = dynamic_data.remove("row01");
        assert!(dynamic_data.is_empty());
    }

    #[test]
    fn update_string_value() {
        let mut dynamic_data = DynamicData::new();
        dynamic_data.insert("key1", DynamicValueImpl::from_str("value1"));
        dynamic_data.update("key1", DynamicValueImpl::from_str("value2"));
        assert_eq!(dynamic_data.get("key1").unwrap().as_str(), DynamicValueImpl::from_str("value2").as_str());
    }

    #[test]
    fn update_object_value() {
        let mut dynamic_data = setup();
        let mut internal_obj = HashMap::new();
        internal_obj.insert("columnBA", DynamicValueImpl::from_str("valueBA2"));
        internal_obj.insert("columnBB", DynamicValueImpl::from_bool(true));

        let mut obj = HashMap::new();
        obj.insert("columnA", DynamicValueImpl::from_str("valueA2"));
        obj.insert("columnB", DynamicValueImpl::from_map(internal_obj));
        obj.insert("columnC", DynamicValueImpl::from_number(25.5));

        dynamic_data.update("row01", DynamicValueImpl::from_map(obj));

        let row01 = dynamic_data.get("row01").unwrap().as_map().unwrap();
        let columnB = row01.get("columnB").unwrap().as_map().unwrap();
        let columnBA = columnB.get("columnBA").unwrap();

        let mut other = DynamicValueImpl::from_str("valueBA2");

        assert_eq!(columnBA.as_str(), other.as_str());
    }

    //get
    //array

}