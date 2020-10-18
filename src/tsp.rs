//mod distance;
use crate::distance::distance;

fn same(a: &(f64, f64), b: &(f64, f64)) -> bool {
   (a.0.to_bits() == b.0.to_bits()) && (a.1.to_bits() == b.1.to_bits()) 
}

pub fn solve_tsp(points: Vec<(f64, f64)>) -> Vec<(f64, f64)> {

    let mut path = Vec::new();

    let mut points_copy = points.clone();

    let mut current = points[0];

    points_copy.remove(0);

    path.push(current);

    let mut min = (0usize, f64::MAX);
    while path.len() < points.len() {

        //println!("### LOOPING AGAIN {} ###", path.len());
        for (i, coords) in points_copy.iter().enumerate() {
      
            //println!("\t##LLAALA {}", i);
            //println!("coords ({}, {}); current ({}, {});", coords.0, coords.1, current.0, current.1);
            /*if same(coords, &current.1) { 
                println!("\t\tJump!");
                continue; 
            }*/

            min = match (i, distance(coords, &current)) {
                (i, d) if min.1 > d => { /*println!("\t\ti {} d {}", i, d);*/ (i, d)},
                _ => { /*println!("\t\tNo new min.");*/ min}
            };
        }
 
        current = points_copy[min.0];
        points_copy.remove(min.0);
        path.push(current);
        min = (0usize, f64::MAX);
    } 

    path
}