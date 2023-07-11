use bevy::prelude::*;

pub(crate) const TITLE: &str = "Triangles";
pub(crate) const WIDTH: f32 = 1920.;
pub(crate) const HEIGHT: f32 = 1080.;
pub(crate) const BG_COLOR: Color = Color::rgb(0., 0., 0.);
pub(crate) const TIME_STEP: f32 = 1. / 60.;

pub(crate) const POLYGON_SIDES: usize = 3;
pub(crate) const TRIANGLE_SCALE: Vec3 = Vec3::ZERO;
pub(crate) const TRIANGLE_RADIUS: f32 = (WIDTH + HEIGHT) / 250.;
pub(crate) const TRIANGLE_STROKE: Color = Color::NONE;
// pub(crate) const TRIANGLE_STROKE: Color = Color::rgba(1., 1., 1., 0.005);
pub(crate) const TRIANGLE_FILL: Color = Color::NONE;
pub(crate) const GAP_X: f32 = 1.1;
pub(crate) const GAP_Y: f32 = 1.1;
