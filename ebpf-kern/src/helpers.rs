use core::mem;

#[inline(always)]
pub unsafe fn probe_read_kernel(
    dst: *mut cty::c_void,
    size: cty::uint32_t,
    unsafe_ptr: *const cty::c_void,
) -> cty::c_long {
    let f: unsafe extern "C" fn(
        *mut cty::c_void,
        cty::uint32_t,
        *const cty::c_void,
    ) -> cty::c_long = mem::transmute(113usize);

    f(dst, size, unsafe_ptr)
}

#[inline(always)]
pub unsafe fn ringbuf_output(
    ringbuf: *mut cty::c_void,
    data: *mut cty::c_void,
    size: cty::uint64_t,
    flags: cty::uint64_t,
) -> cty::c_long {
    let f: unsafe extern "C" fn(
        *mut cty::c_void,
        *mut cty::c_void,
        cty::uint64_t,
        cty::uint64_t,
    ) -> cty::c_long = mem::transmute(130usize);

    f(ringbuf, data, size, flags)
}

#[inline(always)]
pub unsafe fn ringbuf_reserve(
    ringbuf: *mut cty::c_void,
    size: cty::uint64_t,
    flags: cty::uint64_t,
) -> *mut cty::c_void {
    let f: unsafe extern "C" fn(
        *mut cty::c_void,
        cty::uint64_t,
        cty::uint64_t,
    ) -> *mut cty::c_void = mem::transmute(131usize);

    f(ringbuf, size, flags)
}

#[inline(always)]
pub unsafe fn ringbuf_submit(data: *mut cty::c_void, flags: cty::uint64_t) {
    let f: unsafe extern "C" fn(*mut cty::c_void, cty::uint64_t) = mem::transmute(132usize);

    f(data, flags)
}

#[inline(always)]
pub unsafe fn ringbuf_discard(data: *mut cty::c_void, flags: cty::uint64_t) {
    let f: unsafe extern "C" fn(*mut cty::c_void, cty::uint64_t) = mem::transmute(133usize);

    f(data, flags)
}