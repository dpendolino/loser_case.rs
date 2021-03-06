use rand::Rng;
use std::str;

/// Returns a string with random case
///
/// # Arguments
///
/// * `x` - A string to be modified
///
/// # Examples
///
/// ```
/// // You can have rust code between fences inside the comments
/// // If you pass --test to `rustdoc`, it will even test it for you!
/// use loser_case;
/// loser_case::to_loser("test");
/// ```
pub fn to_loser(x: &str) -> String {
    let mut rng = rand::thread_rng();
    let loser: String = x
        .chars()
        .map(|a| {
            if rng.gen_range(0, 10) > 5 {
                a.to_ascii_uppercase()
            } else {
                a.to_ascii_lowercase()
            }
        })
        .collect();
    loser
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!("test", to_loser("test").to_lowercase());
    }
}
