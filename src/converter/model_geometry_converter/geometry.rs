use geometry_rs::figure::circle::Circle;
use geometry_rs::figure::mesh::Mesh;
use geometry_rs::figure::segment::Segment;
use geometry_rs::point::point_2d::Point2D;

pub enum Geometry {
    Point2D(Point2D),
    Segment2D(Segment<Point2D>),
    Circle(Circle),
    Mesh2D(Mesh<Point2D>)
}

// pub trait AddAsEntity {
//     fn add_as_entity(&self, sketch: &mut Sketch);
// }
//
// impl AddAsEntity for Point2D {
//     fn add_as_entity(&self, sketch: &mut Sketch) {
//         let x_param_id = sketch.add_param(Param::f64(self.x));
//         let y_param_id = sketch.add_param(Param::f64(self.y));
//
//         let point_entity = PointEntity {
//             x: x_param_id,
//             y: y_param_id,
//         };
//
//         sketch.add_entity(Entity::Point(point_entity));
//     }
// }
//
// impl AddAsEntity for Segment<Point2D> {
//     fn add_as_entity(&self, sketch: &mut Sketch) {
//         let start_x_param_id = sketch.add_param(Param::f64(self.start().x));
//         let start_y_param_id = sketch.add_param(Param::f64(self.start().y));
//
//         let end_x_param_id = sketch.add_param(Param::f64(self.end().x));
//         let end_y_param_id = sketch.add_param(Param::f64(self.end().y));
//
//         let start_point_entity = PointEntity {
//             x: start_x_param_id,
//             y: start_y_param_id,
//         };
//         let end_point_entity = PointEntity {
//             x: end_x_param_id,
//             y: end_y_param_id,
//         };
//
//         let segment_entity = SegmentEntity {
//             start: start_point_entity,
//             end: end_point_entity,
//         };
//
//         sketch.add_entity(Entity::Segment(segment_entity));
//     }
// }
