use crate::prelude::*;

/// A list of settings used to configure the [Galaxy] generation.
#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct GalaxySettings {
    /// The specific [GalacticNeighborhoodDensity] if any.
    pub fixed_neighborhood: Option<GalacticNeighborhoodDensity>,
    /// The specific [GalaxyCategory] if any.
    pub fixed_category: Option<GalaxyCategory>,
    /// The specific [GalaxySubCategory] if any.
    pub fixed_sub_category: Option<GalaxySubCategory>,
    /// A list of specific [GalaxySpecialTrait]s to use, if any.
    pub fixed_special_traits: Option<Vec<GalaxySpecialTrait>>,
    /// A list of [GalaxySpecialTrait]s forbidden to use in galaxy generation.
    pub forbidden_special_traits: Option<Vec<GalaxySpecialTrait>>,
    /// The specific age to use for galaxy generation, if any.
    pub fixed_age: Option<f32>,
    /// Skip the galaxy generation and just uses a copy of ours.
    pub use_ours: bool,
}

impl Display for GalaxySettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ fixed_era: {}, era_before: {}, era_after: {}, fixed_age: {}, age_before: {}, age_after: {}, use_ours: {} }}",
        if self.fixed_neighborhood.is_some() { format!("{}", self.fixed_neighborhood.unwrap()) } else { String::from("None") },
        if self.fixed_category.is_some() { format!("{}", self.fixed_category.unwrap()) } else { String::from("None") },
        if self.fixed_sub_category.is_some() { format!("{}", self.fixed_sub_category.unwrap()) } else { String::from("None") },
        if self.fixed_special_traits.is_some() { format!("{}", self.fixed_special_traits.as_ref().unwrap()
        .iter()
        .map(|t| format!("{}", t))
        .collect::<Vec<String>>()
        .join(", ")) } else { String::from("None") },
        if self.forbidden_special_traits.is_some() { format!("{}", self.forbidden_special_traits.as_ref().unwrap()
        .iter()
        .map(|t| format!("{}", t))
        .collect::<Vec<String>>()
        .join(", ")) } else { String::from("None") },
        if self.fixed_age.is_some() { format!("{}", self.fixed_age.unwrap()) } else { String::from("None") },
        self.use_ours)
    }
}

/// Defines the density of a galactic neighborhood. The first associated number indicates how many major galaxies we find in that
/// neighborhood, the second indicates how many galaxies are dominant ones. Major galaxies within 2 megaparsecs (or 5 to 10 megaparsecs for
/// giant and dominant galaxies) tend to be gravitationally bound to each others.
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, SmartDefault, Serialize, Deserialize,
)]
pub enum GalacticNeighborhoodDensity {
    /// The emptiest parts of the universe, covers a diameter far greater than the other densities. Contains 0 to 3 major galaxies.
    Void(#[default = 1] u8),
    /// A zone filled with a "regular" amount of galaxies. Contains 1 to 5 major galaxies.
    #[default]
    Group(#[default = 2] u8),
    /// The most crowded parts of the universe. Galaxies within this neighborhood usualy revolve around a huge dominant one. Space between
    /// galaxies is filled with super-hot plasma and a large number of intergalactic stars. Contains 5 to 20+ major galaxies.
    Cluster(#[default = 8] u8, #[default = 1] u8),
}

impl Display for GalacticNeighborhoodDensity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GalacticNeighborhoodDensity::Void(m) => write!(f, "Void with {} major galaxies", m),
            GalacticNeighborhoodDensity::Group(m) => write!(f, "Group of {} major galaxies", m),
            GalacticNeighborhoodDensity::Cluster(m, d) => write!(
                f,
                "Cluster of {} major{}galaxies",
                m - d,
                if d > &0 {
                    format!(" and {} dominant ", d)
                } else {
                    String::from(" ")
                }
            ),
        }
    }
}

