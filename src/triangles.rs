use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use std::f32::consts::PI;

use crate::cfg::*;

#[derive(Debug, Component)]
pub struct Shape;

#[derive(Component, Debug, PartialEq, Clone, Default, Copy)]
pub struct Size(u32, u32);

#[derive(Component, Debug, PartialEq, Clone, Default, Copy)]
pub struct Position(f32, f32);

#[derive(Component, Debug, PartialEq, Clone, Default, Copy)]
pub struct Gap(f32, f32);

#[derive(Component, Debug, PartialEq, Clone, Copy)]
pub struct Index(usize);

pub struct TrianglesPlugin;

impl Plugin for TrianglesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_systems((Self::generate,).chain())
            .add_system(Self::animation);
    }
}

impl TrianglesPlugin {
    pub fn generate(mut cmd: Commands, window: Query<&Window>) {
        let window = window.single();
        let width = window.resolution.physical_width();
        let height = window.resolution.physical_height();

        let shape = shapes::RegularPolygon {
            sides: POLYGON_SIDES,
            center: Vec2::ZERO,
            feature: shapes::RegularPolygonFeature::Radius(TRIANGLE_RADIUS),
        };

        Triangles::default()
            .set_width(window.resolution.physical_width())
            .set_height(window.resolution.physical_height())
            .set_triangle_radius(TRIANGLE_RADIUS)
            .set_gap_x(GAP_X)
            .set_gap_y(GAP_Y)
            .build()
            .iter()
            .for_each(
                |Triangle {
                     x,
                     y,
                     z,
                     angle,
                     stroke,
                     fill,
                     radius,
                 }| {
                    // Check if the triangle fits in the window
                    if *x + radius > -(width as f32 / 2.)
                        && *x - radius < (width as f32 / 2.)
                        && *y + radius > -(height as f32 / 2.)
                        && *y - radius < (height as f32 / 2.)
                    {
                        cmd.spawn((
                            ShapeBundle {
                                path: GeometryBuilder::build_as(&shape),
                                transform: Transform {
                                    translation: Vec3::new(*x, *y, *z),
                                    rotation: Quat::from_rotation_z(*angle),
                                    scale: TRIANGLE_SCALE,
                                    ..default()
                                },
                                ..default()
                            },
                            Stroke::color(*stroke),
                            Fill::color(*fill),
                            Shape,
                        ));
                    }
                },
            );
    }

    pub fn animation(
        mut cmd: Commands,
        mut triangles: Query<(&mut Fill, &mut Transform), With<Shape>>,
        index: Query<(&Index, Entity)>,
        time: Res<Time>,
    ) {
        if let Ok((Index(idx), entity)) = index.get_single() {
            let delta = time.delta_seconds();
            let triangles_len = triangles.iter().len();

            for (i, (mut stroke, mut transform)) in triangles.iter_mut().enumerate() {
                // stroke.color = TRIANGLE_STROKE;

                if transform.scale.x <= 1. && transform.scale.y <= 1. {
                    transform.scale.x += delta;
                    transform.scale.y += delta;
                }

                if let Color::Hsla {
                    mut hue,
                    mut saturation,
                    mut lightness,
                    mut alpha,
                } = stroke.color.as_hsla()
                {
                    hue += delta * 20.;
                    saturation = 1.;
                    lightness = 0.5;
                    alpha = 1.;
                    stroke.color = Color::Hsla {
                        hue,
                        saturation,
                        lightness,
                        alpha,
                    }
                    .as_rgba();
                }

                if i == *idx {
                    let idx = if i + 1 >= triangles_len { 0 } else { i + 1 };

                    cmd.entity(entity).remove::<Index>();
                    cmd.spawn(Index(idx));

                    break;
                }
            }
        } else {
            cmd.spawn(Index(0));
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Triangles {
    size: Size,
    triangle: Triangle,
    gap: Gap,
}

// Triangles builder
impl Triangles {
    pub fn set_width(&mut self, width: u32) -> &mut Self {
        self.size.0 = width;
        self
    }

    pub fn set_height(&mut self, height: u32) -> &mut Self {
        self.size.1 = height;
        self
    }

    pub fn set_triangle_radius(&mut self, radius: f32) -> &mut Self {
        self.triangle.set_radius(radius);
        self
    }

    pub fn set_gap_x(&mut self, x: f32) -> &mut Self {
        self.gap.0 = x;
        self
    }

    pub fn set_gap_y(&mut self, y: f32) -> &mut Self {
        self.gap.1 = y;
        self
    }
}

impl Triangles {
    pub fn build(&mut self) -> Vec<Triangle> {
        let side = self.triangle.side();
        let radius = self.triangle.radius();
        let mut triangles = vec![self.triangle];

        // Deviation
        let (mut a_x, mut a_y) = (side / 2. * self.gap.0, radius / 2. * self.gap.1);
        let (mut b_x, mut b_y) = (-side / 2. * self.gap.0, radius / 2. * self.gap.1);
        let (c_x, mut c_y) = (0., -radius * self.gap.1);

        while a_x <= self.size.0 as f32
            && a_y <= self.size.1 as f32
            && b_x <= 0.
            && b_y <= self.size.1 as f32
            && c_y <= self.size.1 as f32
        {
            let mut a = self.shift(&triangles, a_x, a_y);
            let mut b = self.shift(&triangles, b_x, b_y);
            let mut c = self.shift(&triangles, c_x, c_y);

            a_x *= 2.;
            b_x *= 2.;

            a_y *= -2.;
            b_y *= -2.;
            c_y *= -2.;

            triangles.append(&mut a);
            triangles.append(&mut b);
            triangles.append(&mut c);
        }

        triangles
    }

    pub fn shift(
        &self,
        triangles: &Vec<Triangle>,
        deviation_x: f32,
        deviation_y: f32,
    ) -> Vec<Triangle> {
        (*triangles
            .iter()
            .map(|triangle| {
                let mut triangle = triangle.clone();

                triangle.angle = if triangle.angle == 0. { PI } else { 0. };

                triangle.y *= -1_f32;

                triangle.x += deviation_x;
                triangle.y += deviation_y;

                triangle
            })
            .collect::<Vec<Triangle>>())
        .to_vec()
    }
}

#[derive(Component, Debug, PartialEq, Clone, Copy)]
pub struct Triangle {
    x: f32,
    y: f32,
    z: f32,
    angle: f32,
    radius: f32,
    stroke: Color,
    fill: Color,
}

impl Default for Triangle {
    fn default() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
            radius: TRIANGLE_RADIUS,
            angle: 0.,
            stroke: TRIANGLE_STROKE,
            fill: TRIANGLE_FILL,
        }
    }
}

impl Triangle {
    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn side(&self) -> f32 {
        self.radius * 3_f32.sqrt()
    }

    pub fn set_x(&mut self, x: f32) -> &mut Self {
        self.x = x;
        self
    }

    pub fn set_y(&mut self, y: f32) -> &mut Self {
        self.y = y;
        self
    }

    pub fn set_z(&mut self, z: f32) -> &mut Self {
        self.z = z;
        self
    }

    pub fn set_radius(&mut self, radius: f32) -> &mut Self {
        self.radius = radius;
        self
    }

    pub fn set_angle(&mut self, angle: f32) -> &mut Self {
        self.angle = angle;
        self
    }

    pub fn set_stroke(&mut self, stroke: Color) -> &mut Self {
        self.stroke = stroke;
        self
    }

    pub fn set_fill(&mut self, fill: Color) -> &mut Self {
        self.fill = fill;
        self
    }
}
