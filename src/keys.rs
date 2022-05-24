use evdev::*;

pub type KeyBind = Vec<InputEvent>;

pub trait ConstructKeyBind {
	fn single(key: Key) -> Self;
}

impl ConstructKeyBind for KeyBind {
	fn single(key: Key) -> Self {
		vec![
			InputEvent::new(EventType::KEY, key.0, 1),
			InputEvent::new(EventType::KEY, key.0, 0)
		]
	}
}
