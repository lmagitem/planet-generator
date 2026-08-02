use super::types::{GalacticRegionMap, RegionGrid};
use crate::diagnostics::types::{Diagnostics, GenerationWarning};
use crate::galaxy::constants::REGION_GRID_RESOLUTION;
use crate::galaxy::map::hex::types::SpaceCoordinates;
use crate::galaxy::map::types::GalacticRegion;
use log::warn;

/// The squared RGB distance under which a pixel is snapped to the nearest palette color.
const MAX_PALETTE_SNAP_SQUARED_DISTANCE: i32 = 32 * 32;

/// The regions one can paint in an SVG galactic region map; [`GalacticRegion::Multiple`] is not paintable.
pub(crate) const PAINTABLE_REGIONS: [GalacticRegion; 15] = [
    GalacticRegion::Core,
    GalacticRegion::Nucleus,
    GalacticRegion::Bulge,
    GalacticRegion::Bar,
    GalacticRegion::Arm,
    GalacticRegion::Disk,
    GalacticRegion::Ellipse,
    GalacticRegion::Halo,
    GalacticRegion::Aura,
    GalacticRegion::Void,
    GalacticRegion::GlobularCluster,
    GalacticRegion::OpenCluster,
    GalacticRegion::Association,
    GalacticRegion::Stream,
    GalacticRegion::Exile,
];

impl GalacticRegionMap {
    /// Builds a [`GalacticRegionMap`] by rasterizing the given SVG layers (top/max z first) over the given
    /// galaxy bounds in parsecs. Returns [None] if no layer is given or if any layer fails to parse.
    /// Diagnostics raised along the way are discarded; use [`GalacticRegionMap::build_with_diagnostics`]
    /// to collect them.
    #[must_use]
    pub fn build(
        svgs: &[String],
        start: SpaceCoordinates,
        end: SpaceCoordinates,
    ) -> Option<GalacticRegionMap> {
        Self::build_with_diagnostics(svgs, start, end, &mut Diagnostics::default())
    }

    /// Same as [`GalacticRegionMap::build`], but also reports into the given [`Diagnostics`]:
    /// [`GenerationWarning::SvgLayerParseFailed`] (with the offending layer's index) when a layer
    /// fails to parse, and [`GenerationWarning::SvgPixelsUnmatched`] (with the per-layer unmatched
    /// pixel count) when pixels match no region color.
    #[must_use]
    pub fn build_with_diagnostics(
        svgs: &[String],
        start: SpaceCoordinates,
        end: SpaceCoordinates,
        diagnostics: &mut Diagnostics,
    ) -> Option<GalacticRegionMap> {
        if svgs.is_empty() {
            return None;
        }

        let mut layers = Vec::with_capacity(svgs.len());
        for (layer_index, svg) in svgs.iter().enumerate() {
            let tree = match resvg::usvg::Tree::from_str(svg, &resvg::usvg::Options::default()) {
                Ok(tree) => tree,
                Err(error) => {
                    warn!("Failed to parse SVG galactic map layer {layer_index}: {error}");
                    diagnostics
                        .push_warning(GenerationWarning::SvgLayerParseFailed { layer_index });
                    return None;
                }
            };
            layers.push(rasterize_layer(&tree, layer_index, diagnostics)?);
        }
        Some(GalacticRegionMap { layers, start, end })
    }
}

