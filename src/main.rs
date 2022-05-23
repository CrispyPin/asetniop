use evdev::*;
use std::io::stdin;
use std::time::Duration;
use std::thread;

fn main() {
	
	let selected = choose_kb();
	if selected.is_none() {
		return;
	}
	let mut active_device = selected.unwrap();

	// active_device.grab().unwrap();

	let v = uinput::VirtualDeviceBuilder::new()
		.unwrap()
		.name("asetniop")
	;
	
	let mut a = v.build().unwrap();
	let press = [
		InputEvent::new_now(EventType::KEY, Key::KEY_A.0, 0),
		InputEvent::new_now(EventType::KEY, Key::KEY_A.0, 1)
		];
	a.emit(&press[0..1]).unwrap();
	thread::sleep(Duration::from_secs(1));
	a.emit(&press[1..2]).unwrap();
	thread::sleep(Duration::from_secs(10));
	
}

fn choose_kb() -> Option<Device> {
	let mut keyboards = find_keyboards();
	println!("Found {} keyboard devices", keyboards.len());
	
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