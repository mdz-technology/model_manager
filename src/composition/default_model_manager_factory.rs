use crate::infrastructure::implementations::model_manager::SimpleModelManager;
use crate::infrastructure::implementations::serde_dynamic_value::SerdeDynamicValue;
use crate::ModelManagerFactory;

pub struct DefaultModelManagerFactory;

impl ModelManagerFactory<SerdeDynamicValue> for DefaultModelManagerFactory {
    type Manager = SimpleModelManager<SerdeDynamicValue>;

    fn create() -> Self::Manager {
        SimpleModelManager::new()
    }
}