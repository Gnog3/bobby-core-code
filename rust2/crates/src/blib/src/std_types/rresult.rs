#[repr(u8)]
pub enum RResult<T, E> {
    ROk(T),
    RErr(E),
}

impl<T, E> RResult<T, E> {
    #[inline(always)]
    pub fn from_result(result: Result<T, E>) -> RResult<T, E> {
        match result {
            Ok(ok) => RResult::ROk(ok),
            Err(err) => RResult::RErr(err),
        }
    }

    #[inline(always)]
    pub fn to_result(self) -> Result<T, E> {
        match self {
            RResult::ROk(ok) => Ok(ok),
            RResult::RErr(err) => Err(err),
        }
    }
}