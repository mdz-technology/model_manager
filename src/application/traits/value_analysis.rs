use crate::ModelResult;
pub trait ValueAnalysis: Clone {
    fn deep_clone(&self) -> ModelResult<Self>;
    fn merge(&mut self, other: &Self) -> ModelResult<()>;
    fn calculate_hash(&self) -> ModelResult<u64>;
    fn equals(&self, other: &Self) -> ModelResult<bool>;

    fn equals_fast(&self, other: &Self) -> ModelResult<bool> {
        Ok(self.calculate_hash()? == other.calculate_hash()?)
    }
}