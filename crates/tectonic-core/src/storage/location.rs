// ============================================================
// IMPORTS AND MODULES
// ============================================================

// ============================================================
// CACHE LOCATION
// ============================================================

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ArenaLocation<'a> {
    user_id: Option<&'a str>,
    entry_id: usize,
    entry_index: usize,
}

#[allow(dead_code)]
impl<'a> ArenaLocation<'a> {
    pub fn new(user_id: Option<&'a str>, id: usize, index: usize) -> Self {
        Self { 
            user_id: user_id,
            entry_id: id, 
            entry_index: index 
        }
    }

    pub fn get_user_id(&self) -> Option<&'a str> {
        self.user_id
    }

    pub fn get_entry_id(&self) -> &usize {
        &self.entry_id
    }

    pub fn get_index(&self) -> &usize {
        &self.entry_index
    }
}