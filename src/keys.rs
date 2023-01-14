use evdev::*;

pub type KeyBind = Vec<InputEvent>;
pub type Chord = u64;

const STATE_NAMES: [&str; 2] = ["UP", "DOWN"];

pub trait ConstructKeyBind {
	fn single(key: Key) -> Self;
}

impl ConstructKeyBind for KeyBind {
	fn single(key: Key) -> Self {
		vec![
			InputEvent::new(EventType::KEY, key.0, 1),
			InputEvent::new(EventType::KEY, key.0, 0),
		]
	}
}

pub trait PrintKeyBind {
	fn display(&self) -> Vec<String>;
}

impl PrintKeyBind for KeyBind {
	fn display(&self) -> Vec<String> {
		self.iter()
			.map(|input_event| {
				format!(
					"{:?} {}",
					Key(input_event.code()),
					STATE_NAMES[input_event.value() as usize]
				)
			})
			.collect()
	}
}
