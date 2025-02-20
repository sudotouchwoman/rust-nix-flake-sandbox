pub mod samples;

// public_hello is an example public function in module
pub fn public_hello(name: &str) -> String {
    format!("Hello,  {}", name)
}
