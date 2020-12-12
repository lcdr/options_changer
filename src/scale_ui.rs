use std::fs::read_to_string;

use crate::patch::patch;

const VIEWPORT_HEIGHT_OFFSET: usize = 0x7939ce;
const VIEWPORT_HEIGHT_VALUE: usize = 0x7939d2;

pub fn patch_ui_scale() {
	let lnv = read_to_string("boot.cfg").expect("could not open boot.cfg");

	for name_value in lnv.split(",") {
		let name_value = name_value.trim();
		let (name, value) = name_value.split_at(name_value.find(":").expect(&format!("malformed boot.cfg entry, colon missing: {}", name_value))+1);

		if name == "UI_SCALE=3:" {
			let scale: f32 = value.parse().expect(&format!("could not parse boot.cfg UI_SCALE value as float: {}", value));
			let scale = 1.0 / scale;
			patch(VIEWPORT_HEIGHT_OFFSET, 0x00_00_01_50_u32);
			patch(VIEWPORT_HEIGHT_VALUE, scale);
		}
	}
}