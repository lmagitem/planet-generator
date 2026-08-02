use crate::galaxy::map::hex::types::SpaceCoordinates;
use crate::galaxy::map::types::GalacticRegion;

/// A rasterized, queryable map of [`GalacticRegion`]s built from SVG layers stretched over a galaxy's bounds.
#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct GalacticRegionMap {
    /// Rasterized layers, top (max z) first, one per input SVG.
    pub(super) layers: Vec<RegionGrid>,
    /// The galactic start (minimum bounds) in parsecs captured at build time.
    pub(super) start: SpaceCoordinates,
    /// The galactic end (maximum bounds) in parsecs captured at build time.
    pub(super) end: SpaceCoordinates,
}

/// One rasterized SVG layer of a [`GalacticRegionMap`].
#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub(super) struct RegionGrid {
    /// The row-major cells of this layer (`REGION_GRID_RESOLUTION`² in size); row 0 = max galactic y.
    pub(super) cells: Vec<GalacticRegion>,
}
