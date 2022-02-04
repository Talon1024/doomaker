// Misc. utilities
pub fn copy_into_array<T: Copy, const N: usize>
(array: &mut [T; N], slice: &[T], pos: usize) -> usize
{
	debug_assert_eq!(N, slice.len());
	let size = array.len();
	array.copy_from_slice(&slice[pos..pos+size]);
	pos + size
}
