use lazy_static::lazy_static;

use crate::TestEnv;

lazy_static! {
    pub static ref ENV: TestEnv = TestEnv::new(None);
}

impl TestEnv {
    pub fn get() -> &'static TestEnv {
        &ENV
    }
}
