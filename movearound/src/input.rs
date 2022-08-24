#![allow(dead_code)] // Sorry
// Inputs are represented as actions
use glutin::{
	event::VirtualKeyCode as VKC,
	ContextCurrentState,
	WindowedContext as ConWin
};
use std::collections::HashMap;
use ahash::RandomState;
use glam::Vec3;
use serde::{Serialize, Deserialize};

use crate::App;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Mode {
	/// 3D view, allows selecting and editing independent of view direction
	#[default]
	View3D,
	/// 3D view, allows moving camera around
	Look3D,
	/// 2D view
	View2D,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActionId {
	MoveForward,
	MoveBackward,
	MoveLeft,
	MoveRight,
	MoveUp,
	MoveDown,
	TurnUp,
	TurnDown,
	TurnLeft,
	TurnRight,
	LockPointer,
	ReleasePointer,
	ChangeMode/*(Mode)*/,
}

impl ActionId {
	// Action type is closely coupled to Action ID
	pub fn get_type(&self) -> ActionType {
		use ActionId::*;
		use ActionType::*;
		match self {
			MoveForward => Hold,
			MoveBackward => Hold,
			MoveLeft => Hold,
			MoveRight => Hold,
			MoveUp => Hold,
			MoveDown => Hold,
			TurnUp => Hold,
			TurnDown => Hold,
			TurnLeft => Hold,
			TurnRight => Hold,
			_ => Immediate,
		}
	}
	pub(crate) fn perform<T>(&self, ctx: &ConWin<T>, app: &mut App)
	where T: ContextCurrentState {
		use ActionId::*;
		let camera = &mut app.camera;
		match app.mode {
			Mode::Look3D => {
				match self {
					MoveForward => {
						let quat = camera.vrot_quat();
						let direction = quat
						.mul_vec3(Vec3::new(0., 1., 0.));
						camera.pos += direction;
					},
					MoveBackward => {
						let quat = camera.vrot_quat();
						let direction = quat
						.mul_vec3(Vec3::new(0., -1., 0.));
						camera.pos += direction;
					},
					MoveLeft => {
						let quat = camera.vrot_quat();
						let direction = quat
						.mul_vec3(Vec3::new(1., 0., 0.));
						camera.pos += direction;
					},
					MoveRight => {
						let quat = camera.vrot_quat();
						let direction = quat
						.mul_vec3(Vec3::new(-1., 0., 0.));
						camera.pos += direction;
					},
					MoveUp => {
						let quat = camera.vrot_quat();
						let direction = quat
						.mul_vec3(Vec3::new(0., 0., 1.));
						camera.pos += direction;
					},
					MoveDown => {
						let quat = camera.vrot_quat();
						let direction = quat
						.mul_vec3(Vec3::new(0., 0., -1.));
						camera.pos += direction;
					},
					ReleasePointer => {
						ctx.window().set_cursor_visible(true);
						app.mode = Mode::View3D;
					}
					_ => ()
				}
			},
			Mode::View3D => {
				match self {
					LockPointer => {
						ctx.window().set_cursor_visible(false);
						app.mode = Mode::Look3D;
					}
					_ => ()
				}
			}
			_ => ()
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActionType {
	/// An action that is performed from when the button is pressed until the
	/// button is released. Examples: Move and turn in 3D view, pan in 2D view
	Hold,
	/// An action that is performed immediately upon pressing the button.
	/// Examples: Add thing, select thing
	Immediate,
	/* 
	/// An action that is performed from when the button is pressed until the
	/// button (or another) is pressed again. Examples: Begin drawing line,
	/// begin tilting plane
	Toggle { other: Option<ActionId> }
	 */
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActionState {
	Active,
	Inactive
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Action {
	pub id: ActionId,
	pub state: ActionState,
}

impl Action {
	pub(crate) fn handle<T>(&self, ctx: &ConWin<T>, app: &mut App)
	where T: ContextCurrentState {
		let action_type = self.id.get_type();
		match action_type {
			ActionType::Hold => {
				match self.state {
				ActionState::Active => {app.input_state.insert(self.id);},
				ActionState::Inactive => {app.input_state.remove(&self.id);},
				}
			},
			ActionType::Immediate => {
				self.id.perform(ctx, app);
			},
		}
	}
}

pub type KeyboardBindings = HashMap<VKC, ActionId, RandomState>;
