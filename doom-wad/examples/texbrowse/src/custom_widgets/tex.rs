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