/// The type of a given galaxy.
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, SmartDefault, Serialize, Deserialize,
)]
pub enum GalaxyCategory {
    /// Loose cloud of gas and stars lost in the void between galaxies. The associated numbers are the length, width and height of this
    /// galaxy in parsecs.
    Intergalactic(
        #[default = 1000] u32,
        #[default = 3000] u32,
        #[default = 1000] u32,
    ),
    /// Dwarf galaxies that do not fit clearly into any other category. Protogalaxies began as irregular before growing into, merging with,
    /// or becoming satellites of larger galaxies. The associated numbers represent the length, width and height of this galaxy in parsecs.
    Irregular(
        #[default = 3000] u32,
        #[default = 3000] u32,
        #[default = 2000] u32,
    ),
    /// Disk galaxies with prominent arms rotating around a central bulge. Star formation is most active in the arms, where molecular clouds
    /// are densest. The associated numbers represent the radius and thickness of this galaxy in parsecs.
    #[default]
    Spiral(#[default = 10000] u32, #[default = 100] u32),
    /// Disk galaxies common to galaxy clusters. With the exception of dwarf lenticulars which are thick pure disk spirals, lenticulars have
    /// lost or used most of their star-making gas early in their history. The associated numbers represent the radius and thickness of this
    /// galaxy in parsecs.
    Lenticular(#[default = 10000] u32, #[default = 600] u32),
    /// Spherical or ovoid galaxies that lost or used most of their gas early on which renders their star formation activity minimal.
    /// The associated numbers represent the radius of this galaxy in parsecs.
    Elliptical(#[default = 10000] u32),
    /// Loose cloud of gas and stars between galaxies. The associated numbers are the length, width and height of this galaxy in parsecs.
    Intracluster(#[default = 1] u32, #[default = 3] u32, #[default = 1] u32),
    /// Found near the centres of rich galaxy clusters, dominant elliptical galaxies have grown to very large sizes by merging or eating
    /// other galaxies from their cluster. The associated numbers represent the radius of this galaxy in parsecs.
    DominantElliptical(#[default = 300000] u32),
}

impl Display for GalaxyCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GalaxyCategory::Intergalactic(l, w, h) => write!(
                f,
                "Intergalactic galaxy of size {}×{}×{}kpc",
                (*l as f32 / 1000.0),
                (*w as f32 / 1000.0),
                (*h as f32 / 1000.0)
            ),
            GalaxyCategory::Irregular(l, w, h) => write!(
                f,
                "Irregular galaxy of size {}×{}×{}kpc",
                (*l as f32 / 1000.0),
                (*w as f32 / 1000.0),
                (*h as f32 / 1000.0)
            ),
            GalaxyCategory::Spiral(r, t) => write!(
                f,
                "Spiral galaxy of radius {}kpc and thickness {}kpc",
                (*r as f32 / 1000.0),
                (*t as f32 / 1000.0)
            ),
            GalaxyCategory::Lenticular(r, t) => write!(
                f,
                "Lenticular galaxy of radius {}kpc and thickness {}kpc",
                (*r as f32 / 1000.0),
                (*t as f32 / 1000.0)
            ),
            GalaxyCategory::Elliptical(r) => {
                write!(f, "Elliptical galaxy of radius {}kpc", (*r as f32 / 1000.0))
            }
            GalaxyCategory::Intracluster(l, w, h) => write!(
                f,
                "Intracluster galaxy of size {}×{}×{}kpc",
                (*l as f32 / 1000.0),
                (*w as f32 / 1000.0),
                (*h as f32 / 1000.0)
            ),
            GalaxyCategory::DominantElliptical(r) => write!(
                f,
                "Dominant Elliptical galaxy of radius {}kpc",
                (*r as f32 / 1000.0)
            ),
        }
    }
}

/// The subtype of a given galaxy.
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, SmartDefault, Serialize, Deserialize,
)]
pub enum GalaxySubCategory {
    /// Small congregation of star clusters without discernable order.
    DwarfAmorphous,
    /// A galaxy that has a highly unconventional form. It could be because of a previous interaction with another galaxy that ended badly.
    Amorphous,
    /// Small galaxy that shows traits similar to those of a spiral galaxy but with higher star formation rates.
    DwarfSpiral,
    /// Flat spiral galaxy that have an almost non-existent galactic bulge.
    FlatSpiral,
    /// Spiral galaxy with gas-rich bar-shaped elongations of stars in the center.
    #[default]
    BarredSpiral,
    /// Spiral galaxy with a gas-poor elliptical bulge in the center.
    ClassicSpiral,
    /// Small galaxy that shows traits similar to those of a lenticular galaxy but with higher star formation rates.
    DwarfLenticular,
    /// Often more massive than spirals, with thicker disks and extensive halos of globular clusters, while being dustier than ellitpicals.
    CommonLenticular,
    /// A giant galaxy looking like a flattened elliptical.
    GiantLenticular,
    /// Small galaxy that shows traits similar to those of an elliptical galaxy but with higher star formation rates.
    DwarfElliptical,
    /// Very rare in the Early Stelliferous era, common elliptical galaxies usualy come from early intense starburst activity, or are the
    /// result of galaxy interactions. They tend to be at the center of galactic clusters, eating the other galaxies and growing to be the
    /// most massive galaxies.
    CommonElliptical,
    /// Very rare in the Early Stelliferous era, giant elliptical galaxies usualy come from early intense starburst activity, or are the
    /// result of galaxy interactions. They tend to be at the center of galactic clusters, eating the other galaxies and growing to be the
    /// most massive galaxies.
    GiantElliptical,
}

