use node_bindgen::derive::node_bindgen;
use device_query::{DeviceState, DeviceQuery};

#[node_bindgen(mt)]
fn start<F: Fn(Vec<String>) + Send + 'static>(callback: F) {
	std::thread::spawn(move || {
    let state = DeviceState::new();

		let mut previous_keys = Vec::new();
		loop {
			std::thread::sleep(std::time::Duration::new(0, 1000));
			let keys = state.get_keys();
	    if !keys.is_empty() && keys != previous_keys {
		    callback(keys.clone().iter().map(|key| format!("{}", key)).collect());
	    }
	    previous_keys = keys;
    }
  });
}

#[node_bindgen]
fn works() -> bool { true }
