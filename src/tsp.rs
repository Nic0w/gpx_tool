use crate::distance::distance;

pub fn solve_tsp(start_index: usize, points: &Vec<(f64, f64)>) -> Vec<(f64, f64)> {

    let mut path = Vec::new();

    let mut points_copy = points.clone();

    let mut current = points[start_index];

    points_copy.remove(start_index);

    path.push(current);

    let mut min = (0usize, f64::MAX);
    while path.len() < points.len() {

        for (i, coords) in points_copy.iter().enumerate() {

            min = match (i, distance(coords, &current)) {
                (i, d) if min.1 > d => (i, d),
                _ => min
            };
        }

        current = points_copy[min.0];
        points_copy.remove(min.0);
        path.push(current);
        min = (0usize, f64::MAX);
    }

    path
}
