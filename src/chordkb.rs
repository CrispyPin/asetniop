use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use toml::Value;
use evdev::*;
use evdev::uinput::*;

use crate::keys::*;

pub struct ChordedKeyboard {
	input_dev: Device,
	output_dev: VirtualDevice,
	config: ChordConfig,
	/// physical state of buttons
	state: Chord,
	/// accumulated chord, resets when all keys are released, triggering a virtual press
	chord: Chord,
}

pub struct ChordConfig {
	keys: HashMap<Key, Chord>,
	chords: HashMap<Chord, KeyBind>,
}

pub type Chord = u64;

impl ChordedKeyboard {
	pub fn new(input_dev: Device, output_dev: VirtualDevice) -> Self {
		Self {
			input_dev,
			output_dev,
			config: ChordConfig::load(),
			state: 0,
			chord: 0,
		}
	}

	pub fn start(&mut self) {
		self.input_dev.grab().unwrap();

		loop {
			let events: Vec<InputEvent> = self.input_dev.fetch_events().unwrap().collect();
			for event in events {
				if event.event_type() == EventType::KEY {
					let key = Key(event.code());
					if self.config.keys.contains_key(&key) {
						self.apply(key, event.value());
					}
					else {
						self.output_dev.emit(&[event]).unwrap();
						println!("passing through {} {:?}", event.value(), key);
					}
				}
			}
		}
	}

	fn apply(&mut self, key: Key, state: i32) {
		let chord_part = *self.config.keys.get(&key).unwrap();
		if state == 1 {
			self.state |= chord_part;
			self.chord |= chord_part;
		}
		else if state == 0 {
			self.state &= !chord_part;
		}

		if self.state == 0 {
			if let Some(key_bind) = self.config.chords.get(&self.chord) {
				self.output_dev.emit(key_bind).unwrap();
			}
			self.chord = 0;
		}
	}

	pub fn release(&mut self) {
		self.input_dev.ungrab().unwrap();
	}
}

impl ChordConfig {
	fn new() -> Self {
		Self { keys: HashMap::new(), chords: HashMap::new() }
	}

	fn load() -> Self {
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
		println!("\n{:?}\n", file_config);

		// load input keys
		let loaded_keys = &file_config["input"]["keys"];
		println!("{:?}", loaded_keys);
		if let Value::Array(loaded_keys) = loaded_keys {
			for (i, key) in loaded_keys.iter().enumerate() {
				if let Value::String(key_name) = key {
					let chord_part = 1 << i;
					keys.insert(name_to_key(key_name), chord_part);
				}
			}
		}
		println!("{:?}", keys);

		// load output/emulated keys
		let mapped_keys = &file_config["output"]["keys"];
		println!("{:?}", mapped_keys);
		if let Value::Array(mapped_keys) = mapped_keys {
			for (i, key) in mapped_keys.iter().enumerate() {
				if let Value::String(key_name) = key {
					let chord_part = 1 << i;
					chords.insert(chord_part, KeyBind::single(name_to_key(key_name)));
				}
			}
		}
		println!("{:?}", keys);

		Self {
			keys,
			chords,
		}
	}
}

fn name_to_key(name: &str) -> Key {
	let target_name = format!("KEY_{}", name);
	for code in Key::KEY_RESERVED.code()..Key::BTN_TRIGGER_HAPPY40.code() {
		let key = Key::new(code);
		let name = format!("{:?}", key);
		if name == target_name {
			return key;
		}
	}
	panic!("Error: unknown key: '{}'", name);
}
