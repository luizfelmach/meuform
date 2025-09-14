pub struct Pagination {
    pub limit: u64,
    pub offset: u64,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            limit: 100,
            offset: 0,
        }
    }
}

impl Pagination {
    pub fn new(limit: u64, offset: u64) -> Self {
        Self { limit, offset }
    }
}
