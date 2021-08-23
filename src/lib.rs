use node_bindgen::derive::node_bindgen;


//https://github.com/infinyon/node-bindgen/blob/master/examples/cb/src/lib.rs

#[node_bindgen]
fn sum(first: f64, second: f64)->f64 {
    first + second
}