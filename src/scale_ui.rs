use std::fs::read_to_string;

use crate::patch::patch;

const BBB_OPT_ENABLED: usize = 0x766e91;
const BBB_OPT_CDGO: usize = 0x76709d;
const BBB_OPT_HSR_RAYS_PER_POINT: usize = 0x76709f;
const BBB_OPT_HSR_POINTS_PER_TRI: usize = 0x7670a4;
const BBB_OPT_HSR: usize = 0x7670a9;
const BBB_OPT_AO_BAKING: usize = 0x7670ab;
const BBB_OPT_COLOR_VAR: usize = 0x7670af;
const VIEWPORT_HEIGHT: usize = 0x7939ce;
const VIEWPORT_HEIGHT_VALUE: usize = 0x7939d2;
const MIN_ASPECT_RATIO: usize = 0x1114f88;
const MAX_ASPECT_RATIO: usize = 0x1114f8c;

static mut COLOR_VAR: f32 = 0.05;

pub fn patch_ui_scale() {
	let lnv = read_to_string("boot.cfg").expect("could not open boot.cfg");

	for name_value in lnv.split(",") {
		let name_value = name_value.trim();
		let (name, value) = name_value.split_at(name_value.find(":").expect(&format!("malformed boot.cfg entry, colon missing: {}", name_value))+1);

		if name == "BBB_OPT_ENABLED=7:" && value == "1" {
			patch(BBB_OPT_ENABLED, 1u8);
		} else if name == "BBB_OPT_CDGO=7:" && value == "0" {
			patch(BBB_OPT_CDGO, 0u8);
		} else if name == "BBB_OPT_HSR_RAYS_PER_POINT=5:" {
			let parsed_value: u32 = value.parse().expect(&format!("could not parse boot.cfg BBB_OPT_HSR_RAYS_PER_POINT value as u32: {}", value));
			patch(BBB_OPT_HSR_RAYS_PER_POINT, parsed_value);
		} else if name == "BBB_OPT_HSR_POINTS_PER_TRI=5:" {
			let parsed_value: u32 = value.parse().expect(&format!("could not parse boot.cfg BBB_OPT_HSR_POINTS_PER_TRI value as u32: {}", value));
			patch(BBB_OPT_HSR_POINTS_PER_TRI, parsed_value);
		} else if name == "BBB_OPT_HSR=7:" && value == "0" {
			patch(BBB_OPT_HSR, 0u8);
		} else if name == "BBB_OPT_AO_BAKING=7:" && value == "0" {
			patch(BBB_OPT_AO_BAKING, 0u8);
		} else if name == "BBB_OPT_COLOR_VAR=3:" {
			let parsed_value: f32 = value.parse().expect(&format!("could not parse boot.cfg BBB_OPT_COLOR_VAR value as float: {}", value));
			unsafe { COLOR_VAR = parsed_value; }
			patch(BBB_OPT_COLOR_VAR, unsafe { &COLOR_VAR });
		} else if name == "UI_SCALE=3:" {
			let parsed_value: f32 = value.parse().expect(&format!("could not parse boot.cfg UI_SCALE value as float: {}", value));
			let scale = 1.0 / parsed_value;
			// we override the viewport height initializer to initialize viewport scale instead
			patch(VIEWPORT_HEIGHT, 0x00_00_01_50_u32);
			patch(VIEWPORT_HEIGHT_VALUE, scale);
		} else if name == "MIN_ASPECT_RATIO=3:" {
			let parsed_value: f32 = value.parse().expect(&format!("could not parse boot.cfg MIN_ASPECT_RATIO value as float: {}", value));
			patch(MIN_ASPECT_RATIO, parsed_value);
		} else if name == "MAX_ASPECT_RATIO=3:" {
			let parsed_value: f32 = value.parse().expect(&format!("could not parse boot.cfg MAX_ASPECT_RATIO value as float: {}", value));
			patch(MAX_ASPECT_RATIO, parsed_value);
		}
	}
}
