mod geometry;

use crate::converter::model_geometry_converter::geometry::Geometry;
use parametric_rs::model_3d::{Model3D, ModelConversionError, ModelInto};

impl ModelInto<Geometry> for Model3D {
    fn try_into(&self) -> Result<Vec<Geometry>, ModelConversionError> {
        let mut sketches = self.sketches.values().cloned().collect::<Vec<_>>();
        for feature in &self.features {
            for sketch in &mut sketches {
                feature.apply(sketch);
            }
        }

        todo!();

        Ok(vec![])
    }
}
