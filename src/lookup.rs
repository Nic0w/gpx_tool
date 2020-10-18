use std::str::FromStr;

pub fn same(a: &(f64, f64), b: &(f64, f64)) -> bool {
   (a.0.to_bits() == b.0.to_bits()) && (a.1.to_bits() == b.1.to_bits()) 
}

pub fn coordinates_lookup(point: (f64, f64), points: &Vec<(f64, f64)>) -> usize {

    let mut index = 0usize;
    for (i, p) in points.iter().enumerate() {

        if same(p, &point) {
            index = i;
            break;
        }
    }

    index
}

#[derive(Debug)]
pub enum CardinalPoint {

    North, South, East, West
}

impl FromStr for CardinalPoint {
    type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "North" => Ok(CardinalPoint::North),
                "South" => Ok(CardinalPoint::South),
                "East"  => Ok(CardinalPoint::East),
                "West"  => Ok(CardinalPoint::West),
                _ => Err(format!("'{}' is not a valid value for CardinalType", s)),
            }
        }
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