impl Display for GalaxySubCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GalaxySubCategory::DwarfAmorphous => write!(f, "Dwarf Amorphous"),
            GalaxySubCategory::Amorphous => write!(f, "Amorphous"),
            GalaxySubCategory::DwarfSpiral => write!(f, "Dwarf Spiral"),
            GalaxySubCategory::FlatSpiral => write!(f, "Flat Spiral"),
            GalaxySubCategory::BarredSpiral => write!(f, "Barred Spiral"),
            GalaxySubCategory::ClassicSpiral => write!(f, "Classic Spiral"),
            GalaxySubCategory::DwarfLenticular => write!(f, "Dwarf Lenticular"),
            GalaxySubCategory::CommonLenticular => write!(f, "Common Lenticular"),
            GalaxySubCategory::GiantLenticular => write!(f, "Giant Lenticular"),
            GalaxySubCategory::DwarfElliptical => write!(f, "Dwarf Elliptical"),
            GalaxySubCategory::CommonElliptical => write!(f, "Common Elliptical"),
            GalaxySubCategory::GiantElliptical => write!(f, "Giant Elliptical"),
        }
    }
}

/// Describes the noticeable variation in galactic satellites a galaxy might have.
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, SmartDefault, Serialize, Deserialize,
)]
pub enum GalaxySatellites {
    /// This galaxy has much more satellites than expected for its type.
    MuchMore,
    /// This galaxy has more satellites than expected for its type.
    #[default]
    More,
    /// This galaxy has less satellites than expected for its type.
    Less,
    /// This galaxy has much less satellites than expected for its type.
    MuchLess,
    /// This galaxy has no satellites whatsoever.
    None,
    /// This galaxy has one or multiple very special satellites.
    Special,
}

impl Display for GalaxySatellites {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GalaxySatellites::MuchMore => write!(f, "Much More Satellites"),
            GalaxySatellites::More => write!(f, "More Satellites"),
            GalaxySatellites::Less => write!(f, "Less Satellites"),
            GalaxySatellites::MuchLess => write!(f, "Much Less Satellites"),
            GalaxySatellites::None => write!(f, "No Satellites"),
            GalaxySatellites::Special => write!(f, "Special Satellites"),
        }
    }
}

