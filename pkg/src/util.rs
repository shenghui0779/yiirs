use rand::distributions::{Alphanumeric, DistString};
use std::{borrow::Cow, collections::HashMap};
use validator::ValidationError;

pub fn nonce(size: usize) -> String {
    let mut rng = rand::thread_rng();
    Alphanumeric.sample_string(&mut rng, size)
}

pub fn query_page(args: &HashMap<String, String>) -> (u64, u64) {
    let mut offset: u64 = 0;
    let mut limit: u64 = 20;

    if let Some(v) = args.get("size") {
        let size: u64 = v.parse().unwrap_or_default();
        if size > 0 {
            limit = size
        }
    }

    if limit > 100 {
        limit = 100
    }

    if let Some(v) = args.get("page") {
        let page: u64 = v.parse().unwrap_or_default();
        if page > 0 {
            offset = (page - 1) * limit
        }
    }

    (offset, limit)
}

pub fn new_validation_err(s: String) -> ValidationError {
    ValidationError {
        code: Cow::from(""),
        message: Some(Cow::from(s)),
        params: HashMap::new(),
    }
}
