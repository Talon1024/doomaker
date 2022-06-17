use std::borrow::Cow;
use super::tex_name::TextureName;

pub(crate) struct TextureSquare<'a> {
	pub size: f32,
	tex: egui::Image,
	selected: bool,
	tex_name: TextureName<'a>,
	short_name: String,
	popup_id: egui::Id,
}
impl<'a> TextureSquare<'a> {
	const POPUP_MAX_TEX_SIZE: f32 = 256.;
}

impl<'a> TextureSquare<'a> {
	pub(crate) fn new(size: Option<f32>, tex: egui::Image, name: impl Into<Cow<'a, str>>, selected: bool) -> Self {
		let tex_name = TextureName(name.into());
		let short_name = tex_name.short_name();
		let popup_id = egui::Id::new(tex_name.0.as_ref());
		TextureSquare {
			size: size.unwrap_or(48.),
			selected,
			tex,
			tex_name,
			short_name,
			popup_id,
		}
	}
}
impl<'a> egui::Widget for TextureSquare<'a> {
	fn ui(self, ui: &mut egui::Ui) -> egui::Response {
		// A bit of extra vertical space for the text
		let font = egui::FontId::default();
		let font_height = font.size;
		let desired_size = egui::vec2(self.size, self.size + font.size);
		let (mut rect, mut response) = ui.allocate_exact_size(
			desired_size, egui::Sense::click());

		let hovered = response.hovered();
		if response.clicked() {
			response.mark_changed();
		}

		if hovered {
			egui::show_tooltip_at_pointer(ui.ctx(),
			self.popup_id, |ui| {
				let image_size = self.tex.size();
				let image_size_factor = 1.0f32.min(
					Self::POPUP_MAX_TEX_SIZE / image_size.max_elem());
				let image_resize = image_size * image_size_factor;
				ui.label(self.tex_name.0.as_ref());
				let (rect, _) = ui.allocate_exact_size(image_resize,
					egui::Sense {
						click: false, drag: false, focusable: false });
				self.tex.paint_at(ui, rect);
			});
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
				&self.short_name, font, ui.visuals().text_color());
			// Zoom resize
			let image_size = self.tex.size();
			let image_size_factor = (self.size / image_size.max_elem()).min(1.);
			let image_resize = image_size * image_size_factor;
			// let shrink_rect_by = egui::Vec2::splat((self.size -
			//	image_size_factor.min(1.) * image_size.max_elem()) / 2.);
			*rect.bottom_mut() -= font_height;
			let image_rect = egui::Rect::from_center_size(
				rect.center(), image_resize);
			self.tex.paint_at(ui, image_rect);
		}
		response
	}
}