/// Peculiarities a galaxy might have.
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, SmartDefault, Serialize, Deserialize,
)]
pub enum GalaxySpecialTrait {
    /// This galaxy has the exact traits that one might expect for a member of its type and subtype.
    #[default]
    NoPeculiarity,
    /// This galaxy has a massive central black hole that produces a much-higher-than-normal luminosity over at least some portion of the
    /// electromagnetic spectrum. Jets of plasma emit from the core that extend for thousands of parsecs.
    ActiveNucleus,
    /// This galaxy is made of two previous galaxies merging together and currently has two massive central black holes dancing together
    /// until they merge in a few hundred of millions of years. Both black holes emit greater than typical luminosity across much of the
    /// electromagnetic spectrum and emit jets of plasma thousands of parsecs long.
    DoubleNuclei,
    /// This galaxy's stars are much more densely packed than those in galaxies of similar type. The associated number represents the
    /// percentage of density this galaxy has when compared with a standard galaxy of the same type.
    Compact(#[default = 150] i8),
    /// This galaxy's stars are much less densely packed than those in galaxies of similar type. The associated number represents the
    /// percentage of density this galaxy has when compared with a standard galaxy of the same type.
    Expansive(#[default = 50] i8),
    /// This galaxy has a sizeable stellar population outside the core and disk/ellipse, often with large numbers of globular clusters.
    ExtendedHalo,
    /// This galaxy's stars have lower metallicity than expected in a standard galaxy of the same type.
    MetalPoor,
    /// This galaxy's stars have higher metallicity than expected in a standard galaxy of the same type.
    Dusty,
    /// This galaxy has much less gas than other galaxies, which gives it a slower star formation rate.
    GasPoor,
    /// This galaxy has much more gas than other galaxies, which gives it a much faster star formation rate.
    GasRich,
    /// This galaxy experiences very high star formation rates. It is ofter due to intaractions with one or more other galaxies.
    Starburst,
    /// This galaxy has runned out of the cold hydrogen gas needed to make stars earlier than it should have. It might be the result of an
    /// active nucleus that is heating the galaxy's gas or expelling it, or the gas being heated by other means.
    Dead,
    /// This galaxy has lost too much of its gas while interacting with other galaxies and is no longer able to produce new stars.
    Dormant,
    /// Satellites of galaxies are common, but this galaxy has a noticeable variation when compared to other ones.
    Satellites(GalaxySatellites),
    /// This galaxy is close enough one or multiple other galaxies for their gravity to distort each others. As a result, star formation
    /// rates and extragalactic star populations are greater than normal.
    Interacting,
    /// This galaxy interacted with another one "recently" and has some kind of tail of gas and stars as a result.
    Tail,
    /// This galaxy is not very old and has a larger population of hot bright stars.
    Younger,
    /// This galaxy is very old, which means that it has slowed or even stopped star formation long ago and has older stellar populations.
    Older,
    /// This galaxy is less massive than expected for one of its type. The associated number represents that difference as a percentage.
    SubSize(#[default = 50] i8),
    /// This galaxy is more massive than expected for one of its type. The associated number represents that difference as a percentage.
    SuperSize(#[default = 150] i8),
}

impl Display for GalaxySpecialTrait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GalaxySpecialTrait::NoPeculiarity => write!(f, "No Peculiarity"),
            GalaxySpecialTrait::ActiveNucleus => write!(f, "Active Nucleus"),
            GalaxySpecialTrait::DoubleNuclei => write!(f, "Double Nuclei"),
            GalaxySpecialTrait::Compact(d) => write!(f, "Compact ({}% density)", d),
            GalaxySpecialTrait::Expansive(d) => write!(f, "Expansive ({}% density)", d),
            GalaxySpecialTrait::ExtendedHalo => write!(f, "Extended Halo"),
            GalaxySpecialTrait::MetalPoor => write!(f, "Metal Poor"),
            GalaxySpecialTrait::Dusty => write!(f, "Dusty"),
            GalaxySpecialTrait::GasPoor => write!(f, "Gas Poor"),
            GalaxySpecialTrait::GasRich => write!(f, "Gas Rich"),
            GalaxySpecialTrait::Starburst => write!(f, "Starburst"),
            GalaxySpecialTrait::Dead => write!(f, "Dead"),
            GalaxySpecialTrait::Dormant => write!(f, "Dormant"),
            GalaxySpecialTrait::Satellites(s) => write!(f, "{}", s),
            GalaxySpecialTrait::Interacting => write!(f, "Interacting"),
            GalaxySpecialTrait::Tail => write!(f, "Tail"),
            GalaxySpecialTrait::Younger => write!(f, "Younger"),
            GalaxySpecialTrait::Older => write!(f, "Older"),
            GalaxySpecialTrait::SubSize(m) => write!(f, "Sub-Size ({}% mass)", m),
            GalaxySpecialTrait::SuperSize(m) => write!(f, "Super-Size ({}% mass)", m),
        }
    }
}

/// Data allowing us to model a galaxy Neighborhood (a section of the universe containing multiple galaxies).
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
pub struct GalacticNeighborhood {
    /// The universe this neighborhood belongs to.
    pub universe: Universe,
    /// How dense is this neighborhood, with the number of galaxies present.
    pub density: GalacticNeighborhoodDensity,
}

impl Display for GalacticNeighborhood {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Galactic {}", self.density)
    }
}

