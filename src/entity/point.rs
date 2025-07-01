use crate::converter::sketch_shape_converter::shape::Shape;
use crate::param::f64_param::F64Param;
use geometry::point::point_2d::Point2D;
use parametric_rs::entity::{Entity, EntityConversionError, EntityConverter};
use parametric_rs::param::ParamId;
use parametric_rs::sketch::Sketch;
use std::any::Any;

pub struct Point2DEntity {
    pub x: ParamId,
    pub y: ParamId,
}

impl Entity for Point2DEntity {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Point2DToShapeConverter;
impl EntityConverter<Shape> for Point2DToShapeConverter {
    fn try_convert(&self, sketch: &Sketch, entity: &dyn Entity) -> Result<Shape, EntityConversionError> {
        /* fixme: replace unwrap with proper error handling */
        let point_entity = entity.as_any().downcast_ref::<Point2DEntity>().unwrap();
        let x = **sketch.params().get(&point_entity.x).unwrap().value().downcast_ref::<F64Param>().unwrap();
        let y = **sketch.params().get(&point_entity.y).unwrap().value().downcast_ref::<F64Param>().unwrap();

        let point = Shape::Point2D(Point2D { x, y });

        Ok(point)
    }
}
