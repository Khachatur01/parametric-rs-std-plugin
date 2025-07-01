use std::any::Any;
use geometry::figure::segment::Segment;
use geometry::point::point_2d::Point2D;
use parametric_rs::entity::{EntityConversionError, Entity, EntityConverter};
use parametric_rs::sketch::Sketch;
use crate::converter::sketch_shape_converter::shape::Shape;
use crate::entity::point::Point2DEntity;
use crate::param::f64_param::F64Param;

pub struct Segment2DEntity {
    pub start: Point2DEntity,
    pub end: Point2DEntity,
}

impl Entity for Segment2DEntity {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct Segment2DToShapeConverter;
impl EntityConverter<Shape> for Segment2DToShapeConverter {
    fn try_convert(&self, sketch: &Sketch, entity: &dyn Entity) -> Result<Shape, EntityConversionError> {
        /* fixme: replace unwrap with proper error handling */
        let line_entity = entity.as_any().downcast_ref::<Segment2DEntity>().unwrap();

        let start_x = &line_entity.start.x;
        let start_y = &line_entity.start.y;
        let end_x = &line_entity.end.x;
        let end_y = &line_entity.end.y;

        let start_x = **sketch.params().get(start_x).unwrap().value().downcast_ref::<F64Param>().unwrap();
        let start_y = **sketch.params().get(start_y).unwrap().value().downcast_ref::<F64Param>().unwrap();

        let end_x = **sketch.params().get(end_x).unwrap().value().downcast_ref::<F64Param>().unwrap();
        let end_y = **sketch.params().get(end_y).unwrap().value().downcast_ref::<F64Param>().unwrap();

        let segment = Shape::Segment2D(Segment::new(Point2D { x: start_x, y: start_y }, Point2D { x: end_x, y: end_y }));

        Ok(segment)
    }
}
