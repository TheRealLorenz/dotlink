use std::path::Path;

pub struct Context<'a> {
    pub pwd: &'a Path,
    pub dry_run: bool,
}

impl<'a> Context<'a> {
    pub fn new(pwd: &'a Path) -> Self {
        Context {
            pwd,
            dry_run: false,
        }
    }

    pub fn dry_run(self, flag: bool) -> Self {
        Self {
            dry_run: flag,
            ..self
        }
    }
}
