use crate::custom_widgets::tex_name::TextureName;
use egui::Ui;

#[derive(Debug, Clone, Default)]
pub struct TextureBrowserData {
	pub textures: Vec<TextureInfo>,
	pub selected_index: usize,
	pub tex_name_filter: String,
	pub tex_full_path: bool,
}

#[derive(Debug, Clone)]
pub struct TextureInfo<'a> {
	name: TextureName<'a>,
	short_name: String,
	dimens: String,
	index: usize,
	tex: &'a doomwad::res::Texture<'a>,
	crc: u32,
}

impl<'a> TextureInfo<'a> {
	pub fn popup_info_line(&self, ui: &mut Ui) {
		ui.horizontal(|ui| {
			ui.label(self.name.0.as_ref());
			ui.label("(");
			ui.label(self.dimens);
			ui.label(")");
		});
	}
}
