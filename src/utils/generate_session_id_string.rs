use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub fn invoke() -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(128)
        .map(char::from)
        .collect();
    rand_string
}
