use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use toml::Value;
use evdev::*;

use crate::keys::*;


pub struct ChordConfig {
	pub keys: HashMap<Key, Chord>,
	pub chords: HashMap<Chord, KeyBind>,
}

impl ChordConfig {
	fn new() -> Self {
		Self { keys: HashMap::new(), chords: HashMap::new() }
	}

	pub fn load() -> Self {
		let mut keys = HashMap::new();
		let mut chords = HashMap::new();

		let mut file_config = String::new();
		if let Ok(mut file) = File::open("default.toml") {
			file.read_to_string(&mut file_config).unwrap();
		}
		else {
			println!("No config found");
			return Self::new();
		}
		let file_config = file_config.parse::<Value>().expect("Could not parse config file; not valid TOML");
		// println!("\n{:?}\n", file_config);

		// load input keys
		let loaded_keys = &file_config["input"]["keys"];
		println!("{:?}", loaded_keys);
		if let Value::Array(loaded_keys) = loaded_keys {
			for (i, key) in loaded_keys.iter().enumerate() {
				if let Value::String(key_name) = key {
					let chord_part = 1 << i;
					keys.insert(name_to_key(key_name).unwrap(), chord_part);
				}
			}
		}
		// println!("{:?}", keys);
		
		let mut chord_components: HashMap<Key, Chord> = HashMap::new();
		// load output/emulated keys
		let mapped_keys = &file_config["output"]["keys"];
		println!("{:?}", mapped_keys);
		if let Value::Array(mapped_keys) = mapped_keys {
			for (i, key) in mapped_keys.iter().enumerate() {
				if let Value::String(mapped_key_name) = key {
					let chord_part = 1 << i;
					let mapped_key = name_to_key(mapped_key_name).unwrap();
					chords.insert(chord_part, KeyBind::single(mapped_key));
					chord_components.insert(mapped_key, chord_part);
				}
			}
		}

		let loaded_chords = &file_config["output"]["chords"];
		if let Value::Table(loaded_chords) = loaded_chords {
			for (chord_str, out_key) in loaded_chords.iter() {
				if let Value::String(out_key) = out_key {
					let out_key = name_to_key(out_key);
					if out_key.is_none() {
						continue;
					}
					let out_key = out_key.unwrap();

					let mut chord = 0;
					for component in chord_str.chars() {
						let name = component.to_string();
						let component_key = name_to_key(&name).unwrap();
						chord |= chord_components.get(&component_key).unwrap();
					}
					chords.insert(chord, KeyBind::single(out_key));
				}
				//Todo: key sequences
			}
		}
		println!("{}", loaded_chords);

		Self {
			keys,
			chords,
		}
	}
}

fn name_to_key(name: &str) -> Option<Key> {
	let target_name = format!("KEY_{}", name);
	for code in Key::KEY_RESERVED.code()..Key::BTN_TRIGGER_HAPPY40.code() {
		let key = Key::new(code);
		let name = format!("{:?}", key);
		if name == target_name {
			return Some(key);
		}
	}
	None
}