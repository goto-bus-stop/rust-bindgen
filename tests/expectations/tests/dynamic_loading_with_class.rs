#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct A {
    pub _x: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_A() {
    assert_eq!(
        ::std::mem::size_of::<A>(),
        4usize,
        concat!("Size of: ", stringify!(A))
    );
    assert_eq!(
        ::std::mem::align_of::<A>(),
        4usize,
        concat!("Alignment of ", stringify!(A))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<A>()))._x as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(A), "::", stringify!(_x))
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN1A13some_functionEv"]
    pub fn A_some_function(this: *mut A);
}
extern "C" {
    #[link_name = "\u{1}_ZN1A19some_other_functionEv"]
    pub fn A_some_other_function(this: *mut A);
}
extern "C" {
    #[link_name = "\u{1}_ZN1AC1Ei"]
    pub fn A_A(this: *mut A, x: ::std::os::raw::c_int);
}
impl A {
    #[inline]
    pub unsafe fn some_function(&mut self) {
        A_some_function(self)
    }
    #[inline]
    pub unsafe fn some_other_function(&mut self) {
        A_some_other_function(self)
    }
    #[inline]
    pub unsafe fn new(x: ::std::os::raw::c_int) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        A_A(__bindgen_tmp.as_mut_ptr(), x);
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
    bar: libloading::Symbol<'a, unsafe extern "C" fn()>,
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
