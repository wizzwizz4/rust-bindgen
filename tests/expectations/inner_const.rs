/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy)]
pub struct Foo {
    pub bar: ::std::os::raw::c_int,
}
extern "C" {
    #[link_name = "_ZN3Foo3BOOE"]
    pub static mut Foo_BOO: ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "_ZN3Foo8whateverE"]
    pub static mut Foo_whatever: Foo;
}
#[test]
fn bindgen_test_layout_Foo() {
    assert_eq!(::std::mem::size_of::<Foo>() , 4usize);
    assert_eq!(::std::mem::align_of::<Foo>() , 4usize);
}
impl Clone for Foo {
    fn clone(&self) -> Self { *self }
}
