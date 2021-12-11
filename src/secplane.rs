enum SectorPlane {
	Flat(f32),
	// First number is the height, the other four make up the normal vector
	// and distance from the "origin".
	Sloped(f32, f32, f32, f32, f32)
}
