use node_bindgen::derive::node_bindgen;

#[node_bindgen]
fn sum(a: i32, b: i32) -> i32 {
  a + b
}
