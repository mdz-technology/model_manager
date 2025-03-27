#[cfg(test)]
mod tests {

    use model_manager::dynamic::application::dynamic_value::{DynamicValue};
    use model_manager::dynamic::infrastructure::dynamic_value_impl::DynamicValueImpl;
    use model_manager::model::application::model_manager::ModelManager;
    use model_manager::model::infrastructure::model_manager_impl::ModelManagerImpl;

    async fn setup() -> ModelManagerImpl<DynamicValueImpl> {
        let mut model = ModelManagerImpl::<DynamicValueImpl>::new();

        let mut scooby = DynamicValueImpl::new_object();
        scooby.set("name", DynamicValueImpl::from_str("Scooby")).unwrap();
        scooby.set("age", DynamicValueImpl::from_number(7.0).unwrap()).unwrap();

        let mut donna = DynamicValueImpl::new_object();
        donna.set("name", DynamicValueImpl::from_str("Donna")).unwrap();
        donna.set("age", DynamicValueImpl::from_number(5.0).unwrap()).unwrap();

        let mut pets = DynamicValueImpl::new_array();
        pets.push(scooby).unwrap();
        pets.push(donna).unwrap();

        let mut children = DynamicValueImpl::new_array();
        children.push(DynamicValueImpl::from_str("Tomas")).unwrap();
        children.push(DynamicValueImpl::from_str("Lucas")).unwrap();
        children.push(DynamicValueImpl::from_str("Juan")).unwrap();

        let mut address = DynamicValueImpl::new_object();
        address.set("city", DynamicValueImpl::from_str("Springfield")).unwrap();
        address.set("street", DynamicValueImpl::from_str("Avenue Franklin D. Roosevelt")).unwrap();
        address.set("number", DynamicValueImpl::from_number(102.0).unwrap()).unwrap();
        address.set("has_parking", DynamicValueImpl::from_bool(true)).unwrap();

        let model_name = "people".to_string();
        let id = Some("550e8400-e29b-41d4-a716-446655440000".to_string());

        let mut data = DynamicValueImpl::new_object();
        data.set("name", DynamicValueImpl::from_str("Xavier")).unwrap();
        data.set("surname", DynamicValueImpl::from_str("Smith")).unwrap();
        data.set("age", DynamicValueImpl::from_number(25.0).unwrap()).unwrap();
        data.set("address", address).unwrap();
        data.set("children", children).unwrap();
        data.set("pets", pets).unwrap();

        let _ = model.insert(model_name.clone(), id.clone(), data.clone()).await;

        model
    }


    #[actix_rt::test]
    async fn test_insert() {
        let mut model: ModelManagerImpl<DynamicValueImpl> = setup().await;

        let model = model.get("people".to_string(), "550e8400-e29b-41d4-a716-446655440000".to_string()).await;

        assert!(model.is_ok());
        let model = model.unwrap();

        let field_name =model.get("name").unwrap();
        let field_name = field_name.as_str().unwrap();
        assert_eq!(field_name, "Xavier");

        let address = model.get("address").unwrap();
        let address = address.as_map().unwrap();
        let city = address.get("city").unwrap();
        let city = city.as_str().unwrap();
        assert_eq!(city, "Springfield");

        let children = model.get("children").unwrap();
        let children = children.as_array().unwrap();
        assert_eq!(children.len(), 3);

        let pets = model.get("pets").unwrap();
        let pets = pets.as_array().unwrap();
        let scooby = pets.get(0).unwrap();
        let scooby = scooby.as_map().unwrap();
        let scooby_name = scooby.get("name").unwrap();
        let scooby_name = scooby_name.as_str().unwrap();
        assert_eq!(scooby_name, "Scooby");
    }

    #[actix_rt::test]
    async fn test_update() {
        let mut model: ModelManagerImpl<DynamicValueImpl> = setup().await;

        let mut address = DynamicValueImpl::new_object();
        address.set("city", DynamicValueImpl::from_str("New York")).unwrap();
        address.set("street", DynamicValueImpl::from_str("5th Avenue")).unwrap();
        address.set("number", DynamicValueImpl::from_number(123.0).unwrap()).unwrap();
        address.set("has_parking", DynamicValueImpl::from_bool(false)).unwrap();

        let mut data = DynamicValueImpl::new_object();
        data.set("address", address).unwrap();

        let _ = model.update("people".to_string(), "550e8400-e29b-41d4-a716-446655440000".to_string(), data).await;

        let model = model.get("people".to_string(), "550e8400-e29b-41d4-a716-446655440000".to_string()).await;

        assert!(model.is_ok());
        let model = model.unwrap();

        let address = model.get("address").unwrap();
        let address = address.as_map().unwrap();
        let city = address.get("city").unwrap();
        let city = city.as_str().unwrap();
        assert_eq!(city, "New York");
    }

    #[actix_rt::test]
    async fn test_remove() {
        let mut model: ModelManagerImpl<DynamicValueImpl> = setup().await;


        let _ = model.remove("people".to_string(), "550e8400-e29b-41d4-a716-446655440000".to_string()).await;

        let model = model.get("people".to_string(), "550e8400-e29b-41d4-a716-446655440000".to_string()).await;

        assert!(model.is_err());
    }

    #[actix_rt::test]
    async fn test_internal_object_remove() {
        let mut model: ModelManagerImpl<DynamicValueImpl> = setup().await;

        let model = model.get("people".to_string(), "550e8400-e29b-41d4-a716-446655440000".to_string()).await;

        assert!(model.is_ok());
        let mut model = model.unwrap();

        // Remove children of model
        model.remove("children").unwrap();

        let children = model.get("children");

        assert!(children.is_none());
    }

    #[actix_rt::test]
    async fn test_get_all() {
        let mut model: ModelManagerImpl<DynamicValueImpl> = setup().await;

        let model = model.get_all("people".to_string()).await;

        assert!(model.is_ok());
        let model = model.unwrap();

        assert_eq!(model.len(), 1);
    }

}