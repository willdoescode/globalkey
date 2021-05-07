use node_bindgen::derive::node_bindgen;
use device_query::{DeviceState, DeviceQuery};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[node_bindgen(mt)]
fn start<F: Fn(Vec<String>) + Send + 'static>(callback: F) {
	stoppable_thread::spawn(move |stop| {
    let state = DeviceState::new();

		let mut previous_keys = Vec::new();
		while !stop.get() {
			let keys = state.get_keys();
	    if !keys.is_empty() && keys != previous_keys {
		    callback(keys.clone().into_par_iter().map(|key| format!("{}", key)).collect());
	    }
	    previous_keys = keys;
    }
  });
}

#[node_bindgen]
fn works() -> bool { true }
