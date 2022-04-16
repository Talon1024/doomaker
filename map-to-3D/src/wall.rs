use crate::secplane::SectorPlane;
use crate::vector::Vector2;

#[derive(Debug, Clone)]
pub struct LineQuad {
    a: Vector2,
    b: Vector2,
    width: f32,
    height: f32,
    tscale: Vector2,
    upper: SectorPlane,
    lower: SectorPlane,
    colour_top: Option<[u8; 3]>,
    colour_btm: Option<[u8; 3]>,
    quad_type: QuadType,
}

#[derive(Debug, Clone)]
enum QuadType {
    NormalQuad,
    OppositeTriangles,
    Vavoom3DFloor,
}

impl LineQuad {
    pub fn new(a: usize, b: usize, vertices: &[Vector2], sector: &Sector) -> LineQuad {
        let mut positions: Vec<Vector3> = Vec::new();
        let va = vertices[a];
        let vb = vertices[b];
        let upper = sector.upper.clone();
        let lower = sector.lower.clone();
        // "A/B upper/lower (absolute) height"
        let auh = upper.z_at(va);
        let buh = upper.z_at(vb);
        let alh = lower.z_at(va);
        let blh = lower.z_at(vb);
        let quad_type = {
            if alh > auh && blh > buh {
                QuadType::Vavoom3DFloor
            } else if alh > auh || blh > buh {
                QuadType::OppositeTriangles
            } else {
                QuadType::NormalQuad
            }
        };
        LineQuad {
            a: va,
            b: vb,
            width: (vb - va).length(),
            height: 64.,
            tscale: Vector2::new(1., 1.),
            upper,
            lower,
            colour_top: sector.colour_top,
            colour_btm: sector.colour_btm,
            quad_type
        }
    }
    pub fn colour(&self) -> Vec<Vector3> {
        //
    }
}
