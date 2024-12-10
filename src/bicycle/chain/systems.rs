use core::f64;

use avian2d::prelude::*;
use bevy::{math::vec3, prelude::*};

use crate::bicycle::{groupset::components::{Axle, Cog, Disc, Point, Radius}, systems::GameLayer};

use super::{components::Chain, plugin::ChainPlugin};

impl ChainPlugin {

    pub fn reset_chain(
        mut commands: Commands,
        // axles: Query<(&Axle, Option<&Disc>, &Transform)>,
        mut chain: Query<(Entity, &Chain)>,
        cogs: Query<(&Cog, &Radius, &Position)>,

        keys: Res<ButtonInput<KeyCode>>,

    ) {
        if keys.just_pressed(KeyCode::KeyR) {

            if let Some(chain) = chain.get_single_mut().ok() {
                commands.entity(chain.0).despawn_recursive();
            }

            let mut point_set = vec![];

            // R(eset) was pressed
            for (cog, radius, transform) in cogs.iter() {

                println!("{:?}", cog);

                let larger_disc = Disc {
                    center: Point {x: transform.x as f64, y: transform.y as f64},
                    radius: radius.0 as f64 + 0.8
                };

                let poly = larger_disc.simplify_disc_as_polygon(40).iter().map(|point| {
                    Point {x: point.x as f64, y: point.y as f64}
                }).collect::<Vec<Point>>();
                point_set.extend(poly);

            }

            let chain_links = ChainPlugin::generate_chain_link_points_from_point_set(&point_set);
            ChainPlugin::setup_chain(&mut commands, chain_links);

        }
        
    }

    pub fn generate_chain_link_points_from_point_set(points: &Vec<Point>) -> Vec<Point> {
        let convex_hull = gift_wrapping(&points);
        let equidistant_points = equidistant_points_on_polygon(&convex_hull, 50);

        equidistant_points
    }

    pub fn setup_chain(commands: &mut Commands, links: Vec<Point>) {
        let link_radius = 0.5;
        let r = links[0].distance(&links[1]);
        let compliance: f64 = 0.0;

        commands.spawn((Chain, GlobalTransform::default())).with_children(|parent| {
            let mut previous_link = None;

            let mut link_ents = vec![];
    
            for link in links[0..].iter() {
    
                let current_link = parent
                            .spawn((
                                RigidBody::Dynamic,
                                Collider::circle(link_radius),
                                SweptCcd::default(),
                                Friction::new(1.0),
                                LockedAxes::ROTATION_LOCKED, // VERY IMPORTANT SO LINK PIVOTS DONT ROTATE
                                MassPropertiesBundle {
                                    mass: Mass::new(0.01),
                                    ..default()
                                },
                                Transform {
                                    translation: vec3(link.x as f32, link.y as f32, 0.0),
                                    ..default()
                                },
                                CollisionLayers::new(GameLayer::Groupset, GameLayer::Groupset)
                            ))
                            .id();
    
                    link_ents.push(current_link);
    
                if previous_link.is_some() {
                    parent.spawn(
                        DistanceJoint::new(previous_link.unwrap(), current_link)
                            .with_rest_length(r as f64)
                    );
                }
                previous_link = Some(current_link);
                
            }
    
            // Complete the Loop
            parent.spawn(
                DistanceJoint::new(*link_ents.first().unwrap(), *link_ents.last().unwrap())
                    .with_rest_length(r as f64)
                    .with_compliance(compliance),
            );
        });

    }
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
            if cross_product > 0.0 || (cross_product == 0.0 && current_point.distance(&point) > current_point.distance(&next_point)) {
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
fn polygon_perimeter(polygon: &Vec<Point>) -> f64 {

    let mut perimeter = 0.0;

    for i in 0..polygon.len() {
        let current_point = &polygon[i];
        let next_point = &polygon[(i + 1) % polygon.len()];
        perimeter += current_point.distance(next_point);
    }

    perimeter
}

// Calculate num_points on a polygon that are equally spaced apart
fn equidistant_points_on_polygon(polygon: &Vec<Point>, num_points: usize) -> Vec<Point> {
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