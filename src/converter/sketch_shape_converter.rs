pub mod shape;

use parametric_rs::entity::EntityConverter;
use parametric_rs::sketch::{Sketch, SketchConversionError, SketchConverter};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use crate::converter::sketch_shape_converter::shape::{AddAsEntity, Shape};
use crate::entity::point::Point2DEntity;
use crate::entity::segment::Segment2DEntity;
use crate::param::f64_param::F64Param;

pub struct SketchShapeConverter {
    entity_converters: HashMap<TypeId, Box<dyn EntityConverter<Shape>>>,
}

impl From<Vec<Box<dyn EntityConverter<Shape>>>> for SketchShapeConverter {
    fn from(converters: Vec<Box<dyn EntityConverter<Shape>>>) -> Self {
        let mut entity_converters = HashMap::new();

        for converter in converters {
            entity_converters.insert(converter.type_id(), converter);
        }

        Self {
            entity_converters
        }
    }
}

impl SketchConverter<Shape> for SketchShapeConverter {
    fn try_into(&self, sketch: &Sketch) -> Result<Vec<Shape>, SketchConversionError> {
        todo!()
    }

    fn into(&self, sketch: &Sketch) -> Vec<Shape> {
        sketch.entities()
            .values()
            .filter_map(|entity|
                self.entity_converters
                    .get(&entity.as_ref().type_id())?
                    .try_convert(sketch, entity.as_ref())
                    .ok()
            )
            .collect()
    }

    fn from(&self, shapes: &[Shape]) -> Sketch {
        let mut sketch: Sketch = Sketch::empty();

        for shape in shapes {
            match shape {
                Shape::Point2D(point_2d) => point_2d.add_as_entity(&mut sketch),
                Shape::Segment2D(segment_2d) => segment_2d.add_as_entity(&mut sketch),
                Shape::Circle(circle) => {}
                Shape::Mesh2D(mesh_2d) => {}
            }
        }

        sketch
    }
}
