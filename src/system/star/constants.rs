use crate::prelude::*;

pub const BROWN_DWARF_MIN_MASS: f32 = 0.015;
pub const RED_DWARF_POP_0_MIN_MASS: f32 = 0.04;
pub const RED_DWARF_POP_I_MIN_MASS: f32 = 0.07;
pub const RED_DWARF_POP_II_MIN_MASS: f32 = 0.125;
pub const RED_DWARF_POP_III_MIN_MASS: f32 = 0.4;
pub const ORANGE_DWARF_MIN_MASS: f32 = 0.5;
pub const YELLOW_DWARF_MIN_MASS: f32 = 1.0;
pub const WHITE_DWARF_MIN_MASS: f32 = 2.0;
pub const WHITE_GIANT_MIN_MASS: f32 = 4.0;
pub const BLUE_GIANT_MIN_MASS: f32 = 8.0;
pub const BLUE_GIANT_POP_0_MAX_MASS: f32 = 50.0;
pub const BLUE_GIANT_POP_I_MAX_MASS: f32 = 100.0;
pub const BLUE_GIANT_POP_II_MAX_MASS: f32 = 200.0;
pub const BLUE_GIANT_POP_III_MAX_MASS: f32 = 500.0;
pub const TEMPERATURE_TO_SPECTRAL_TYPE_DATASET: &[(u32, u32); 69] = &[
    (u32::MAX, 0), // WR
    (1500000, 0),
    (500000, 1),
    (380000, 2),
    (170000, 3),
    (117000, 4),
    (54000, 12), // O
    (45000, 13),
    (43300, 14),
    (40600, 15),
    (39500, 16),
    (37100, 17),
    (35100, 18),
    (33300, 19),
    (29200, 20), // B
    (23000, 21),
    (21000, 22),
    (17600, 23),
    (15200, 25),
    (14300, 26),
    (13500, 27),
    (12300, 28),
    (11400, 29),
    (9600, 30), // A
    (9330, 31),
    (9040, 32),
    (8750, 33),
    (8480, 34),
    (8310, 35),
    (7920, 37),
    (7350, 40), // F
    (7050, 42),
    (6850, 43),
    (6700, 45),
    (6550, 46),
    (6400, 47),
    (6300, 48),
    (6050, 50), // G
    (5930, 51),
    (5800, 52),
    (5660, 55),
    (5440, 58),
    (5240, 60), // K
    (5110, 61),
    (4960, 62),
    (4800, 63),
    (4600, 64),
    (4400, 65),
    (4000, 67),
    (3750, 70), // M
    (3700, 71),
    (3600, 72),
    (3500, 73),
    (3400, 74),
    (3200, 75),
    (3100, 76),
    (2900, 77),
    (2700, 78),
    (2600, 80), // L
    (2200, 83),
    (1500, 88),
    (1400, 92), // T
    (1000, 96),
    (800, 98),
    (370, 100), // Y
    (350, 101),
    (320, 102),
    (250, 104),
    (0, 109),
];

pub fn is_star_a_main_sequence_dwarf_or_giant(star: &Star) -> bool {
    star.luminosity_class == StarLuminosityClass::V
        && (discriminant(&star.spectral_type) == discriminant(&StarSpectralType::O(0))
            || discriminant(&star.spectral_type) == discriminant(&StarSpectralType::B(0))
            || discriminant(&star.spectral_type) == discriminant(&StarSpectralType::A(0))
            || discriminant(&star.spectral_type) == discriminant(&StarSpectralType::F(0))
            || discriminant(&star.spectral_type) == discriminant(&StarSpectralType::G(0))
            || discriminant(&star.spectral_type) == discriminant(&StarSpectralType::K(0))
            || discriminant(&star.spectral_type) == discriminant(&StarSpectralType::M(0)))
}

pub fn print_real_to_generated_stars_comparison_results(
    rad_sum: f32,
    lum_ms_sum: f32,
    lum_calc_sum: f32,
    temp_sum: f32,
) {
    println!(
        "\nVariance from generated values to real ones - radius: {}%, main sequence luminosity: {}%, luminosity from temperature: {}%, temperature from luminosity: {}%\n",
        format!("{}{}", if rad_sum > 0.0 {"+"} else {""}, rad_sum * 100.0),
        format!("{}{}", if lum_ms_sum > 0.0 {"+"} else {""}, lum_ms_sum * 100.0),
        format!("{}{}", if lum_calc_sum > 0.0 {"+"} else {""}, lum_calc_sum * 100.0),
        format!("{}{}", if temp_sum > 0.0 {"+"} else {""}, temp_sum * 100.0),
    );
}

