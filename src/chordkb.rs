use std::collections::HashMap;

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
	// keys: Vec<Key>,
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
		Self::new()	
	}
}