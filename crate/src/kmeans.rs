use rand::distributions::{Distribution, Normal};
use rand::prelude::*;
use rand::seq::SliceRandom;

use point::Point;
use std::cmp::Ordering::Equal;

// fn dist(from: Point, to: Point) -> f32 {
//     ((from.x - to.x).powi(2) + (from.y - to.y).powi(2)).sqrt()
// }

fn fast_dist(from: Point, to: Point) -> f32 {
    (from.x - to.x).powi(2) + (from.y - to.y).powi(2)
}

fn compute_center(points: &[Point]) -> Point {
    points.iter().fold(Point::ORIGIN, |acc, pt| pt + &acc) / points.len() as f32
}

fn dist_to_cluster(point: Point, clusters: &[Point]) -> Vec<f32> {
    clusters
        .iter()
        .map(|cluster| fast_dist(point, *cluster))
        .collect()
}

fn argmin<T: PartialOrd>(arr: &[T]) -> usize {
    arr.iter()
        .enumerate()
        .min_by(|&(_, a), &(_, b)| a.partial_cmp(b).unwrap_or(Equal))
        .unwrap()
        .0
}

pub fn gen_clusters(n_clusters: i32, points_per_cluster: i64) -> Vec<Point> {
    let mut rng = rand::thread_rng();
    let distro = Normal::new(0.0, 20.0);
    let mut points: Vec<Point> = Vec::with_capacity(n_clusters as usize);
    for _ in 0..n_clusters {
        let offset_x: f32 = rng.gen::<f32>() * 1000.0;
        let offset_y: f32 = rng.gen::<f32>() * 1000.0;
        points.extend::<Vec<Point>>(
            (0..points_per_cluster)
                .map(|_| Point {
                    x: distro.sample(&mut rng) as f32 + offset_x,
                    y: distro.sample(&mut rng) as f32 + offset_y,
                })
                .collect(),
        )
    }
    points
}

pub fn kmeans(points: Vec<Point>, k: i32) -> (Vec<Point>, Vec<Vec<Point>>) {
    let mut clusters: Vec<Point> = points
        .choose_multiple(&mut thread_rng(), k as usize)
        .cloned()
        .collect();
    let mut old_clusters: Vec<Point> = Vec::new();
    let mut assignments: Vec<Vec<Point>> = Vec::new();
    while old_clusters != clusters {
        assignments = vec![Vec::new(); k as usize];
        for point in points.iter() {
            let near = argmin(&dist_to_cluster(*point, &clusters));

            assignments[near].push(*point);
        }
        old_clusters = clusters.clone();
        clusters = assignments
            .iter()
            .map(|group| compute_center(group.as_slice()))
            .collect()
    }
    (clusters, assignments)
}
