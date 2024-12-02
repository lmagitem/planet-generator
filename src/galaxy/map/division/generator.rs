use crate::internal::*;
use crate::prelude::*;
use std::collections::HashMap;
use usvg::tiny_skia_path::PathSegment;
use usvg::Node::Path;
use usvg::{Options, Paint, Tree};

impl GalacticMapDivision {
    pub fn generate(
        index: SpaceCoordinates,
        level: u8,
        parent_division_level: &GalacticMapDivisionLevel,
        galaxy: &Galaxy,
    ) -> Self {
        let mut division = Self {
            name: "GalaxyDivision".into(),
            region: GalacticRegion::Multiple,
            level,
            x: (index.x % parent_division_level.x_subdivisions as i64) as u8,
            y: (index.y % parent_division_level.y_subdivisions as i64) as u8,
            z: (index.z % parent_division_level.z_subdivisions as i64) as u8,
            index,
            size: SpaceCoordinates::new(-1, -1, -1),
        };
        division.region = get_region(&mut division, galaxy);
        division
    }
}

fn get_region(division: &mut GalacticMapDivision, galaxy: &Galaxy) -> GalacticRegion {
    let start = division.get_top_left_up(galaxy);
    let size = division.get_size(galaxy);
    let half_size = size / SpaceCoordinates::new(2, 2, 2);
    let mut region_count = HashMap::new();

    for xi in 0..3 {
        let x = if xi == 0 {
            start.x
        } else if xi == 1 {
            half_size.x
        } else {
            size.x
        };
        for yi in 0..3 {
            let y = if yi == 0 {
                start.y
            } else if yi == 1 {
                half_size.y
            } else {
                size.y
            };
            for zi in 0..3 {
                let z = if zi == 0 {
                    start.z
                } else if zi == 1 {
                    half_size.z
                } else {
                    size.z
                };

                // Finds which region this point belongs to and remembers it
                let point_region = generate_region(SpaceCoordinates::new(x, y, z), galaxy);
                *region_count.entry(point_region).or_insert(0) += 1;
            }
        }
    }

    // If there was only one region in the whole division, set that division's region to it, otherwise use Multiple.
    if region_count.len() == 1 {
        let (region, _) = region_count.into_iter().next().unwrap();
        region
    } else {
        GalacticRegion::Multiple
    }
}

fn generate_region(coord: SpaceCoordinates, galaxy: &Galaxy) -> GalacticRegion {
    if galaxy.galactic_map_layers.is_empty() {
        return GalacticRegion::Void;
    }

    let z = coord.z;
    let layer_count = galaxy.galactic_map_layers.len();
    let layer_index = if layer_count == 1 {
        0
    } else {
        ((z as f64 / galaxy.get_galaxy_size().z as f64) * (layer_count as f64 - 1.0)).round()
            as usize
    };

    let svg_data = &galaxy.galactic_map_layers[layer_index];
    let opt = Options::default();
    let tree = Tree::from_str(svg_data, &opt).unwrap();

    let mut color_map = HashMap::new();
    for node in tree.root().children() {
        if let Path(ref path) = *node {
            if let Some(ref fill) = path.fill() {
                if let Paint::Color(color) = fill.paint() {
                    let usvg::Color { red, green, blue } = color;
                    let color_string = format!("{:02X}{:02X}{:02X}", red, green, blue);

                    // Convert path data to a serializable format
                    let path_segments: Vec<(i32, i32)> = path
                        .data()
                        .segments()
                        .filter_map(|segment| match segment {
                            PathSegment::MoveTo(point) | PathSegment::LineTo(point) => {
                                Some((point.x.round() as i32, point.y.round() as i32))
                            }
                            _ => None,
                        })
                        .collect();

                    // Convert path segments to a key that implements `Eq` and `Hash`
                    let key = format!("{:?}", path_segments);
                    color_map.insert(key, color_string);
                }
            }
        }
    }

    let x = coord.x as f64;
    let y = coord.y as f64;
    let mut closest_color = "000000".to_string();
    let mut min_distance = f64::MAX;

    for (path_segments_key, color) in color_map {
        // Deserialize the `path_segments_key` back into a vector of `(i32, i32)`
        let path_segments: Vec<(i32, i32)> =
            serde_json::from_str(&path_segments_key).unwrap_or_else(|_| Vec::new());

        for (x1, y1) in path_segments {
            let distance = ((x - x1 as f64).powi(2) + (y - y1 as f64).powi(2)).sqrt();
            if distance < min_distance {
                min_distance = distance;
                closest_color = color.clone();
            }
        }
    }

    let region = match closest_color.as_str() {
        "000000" => GalacticRegion::Void,
        "111111" => GalacticRegion::Aura,
        "222222" => GalacticRegion::Exile,
        "333333" => GalacticRegion::Stream,
        "444444" => GalacticRegion::Association,
        "555555" => GalacticRegion::Halo,
        "666666" => GalacticRegion::GlobularCluster,
        "777777" => GalacticRegion::OpenCluster,
        "888888" => GalacticRegion::Disk,
        "999999" => GalacticRegion::Ellipse,
        "AAAAAA" => GalacticRegion::Multiple,
        "BBBBBB" => GalacticRegion::Arm,
        "CCCCCC" => GalacticRegion::Bar,
        "DDDDDD" => GalacticRegion::Bulge,
        "EEEEEE" => GalacticRegion::Core,
        "FFFFFF" => GalacticRegion::Nucleus,
        _ => GalacticRegion::Multiple,
    };

    region
}

#[cfg(test)]
mod tests {
    use super::*;
    use usvg::Tree;

    #[test]
    fn test_generate_region_with_default_svg() {
        let galaxy = Galaxy {
            galactic_map_layers: vec![r##"
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
        <rect x="0" y="0" width="100" height="100" fill="#000000" />
    </svg>
"##
            .to_string()],
            ..Default::default()
        };

        let coord = SpaceCoordinates::new(50, 50, 0);
        let region = generate_region(coord, &galaxy);
        assert_eq!(region, GalacticRegion::Void);
    }

    #[test]
    fn test_generate_region_with_multiple_svg_layers() {
        let galaxy = Galaxy {
            galactic_map_layers: vec![
                r##"
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
                    <rect x="0" y="0" width="100" height="100" fill="#000000" />
                </svg>
                "##
                .to_string(),
                r##"
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
                    <rect x="0" y="0" width="100" height="100" fill="#FFFFFF" />
                </svg>
                "##
                .to_string(),
            ],
            ..Default::default()
        };

        let coord = SpaceCoordinates::new(50, 50, 0);
        let region = generate_region(coord, &galaxy);
        assert_eq!(region, GalacticRegion::Void);

        let coord = SpaceCoordinates::new(50, 50, 100);
        let region = generate_region(coord, &galaxy);
        assert_eq!(region, GalacticRegion::Nucleus);
    }

    #[test]
    fn test_generate_region_with_interpolation() {
        let galaxy = Galaxy {
            galactic_map_layers: vec![
                r##"
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
                    <rect x="0" y="0" width="100" height="100" fill="#000000" />
                </svg>
                "##
                .to_string(),
                r##"
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
                    <rect x="0" y="0" width="100" height="100" fill="#FFFFFF" />
                </svg>
                "##
                .to_string(),
            ],
            ..Default::default()
        };

        let coord = SpaceCoordinates::new(50, 50, 50);
        let region = generate_region(coord, &galaxy);
        assert_eq!(region, GalacticRegion::Void);
    }
}
