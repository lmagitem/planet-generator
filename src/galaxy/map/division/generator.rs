use crate::internal::*;
use crate::prelude::*;
use std::collections::HashMap;
use usvg::{Tree, Options, NodeKind, OptionsRef, PathSegment, Paint};

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
        ((z as f64 / galaxy.get_galaxy_size().z as f64) * (layer_count as f64 - 1.0)).round() as usize
    };

    let svg_data = &galaxy.galactic_map_layers[layer_index];
    let tree = Tree::from_str(svg_data, &OptionsRef::default()).unwrap();

    let mut color_map = HashMap::new();
    for node in tree.root().descendants() {
        if let NodeKind::Path(ref path) = *node.borrow() {
            if let Some(ref fill) = path.fill {
                if let Paint::Color(color) = fill.paint {
                    color_map.insert(path.data.to_vec(), format!("{:06X}", color));
                }
            }
        }
    }

    let x = coord.x;
    let y = coord.y;
    let mut closest_color = "000000".to_string();
    let mut min_distance = f64::MAX;

    for (path_data, color) in color_map {
        for segment in path_data.windows(2) {
            let (x1, y1) = (segment[0].x, segment[0].y);
            let (x2, y2) = (segment[1].x, segment[1].y);
            let distance = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
            if distance < min_distance {
                min_distance = distance;
                closest_color = color.clone();
            }
        }
    }

    match closest_color.as_str() {
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use usvg::Tree;

    #[test]
    fn test_generate_region_with_default_svg() {
        let galaxy = Galaxy {
            galactic_map_layers: vec![r#"
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
                    <rect x="0" y="0" width="100" height="100" fill="#000000" />
                </svg>
            "#.to_string()],
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
                r#"
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
                    <rect x="0" y="0" width="100" height="100" fill="#000000" />
                </svg>
                "#.to_string(),
                r#"
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
                    <rect x="0" y="0" width="100" height="100" fill="#FFFFFF" />
                </svg>
                "#.to_string(),
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
                r#"
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
                    <rect x="0" y="0" width="100" height="100" fill="#000000" />
                </svg>
                "#.to_string(),
                r#"
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
                    <rect x="0" y="0" width="100" height="100" fill="#FFFFFF" />
                </svg>
                "#.to_string(),
            ],
            ..Default::default()
        };

        let coord = SpaceCoordinates::new(50, 50, 50);
        let region = generate_region(coord, &galaxy);
        assert_eq!(region, GalacticRegion::Void);
    }
}
