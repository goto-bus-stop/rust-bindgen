#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct X {
    pub _x: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_X() {
    assert_eq!(
        ::std::mem::size_of::<X>(),
        4usize,
        concat!("Size of: ", stringify!(X))
    );
    assert_eq!(
        ::std::mem::align_of::<X>(),
        4usize,
        concat!("Alignment of ", stringify!(X))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<X>()))._x as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(X), "::", stringify!(_x))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN1X13some_functionEv"]
    pub fn X_some_function(this: *mut X);
}
extern "C" {
    #[link_name = "\u{1}_ZN1X19some_other_functionEv"]
    pub fn X_some_other_function(this: *mut X);
}
extern "C" {
    #[link_name = "\u{1}_ZN1XC1Ei"]
    pub fn X_X(this: *mut X, x: ::std::os::raw::c_int);
}
impl X {
    #[inline]
    pub unsafe fn some_function(&mut self) {
        X_some_function(self)
    }
    #[inline]
    pub unsafe fn some_other_function(&mut self) {
        X_some_other_function(self)
    }
    #[inline]
    pub unsafe fn new(x: ::std::os::raw::c_int) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        X_X(__bindgen_tmp.as_mut_ptr(), x);
        __bindgen_tmp.assume_init()
    }
}
extern crate libloading;
pub struct TestLib<'a> {
    foo: libloading::Symbol<
        'a,
        unsafe extern "C" fn(
            x: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    bar: libloading::Symbol<
        'a,
        unsafe extern "C" fn(
            x: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
}
impl<'a> TestLib<'a> {
    pub fn new(lib: &libloading::Library) -> TestLib {
        unsafe {
            TestLib {
                foo: lib.get("foo".as_bytes()).unwrap(),
                bar: lib.get("bar".as_bytes()).unwrap(),
            }
        }
    }
}