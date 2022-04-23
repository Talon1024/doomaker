use crate::plane::Plane;
use crate::edge::EdgeVertexIndex;
use glam::{Vec2, Vec3};

#[derive(Debug, Clone)]
pub struct LineQuad {
    quad_type: QuadType,
    vertices: Box<[Vec3]>,
    indices: Box<[u32]>,
}

#[derive(Debug, Clone)]
enum QuadType {
    NormalQuad,
    OppositeTriangles,
    Vavoom3DFloor,
    SkewedAtOneEnd, // Only for midtex quads
    SkewedAtBothEnds,
}

/// The minimum and maximum heights for a quad which does not span the full
/// height between the top and bottom sector planes.
#[derive(Debug, Clone, Copy)]
pub struct HeightLimits {
    pub top: f32,
    pub bottom: f32
}

impl From<HeightLimits> for Option<HeightLimits> {
    fn from(v: HeightLimits) -> Option<HeightLimits> {
        if v.bottom < v.top {
            Some(v)
        } else {
            None
        }
    }
}

pub fn calc_quad(va: Vec2, vb: Vec2, upper: Plane, lower: Plane, heights: Option<HeightLimits>) -> LineQuad {
    let mut positions: Vec<Vector3> = Vec::new();
    // "A/B upper/lower (absolute) height"
    let quad_type = {
        let auh = upper.z_at(va);
        let buh = upper.z_at(vb);
        let alh = lower.z_at(va);
        let blh = lower.z_at(vb);
        if alh > auh && blh > buh {
            QuadType::Vavoom3DFloor
        } else if alh > auh || blh > buh {
            QuadType::OppositeTriangles
        } else {
            QuadType::NormalQuad
        }
    };
    LineQuad {
        quad_type
    }
}
