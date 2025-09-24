pub struct Paging {
    pub limit: usize,
    pub offset: usize,
}

impl Default for Paging {
    fn default() -> Self {
        Self {
            limit: 100,
            offset: 0,
        }
    }
}
