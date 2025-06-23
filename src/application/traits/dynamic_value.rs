use super::{
    core_value::CoreValue, path_navigation::PathNavigation, value_analysis::ValueAnalysis,
    value_introspection::ValueIntrospection,
};

pub trait DynamicValue: CoreValue + PathNavigation + ValueIntrospection + ValueAnalysis {}

impl<T> DynamicValue for T where T: CoreValue + PathNavigation + ValueIntrospection + ValueAnalysis {}
