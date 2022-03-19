use crate::secplane::SectorPlane;
use crate::edge::EdgeVertexIndex;

pub struct LineQuad<'a> {
    a: EdgeVertexIndex,
    b: EdgeVertexIndex,
    front_floor: &'a SectorPlane,
    front_ceil: &'a SectorPlane,
    back_floor: Option<&'a SectorPlane>,
    back_ceil: Option<&'a SectorPlane>,
    colour_top: Option<[u8; 3]>,
    colour_btm: Option<[u8; 3]>,
}
