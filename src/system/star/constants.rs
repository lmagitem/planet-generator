use crate::prelude::*;

pub fn print_tested_star(
    star: &Star,
    calc_radius: f32,
    main_seq_lum: f32,
    calc_lum: f32,
    calc_temp: f32,
) {
    println!(
        "- {} - {} {}\n>>>>>>>>>> age: {},\n>>>>>>>>>> mass: {},\n>>>>>>>>>> radius: {} (calc: {} ({})),\n>>>>>>>>>> luminosity: {} (calc: {} ({}), main seq: {} ({})),\n>>>>>>>>>> temp: {} (calc: {} ({}))",
        star.name,
        star.spectral_type,
        star.luminosity_class,
        star.age,
        (star.mass * 1000000.0).round() / 1000000.0,
        (star.radius * 1000000.0).round() / 1000000.0,
        calc_radius,
        get_difference_percentage(calc_radius, star.radius),
        (star.luminosity * 1000000.0).round() / 1000000.0,
        (calc_lum * 1000000.0).round() / 1000000.0,
        get_difference_percentage(calc_lum, star.luminosity),
        (main_seq_lum * 1000000.0).round() / 1000000.0,
        get_difference_percentage(main_seq_lum, star.luminosity),
        star.temperature,
        (calc_temp * 1000000.0).round() / 1000000.0,
        get_difference_percentage(calc_temp, star.temperature as f32),
    );
}

pub fn get_test_stars() -> Vec<Star> {
    vec![
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
            "WISE 0855−0714".to_string(),
            0.004772,  // Mass
            0.0000011, // Luminosity
            0.021,     // Radius
            -1.0,      // Age
            240,       // Temperature
            StarSpectralType::Y(4),
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
            "Luyten 726-8 A".to_string(),
            0.102,   // Mass
            0.00006, // Luminosity
            0.14,    // Radius
            8.0,     // Age
            2670,    // Temperature
            StarSpectralType::M(5),
            StarLuminosityClass::V,
        ),
        Star::new(
            "Luyten 726-8 B".to_string(),
            0.1,     // Mass
            0.00004, // Luminosity
            0.14,    // Radius
            8.0,     // Age
            2650,    // Temperature
            StarSpectralType::M(6),
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
            "40 Eridani C".to_string(),
            0.2036, // Mass
            0.008,  // Luminosity
            0.31,   // Radius
            5.6,    // Age
            3100,   // Temperature
            StarSpectralType::M(4),
            StarLuminosityClass::V,
        ),
    ]
}

pub const dataset: &[(u32, u32); 72] = &[
    (u32::MAX, 0),
    (1500000, 0),
    (500000, 1),
    (380000, 2),
    (170000, 3),
    (117000, 4),
    (54000, 12),
    (45000, 13),
    (43300, 14),
    (40600, 15),
    (39500, 16),
    (37100, 17),
    (35100, 18),
    (33300, 19),
    (29200, 20),
    (23000, 21),
    (21000, 22),
    (17600, 23),
    (15200, 25),
    (14300, 26),
    (13500, 27),
    (12300, 28),
    (11400, 29),
    (9600, 30),
    (9330, 31),
    (9040, 32),
    (8750, 33),
    (8480, 34),
    (8310, 35),
    (7920, 37),
    (7350, 40),
    (7050, 42),
    (6850, 43),
    (6700, 45),
    (6550, 46),
    (6400, 47),
    (6300, 48),
    (6050, 50),
    (5930, 51),
    (5800, 52),
    (5660, 55),
    (5440, 58),
    (5240, 60),
    (5110, 61),
    (4960, 62),
    (4800, 63),
    (4600, 64),
    (4400, 65),
    (4000, 67),
    (3750, 70),
    (3700, 71),
    (3600, 72),
    (3500, 73),
    (3400, 74),
    (3200, 75),
    (3100, 76),
    (2900, 77),
    (2700, 78),
    (2600, 80),
    (2200, 83),
    (1500, 88),
    (1400, 92),
    (1000, 96),
    (800, 98),
    (370, 100),
    (350, 101),
    (320, 102),
    (250, 104),
    (200, 106),
    (150, 108),
    (100, 109),
    (0, 109),
];
