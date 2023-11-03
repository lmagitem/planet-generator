pub fn calculate_blackbody_temperature(luminosity: f32, orbital_radius: f64) -> u32 {
    if orbital_radius <= 0.0 {
        panic!("Orbital radius should be greater than 0");
    }

    let b = 278.0 * ((luminosity as f64).powf(0.25)) / (orbital_radius).sqrt();
    b.round() as u32
}
