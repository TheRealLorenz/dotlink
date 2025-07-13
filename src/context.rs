use std::path::Path;

pub struct Context<'a> {
    pub pwd: &'a Path,
    pub dry_run: bool,
}
