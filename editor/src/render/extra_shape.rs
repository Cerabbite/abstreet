use aabb_quadtree::geom::Rect;
use ezgui::GfxCtx;
use geom::{Polygon, Pt2D};
use graphics::types::Color;
use kml::{ExtraShape, ExtraShapeGeom, ExtraShapeID};
use map_model::geometry;
use render::{get_bbox, EXTRA_SHAPE_POINT_RADIUS, EXTRA_SHAPE_THICKNESS};
use std::collections::BTreeMap;

#[derive(Debug)]
enum Shape {
    Polygon(Polygon),
    Circle([f64; 4]),
}

#[derive(Debug)]
pub struct DrawExtraShape {
    pub id: ExtraShapeID,
    shape: Shape,
    attributes: BTreeMap<String, String>,
}

impl DrawExtraShape {
    pub fn new(s: ExtraShape) -> DrawExtraShape {
        DrawExtraShape {
            id: s.id,
            shape: match s.geom {
                ExtraShapeGeom::Point(pt) => {
                    Shape::Circle(geometry::make_circle(pt, EXTRA_SHAPE_POINT_RADIUS))
                }
                ExtraShapeGeom::Points(pl) => {
                    Shape::Polygon(pl.make_polygons(EXTRA_SHAPE_THICKNESS).unwrap())
                }
            },
            attributes: s.attributes,
        }
    }

    pub fn draw(&self, g: &mut GfxCtx, color: Color) {
        match self.shape {
            Shape::Polygon(ref p) => g.draw_polygon(color, &p),
            Shape::Circle(c) => g.draw_ellipse(color, c),
        }
    }

    pub fn contains_pt(&self, pt: Pt2D) -> bool {
        match self.shape {
            Shape::Polygon(ref p) => p.contains_pt(pt),
            Shape::Circle(c) => geometry::point_in_circle(&c, pt),
        }
    }

    pub fn get_bbox(&self) -> Rect {
        match self.shape {
            Shape::Polygon(ref p) => get_bbox(&p.get_bounds()),
            Shape::Circle(c) => geometry::circle_to_bbox(&c),
        }
    }

    pub fn tooltip_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        for (k, v) in &self.attributes {
            // Make interesting atributes easier to spot
            if k == "TEXT" {
                lines.push(format!("*** {} = {}", k, v));
            } else {
                lines.push(format!("{} = {}", k, v));
            }
        }
        lines
    }
}
