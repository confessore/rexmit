use std::env;

use tracing::{debug, error};

pub fn parse_u64_from_string(input: String) -> Option<u64> {
    match input.parse::<u64>() {
        Ok(output) => {
            debug!("u64 parse is ok");
            return Some(output);
        }
        Err(why) => {
            error!("u64 parse is err: {}", why);
            return None;
        }
    }
}

pub fn get_max_slots() -> Option<u64> {
    let max_slots = env::var("MAX_SLOTS").expect("Expected a MAX_SLOTS in the environment");
    return parse_u64_from_string(max_slots);
}
