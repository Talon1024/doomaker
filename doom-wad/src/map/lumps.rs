use crate::wad::LumpName;

pub(crate) const REQUIRED_LUMPS: [LumpName; 5] = [
	LumpName(*b"THINGS\0\0"), LumpName(*b"LINEDEFS"), LumpName(*b"SIDEDEFS"),
	LumpName(*b"VERTEXES"), LumpName(*b"SECTORS\0")];

pub(crate) const BASE_LUMPS: [LumpName; 10] = [
	LumpName(*b"THINGS\0\0"), LumpName(*b"LINEDEFS"), LumpName(*b"SIDEDEFS"),
	LumpName(*b"VERTEXES"), LumpName(*b"SEGS\0\0\0\0"), LumpName(*b"SSECTORS"),
	LumpName(*b"NODES\0\0\0"), LumpName(*b"SECTORS\0"),
	LumpName(*b"REJECT\0\0"), LumpName(*b"BLOCKMAP"),
];
pub(crate) const HEXEN_LUMPS: LumpName = LumpName(*b"BEHAVIOR");
pub(crate) const PSX_LUMPS: [LumpName; 2] = [
	LumpName(*b"LEAFS\0\0\0"), LumpName(*b"LIGHTS\0\0")];
pub(crate) const D64_LUMPS: [LumpName; 3] = [
	LumpName(*b"LEAFS\0\0\0"), LumpName(*b"LIGHTS\0\0"),
	LumpName(*b"MACROS\0\0")];

pub(crate) const ALL_LUMPS: [LumpName; 16] = [
	LumpName(*b"THINGS\0\0"), LumpName(*b"LINEDEFS"), LumpName(*b"SIDEDEFS"),
	LumpName(*b"VERTEXES"), LumpName(*b"SECTORS\0"),
	LumpName(*b"SEGS\0\0\0\0"), LumpName(*b"SSECTORS"),
	LumpName(*b"NODES\0\0\0"), LumpName(*b"REJECT\0\0"),
	LumpName(*b"BLOCKMAP"), LumpName(*b"LEAFS\0\0\0"),
	LumpName(*b"LIGHTS\0\0"), LumpName(*b"MACROS\0\0"),
	LumpName(*b"BEHAVIOR"), LumpName(*b"SCRIPTS\0"), LumpName(*b"DIALOGUE")];
