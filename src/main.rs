use evdev::{Device, Key};
use std::{fs, io::stdin};

fn main() {
	
	let selected = choose_kb();
	
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
		if answer.to_lowercase().starts_with("q") {
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
	
	fn print_devices(devices: &Vec<Device>) {
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
	
	let devices = fs::read_dir("/dev/input").unwrap();
	for device_path in devices.flatten() {
		if let Ok(device) = Device::open(device_path.path()) {
			let keys = device.supported_keys();
			if let Some(keys) = keys {
				if keys.contains(Key::KEY_SPACE) {
					keyboards.push(device)
				}
			}
		}
	}
	keyboards
}