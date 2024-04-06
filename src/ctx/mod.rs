mod error;

pub use self::error::{Error, Result};

#[derive(Debug, Clone)]
pub struct Ctx {
    user_id: i64,
}

// Constructor
impl Ctx {
    pub fn root_ctx() -> Self {
        Self { user_id: 0 }
    }

    pub fn new(user_id: i64) -> Result<Self> {
        if user_id == 0 {
            Err(Error::CtxCannotNewRoot)
        } else {
            Ok(Self { user_id })
        }
    }
}

// Property Accessors
impl Ctx {
    pub fn user_id(&self) -> i64 {
        self.user_id
    }
}
