use crate::plane::Plane;
use crate::edge::EdgeVertexIndex;
use glam::Vec2;

#[derive(Debug, Clone)]
pub struct LineQuad {
    quad_type: QuadType,
}

#[derive(Debug, Clone)]
enum QuadType {
    NormalQuad,
    OppositeTriangles,
    Vavoom3DFloor,
    SkewedAtOneEnd, // Only for midtex quads
    SkewedAtBothEnds,
}

pub fn solid_quad(va: Vec2, vb: Vec2, upper: Plane, lower: Plane, max_height: Option<f32>) -> LineQuad {
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
