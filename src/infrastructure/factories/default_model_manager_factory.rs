use crate::infrastructure::implementations::actix_model_manager::ActixModelManager;
use crate::infrastructure::implementations::serde_dynamic_value::SerdeDynamicValue;
use crate::ModelManagerFactory;

pub struct DefaultModelManagerFactory;

impl ModelManagerFactory<SerdeDynamicValue> for DefaultModelManagerFactory {
    type Manager = ActixModelManager<SerdeDynamicValue>;
    fn create() -> Self::Manager {
        ActixModelManager::new()
    }
}