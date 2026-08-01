use crate::galaxy::map::constants::REGION_GRID_RESOLUTION;
use crate::internal::*;
use crate::prelude::*;
pub mod generator;
pub mod types;
use types::GalacticRegionMap;

impl GalacticRegionMap {
    /// Returns the [`GalacticRegion`] at the given parsec coordinates.
    ///
    /// Coordinates are normalized against the galaxy bounds captured at build time: out-of-bounds
    /// coordinates are clamped onto the edge cells (there is no Void path), a zero-length axis maps to the
    /// middle of the grid, the SVG top row corresponds to the maximum galactic y, and layers are ordered
    /// from the maximum galactic z downwards.
    #[must_use]
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    pub fn region_at(&self, coord: SpaceCoordinates) -> GalacticRegion {
        if self.layers.is_empty() {
            return GalacticRegion::Void;
        }
        let u_x = axis_position(self.start.x, self.end.x, coord.x);
        let u_y = axis_position(self.start.y, self.end.y, coord.y);
        let u_z = axis_position(self.start.z, self.end.z, coord.z);

        let resolution = REGION_GRID_RESOLUTION as f64;
        let last_cell = (REGION_GRID_RESOLUTION - 1) as f64;
        let col = (u_x * resolution).floor().min(last_cell) as usize;
        let row = ((1.0 - u_y) * resolution).floor().min(last_cell) as usize;
        // layers[0] = top = max z; f64::round is half away from zero.
        let layer = ((1.0 - u_z) * (self.layers.len() - 1) as f64).round() as usize;
        self.layers[layer].cells[row * REGION_GRID_RESOLUTION + col]
    }
}

/// Normalizes `coord_axis` within `[start_axis, end_axis]` into `[0.0, 1.0]`, clamping overshoot onto the
/// edges; a zero-length span maps to the middle of the axis.
#[allow(clippy::cast_precision_loss)]
fn axis_position(start_axis: i64, end_axis: i64, coord_axis: i64) -> f64 {
    let span = end_axis - start_axis;
    if span == 0 {
        0.5
    } else {
        ((coord_axis - start_axis) as f64 / span as f64).clamp(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::types::RegionGrid;
    use super::*;

    fn grid_filled_with(region: GalacticRegion) -> RegionGrid {
        RegionGrid {
            cells: vec![region; REGION_GRID_RESOLUTION * REGION_GRID_RESOLUTION],
        }
    }

    #[test]
    fn zero_span_axes_map_to_middle_cells() {
        let mut grid = grid_filled_with(GalacticRegion::Void);
        // u = 0.5 on a zero-span axis -> cell 128 on both x and y.
        grid.cells[128 * REGION_GRID_RESOLUTION + 128] = GalacticRegion::Arm;
        let map = GalacticRegionMap {
            layers: vec![grid],
            start: SpaceCoordinates::new(0, 0, 0),
            end: SpaceCoordinates::new(0, 0, 0),
        };
        assert_eq!(
            map.region_at(SpaceCoordinates::new(0, 0, 0)),
            GalacticRegion::Arm
        );
        // Any coordinate stays on the middle cells since every span is zero.
        assert_eq!(
            map.region_at(SpaceCoordinates::new(-999, 999, 42)),
            GalacticRegion::Arm
        );
    }

    #[test]
    fn lookup_maps_svg_top_row_to_max_y_and_clamps_edges() {
        let mut grid = grid_filled_with(GalacticRegion::Void);
        grid.cells[0] = GalacticRegion::Bar; // row 0, col 0
        grid.cells[REGION_GRID_RESOLUTION * REGION_GRID_RESOLUTION - 1] = GalacticRegion::Aura; // row 255, col 255
        let map = GalacticRegionMap {
            layers: vec![grid],
            start: SpaceCoordinates::new(-50, -50, 0),
            end: SpaceCoordinates::new(50, 50, 0),
        };
        // Min x / max y -> top-left cell.
        assert_eq!(
            map.region_at(SpaceCoordinates::new(-50, 50, 0)),
            GalacticRegion::Bar
        );
        // Max x / min y -> bottom-right cell; u = 1 clamps onto the last cell instead of overflowing.
        assert_eq!(
            map.region_at(SpaceCoordinates::new(50, -50, 0)),
            GalacticRegion::Aura
        );
    }

    #[test]
    fn two_layers_select_nearest_layer_by_z() {
        let map = GalacticRegionMap {
            layers: vec![
                grid_filled_with(GalacticRegion::Bar), // top: max z
                grid_filled_with(GalacticRegion::Arm), // bottom: min z
            ],
            start: SpaceCoordinates::new(-50, -50, -50),
            end: SpaceCoordinates::new(50, 50, 50),
        };
        // z = 50 -> u_z = 1 -> layer position 0 -> top layer.
        assert_eq!(
            map.region_at(SpaceCoordinates::new(0, 0, 50)),
            GalacticRegion::Bar
        );
        // z = -50 -> u_z = 0 -> layer position 1 -> bottom layer.
        assert_eq!(
            map.region_at(SpaceCoordinates::new(0, 0, -50)),
            GalacticRegion::Arm
        );
        // z = 0 -> u_z = 0.5 -> layer position 0.5 -> round is half away from zero -> bottom layer.
        assert_eq!(
            map.region_at(SpaceCoordinates::new(0, 0, 0)),
            GalacticRegion::Arm
        );

        // Flat map (zero z span): u_z = 0.5 -> always the bottom layer with two layers.
        let flat_map = GalacticRegionMap {
            layers: vec![
                grid_filled_with(GalacticRegion::Bar),
                grid_filled_with(GalacticRegion::Arm),
            ],
            start: SpaceCoordinates::new(-50, -50, 0),
            end: SpaceCoordinates::new(50, 50, 0),
        };
        assert_eq!(
            flat_map.region_at(SpaceCoordinates::new(0, 0, 0)),
            GalacticRegion::Arm
        );
    }

    #[test]
    fn region_at_on_empty_map_returns_void() {
        // A default-constructed map has no layers; querying it must not panic.
        let map = GalacticRegionMap::default();
        assert_eq!(
            map.region_at(SpaceCoordinates::new(0, 0, 0)),
            GalacticRegion::Void
        );
    }

    #[test]
    fn three_layers_select_middle_layer_at_mid_z() {
        let map = GalacticRegionMap {
            layers: vec![
                grid_filled_with(GalacticRegion::Bar),
                grid_filled_with(GalacticRegion::Aura),
                grid_filled_with(GalacticRegion::Arm),
            ],
            start: SpaceCoordinates::new(-50, -50, -50),
            end: SpaceCoordinates::new(50, 50, 50),
        };
        assert_eq!(
            map.region_at(SpaceCoordinates::new(0, 0, 50)),
            GalacticRegion::Bar
        );
        // z = 0 -> u_z = 0.5 -> layer position 1.0 -> the middle layer.
        assert_eq!(
            map.region_at(SpaceCoordinates::new(0, 0, 0)),
            GalacticRegion::Aura
        );
        // z = 25 -> u_z = 0.75 -> layer position 0.5 -> round half away from zero -> middle layer.
        assert_eq!(
            map.region_at(SpaceCoordinates::new(0, 0, 25)),
            GalacticRegion::Aura
        );
        assert_eq!(
            map.region_at(SpaceCoordinates::new(0, 0, -50)),
            GalacticRegion::Arm
        );
    }
}
