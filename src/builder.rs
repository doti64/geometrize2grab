use std::fs::{File};
use std::io::{Write};
use serde::{Serialize, Deserialize};
use serde_json;

// 3d types
#[derive(Serialize, Deserialize)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Vector4 {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// GRAB's color type is 0 - 1 wtv you call that
#[derive(Serialize, Deserialize)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

// conversion types
#[derive(Serialize, Deserialize)]
#[repr(u32)]
pub enum BlockType {
    Cube = 1000,
    Sphere = 1001,
    Cylinder = 1002,
    Pyramid = 1003,
    Prism = 1004,
    Cone = 1005,
    PyramidSquare = 1006
}

#[derive(Serialize, Deserialize)]
pub struct GrabObject {
    pub block_type: BlockType,
    pub position: Vector3,
    pub rotation: Vector4,
    pub scale: Vector3,
    pub color: Color,
}

// most optimized method of all time!!
pub fn generate_object(object: GrabObject) {
    // read file
    let data = std::fs::read_to_string("geometrize2grab.json")
        .expect("Failed to read file");

    // parse file
    let mut level_json: serde_json::Value =
        serde_json::from_str(&data).expect("Invalid JSON");

    // add json data for block
    let new_block = serde_json::json!({
        "isLocked": false,
        "animations": [],
        "activeAnimation": 0,
        "levelNodeStatic": {
            "shape": object.block_type as u32,
            "material": 8,
            "position": {
                "x": object.position.x,
                "y": object.position.y,
                "z": object.position.z
            },
            "scale": {
                "x": object.scale.x,
                "y": object.scale.y,
                "z": object.scale.z
            },
            "rotation": {
                "x": object.rotation.x,
                "y": object.rotation.y,
                "z": object.rotation.z,
                "w": object.rotation.w
            },
            "color1": {
                "r": object.color.r,
                "g": object.color.g,
                "b": object.color.b,
                "a": object.color.a
            },
            "isNeon": false,
            "isTransparent": false,
            "isGrabbable": false,
            "isGrapplable": false,
            "isPassable": false
        }
    });

    level_json["levelNodes"]
        .as_array_mut()
        .unwrap()
        .push(new_block);

    // write json data to file
    let output = serde_json::to_string_pretty(&level_json)
        .expect("Failed to serialize JSON");

    std::fs::write("geometrize2grab.json", output)
        .expect("Failed to write file");
}

pub fn generate_level_base() {
    let mut file = File::create("geometrize2grab.json").unwrap();

    let json_text = r#"{
        "formatVersion": 19,
        "title": "Generated G2G Level",
        "creators": "doti64.lol/g2g",
        "description": "Auto-generated geometrize2grab level.",
        "tags": [],
        "maxCheckpointCount": 10,
        "defaultSpawnPointID": 0,
        "unlisted": false,
        "showReplays": true,
        "complexity": 0,
        "ambienceSettings": {
            "skyHorizonColor": {
                "a": 1,
                "b": 0.9574,
                "g": 0.9574,
                "r": 0.916
            },
            "skyZenithColor": {
                "a": 1,
                "b": 0.73,
                "g": 0.476,
                "r": 0.28
            },
            "sunAltitude": 45,
            "sunAzimuth": 315,
            "sunSize": 1,
            "fogDensity": 0
        },
        "levelNodes": []
    }"#;

    file.write_all(json_text.as_bytes()).unwrap();
}