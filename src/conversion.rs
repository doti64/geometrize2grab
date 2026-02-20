use crate::builder::*;

/*
pub fn parse_geometrize_data(geometrize_data: &str) {
    let json_value: serde_json::Value = serde_json::from_str(geometrize_data).unwrap();
    let array = json_value.as_array().unwrap();

    println!("[i] array length {}", array.len());
    println!("[i] complexity {}", array.len() * 2);

    for (i, entry) in array.iter().enumerate() {
        let shape_type = entry["type"].as_u64().unwrap();

        if shape_type != 0 && shape_type != 1 {
            continue;
        }

        let data = entry["data"].as_array().unwrap();

        // pos properties
        let x1 = data[0].as_f64().unwrap() as f32;
        let y1 = data[1].as_f64().unwrap() as f32;
        let x2 = data[2].as_f64().unwrap() as f32;
        let y2 = data[3].as_f64().unwrap() as f32;

        // size properties
        let center_x = (x1 + x2) / 2.0;
        let center_y = (y1 + y2) / 2.0;
        let width = (x2 - x1).abs();
        let height = (y2 - y1).abs();

        // rotation 
        let rotation = if shape_type == 1 {
            // Rotated rectangle
            let angle_deg = data[4].as_f64().unwrap() as f32;

            // Flip Y-axis for Grab engine (image Y-down → engine Y-up)
            let flipped_angle_deg = -angle_deg;

            // Convert to radians
            let angle_rad = flipped_angle_deg.to_radians();

            // Quaternion needs half-angle
            let half = angle_rad / 2.0;

            Vector4 {
                x: 0.0,
                y: 0.0,
                z: half.sin(),
                w: half.cos(),
            }
        } else {
            // Normal rectangle (no rotation)
            Vector4 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            }
        };

        let color = entry["color"].as_array().unwrap();

        let block = GrabObject {
            block_type: BlockType::Cube,
            position: Vector3 {
                x: center_x,
                y: center_y,
                z: 0.001 * i as f32,
            },
            rotation: rotation,
            scale: Vector3 {
                x: width,
                y: -height, // to flip the object hopefully
                z: 0.01,
            },
            color: Color {
                r: color[0].as_u64().unwrap() as f32 / 255.0,
                g: color[1].as_u64().unwrap() as f32 / 255.0,
                b: color[2].as_u64().unwrap() as f32 / 255.0,
                a: 1.0,
            },
        };
        generate_object(block);
    }

    //let output_string = serde_json::to_string_pretty(&level_json).unwrap();
    //std::fs::write("output_level.json", output_string).unwrap();
}
*/

pub fn parse_geometrize_data(geometrize_data: &str) {
    let json_value: serde_json::Value = serde_json::from_str(geometrize_data).unwrap();
    let array = json_value.as_array().unwrap();

    println!("[i] array length {}", array.len());
    println!("[i] complexity {}", array.len() * 2);

    for (i, entry) in array.iter().enumerate() {
        let shape_type = entry["type"].as_u64().unwrap();

        // squares - 0
        // rot rects - 1
        if shape_type != 0 && shape_type != 1 {
            continue;
        }

        let data = entry["data"].as_array().unwrap();

        // position properties
        let x1 = data[0].as_f64().unwrap() as f32;
        let y1 = data[1].as_f64().unwrap() as f32;
        let x2 = data[2].as_f64().unwrap() as f32;
        let y2 = data[3].as_f64().unwrap() as f32;

        let center_x = (x1 + x2) / 2.0;
        let center_y = (y1 + y2) / 2.0;

        let width = (x2 - x1).abs();
        let height = (y2 - y1).abs();

        // rotation (oh lord)
        let rotation = if shape_type == 1 {
            let angle_deg = data[4].as_f64().unwrap() as f32;
            let angle_rad = angle_deg.to_radians();
            let half = angle_rad / 2.0;

            Vector4 {
                x: 0.0,
                y: 0.0,
                z: half.sin(),
                w: half.cos(),
            }
        } else {
            Vector4 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            }
        };

        // color is 0 -> 1
        let color = entry["color"].as_array().unwrap();
        let r = color[0].as_u64().unwrap() as f32 / 255.0;
        let g = color[1].as_u64().unwrap() as f32 / 255.0;
        let b = color[2].as_u64().unwrap() as f32 / 255.0;
        let a = 1.0; // fully opaque

        let block = GrabObject {
            block_type: BlockType::Cube,
            position: Vector3 {
                x: center_x,
                y: -center_y,
                z: 0.001 * i as f32, // slight z to prevent z fighting
            },
            rotation,
            scale: Vector3 {
                x: width,
                y: height,
                z: 0.01,
            },
            color: Color {
                r,
                g,
                b,
                a,
            },
        };

        generate_object(block);
    }
}
