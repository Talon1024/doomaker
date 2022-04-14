use crate::secplane::SectorPlane;
use crate::edge::EdgeVertexIndex;
use crate::vector::Vector2;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct LineQuad {
    a: EdgeVertexIndex,
    b: EdgeVertexIndex,
    upper: Rc<SectorPlane>,
    lower: Rc<SectorPlane>,
    colour_top: Option<[u8; 3]>,
    colour_btm: Option<[u8; 3]>,
}

enum QuadType {
    NormalQuad,
    OppositeTriangles,
    Vavoom3DFloor,
}

type VertexPosition = [f32; 3];

impl LineQuad {
    pub fn to_3D(&self, vertices: &[Vector2]) -> Vec<VertexPosition> {
        let mut positions: Vec<VertexPosition> = Vec::new();
        let va = &vertices[self.a];
        let vb = &vertices[self.b];
        // "A/B upper/lower height"
        let auh = self.upper.z_at(va);
        let buh = self.upper.z_at(vb);
        let alh = self.lower.z_at(va);
        let blh = self.lower.z_at(vb);
        let quad_type = {
            if alh > auh && blh > buh {
                QuadType::Vavoom3DFloor
            } else if alh > auh || blh > buh {
                QuadType::OppositeTriangles
            } else {
                QuadType::NormalQuad
            }
        };
        positions
    }
}