/// Rasterizes one parsed SVG layer into a [`RegionGrid`], stretching it (non-uniformly if needed) so that its
/// edges map exactly to the galaxy bounds.
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
fn rasterize_layer(
    tree: &resvg::usvg::Tree,
    layer_index: usize,
    diagnostics: &mut Diagnostics,
) -> Option<RegionGrid> {
    let size = tree.size();
    let transform = resvg::tiny_skia::Transform::from_scale(
        REGION_GRID_RESOLUTION as f32 / size.width(),
        REGION_GRID_RESOLUTION as f32 / size.height(),
    );
    // A 256x256 pixmap allocation can only fail under extreme memory pressure; give up on
    // the map (the caller falls back to defaults) instead of panicking.
    let Some(mut pixmap) =
        resvg::tiny_skia::Pixmap::new(REGION_GRID_RESOLUTION as u32, REGION_GRID_RESOLUTION as u32)
    else {
        warn!(
            "Failed to allocate the rasterization pixmap for SVG galactic map layer {layer_index}."
        );
        return None;
    };
    resvg::render(tree, transform, &mut pixmap.as_mut());

    let mut cells = Vec::with_capacity(REGION_GRID_RESOLUTION * REGION_GRID_RESOLUTION);
    let mut unmatched_pixels = 0;
    for pixel in pixmap.pixels() {
        // tiny-skia pixels are premultiplied: demultiply before classifying, or a half-opacity Arm blue
        // would read as Association navy.
        let color = pixel.demultiply();
        if color.alpha() < 128 {
            cells.push(GalacticRegion::Void);
        } else if let Some(region) =
            GalacticRegion::from_rgb(color.red(), color.green(), color.blue())
        {
            cells.push(region);
        } else if let Some(region) =
            nearest_palette_region(color.red(), color.green(), color.blue())
        {
            cells.push(region);
        } else {
            unmatched_pixels += 1;
            cells.push(GalacticRegion::Void);
        }
    }
    if unmatched_pixels > 0 {
        warn!(
            "{unmatched_pixels} pixels of SVG galactic map layer {layer_index} matched no region color; use the exact palette documented on GalacticRegion."
        );
        diagnostics.push_warning(GenerationWarning::SvgPixelsUnmatched {
            layer_index,
            count: unmatched_pixels,
        });
    }
    Some(RegionGrid { cells })
}