impl GalacticNeighborhood {
    /// Generates a brand new [GalacticNeighborhood] using the given seed and [GenerationSettings].
    pub fn generate(universe: Universe, seed: &String, settings: &GenerationSettings) -> Self {
        let density;

        if let Some(fixed_neighborhood) = settings.galaxy.fixed_neighborhood {
            density = fixed_neighborhood;
        } else if settings.galaxy.use_ours {
            density = GalacticNeighborhoodDensity::Group(2);
        } else {
            let mut rng = SeededDiceRoller::new(seed.as_str(), "gal_den");
            let is_group = rng.roll(1, 4, 0) != 4;

            if is_group {
                let galaxies = rng.roll(1, 6, -1) as u8;
                if galaxies == 0 {
                    density = GalacticNeighborhoodDensity::Void(rng.roll(1, 4, -1) as u8);
                } else {
                    density = GalacticNeighborhoodDensity::Group(galaxies);
                }
            } else {
                let mut galaxies = 0 as u8;
                let mut dominant = 0;
                let mut roll = 0;
                let mut turn = 0;

                while roll == 10 || turn < 2 {
                    roll = rng.roll(1, 10, 0) as u8;
                    galaxies += if roll == 1 { 0 } else { roll };
                    dominant += if roll == 1 { 1 } else { 0 };
                    turn += 1;
                }

                density = GalacticNeighborhoodDensity::Cluster(galaxies, dominant);
            }
        };

        Self { universe, density }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_a_galactic_neighborhood() {
        for i in 0..10000 {
            let settings = GenerationSettings {
                ..Default::default()
            };
            let seed = String::from(&i.to_string());
            let neighborhood = GalacticNeighborhood::generate(
                Universe::generate(&seed, &settings),
                &seed,
                &settings,
            );
            match neighborhood.density {
                GalacticNeighborhoodDensity::Void(galaxies) => assert!(galaxies < 4),
                GalacticNeighborhoodDensity::Group(galaxies) => {
                    assert!(galaxies > 0 && galaxies < 6)
                }
                GalacticNeighborhoodDensity::Cluster(galaxies, dominant) => {
                    assert!(galaxies > 0 || dominant > 0)
                }
            }
        }
    }

    #[test]
    fn generate_our_galactic_neighborhood() {
        for i in 0..100 {
            let settings = GenerationSettings {
                universe: UniverseSettings {
                    use_ours: true,
                    ..Default::default()
                },
                galaxy: GalaxySettings {
                    use_ours: true,
                    ..Default::default()
                },
                ..Default::default()
            };
            let seed = String::from(&i.to_string());
            let neighborhood = GalacticNeighborhood::generate(
                Universe::generate(&seed, &settings),
                &seed,
                &settings,
            );
            assert_eq!(neighborhood.density, GalacticNeighborhoodDensity::Group(2));
        }
    }

    #[test]
    fn generate_a_galactic_neighborhood_with_specific_density() {
        for i in 0..1000 {
            let mut rng = SeededDiceRoller::new(&String::from(i.to_string()), "t");
            let fixed_neighborhood = rng
                .get_result(&CopyableRollToProcess {
                    possible_results: SeededDiceRoller::to_copyable_possible_results(vec![
                        GalacticNeighborhoodDensity::Void(0),
                        GalacticNeighborhoodDensity::Void(1),
                        GalacticNeighborhoodDensity::Void(2),
                        GalacticNeighborhoodDensity::Void(3),
                        GalacticNeighborhoodDensity::Group(1),
                        GalacticNeighborhoodDensity::Group(2),
                        GalacticNeighborhoodDensity::Group(3),
                        GalacticNeighborhoodDensity::Group(4),
                        GalacticNeighborhoodDensity::Group(5),
                        GalacticNeighborhoodDensity::Cluster(1, 0),
                        GalacticNeighborhoodDensity::Cluster(2, 0),
                        GalacticNeighborhoodDensity::Cluster(3, 0),
                        GalacticNeighborhoodDensity::Cluster(4, 0),
                        GalacticNeighborhoodDensity::Cluster(5, 0),
                        GalacticNeighborhoodDensity::Cluster(6, 0),
                        GalacticNeighborhoodDensity::Cluster(7, 0),
                        GalacticNeighborhoodDensity::Cluster(1, 1),
                        GalacticNeighborhoodDensity::Cluster(2, 1),
                        GalacticNeighborhoodDensity::Cluster(3, 1),
                    ]),
                    roll_method: RollMethod::SimpleRoll,
                })
                .expect("Should return a density as result");

            let settings = &GenerationSettings {
                galaxy: GalaxySettings {
                    fixed_neighborhood: Some(fixed_neighborhood),
                    ..Default::default()
                },
                ..Default::default()
            };
            let seed = String::from(&i.to_string());
            let neighborhood = GalacticNeighborhood::generate(
                Universe::generate(&seed, &settings),
                &seed,
                &settings,
            );
            assert_eq!(neighborhood.density, fixed_neighborhood);
        }
    }
}
