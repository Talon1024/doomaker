use std::borrow::Cow;

pub(crate) struct TextureSquare {
	pub size: f32,
	pub selected: bool,
	pub tex: egui::Image,
}

impl<'a> TextureSquare {
	pub(crate) fn show(&mut self, ui: &mut egui::Ui) -> egui::Response {
		// A bit of extra vertical space for the text
		let font = egui::FontId::default();
		let font_height = font.size;
		let desired_size = egui::vec2(self.size, self.size + font.size);
		let (mut rect, mut response) = ui.allocate_exact_size(
			desired_size, egui::Sense::click());
		
		let hovered = response.hovered();
		if response.clicked() {
			self.selected = true;
			response.mark_changed();
		}

		if ui.is_rect_visible(rect) {
			let visuals = if !self.selected {
				ui.style().interact(&response).clone()
			} else {
				ui.style().interact_selectable(&response, self.selected)
			};
			let new_rect = rect.expand(visuals.expansion);
			if hovered || self.selected {
				ui.painter()
				.rect(new_rect, 7., visuals.bg_fill, visuals.bg_stroke);
			}
			// Texture name at bottom of rectangle
			let text_pos = rect.center_bottom();
			ui.painter().text(text_pos, egui::Align2::CENTER_BOTTOM,
				"Tex", font, ui.visuals().text_color());
			// Zoom resize
			let image_size = self.tex.size();
			let image_size_factor = self.size / image_size.max_elem();
			let image_resize = image_size * image_size_factor;
			*rect.bottom_mut() -= font_height;
			let image_rect = egui::Rect::from_center_size(
				rect.center(), image_resize);
			self.tex.paint_at(ui, image_rect);
			// ui.painter().rect_filled(image_rect, 0., egui::Color32::BLACK);
		}
		response
	}
}

#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
enum ComponentType {
	FolderNameStart,
	FolderName,
	TextureNameFirst8Chars,
	TextureName,
	BeforeSlash,
	OnSlash,
	OnDot,
	BeforeLastDot,
	OnOrAfterLastDot
}

pub struct TextureName<'a>(pub Cow<'a, str>);
impl<'a> std::fmt::Display for TextureName<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut dir_count = self.0.chars().filter(|&ch| ch == '/').count();
		if dir_count == 0 {
			write!(f, "{}", self.0)
		} else {
			let mut dot_count = self.0.chars()
				.filter(|&ch| ch == '.').count();
			let mut ct = ComponentType::FolderNameStart;
			let mut tex_name_chars = 0u16;
			let mut ci = self.0.chars().skip(1);
			let text: String = self.0.chars().filter_map(|ch| {
				use ComponentType::*;
				let next_char = ci.next();
				if next_char == Some('/') {
					ct = BeforeSlash;
				} else if ch == '/' {
					dir_count -= 1;
					ct = OnSlash;
				}
				let rv = match ct {
					FolderNameStart => Some(ch),
					FolderName => None,
					TextureNameFirst8Chars => Some(ch),
					TextureName => None,
					BeforeSlash => Some('.'),
					OnSlash => Some(ch),
					OnDot => None,
					BeforeLastDot => Some('.'),
					OnOrAfterLastDot => Some(ch)
				};
				if ct == OnSlash {
					ct = match dir_count {
						0 => TextureNameFirst8Chars,
						_ => FolderNameStart
					}
				} else if ct == FolderNameStart {
					ct = FolderName;
				} else if ct == TextureNameFirst8Chars {
					if tex_name_chars != 8 {
						tex_name_chars += 1;
					} else {
						ct = TextureName;
					}
				}
				rv
			}).collect();
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
	#[ignore]
	fn texture_name_full_path() {
		let tex_name = "textures/studs/stud3_5.png".to_string();
		let tex_name = TextureName(Cow::from(&tex_name));
		assert_eq!(tex_name.to_string(), "t./s./stud3_5.png".to_string())
	}

	#[test]
	#[ignore]
	fn texture_name_stupidly_long() {
		let tex_name = "textures/studs/this_is_a_stupidly_and_pointlessly_long_texture_name_why_did_you_call_your_texture_this_stupidly_and_pointlessly_long_name_and_just_how_insane_do_you_have_to_be_to_do_something_like_this.png".to_string();
		let tex_name = TextureName(Cow::from(&tex_name));
		assert_eq!(tex_name.to_string(), "t./s./this_is_..png".to_string())
	}
}
