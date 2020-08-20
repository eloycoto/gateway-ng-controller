use std::collections::HashMap;

#[derive(Debug)]
struct Snapshot {
    data: HashMap<i32, i32>,
    version: u32,
}

impl Snapshot {
    pub fn new() -> Self {
        return Snapshot {
            data: HashMap::new(),
            version: 0,
        };
    }
}

#[derive(Debug)]
pub struct LocalCache<T> {
    cache: HashMap<std::string::String, T>,
    snapshot: Snapshot,
}

impl<T> LocalCache<T> {
    pub fn new() -> LocalCache<T> {
        // @TODO review why cannot be initlize inside LocalCache{}
        let local_cache: HashMap<std::string::String, T> = HashMap::new();
        return LocalCache {
            cache: local_cache,
            snapshot: Snapshot::new(),
        };
    }

    pub fn add(&self) -> bool {
        return true;
    }

    pub fn release(&self) -> bool {
        return true;
    }

    pub fn get_snapshot(&self) -> bool {
        return true;
    }
}
