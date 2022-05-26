use std::process::exit;

use evdev::*;
use evdev::uinput::*;

use crate::keys::*;
use crate::config::*;

const DEBUG_PASSTHROUGH: bool = false;
const DEBUG_OUTPUT: bool = false;

pub struct ChordedKeyboard {
	input_dev: Device,
	output_dev: VirtualDevice,
	config: ChordConfig,
	/// physical state of buttons
	state: Chord,
	/// accumulated chord, resets when all keys are released, triggering a virtual press
	chord: Chord,
}


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
						self.update_chord(key, event.value());
					}
					else if self.config.remaps.contains_key(&key) {
						let mapped_key = self.config.remaps.get(&key).unwrap();
						let mapped_event = InputEvent::new(EventType::KEY, mapped_key.code(), event.value());
						self.output_dev.emit(&[mapped_event]).unwrap();
					}
					else {
						self.output_dev.emit(&[event]).unwrap();
						if DEBUG_PASSTHROUGH {
							println!("passing through {} {:?}", event.value(), key);
						}
					}
				}
			}
		}
	}

	fn update_chord(&mut self, key: Key, state: i32) {
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
				if key_bind[0].code() == Key::KEY_EXIT.code() {
					self.input_dev.ungrab().unwrap();
					println!("Exiting");
					exit(0);
				}

				self.output_dev.emit(key_bind).unwrap();
				if DEBUG_OUTPUT {
					println!("Chord pressed: {:?}", key_bind.display());
				}
			}
			self.chord = 0;
		}
	}
}

