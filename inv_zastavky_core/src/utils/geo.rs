const EARTH_RADIUS: f64 = 6371.0;

pub fn approx_distance(first_lat: f64, first_lon: f64, second_lat: f64, second_lon: f64) -> f64 {
    let lat_diff = second_lat - first_lat;
    let lon_diff = second_lon - first_lon;
    2f64 * EARTH_RADIUS
        * ((1f64 - lat_diff.to_radians().cos()
            + first_lat.to_radians().cos()
                * second_lat.to_radians().cos()
                * (1f64 - lon_diff.to_radians().cos()))
            / 2f64)
            .sqrt()
            .asin()
}

pub fn approx_len(line: &Vec<(f64, f64)>) -> f64 {
    line.windows(2)
        .map(|w| approx_distance(w[0].0, w[0].1, w[1].0, w[1].1))
        .sum()
}
