pub type BlendFunction = dyn Fn(&mut [u8], &[u8], u8) -> ();

pub fn mix(a: &mut [u8], b: &[u8], alpha: u8) -> () {
	let b_alpha = alpha as f32 / 255.;
	let a_alpha = 1. - b_alpha;
}
