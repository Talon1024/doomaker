#![allow(non_snake_case)]
//! # Map 3D-ifier
//! 
//! `map_to_3D` has utilities to help turn "maps" into 3D models, which is
//! useful for 3D WYSIWYG viewers/editors. A "map" consists of lines,
//! vertices, sectors, and objects.

// pub mod vector; // Now using glam
pub mod plane;
pub(crate) mod boundingbox;
pub mod sectorpolygonbuilder;
pub mod edge;
// pub mod wall;
// pub mod sector;
pub(crate) mod util;
