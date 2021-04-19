use core::marker::PhantomData;

pub struct ProgRef {
    _private: PhantomData<fn(ctx: *const cty::c_void) -> cty::c_int>,
}

impl ProgRef {
    #[inline(always)]
    pub fn new(function: fn(ctx: *const cty::c_void) -> cty::c_int) -> Self {
        let _ = function;
        ProgRef {
            _private: PhantomData,
        }
    }
}