pub fn print_real_to_generated_star_comparison(
    star: &Star,
    mass: f32,
    radius: f32,
    luminosity: f32,
    calc_luminosity: f32,
    temperature: u32,
    spectral_type: StarSpectralType,
) {
    println!(
        "Real {} - mass: {}, rad: {}, lum: {}, temp: {}K, type: {}",
        star.name, star.mass, star.radius, star.luminosity, star.temperature, star.spectral_type
    );
    println!(
        "Generated {} - mass: {}, rad: {} ({}), lum: {} (ms: {}, calc: {}), temp: {}K ({}), type: {}",
        star.name,
        mass,
        radius,
        get_difference_percentage_str(radius, star.radius),
        luminosity,
        get_difference_percentage_str(luminosity, star.luminosity as f32),
        get_difference_percentage_str(calc_luminosity, star.luminosity as f32),
        temperature,
        get_difference_percentage_str(temperature as f32, star.temperature as f32),
        spectral_type
    );
}

pub fn get_test_stars() -> Vec<Star> {
    vec![
        Star::new(
            "40 Eridani B".to_string(),
            0.573, // Mass
            0.013, // Luminosity
            0.014, // Radius
            9.0,   // Age
            16500, // Temperature
            StarSpectralType::DA(4),
            StarLuminosityClass::VII,
        ),
        Star::new(
            "Sirius B".to_string(),
            1.018,  // Mass
            0.056,  // Luminosity
            0.0084, // Radius
            0.228,  // Age
            25200,  // Temperature
            StarSpectralType::DA(2),
            StarLuminosityClass::VII,
        ),
        Star::new(
            "Cygnus OB2-12".to_string(),
            110.0,     // Mass
            1660000.0, // Luminosity
            246.0,     // Radius
            0.003,     // Age
            13700,     // Temperature
            StarSpectralType::B(3),
            StarLuminosityClass::Ia,
        ),
        Star::new(
            "Rigel A".to_string(),
            21.0,     // Mass
            120000.0, // Luminosity
            78.9,     // Radius
            0.008,    // Age
            12100,    // Temperature
            StarSpectralType::F(8),
            StarLuminosityClass::Ia,
        ),
        Star::new(
            "WISE 0855-0714".to_string(),
            0.004772,  // Mass
            0.0000011, // Luminosity
            0.021,     // Radius
            -1.0,      // Age
            240,       // Temperature
            StarSpectralType::Y(4),
            StarLuminosityClass::V,
        ),
        Star::new(
            "Luhman 16".to_string(),
            0.03197,   // Mass
            0.0000219, // Luminosity
            0.08734,   // Radius
            -1.0,      // Age
            1350,      // Temperature
            StarSpectralType::L(7),
            StarLuminosityClass::V,
        ),
        Star::new(
            "Wolf 359".to_string(),
            0.11,    // Mass
            0.00106, // Luminosity
            0.144,   // Radius
            0.25,    // Age
            2749,    // Temperature
            StarSpectralType::M(6),
            StarLuminosityClass::V,
        ),
        Star::new(
            "Proxima Centauri".to_string(),
            0.1221,   // Mass
            0.001567, // Luminosity
            0.1542,   // Radius
            4.85,     // Age
            2992,     // Temperature
            StarSpectralType::M(5),
            StarLuminosityClass::V,
        ),
        Star::new(
            "Barnard's Star".to_string(),
            0.144,  // Mass
            0.0035, // Luminosity
            0.196,  // Radius
            10.0,   // Age
            3134,   // Temperature
            StarSpectralType::M(4),
            StarLuminosityClass::V,
        ),
        Star::new(
            "Ross 154".to_string(),
            0.177,    // Mass
            0.004015, // Luminosity
            0.2,      // Radius
            0.9,      // Age
            3248,     // Temperature
            StarSpectralType::M(3),
            StarLuminosityClass::V,
        ),
        Star::new(
            "40 Eridani C".to_string(),
            0.2036, // Mass
            0.008,  // Luminosity
            0.31,   // Radius
            5.6,    // Age
            3100,   // Temperature
            StarSpectralType::M(4),
            StarLuminosityClass::V,
        ),
        Star::new(
            "Lalande 21185".to_string(),
            0.389,  // Mass
            0.0195, // Luminosity
            0.392,  // Radius
            7.5,    // Age
            3547,   // Temperature
            StarSpectralType::M(2),
            StarLuminosityClass::V,
        ),
        Star::new(
            "Lacaille 9352".to_string(),
            0.479,  // Mass
            0.0368, // Luminosity
            0.474,  // Radius
            4.57,   // Age
            3672,   // Temperature
            StarSpectralType::M(0),
            StarLuminosityClass::V,
        ),
        Star::new(
            "TW Piscis Austrini".to_string(),
            0.725, // Mass
            0.19,  // Luminosity
            0.629, // Radius
            0.44,  // Age
            4711,  // Temperature
            StarSpectralType::K(5),
            StarLuminosityClass::V,
        ),
        Star::new(
            "40 Eridani A".to_string(),
            0.78,  // Mass
            0.457, // Luminosity
            0.812, // Radius
            7.0,   // Age
            5072,  // Temperature
            StarSpectralType::K(0),
            StarLuminosityClass::V,
        ),
        Star::new(
            "Ran".to_string(),
            0.82,  // Mass
            0.34,  // Luminosity
            0.735, // Radius
            0.6,   // Age
            5084,  // Temperature
            StarSpectralType::K(2),
            StarLuminosityClass::V,
        ),
        Star::new(
            "Alpha Centauri B".to_string(),
            0.9092, // Mass
            0.4981, // Luminosity
            0.8591, // Radius
            5.3,    // Age
            5260,   // Temperature
            StarSpectralType::K(1),
            StarLuminosityClass::V,
        ),
        Star::new(
            "61 Ursae Majoris".to_string(),
            0.93,  // Mass
            0.609, // Luminosity
            0.86,  // Radius
            2.1,   // Age
            5488,  // Temperature
            StarSpectralType::G(8),
            StarLuminosityClass::V,
        ),
        Star::new(
            "Beta Canum Venaticorum".to_string(),
            0.97,  // Mass
            1.151, // Luminosity
            1.123, // Radius
            3.4,   // Age
            6043,  // Temperature
            StarSpectralType::G(0),
            StarLuminosityClass::V,
        ),
        Star::new(
            "Sun".to_string(),
            1.0,  // Mass
            1.0,  // Luminosity
            1.0,  // Radius
            4.6,  // Age
            5772, // Temperature
            StarSpectralType::G(2),
            StarLuminosityClass::V,
        ),
        Star::new(
            "Kappa Ceti".to_string(),
            1.037, // Mass
            0.85,  // Luminosity
            0.95,  // Radius
            0.3,   // Age
            5708,  // Temperature
            StarSpectralType::G(5),
            StarLuminosityClass::V,
        ),
        Star::new(
            "Alpha Centauri A".to_string(),
            1.0788, // Mass
            1.5059, // Luminosity
            1.2175, // Radius
            5.3,    // Age
            5790,   // Temperature
            StarSpectralType::G(2),
            StarLuminosityClass::V,
        ),
        Star::new(
            "Upsilon Andromedae".to_string(),
            1.27, // Mass
            3.57, // Luminosity
            1.48, // Radius
            3.12, // Age
            6213, // Temperature
            StarSpectralType::F(8),
            StarLuminosityClass::V,
        ),
        Star::new(
            "Tau Boötis".to_string(),
            1.39, // Mass
            3.06, // Luminosity
            1.42, // Radius
            1.6,  // Age
            6399, // Temperature
            StarSpectralType::F(7),
            StarLuminosityClass::V,
        ),
        Star::new(
            "Procyon A".to_string(),
            1.42,  // Mass
            7.73,  // Luminosity
            2.048, // Radius
            2.4,   // Age
            6550,  // Temperature
            StarSpectralType::F(5),
            StarLuminosityClass::V,
        ),
        Star::new(
            "Altair".to_string(),
            1.86, // Mass
            10.6, // Luminosity
            1.57, // Radius
            0.1,  // Age
            7760, // Temperature
            StarSpectralType::A(7),
            StarLuminosityClass::V,
        ),
        Star::new(
            "Fomalhaut".to_string(),
            1.92,  // Mass
            16.63, // Luminosity
            1.842, // Radius
            0.44,  // Age
            8590,  // Temperature
            StarSpectralType::A(3),
            StarLuminosityClass::V,
        ),
        Star::new(
            "Delta Capricorni".to_string(),
            2.0,   // Mass
            8.5,   // Luminosity
            1.91,  // Radius
            0.228, // Age
            7301,  // Temperature
            StarSpectralType::A(7),
            StarLuminosityClass::V,
        ),
        Star::new(
            "Sirius A".to_string(),
            2.063, // Mass
            25.4,  // Luminosity
            1.711, // Radius
            0.228, // Age
            9940,  // Temperature
            StarSpectralType::A(0),
            StarLuminosityClass::V,
        ),
        Star::new(
            "HD 21071".to_string(),
            3.69,  // Mass
            278.0, // Luminosity
            2.21,  // Radius
            0.009, // Age
            14768, // Temperature
            StarSpectralType::B(7),
            StarLuminosityClass::V,
        ),
        Star::new(
            "Eta Aurigae".to_string(),
            5.4,   // Mass
            955.0, // Luminosity
            3.25,  // Radius
            0.022, // Age
            17201, // Temperature
            StarSpectralType::B(3),
            StarLuminosityClass::V,
        ),
        Star::new(
            "10 Lacertae".to_string(),
            26.9,     // Mass
            102000.0, // Luminosity
            8.27,     // Radius
            0.028,    // Age
            36000,    // Temperature
            StarSpectralType::O(9),
            StarLuminosityClass::V,
        ),
    ]
}