/// Returns the paintable region whose palette color is the nearest to the given RGB color, if it lies
/// within snapping distance. The tolerance can never tie between two palette colors.
fn nearest_palette_region(r: u8, g: u8, b: u8) -> Option<GalacticRegion> {
    let mut nearest: Option<(i32, GalacticRegion)> = None;
    for region in PAINTABLE_REGIONS {
        let (palette_r, palette_g, palette_b) = region.to_rgb();
        let squared_distance = (i32::from(r) - i32::from(palette_r)).pow(2)
            + (i32::from(g) - i32::from(palette_g)).pow(2)
            + (i32::from(b) - i32::from(palette_b)).pow(2);
        if squared_distance <= MAX_PALETTE_SNAP_SQUARED_DISTANCE
            && nearest.is_none_or(|(best, _)| squared_distance < best)
        {
            nearest = Some((squared_distance, region));
        }
    }
    nearest.map(|(_, region)| region)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Left half Arm blue, right half unfilled (Void).
    const LEFT_HALF_ARM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100"><rect x="0" y="0" width="50" height="100" fill="#0000FF"/></svg>"##;
    /// Top half Bar red, bottom half unfilled (Void).
    const TOP_HALF_BAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100"><rect x="0" y="0" width="100" height="50" fill="#FF0000"/></svg>"##;
    const FULL_BAR_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100"><rect x="0" y="0" width="100" height="100" fill="#FF0000"/></svg>"##;
    const FULL_ARM_SVG: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100"><rect x="0" y="0" width="100" height="100" fill="#0000FF"/></svg>"##;

    fn flat_bounds() -> (SpaceCoordinates, SpaceCoordinates) {
        (
            SpaceCoordinates::new(-50, -50, 0),
            SpaceCoordinates::new(50, 50, 0),
        )
    }

    fn spanned_bounds() -> (SpaceCoordinates, SpaceCoordinates) {
        (
            SpaceCoordinates::new(-50, -50, -50),
            SpaceCoordinates::new(50, 50, 50),
        )
    }

    #[test]
    fn build_returns_none_on_empty_input() {
        let (start, end) = flat_bounds();
        assert_eq!(GalacticRegionMap::build(&[], start, end), None);
    }

    #[test]
    fn build_returns_none_on_unparseable_svg() {
        let (start, end) = flat_bounds();
        assert_eq!(
            GalacticRegionMap::build(&["not_an_svg".to_string()], start, end),
            None
        );
        assert_eq!(
            GalacticRegionMap::build(&["<svg><rect".to_string()], start, end),
            None
        );
    }

    #[test]
    fn build_returns_none_if_any_layer_is_malformed() {
        let (start, end) = flat_bounds();
        let svgs = vec![FULL_ARM_SVG.to_string(), "not_an_svg".to_string()];
        assert_eq!(GalacticRegionMap::build(&svgs, start, end), None);
    }

    #[test]
    fn single_layer_left_arm_right_void() {
        let (start, end) = flat_bounds();
        let map = GalacticRegionMap::build(&[LEFT_HALF_ARM_SVG.to_string()], start, end)
            .expect("The SVG should parse.");
        assert_eq!(map.layers.len(), 1);
        // x = -25 -> u_x = 0.25 -> column 64, inside the filled left half.
        assert_eq!(
            map.region_at(SpaceCoordinates::new(-25, 0, 0)),
            GalacticRegion::Arm
        );
        // x = 25 -> u_x = 0.75 -> column 192, inside the unfilled right half.
        assert_eq!(
            map.region_at(SpaceCoordinates::new(25, 0, 0)),
            GalacticRegion::Void
        );
        // Min x -> column 0, max x -> last column (u = 1 clamps onto it).
        assert_eq!(
            map.region_at(SpaceCoordinates::new(-50, 0, 0)),
            GalacticRegion::Arm
        );
        assert_eq!(
            map.region_at(SpaceCoordinates::new(50, 0, 0)),
            GalacticRegion::Void
        );
    }

    #[test]
    fn y_axis_is_flipped() {
        let (start, end) = flat_bounds();
        let map = GalacticRegionMap::build(&[TOP_HALF_BAR_SVG.to_string()], start, end)
            .expect("The SVG should parse.");
        // The SVG top half maps to the maximum galactic y, the bottom half to the minimum.
        assert_eq!(
            map.region_at(SpaceCoordinates::new(0, 50, 0)),
            GalacticRegion::Bar
        );
        assert_eq!(
            map.region_at(SpaceCoordinates::new(0, 25, 0)),
            GalacticRegion::Bar
        );
        assert_eq!(
            map.region_at(SpaceCoordinates::new(0, -25, 0)),
            GalacticRegion::Void
        );
        assert_eq!(
            map.region_at(SpaceCoordinates::new(0, -50, 0)),
            GalacticRegion::Void
        );
    }

    #[test]
    fn out_of_bounds_coords_clamp_to_edge_cells() {
        let (start, end) = flat_bounds();
        let map = GalacticRegionMap::build(&[FULL_ARM_SVG.to_string()], start, end)
            .expect("The SVG should parse.");
        for coord in [
            SpaceCoordinates::new(500, 0, 0),
            SpaceCoordinates::new(-500, 0, 0),
            SpaceCoordinates::new(0, 500, 0),
            SpaceCoordinates::new(0, -500, 0),
            SpaceCoordinates::new(5000, 5000, 0),
        ] {
            assert_eq!(map.region_at(coord), GalacticRegion::Arm);
        }
    }

    #[test]
    fn multiple_layers_select_nearest_by_z() {
        let svgs = vec![FULL_BAR_SVG.to_string(), FULL_ARM_SVG.to_string()];
        let (start, end) = spanned_bounds();
        let map = GalacticRegionMap::build(&svgs, start, end).expect("The SVGs should parse.");
        assert_eq!(map.layers.len(), 2);
        // z = 50 -> u_z = 1 -> layer 0 (the top one).
        assert_eq!(
            map.region_at(SpaceCoordinates::new(0, 0, 50)),
            GalacticRegion::Bar
        );
        // z = -50 -> u_z = 0 -> layer 1 (the bottom one).
        assert_eq!(
            map.region_at(SpaceCoordinates::new(0, 0, -50)),
            GalacticRegion::Arm
        );
        // z = 0 -> u_z = 0.5 -> layer position 0.5 -> round is half away from zero -> bottom layer.
        assert_eq!(
            map.region_at(SpaceCoordinates::new(0, 0, 0)),
            GalacticRegion::Arm
        );

        // Same layers on a flat map (zero z span): u_z = 0.5 -> always the bottom layer.
        let (flat_start, flat_end) = flat_bounds();
        let map =
            GalacticRegionMap::build(&svgs, flat_start, flat_end).expect("The SVGs should parse.");
        assert_eq!(
            map.region_at(SpaceCoordinates::new(0, 0, 0)),
            GalacticRegion::Arm
        );
    }

    #[test]
    fn antialiased_or_blended_pixels_classify_sanely() {
        // A half-opacity Arm blue rect over an empty background: premultiplied it reads as Association
        // navy, demultiplying recovers the original blue. The off-palette color matches nothing.
        let svg = r##"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100"><rect x="0" y="0" width="50" height="100" fill="#0000FF" fill-opacity="0.5"/><rect x="50" y="0" width="50" height="100" fill="#123456"/></svg>"##;
        let (start, end) = flat_bounds();
        let map = GalacticRegionMap::build(&[svg.to_string()], start, end)
            .expect("The SVG should parse.");
        assert_eq!(
            map.region_at(SpaceCoordinates::new(-25, 0, 0)),
            GalacticRegion::Arm
        );
        assert_eq!(
            map.region_at(SpaceCoordinates::new(25, 0, 0)),
            GalacticRegion::Void
        );
    }

    #[test]
    fn near_palette_pixels_snap_within_tolerance() {
        // #0000E0 is 31 units away from Arm blue (squared distance 961 <= 1024) -> snapped to Arm.
        // #0000DD is 34 units away (squared distance 1156 > 1024) -> no match -> Void.
        let svg = r##"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100"><rect x="0" y="0" width="50" height="100" fill="#0000E0"/><rect x="50" y="0" width="50" height="100" fill="#0000DD"/></svg>"##;
        let (start, end) = flat_bounds();
        let map = GalacticRegionMap::build(&[svg.to_string()], start, end)
            .expect("The SVG should parse.");
        assert_eq!(
            map.region_at(SpaceCoordinates::new(-25, 0, 0)),
            GalacticRegion::Arm
        );
        assert_eq!(
            map.region_at(SpaceCoordinates::new(25, 0, 0)),
            GalacticRegion::Void
        );
    }

    #[test]
    fn overlapping_shapes_use_painters_order() {
        // The full-map Arm rect is drawn first, the right-half Bar rect second: the later shape wins.
        let svg = r##"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100"><rect x="0" y="0" width="100" height="100" fill="#0000FF"/><rect x="50" y="0" width="50" height="100" fill="#FF0000"/></svg>"##;
        let (start, end) = flat_bounds();
        let map = GalacticRegionMap::build(&[svg.to_string()], start, end)
            .expect("The SVG should parse.");
        assert_eq!(
            map.region_at(SpaceCoordinates::new(-25, 0, 0)),
            GalacticRegion::Arm
        );
        assert_eq!(
            map.region_at(SpaceCoordinates::new(25, 0, 0)),
            GalacticRegion::Bar
        );
    }

    /// Golden hashes of the rasterized default maps: any drift in resvg/tiny-skia rendering
    /// (or an accidental edit of the default SVGs) breaks this test. FNV-1a over the grid cells.
    #[test]
    fn default_maps_rasterize_deterministically() {
        use crate::galaxy::constants::*;
        fn fnv1a(cells: &[GalacticRegion]) -> u64 {
            cells
                .iter()
                .fold(0xcbf2_9ce4_8422_2325_u64, |hash, region| {
                    (hash ^ u64::from(*region as u8)).wrapping_mul(0x0000_0100_0000_01b3)
                })
        }
        let (start, end) = (
            SpaceCoordinates::new(-10000, -10000, -50),
            SpaceCoordinates::new(10000, 10000, 50),
        );
        for (svg, expected) in [
            (DEFAULT_REGION_MAP_SPIRAL, 0x59e5_de4b_49c6_ac3e_u64),
            (DEFAULT_REGION_MAP_LENTICULAR, 0x595b_50f1_130d_3dd6_u64),
            (DEFAULT_REGION_MAP_ELLIPTICAL, 0x2002_8bef_f7b0_accd_u64),
            (DEFAULT_REGION_MAP_IRREGULAR, 0x9b9e_3741_b5a7_ed80_u64),
        ] {
            let map = GalacticRegionMap::build(&[svg.to_string()], start, end)
                .expect("The default SVG should parse.");
            let hash = fnv1a(&map.layers[0].cells);
            assert_eq!(hash, expected);
        }
    }

    #[test]
    fn build_works_without_width_height_using_viewbox() {
        // usvg falls back to the viewBox size: the 200x50 viewBox is stretched non-uniformly onto the
        // 256x256 grid, so the left half of the viewBox still maps to the left half of the galaxy.
        let svg = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 200 50"><rect x="0" y="0" width="100" height="50" fill="#0000FF"/></svg>"##;
        let (start, end) = flat_bounds();
        let map = GalacticRegionMap::build(&[svg.to_string()], start, end)
            .expect("The SVG should parse.");
        assert_eq!(
            map.region_at(SpaceCoordinates::new(-25, 0, 0)),
            GalacticRegion::Arm
        );
        assert_eq!(
            map.region_at(SpaceCoordinates::new(25, 0, 0)),
            GalacticRegion::Void
        );
    }
}
