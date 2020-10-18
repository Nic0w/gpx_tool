
pub fn lookup_coordinates() {

}

#[derive(Debug)]
pub enum CardinalPoint {

    North, South, East, West
}

/*
 * Look for the easternest, nothernest, southernest, westernest location.
 */
pub fn directional_lookup(dir: CardinalPoint, points: &Vec<(f64, f64)>) -> usize {

    let mut min = (0usize, f64::MAX);
    let mut max = (0usize, f64::MIN);

    for (i, point) in points.iter().enumerate() {

        //println!("max {}, p {}, {:?}", max.1, point.1, dir);
        match dir {
            CardinalPoint::North   => max = if point.0 > max.1 { (i, point.1) } else { max },
            CardinalPoint::South   => min = if point.0 < min.1 { (i, point.0) } else { min },
            CardinalPoint::East    => max = if point.1 > max.1 { (i, point.1) } else { max },
            CardinalPoint::West    => min = if point.0 < min.1 { (i, point.0) } else { min }
        }
    }

    match dir {
        CardinalPoint::North => max.0,
        CardinalPoint::South => min.0,
        CardinalPoint::East  => max.0,
        CardinalPoint::West  => min.0
    }
}
