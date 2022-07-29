// Inputs are represented as actions
use glutin::event::VirtualKeyCode as VKC;
use std::collections::HashMap;
use ahash::RandomState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionType {
	/// An action that is performed from when the button is pressed until the
	/// button is released. Examples: Move and turn in 3D view, pan in 2D view
	Hold,
	/// An action that is performed immediately upon pressing the button.
	/// Examples: Add thing, select thing
	Immediate,
	/// An action that is performed from when the button is pressed until the
	/// button (or another) is pressed again. Examples: Begin drawing line,
	/// begin tilting plane
	Continuous { other: Option<Action> }
}

impl Action {
	pub fn action_type(&self) -> ActionType {
		use Action::*;
		use ActionType::*;
		match self {
			_ => Hold
		}
	}
}

pub type Configuration = HashMap<VKC, Action, RandomState>;
