#![allow(dead_code)]
use crate::internal::*;
use crate::prelude::*;

/// The following array contains equivalencies between masses and densities.
/// Masses are in Earth masses, and density in gram per cubic centimeter.
pub const MASS_TO_DENSITY_DATASET: &[(f64, f64); 28] = &[
    (f64::MAX, 11.02),
    (25440.0, 60.0),
    (4131.0, 6.0),
    (4000.0, 8.82),
    (3500.0, 7.72),
    (3000.0, 6.62),
    (2500.0, 5.51),
    (2000.0, 4.41),
    (1500.0, 3.3),
    (1000.0, 2.2),
    (800.0, 1.93),
    (600.0, 1.7),
    (500.0, 1.6),
    (450.0, 1.49),
    (400.0, 1.43),
    (350.0, 1.38),
    (300.0, 1.32),
    (250.0, 1.21),
    (200.0, 1.1),
    (150.0, 1.05),
    (100.0, 0.99),
    (80.0, 0.94),
    (40.0, 0.94),
    (30.0, 1.05),
    (20.0, 1.21),
    (15.0, 1.43),
    (10.0, 2.31),
    (0.0, 0.687),
];
