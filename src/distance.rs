use std::f64::consts::PI;

const EARTH_RADIUS: f64 = 6371e3;

/*
Formula from: https://www.movable-type.co.uk/scripts/latlong.html

*/
pub fn distance(coords_1: &(f64, f64), coords_2: &(f64, f64)) -> f64 {

    let (lat1, lon1) = *coords_1;
    let (lat2, lon2) = *coords_2;

    let to_radians = |x| x * (PI/180.0);

    let phi1 = to_radians(lat1);
    let phi2 = to_radians(lat2);

    let d_phi = to_radians(lat2-lat1);
    let d_del = to_radians(lon2-lon1);

    let a = (d_phi/2.0).sin().powi(2) + phi1.cos()*phi2.cos()*(d_del/2.0).sin().powi(2);

    let c = a.sqrt().atan2((1.0-a).sqrt()) * 2.0;

    EARTH_RADIUS * c
}
