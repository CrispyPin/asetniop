use evdev::*;
use evdev::uinput::*;
use std::io::stdin;
use std::time::Duration;
use std::thread;

mod chordkb;
use chordkb::*;
mod keys;
mod config;

const DEBUG_SUPPORTED_KEYS: bool = false;

fn main() {
	
	let selected = choose_kb();
	if selected.is_none() {
		return;
	}
	let input_kb = selected.unwrap();
	
	let keys = input_kb.supported_keys().unwrap();
	if DEBUG_SUPPORTED_KEYS {
		println!("{:?}", keys);
	}
	let virtual_kb = create_device(keys);
	thread::sleep(Duration::from_millis(200));

	let mut kb = ChordedKeyboard::new(input_kb, virtual_kb);
	kb.start();
	kb.release();
}

fn create_device(keys: &AttributeSetRef<Key>) -> VirtualDevice {
	 VirtualDeviceBuilder::new()
	 	.unwrap()
		.name("asetniop")
		.with_keys(keys)
		.unwrap()
		.build()
		.unwrap()
}

fn choose_kb() -> Option<Device> {
	let mut keyboards = find_keyboards();
	println!("Found {} keyboard device(s)", keyboards.len());
	
	let stdin = stdin();
	loop {
		println!("Select one of the following:");
		print_devices(&keyboards);
		let mut answer = String::new();
		stdin.read_line(&mut answer).unwrap();
		answer.remove(answer.len() - 1);
		if answer.to_lowercase().starts_with('q') {
			return None;
		}
		if let Ok(index) = answer.parse::<usize>() {
			if index < keyboards.len() {
				return Some(keyboards.remove(index));
			}
			else {
				println!("Index outside of range");
			}
		}
		else {
			println!("Not a valid integer");
		}
		println!("\n");
	}
	
	fn print_devices(devices: &[Device]) {
		for (i, kb) in devices.iter().enumerate() {
			if let Some(name) = kb.name() {
				println!("{}: {}", i, name);
			}
		}
		println!();
		println!("Enter id or Q to quit: ");
	}
}

fn find_keyboards() -> Vec<Device> {
	let mut keyboards = Vec::new();
	println!("Scanning /dev/input/ for keyboards");
	for device in evdev::enumerate() {
		let keys = device.supported_keys();
		if let Some(keys) = keys {
			if keys.contains(Key::KEY_SPACE) {
				keyboards.push(device)
			}
		}
	}
	keyboards
}