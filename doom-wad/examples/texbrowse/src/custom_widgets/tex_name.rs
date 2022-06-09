use std::borrow::Cow;
use regex::{Regex, Captures as Caps};
use lazy_static::lazy_static;

const MIN_BASE_CHARS: usize = 7;
const MAX_BASE_CHARS: usize = 10;

pub struct TextureName<'a>(pub Cow<'a, str>);
impl<'a> std::fmt::Display for TextureName<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		lazy_static! {
		static ref TEXNAME_NAME_R: Regex = Regex::new(r"/([^/]+)$").unwrap();
		static ref TEXNAME_FOLDER_R: Regex = Regex::new(r"([^/]+)/").unwrap();
		}
		let full_path = self.0.contains('/');
		if !full_path {
			if self.0.len() > 8 {
				write!(f, "{}..", &self.0[0..8])
			} else {
				write!(f, "{}", self.0)
			}
		} else {
			let text = TEXNAME_FOLDER_R.replace_all(&self.0, |caps: &Caps| {
				let folder_first = (&caps[1]).chars().next().unwrap();
				let mut dst = String::new();
				dst.push(folder_first);
				dst.push_str("./");
				dst
			});
			let text = TEXNAME_NAME_R.replace(&text, |caps: &Caps| {
				let cap = &caps[1];
				let mut extn = String::new();
				let basename_len =
				if let Some((base, ext)) = cap.rsplit_once('.') {
					extn = ext.to_string();
					base.len()
				} else {
					cap.len()
				};
				if basename_len > MAX_BASE_CHARS {
					format!("/{}..{}", &cap[0..MIN_BASE_CHARS], extn)
				} else {
					format!("/{}", cap)
				}
			});
			write!(f, "{}", text)
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn texture_name_short() {
		let tex_name = "STUD3_5".to_string();
		let tex_name = TextureName(Cow::from(&tex_name));
		assert_eq!(tex_name.to_string(), "STUD3_5".to_string())
	}

	#[test]
	fn texture_name_long() {
		let tex_name = "LONGABILLABONGA".to_string();
		let tex_name = TextureName(Cow::from(&tex_name));
		assert_eq!(tex_name.to_string(), "LONGABIL..".to_string())
	}

	#[test]
	fn texture_name_full_path() {
		let tex_name = "textures/studs/stud3_5.png".to_string();
		let tex_name = TextureName(Cow::from(&tex_name));
		assert_eq!(tex_name.to_string(), "t./s./stud3_5.png".to_string())
	}

	#[test]
	fn texture_name_stupidly_long() {
		let tex_name = "textures/studs/this_is_a_stupidly_and_pointlessly_long_texture_name_why_did_you_call_your_texture_this_stupidly_and_pointlessly_long_name_and_just_how_insane_do_you_have_to_be_to_do_something_like_this.png".to_string();
		let tex_name = TextureName(Cow::from(&tex_name));
		assert_eq!(tex_name.to_string(), "t./s./this_is..png".to_string())
	}
}
