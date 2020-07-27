#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

extern crate libloading;
pub struct TestLib<'a> {
    foo: libloading::Symbol<
        'a,
        unsafe extern "C" fn(
            x: ::std::os::raw::c_int,
            y: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    bar: libloading::Symbol<
        'a,
        unsafe extern "C" fn(
            x: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    baz:
        libloading::Symbol<'a, unsafe extern "C" fn() -> ::std::os::raw::c_int>,
}
impl<'a> TestLib<'a> {
    pub fn new(lib: &libloading::Library) -> TestLib {
        unsafe {
            TestLib {
                foo: lib.get("foo".as_bytes()).unwrap(),
                bar: lib.get("bar".as_bytes()).unwrap(),
                baz: lib.get("baz".as_bytes()).unwrap(),
            }
        }
    }
}
