use crate::camera::Camera;
use egui::Context as EContext;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum MatrixChoice {
	#[default]
	View,
	Projection,
	ProjectionView,
}

pub fn matrix_debug_window(camera: &Camera, ectx: &EContext, choice: &mut MatrixChoice) {
	egui::Window::new("camatrix").show(ectx, |ui| {
		let camatrix = match choice {
			MatrixChoice::View => camera.view(),
			MatrixChoice::Projection => camera.projection(),
			MatrixChoice::ProjectionView => camera.projection_view(),
		}.to_cols_array();
		ui.horizontal(|ui| {
			ui.selectable_value(choice, MatrixChoice::View, "View");
			ui.selectable_value(choice, MatrixChoice::Projection, "Projection");
			ui.selectable_value(choice, MatrixChoice::ProjectionView, "Proj×View");
		});
		egui::Grid::new("camatrix2").show(ui, |ui| {
			ui.label(format!("{:12.7}", camatrix[0]));
			ui.label(format!("{:12.7}", camatrix[4]));
			ui.label(format!("{:12.7}", camatrix[8]));
			ui.label(format!("{:12.7}", camatrix[12]));
			ui.end_row();
			ui.label(format!("{:12.7}", camatrix[1]));
			ui.label(format!("{:12.7}", camatrix[5]));
			ui.label(format!("{:12.7}", camatrix[9]));
			ui.label(format!("{:12.7}", camatrix[13]));
			ui.end_row();
			ui.label(format!("{:12.7}", camatrix[2]));
			ui.label(format!("{:12.7}", camatrix[6]));
			ui.label(format!("{:12.7}", camatrix[10]));
			ui.label(format!("{:12.7}", camatrix[14]));
			ui.end_row();
			ui.label(format!("{:12.7}", camatrix[3]));
			ui.label(format!("{:12.7}", camatrix[7]));
			ui.label(format!("{:12.7}", camatrix[11]));
			ui.label(format!("{:12.7}", camatrix[15]));
			ui.end_row();
		});
	});
}
