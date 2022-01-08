use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {}

#[cfg(test)]
mod utils {
    use dotenv::dotenv;
    use std::collections::HashMap;
    use std::env;
    pub fn get_env_hashmap() -> HashMap<String, String> {
        dotenv().ok();
        let mut env_map: HashMap<String, String> = HashMap::new();
        for (key, value) in env::vars() {
            env_map.insert(key, value);
        }
        env_map
    }
}

mod tests {
    use super::*;
    #[test]
    fn dotenv_works() {
        let env_map = utils::get_env_hashmap();
        assert_eq!("123", match env_map.get("TEST_VAR") {
            Some(value) => value,
            None => ""
        });
    }
}
