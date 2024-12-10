use std::f64::consts::PI;

use bevy::prelude::Component;

#[derive(Component)]
pub struct Groupset;

#[derive(Component, PartialEq, Debug)]
pub enum Axle {
    FRONT,
    REAR,
}

#[derive(Component, PartialEq, Debug)]
pub enum Cog {
    FrontChainring,
    RearCassette,
}

#[derive(Component, Clone, Copy)]
pub struct Radius(pub f32);

// Define a struct for a 2D point
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    // // Function to calculate the Euclidean distance between two points
    pub fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    // Interpolate between two points, given a ratio (0.0 to 1.0).
    pub fn interpolate(&self, other: &Point, t: f64) -> Point {
        Point {
            x: self.x + t * (other.x - self.x),
            y: self.y + t * (other.y - self.y),
        }
    }
}

// Define a struct for a 2D disc (circle)
#[derive(Component, PartialEq, Debug, Clone, Copy)]
pub struct Disc {
    pub center: Point,
    pub radius: f64
}

impl Disc {
    // Function to generate points on a circle's boundary and return those points as a resulting simplified polygon
    pub fn simplify_disc_as_polygon(&self, num_vertices: usize) -> Vec<Point> {
        let mut points = Vec::new();
        for i in 0..num_vertices {
            let angle = 2.0 * PI * (i as f64) / (num_vertices as f64);
            let x = self.center.x + self.radius * angle.cos();
            let y = self.center.y + self.radius * angle.sin();
            points.push(Point { x, y });
        }
        points
    }
}