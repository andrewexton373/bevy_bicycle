use avian2d::prelude::*;
use bevy::{math::vec3, prelude::*};

use crate::bicycle::groupset::components::{Axle, Disc, Point};

use super::plugin::ChainPlugin;

impl ChainPlugin {

    pub fn reset_chain(
        mut commands: Commands,
        axles: Query<(&Axle, Option<&Disc>, &Transform)>,
        keys: Res<ButtonInput<KeyCode>>,

    ) {
        if keys.just_pressed(KeyCode::KeyR) {
            let mut point_set = vec![];

            // Space was pressed
            for (axle, disc, transform) in axles.iter() {

                if let Some(disc) = disc {

                    let larger_disc = Disc {
                        center: disc.center,
                        radius: disc.radius + 2.5
                    };

                    let poly = larger_disc.simplify_disc_as_polygon(20).iter().map(|point| {
                        Point {x: point.x + transform.translation.x as f64, y: point.y + transform.translation.y as f64}
                    }).collect::<Vec<Point>>();
                    point_set.extend(poly);
                }

            }

            let chain_links = ChainPlugin::generate_chain_link_points_from_point_set(&point_set);
            ChainPlugin::setup_chain(&mut commands, chain_links);

        }
        
    }

    pub fn generate_chain_link_points_from_point_set(points: &Vec<Point>) -> Vec<Point> {
        let convex_hull = gift_wrapping(&points);
        let equidistant_points = equidistant_points_on_polygon(&convex_hull, 25);

        equidistant_points
    }

    pub fn setup_chain(mut commands: &mut Commands, links: Vec<Point>) {
        let link_radius = 5.0;
        let r = links[0].distance(&links[1]);
        let compliance: f64 = 0.000000001;

        let mut previous_link = None;

        let mut link_ents = vec![];

        for link in links[0..].iter() {

            let current_link = commands
                        .spawn((
                            RigidBody::Dynamic,
                            Collider::circle(link_radius),
                            SweptCcd::new_with_mode(SweepMode::NonLinear).include_dynamic(true),
                            Friction::new(0.99),
                            LockedAxes::ROTATION_LOCKED, // VERY IMPORTANT SO LINK PIVOTS DONT ROTATE
                            MassPropertiesBundle {
                                mass: Mass::new(0.01),
                                ..default()
                            },
                            Transform {
                                translation: vec3(link.x as f32, link.y as f32, 0.0),
                                ..default()
                            },
                        ))
                        .id();

                link_ents.push(current_link);

            if previous_link.is_some() {
                commands.spawn(
                    DistanceJoint::new(previous_link.unwrap(), current_link)
                        .with_rest_length(r as f64)
                        .with_compliance(compliance),
                );
            }
            previous_link = Some(current_link);
            
        }

        // Complete the Loop
        commands.spawn(
            DistanceJoint::new(*link_ents.first().unwrap(), *link_ents.last().unwrap())
                .with_rest_length(r as f64)
                .with_compliance(compliance),
        );
        
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

fn polygon_perimeter(polygon: &Vec<Point>) -> f64 {

    let mut perimeter = 0.0;

    for i in 0..polygon.len() {
        let current_point = &polygon[i];
        let next_point = &polygon[(i + 1) % polygon.len()];
        perimeter += current_point.distance(next_point);
    }

    perimeter
}

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