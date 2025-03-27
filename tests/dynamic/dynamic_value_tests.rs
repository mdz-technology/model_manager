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
    fn new_creates_empty_dynamic_data_test() {
        let dynamic_data = DynamicValueImpl::new_object();
        assert!(dynamic_data.is_empty());
    }

    #[test]
    fn insert_string_value_test() {
        let mut dynamic_data = DynamicValueImpl::new_object();
        dynamic_data.set("key1", DynamicValueImpl::from_str("value1")).unwrap();
        let dynamic_value = dynamic_data.get("key1").unwrap();
        let other = DynamicValueImpl::from_str("value1");
        assert_eq!(dynamic_value.as_str(), other.as_str());
    }

    #[test]
    fn insert_object_value_test() {
        let dynamic_data = setup();
        let row01 = dynamic_data.get("row01").unwrap().as_map().unwrap();
        let column_b = row01.get("columnB").unwrap().as_map().unwrap();
        let column_ba = column_b.get("columnBA").unwrap();
        let other = DynamicValueImpl::from_str("valueBA1");
        assert_eq!(column_ba.as_str(), other.as_str());
    }

    #[test]
    fn remove_string_value_test() {
        let mut dynamic_data = DynamicValueImpl::new_object();
        dynamic_data.set("key1", DynamicValueImpl::from_str("value1")).unwrap();
        dynamic_data.remove("key1").unwrap();
        assert!(dynamic_data.is_empty());
    }

    #[test]
    fn remove_object_value_test() {
        let mut dynamic_data = setup();
        dynamic_data.remove("row01").unwrap();
        assert!(dynamic_data.is_empty());
    }

    #[test]
    fn update_string_value_test() {
        let mut dynamic_data = DynamicValueImpl::new_object();
        dynamic_data.set("key1", DynamicValueImpl::from_str("value1")).unwrap();
        dynamic_data.set("key1", DynamicValueImpl::from_str("value2")).unwrap();
        let updated = dynamic_data.get("key1").unwrap();
        assert_eq!(updated.as_str(), DynamicValueImpl::from_str("value2").as_str());
    }

    #[test]
    fn update_object_value_test() {
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
    fn insert_array_test() {
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
}