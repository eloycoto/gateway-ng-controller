use crate::envoy_helpers::EnvoyExport;

#[derive(Debug, Clone, Copy)]
pub struct LocalCache {
    cache: Vec<EnvoyExport>,
    version: u32,
    temp_cache: Vec<EnvoyExport>,
}

impl LocalCache {
    pub fn new() -> LocalCache {
        return LocalCache {
            cache: Vec::new(),
            version: 0,
            temp_cache: Vec::new(),
        };
    }

    pub fn add(&mut self, element: EnvoyExport) {
        self.temp_cache.push(element)
    }

    pub fn add_multiple(&mut self, elements: &mut Vec<EnvoyExport>) {
        self.temp_cache.append(elements);
    }

    pub fn read_all(&mut self) -> &Vec<EnvoyExport> {
        return &self.cache;
    }

    pub fn release(&mut self) -> u32 {
        self.cache = self.temp_cache.clone();
        self.version += 1;
        self.temp_cache = Vec::new();
        return self.version;
    }
}
