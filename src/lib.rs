use device_query::{DeviceQuery, DeviceState};
use node_bindgen::derive::node_bindgen;

#[node_bindgen(mt)]
fn start<F: Fn(Vec<String>) + Send + 'static, X: Fn(Vec<String>) + Send + 'static>(
    keydown_callback: F,
    keyup_callback: X,
) {
    std::thread::spawn(move || {
        let state = DeviceState::new();

        let mut previous_keys = Vec::new();
        loop {
            std::thread::sleep(std::time::Duration::new(0, 1000));
            let keys = state.get_keys();
            let state = keys != previous_keys;

            if !keys.is_empty() && state {
                keydown_callback(keys.clone().iter().map(|key| key.to_string()).collect());
            }

            if !previous_keys.is_empty() && state {
                keyup_callback(
                    previous_keys
                        .clone()
                        .iter()
                        .filter(|x| !keys.contains(x))
                        .map(|x| x.to_string())
                        .collect(),
                );
            }
            previous_keys = keys;
        }
    });
}

#[node_bindgen]
fn works() -> bool {
    true
}
