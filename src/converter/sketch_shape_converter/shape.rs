use geometry::figure::circle::Circle;
use geometry::figure::mesh::Mesh;
use geometry::figure::segment::Segment;
use geometry::point::point_2d::Point2D;
use parametric_rs::sketch::Sketch;
use crate::entity::point::Point2DEntity;
use crate::entity::segment::Segment2DEntity;
use crate::param::f64_param::F64Param;

pub enum Shape {
    Point2D(Point2D),
    Segment2D(Segment<Point2D>),
    Circle(Circle),
    Mesh2D(Mesh<Point2D>)
}

pub trait AddAsEntity {
    fn add_as_entity(&self, sketch: &mut Sketch);
}

impl AddAsEntity for Point2D {
    fn add_as_entity(&self, sketch: &mut Sketch) {
        let x_param_id = sketch.add_param(F64Param(self.x));
        let y_param_id = sketch.add_param(F64Param(self.y));

        let point_entity = Point2DEntity {
            x: x_param_id,
            y: y_param_id,
        };

        sketch.add_entity(point_entity);
    }
}

impl AddAsEntity for Segment<Point2D> {
    fn add_as_entity(&self, sketch: &mut Sketch) {
        let start_x_param_id = sketch.add_param(F64Param(self.start().x));
        let start_y_param_id = sketch.add_param(F64Param(self.start().y));

        let end_x_param_id = sketch.add_param(F64Param(self.end().x));
        let end_y_param_id = sketch.add_param(F64Param(self.end().y));

        let start_point_entity = Point2DEntity {
            x: start_x_param_id,
            y: start_y_param_id,
        };
        let end_point_entity = Point2DEntity {
            x: end_x_param_id,
            y: end_y_param_id,
        };

        let segment_entity = Segment2DEntity {
            start: start_point_entity,
            end: end_point_entity,
        };

        sketch.add_entity(segment_entity);
    }
}
