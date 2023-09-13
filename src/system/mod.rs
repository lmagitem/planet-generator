use crate::prelude::*;
pub mod celestial_body;
pub mod contents;
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
        }
    }

    /// Returns a reference to the [OrbitalPoint] at the center of the system. It can either be a [Star] or the barycentre of a binary pair.
    pub fn get_center(&self) -> &OrbitalPoint {
        self.get_point(self.center_id)
            .expect("There should always be a center point.")
    }

    /// Returns a mutable reference to the [OrbitalPoint] at the center of the system. It can either be a [Star] or the barycentre
    /// of a binary pair.
    pub fn get_center_mut(&mut self) -> &mut OrbitalPoint {
        self.get_point_mut(self.center_id)
            .expect("There should always be a center point.")
    }

    /// Returns a reference to the [OrbitalPoint] containing the main [Star] of the system.
    pub fn get_main_star(&self) -> &OrbitalPoint {
        self.get_point(self.main_star_id)
            .expect("There should always be a main star.")
    }

    /// Returns a mutable reference to the [OrbitalPoint] containing the main [Star] of the system.
    pub fn get_main_star_mut(&mut self) -> &mut OrbitalPoint {
        self.get_point_mut(self.main_star_id)
            .expect("There should always be a main star.")
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
