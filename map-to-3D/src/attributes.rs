
pub trait VertexAttributes {
	fn position(&self);
	fn index(&self);
	fn colour(&self);
	fn fogdensity(&self);
	fn fogcolour(&self);
	fn uv(&self);
}
