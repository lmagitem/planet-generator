use crate::diagnostics::types::{GenerationError, GenerationWarning};
use crate::diagnostics::Diagnostics;
use crate::galaxy::map::hex::types::SpaceCoordinates;
use crate::galaxy::map::hex::GalacticHex;
use crate::generator::types::GenerationSettings;
use crate::system::orbital_point::OrbitalPoint;
use crate::system::types::SystemGenContext;
use crate::system::types::SystemPeculiarity;
use crate::universe::Universe;
use log::debug;
use seeded_dice_roller::SeededDiceRoller;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use std::cell::RefCell;
use std::rc::Rc;
pub mod celestial_body;
pub mod celestial_disk;
pub mod contents;
mod display;
pub mod generator;
pub mod neighborhood;
pub mod orbital_point;
pub mod star;
pub mod types;

#[derive(Clone, PartialEq, PartialOrd, Debug, SmartDefault, Serialize, Deserialize)]
pub struct StarSystem {
    /// That star's name.
    #[default("default")]
    pub name: Rc<str>,
    /// The id of the [OrbitalPoint] at the center of the system.
    pub center_id: u32,
    /// The id of the [OrbitalPoint] containing the main star of the system.
    pub main_star_id: u32,
    /// The list of [OrbitalPoint]s that can be found in the system.
    pub all_objects: Vec<OrbitalPoint>,
    /// What are the pecularities of this system.
    pub special_traits: Vec<SystemPeculiarity>,
    /// The non-fatal issues raised while generating this system (from the accepted
    /// generation try only; rejected tries' diagnostics are dropped). Transient:
    /// deliberately not serialized. Note that a deserialized system (whose
    /// diagnostics are rebuilt empty) compares unequal to the original whenever
    /// the original's diagnostics were non-empty.
    #[serde(skip)]
    pub diagnostics: Diagnostics,
}

impl StarSystem {
    /// Creates a new star system with the given array of [OrbitalPoint], and the id of the system's main star.
    pub fn new(
        name: Rc<str>,
        center_id: u32,
        main_star_id: u32,
        all_objects: Vec<OrbitalPoint>,
        special_traits: Vec<SystemPeculiarity>,
    ) -> Self {
        Self {
            name,
            center_id,
            main_star_id,
            all_objects,
            special_traits,
            diagnostics: Diagnostics::default(),
        }
    }

    /// Returns a reference to the [OrbitalPoint] at the center of the system. It can either be a [Star] or the barycentre of a binary pair.
    pub fn get_center(&self) -> &OrbitalPoint {
        // `center_id` always points into `all_objects` by construction; the
        // first-element fallback is unreachable (and deterministic).
        let index = self
            .all_objects
            .iter()
            .position(|p| p.id == self.center_id)
            .unwrap_or(0);
        &self.all_objects[index]
    }

    /// Returns a mutable reference to the [OrbitalPoint] at the center of the system. It can either be a [Star] or the barycentre
    /// of a binary pair.
    pub fn get_center_mut(&mut self) -> &mut OrbitalPoint {
        // See `get_center`: fallback is unreachable by construction.
        let index = self
            .all_objects
            .iter()
            .position(|p| p.id == self.center_id)
            .unwrap_or(0);
        &mut self.all_objects[index]
    }

    /// Returns a reference to the [OrbitalPoint] containing the main [Star] of the system.
    pub fn get_main_star(&self) -> &OrbitalPoint {
        // `main_star_id` always points into `all_objects` by construction; the
        // first-element fallback is unreachable (and deterministic).
        let index = self
            .all_objects
            .iter()
            .position(|p| p.id == self.main_star_id)
            .unwrap_or(0);
        &self.all_objects[index]
    }

    /// Returns a mutable reference to the [OrbitalPoint] containing the main [Star] of the system.
    pub fn get_main_star_mut(&mut self) -> &mut OrbitalPoint {
        // See `get_main_star`: fallback is unreachable by construction.
        let index = self
            .all_objects
            .iter()
            .position(|p| p.id == self.main_star_id)
            .unwrap_or(0);
        &mut self.all_objects[index]
    }

    /// Returns an [Option] that might contain a reference to the object with the given id.
    pub fn get_point(&self, id: u32) -> Option<&OrbitalPoint> {
        self.all_objects.iter().find(|p| p.id == id)
    }

    /// Returns an [Option] that might contain a mutable reference to the object with the given id.
    pub fn get_point_mut(&mut self, id: u32) -> Option<&mut OrbitalPoint> {
        self.all_objects.iter_mut().find(|p| p.id == id)
    }
}

impl<'a> SystemGenContext<'a> {
    pub(crate) fn new(
        system_gen_try: u32,
        settings: &'a GenerationSettings,
        coord: SpaceCoordinates,
        system_index: u16,
        system_traits: &'a Vec<SystemPeculiarity>,
        hex: &'a GalacticHex,
        universe: Universe,
    ) -> Self {
        Self {
            settings,
            coord,
            system_index,
            // Byte-identical to both existing try-seed sites.
            seed: format!("{}{}", system_gen_try, settings.seed).into(),
            system_traits,
            hex,
            universe,
            diagnostics: RefCell::new(Diagnostics::default()),
            step_scratch: RefCell::new(String::new()),
        }
    }

    /// Builds a `SeededDiceRoller` from a step written into the reusable scratch buffer
    /// and the supplied seed.
    /// `write!` into a `String` is infallible; callers use `let _ =` on the
    /// `Result` (no `unwrap`).
    pub(crate) fn with_step(&self, seed: &str, f: impl FnOnce(&mut String)) -> SeededDiceRoller {
        let mut buf = self.step_scratch.borrow_mut();
        buf.clear();
        f(&mut buf);
        buf.push('_');
        buf.push_str(seed);
        SeededDiceRoller::from_combined(&buf)
    }

    pub(crate) fn warn(&self, w: GenerationWarning) {
        self.diagnostics.borrow_mut().push_warning(w);
    }

    pub(crate) fn fail(&self, e: GenerationError) {
        self.diagnostics.borrow_mut().push_error(e);
    }

    /// Keeps the field read (no dead_code) and gives phase 6 a hook.
    /// Called once at the end of `generate_stars_systems`.
    pub(crate) fn report(&self) {
        let d = self.diagnostics.borrow();
        if !d.is_empty() {
            debug!("Generation diagnostics: {}", *d);
        }
    }
}
