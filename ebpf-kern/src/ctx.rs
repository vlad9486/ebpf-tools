use super::helpers;

// TODO: cross-platform
pub struct Context {
    inner: *const cty::c_void,
}

impl Context {
    #[inline(always)]
    pub unsafe fn cast(ctx: *const cty::c_void) -> Self {
        Context {
            inner: ctx,
        }
    }

    #[inline(always)]
    pub fn read<T>(&self, offset: usize) -> T {
        use core::mem;

        let mut value = mem::MaybeUninit::uninit();
        unsafe {
            helpers::probe_read_kernel(
                value.as_mut_ptr() as _,
                mem::size_of::<T>() as u32,
             self.inner.offset(offset as isize) as *const _,
            );
            value.assume_init()
        }
    }
}
