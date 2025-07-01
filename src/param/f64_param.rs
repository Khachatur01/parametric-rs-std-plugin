use std::any::Any;
use std::ops::Deref;
use parametric_rs::param::Param;

pub struct F64Param(pub f64);
impl Param for F64Param {
    fn label(&self) -> Option<String> {
        None
    }

    fn value(&self) -> &dyn Any {
        &self.0
    }
}
impl Deref for F64Param {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
