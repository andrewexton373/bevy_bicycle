use bevy::prelude::*;
use bevy::{ecs::system::SystemState, math::vec3};

use core::f64;

use avian2d::prelude::*;

use super::groupset::{Cog, Disc, Point, Radius};
use super::systems::GameLayer;

#[derive(Component)]
pub struct ChainPivot;

#[derive(Component)]
pub struct Chain;

pub(crate) fn spawn_chain(world: &mut World) {
    let mut system_state: SystemState<(
        Commands,
        Query<Entity, With<Chain>>,
        Query<(&Radius, &Position), With<Cog>>,
    )> = SystemState::new(world);

    let (mut commands, mut chain, cogs) = system_state.get_mut(world);

    if let Ok(chain) = chain.get_single_mut() {
        commands.entity(chain).despawn_recursive();
    }

    let mut point_set = vec![];

    // R(eset) was pressed
    for (radius, transform) in cogs.iter() {
        let larger_disc = Disc {
            center: Point {
                x: transform.x,
                y: transform.y,
            },
            radius: radius.0 as f64 + 1.35,
        };

        let poly = larger_disc
            .simplify_disc_as_polygon(60)
            .iter()
            .map(|point| Point {
                x: point.x,
                y: point.y,
            })
            .collect::<Vec<Point>>();
        point_set.extend(poly);
    }

    // info!("POINT SET: {:?}", point_set);

    let chain_links = generate_chain_link_points_from_point_set(&point_set);
    setup_chain(&mut commands, chain_links);

    system_state.apply(world);
}

pub fn setup_chain(commands: &mut Commands, links: Vec<Point>) {
    let r = links[0].distance(&links[1]);
    let compliance: f64 = 0.0;

    commands
        .spawn((Chain, Transform::default(), GlobalTransform::default()))
        .with_children(|parent| {
            let mut previous_link = None;

            let mut link_ents = vec![];

            for pos in links[0..].iter() {
                let current_link = parent.spawn(generate_link(pos)).id();

                link_ents.push(current_link);

                if let Some(previous_link) = previous_link {
                    parent.spawn(
                        DistanceJoint::new(previous_link, current_link)
                            .with_angular_velocity_damping(0.0)
                            .with_linear_velocity_damping(0.0)
                            .with_rest_length(r)
                            .with_compliance(compliance),
                    );
                }
                previous_link = Some(current_link);
            }

            // Complete the Loop
            parent.spawn(
                DistanceJoint::new(*link_ents.first().unwrap(), *link_ents.last().unwrap())
                    .with_angular_velocity_damping(0.0)
                    .with_linear_velocity_damping(0.0)
                    .with_rest_length(r)
                    .with_compliance(compliance),
            );
        });
}

pub fn generate_chain_link_points_from_point_set(points: &Vec<Point>) -> Vec<Point> {
    let convex_hull = gift_wrapping(points);

    equidistant_points_on_polygon(&convex_hull.clone(), 60)
}

pub fn generate_link(pos: &Point) -> impl Bundle {
    let link_radius: f64 = 0.5;

    (
        RigidBody::Dynamic,
        Collider::circle(link_radius),
        CollisionMargin(0.05),
        Friction::new(1.0).with_combine_rule(CoefficientCombine::Max),
        LockedAxes::ROTATION_LOCKED, // VERY IMPORTANT SO LINK PIVOTS DONT ROTATE
        MassPropertiesBundle {
            mass: Mass(0.01),
            ..default()
        },
        Transform {
            translation: vec3(pos.x as f32, pos.y as f32, 0.0),
            ..default()
        },
        CollisionLayers::new(
            GameLayer::Chain,
            GameLayer::Groupset.to_bits() | GameLayer::Chain.to_bits(),
        ),
    )
}

// Cross product of vectors OA and OB
fn cross(o: &Point, a: &Point, b: &Point) -> f64 {
    (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x)
}

// Function to implement the Gift Wrapping (Jarvis march) algorithm to find the convex hull
fn gift_wrapping(points: &Vec<Point>) -> Vec<Point> {
    let mut hull = Vec::new();

    // Find the leftmost point (with the smallest x value)
    let mut leftmost = points[0];
    for &point in points {
        if point.x < leftmost.x {
            leftmost = point;
        }
    }

    let mut current_point = leftmost;

    loop {
        // Add the current point to the convex hull
        hull.push(current_point);

        // Find the next point that is the most counter-clockwise from the current point
        let mut next_point = points[0];
        for &point in points {
            if point == current_point {
                continue;
            }

            // Cross product to determine the turn direction
            let cross_product = cross(&current_point, &next_point, &point);
            if cross_product > 0.0
                || (cross_product == 0.0
                    && current_point.distance(&point) > current_point.distance(&next_point))
            {
                next_point = point;
            }
        }

        // If we have wrapped around to the leftmost point, we are done
        if next_point == leftmost {
            break;
        }

        current_point = next_point;
    }

    hull
}

// Calculate the perimter of a polygon
fn polygon_perimeter(polygon: &[Point]) -> f64 {
    let mut perimeter = 0.0;

    for i in 0..polygon.len() {
        let current_point = &polygon[i];
        let next_point = &polygon[(i + 1) % polygon.len()];
        perimeter += current_point.distance(next_point);
    }

    perimeter
}

// Calculate num_points on a polygon that are equally spaced apart
fn equidistant_points_on_polygon(polygon: &[Point], num_points: usize) -> Vec<Point> {
    let mut result = Vec::new();
    let perimeter = polygon_perimeter(polygon);
    let distance_between_points = perimeter / num_points as f64;

    let mut remaining_distance = distance_between_points;
    let mut current_point_index = 0;
    let mut last_point = polygon[0];

    while result.len() < num_points {
        let next_point_index = (current_point_index + 1) % polygon.len();
        let next_point = polygon[next_point_index];

        let edge_length = last_point.distance(&next_point);

        // If the remaining distance can be covered by this edge, interpolate.
        if remaining_distance <= edge_length {
            let t = remaining_distance / edge_length;
            let new_point = last_point.interpolate(&next_point, t);
            result.push(new_point);

            // Update remaining distance and last point
            remaining_distance = distance_between_points;
            last_point = new_point;
        } else {
            // Otherwise, move to the next edge
            remaining_distance -= edge_length;
            current_point_index = next_point_index;
            last_point = next_point;
        }
    }

    result
}
