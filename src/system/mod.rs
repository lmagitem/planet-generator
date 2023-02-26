use crate::prelude::*;
pub mod generator;
pub mod neighborhood;
pub mod orbital_point;
pub mod planet;
pub mod star;
pub mod types;

#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct StarSystem {
    center_id: u32,
    main_star_id: u32,
    pub all_objects: Vec<OrbitalPoint>,
}

impl StarSystem {
    /// Creates a new star system with the given array of [OrbitalPoint], and the id of the system's main star.
    pub fn new(all_objects: Vec<OrbitalPoint>, center_id: u32, main_star_id: u32) -> Self {
        Self {
            center_id,
            main_star_id,
            all_objects,
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
