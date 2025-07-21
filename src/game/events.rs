use bevy::prelude::*;

#[derive(Event, Debug, Clone, Copy)]
pub struct UpdateHoverOutline;

#[derive(Event, Debug, Clone, Copy)]
pub struct UpdateHoverGizmo;
