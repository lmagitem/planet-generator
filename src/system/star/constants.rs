#![allow(dead_code)]
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
/// The following array gives expected temperatures and luminosity (powers of 10) for solar masses at each stage of their lifespan
/// 1st entry is "birth", 3rd is beginning of subgiant, 5th beginning of giant/supergiant, 7th end of giant/supergiant, and inbetween are mid-points
pub const STAR_LIFECYCLE_DATASET: [[TemperatureAndLuminosity; 7]; 8] = [
    // 0.4 solar masses
    [
        TemperatureAndLuminosity(3375.0, -2.05),
        TemperatureAndLuminosity(4300.0, -0.8),
        TemperatureAndLuminosity(4100.0, -0.2),
        TemperatureAndLuminosity(3950.0, 0.7),
        TemperatureAndLuminosity(3800.0, 1.2),
        TemperatureAndLuminosity(3650.0, 1.75),
        TemperatureAndLuminosity(3200.0, 0.5),
    ],
    // 0.5 solar masses
    [
        TemperatureAndLuminosity(4200.0, -1.3),
        TemperatureAndLuminosity(4300.0, -0.8),
        TemperatureAndLuminosity(4100.0, -0.2),
        TemperatureAndLuminosity(3950.0, 0.7),
        TemperatureAndLuminosity(3800.0, 1.2),
        TemperatureAndLuminosity(3650.0, 1.75),
        TemperatureAndLuminosity(3300.0, 2.3),
    ],
    // 1 solar mass
    [
        TemperatureAndLuminosity(5400.0, -0.1455),
        TemperatureAndLuminosity(5805.0, 0.0126543),
        TemperatureAndLuminosity(5500.0, 0.6),
        TemperatureAndLuminosity(5200.0, 0.75),
        TemperatureAndLuminosity(4300.0, 0.75),
        TemperatureAndLuminosity(3900.0, 1.25),
        TemperatureAndLuminosity(3500.0, 2.6),
    ],
    // 2 solar masses
    [
        TemperatureAndLuminosity(8450.0, 0.8),
        TemperatureAndLuminosity(7800.0, 1.4),
        TemperatureAndLuminosity(6700.0, 1.4),
        TemperatureAndLuminosity(7500.0, 1.5),
        TemperatureAndLuminosity(5100.0, 1.7),
        TemperatureAndLuminosity(4500.0, 2.0),
        TemperatureAndLuminosity(3950.0, 2.9),
    ],
    // 5 solar masses
    [
        TemperatureAndLuminosity(17000.0, 2.75),
        TemperatureAndLuminosity(16000.0, 3.1),
        TemperatureAndLuminosity(13800.0, 3.1),
        TemperatureAndLuminosity(8000.0, 3.2),
        TemperatureAndLuminosity(3600.0, 3.5),
        TemperatureAndLuminosity(8600.0, 3.8),
        TemperatureAndLuminosity(5500.0, 3.9),
    ],
    // 15 solar masses
    [
        TemperatureAndLuminosity(31000.0, 4.4),
        TemperatureAndLuminosity(25000.0, 4.6),
        TemperatureAndLuminosity(27000.0, 4.7),
        TemperatureAndLuminosity(17000.0, 4.75),
        TemperatureAndLuminosity(12000.0, 4.8),
        TemperatureAndLuminosity(6000.0, 4.6),
        TemperatureAndLuminosity(3600.0, 4.8),
    ],
    // 60 solar masses
    [
        TemperatureAndLuminosity(43520.0, 5.75),
        TemperatureAndLuminosity(17000.0, 5.95),
        TemperatureAndLuminosity(6000.0, 6.0),
        TemperatureAndLuminosity(19000.0, 6.1),
        TemperatureAndLuminosity(46000.0, 6.0),
        TemperatureAndLuminosity(27000.0, 5.9),
        TemperatureAndLuminosity(62000.0, 5.4),
    ],
    // 500 solar masses
    [
        TemperatureAndLuminosity(53000.0, 6.7),
        TemperatureAndLuminosity(22000.0, 6.9),
        TemperatureAndLuminosity(7000.0, 6.8),
        TemperatureAndLuminosity(24000.0, 7.0),
        TemperatureAndLuminosity(48000.0, 6.9),
        TemperatureAndLuminosity(30000.0, 6.8),
        TemperatureAndLuminosity(70000.0, 6.2),
    ],
];
/// The following array contains equivalencies between temperatures and spectral types.
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

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct TemperatureAndLuminosity(pub f32, pub f32);

pub fn get_test_stars() -> Vec<Star> {
    vec![
        Star::new(
            "40 Eridani B".to_string(),
            0.573, // Mass
            0.013, // Luminosity
            0.014, // Radius
            9.0,   // Age
            16500, // Temperature
            StarSpectralType::DA,
            StarLuminosityClass::VII,
            0.0,
        ),
        Star::new(
            "Sirius B".to_string(),
            1.018,  // Mass
            0.056,  // Luminosity
            0.0084, // Radius
            0.228,  // Age
            25200,  // Temperature
            StarSpectralType::DA,
            StarLuminosityClass::VII,
            0.0,
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
            0.0,
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
            0.0,
        ),
        Star::new(
            "Epsilon Canis Majoris".to_string(),
            12.6,    // Mass
            38700.0, // Luminosity
            13.9,    // Radius
            0.022,   // Age
            22900,   // Temperature
            StarSpectralType::B(2),
            StarLuminosityClass::II,
            0.0,
        ),
        Star::new(
            "Canopus".to_string(),
            8.0,     // Mass
            10700.0, // Luminosity
            71.0,    // Radius
            0.025,   // Age
            7400,    // Temperature
            StarSpectralType::A(9),
            StarLuminosityClass::II,
            0.0,
        ),
        Star::new(
            "Beta Draconis".to_string(),
            6.0,   // Mass
            996.0, // Luminosity
            40.0,  // Radius
            0.095, // Age
            5160,  // Temperature
            StarSpectralType::G(2),
            StarLuminosityClass::Ib,
            0.0,
        ),
        Star::new(
            "Theta Scorpii".to_string(),
            3.1,    // Mass
            1400.0, // Luminosity
            26.3,   // Radius
            0.5,    // Age
            6294,   // Temperature
            StarSpectralType::F(0),
            StarLuminosityClass::II,
            0.0,
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
            0.0,
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
            0.0,
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
            0.0,
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
            0.0,
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
            0.0,
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
            0.0,
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
            0.0,
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
            0.0,
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
            0.0,
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
            0.0,
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
            0.0,
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
            0.0,
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
            0.0,
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
            0.0,
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
            0.0,
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
            0.0,
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
            0.0,
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
            0.0,
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
            0.0,
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
            0.0,
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
            0.0,
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
            0.0,
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
            0.0,
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
            0.0,
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
            0.0,
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
            0.0,
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
            0.0,
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
            0.0,
        ),
    ]
}
