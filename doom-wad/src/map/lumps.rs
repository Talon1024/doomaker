use crate::wad::LumpName;

// These four lumps are always the first four lumps of a Doom-format map
pub(crate) const DOOM_START: [LumpName; 4] = [
    LumpName(*b"THINGS\0\0"), LumpName(*b"LINEDEFS"), LumpName(*b"SIDEDEFS"),
    LumpName(*b"VERTEXES")];
// And then some BSP lumps, and then the SECTORS lump
pub(crate) const DOOM_SECTORS: LumpName = LumpName(*b"SECTORS\0");
// And then the REJECT and BLOCKMAP

pub(crate) const DOOM_VANILLA: [LumpName; 10] = [
    LumpName(*b"THINGS\0\0"), LumpName(*b"LINEDEFS"), LumpName(*b"SIDEDEFS"),
    LumpName(*b"VERTEXES"), LumpName(*b"SEGS\0\0\0\0"), LumpName(*b"SSECTORS"),
    LumpName(*b"NODES\0\0\0"), LumpName(*b"SECTORS\0"),
    LumpName(*b"REJECT\0\0"), LumpName(*b"BLOCKMAP"),
];
pub(crate) const HEXEN_END: LumpName = LumpName(*b"BEHAVIOR");
pub(crate) const HEXEN_END_OPTIONAL: [LumpName; 2] = [
    LumpName(*b"BEHAVIOR"), LumpName(*b"SCRIPTS\0")];
pub(crate) const PSX_END: [LumpName; 2] = [
    LumpName(*b"LEAFS\0\0\0"), LumpName(*b"LIGHTS\0\0")];
pub(crate) const D64_END: [LumpName; 3] = [
    LumpName(*b"LEAFS\0\0\0"), LumpName(*b"LIGHTS\0\0"),
    LumpName(*b"MACROS\0\0")];

pub(crate) const ALL_MAP_LUMPS: [LumpName; 16] = [
    LumpName(*b"THINGS\0\0"), LumpName(*b"LINEDEFS"), LumpName(*b"SIDEDEFS"),
    LumpName(*b"VERTEXES"), LumpName(*b"SECTORS\0"),
    LumpName(*b"SEGS\0\0\0\0"), LumpName(*b"SSECTORS"),
    LumpName(*b"NODES\0\0\0"), LumpName(*b"REJECT\0\0"),
    LumpName(*b"BLOCKMAP"), LumpName(*b"LEAFS\0\0\0"),
    LumpName(*b"LIGHTS\0\0"), LumpName(*b"MACROS\0\0"),
    LumpName(*b"BEHAVIOR"), LumpName(*b"SCRIPTS\0"), LumpName(*b"DIALOGUE")];
// Add 1 for map header lump
pub(crate) const MAX_LUMP_COUNT: usize = 1 + DOOM_VANILLA.len() + D64_END.len();
// Add 2 for SECTORS lump and map header lump
pub(crate) const MIN_LUMP_COUNT: usize = DOOM_START.len() + 2;
/* 
pub(crate) const UDMF_START: LumpName = LumpName(*b"TEXTMAP\0");
pub(crate) const UDMF_END: LumpName = LumpName(*b"ENDMAP\0\0");
 */
