/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy)]
pub struct Rooted {
    pub ptr: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_Rooted() {
    assert_eq!(::std::mem::size_of::<Rooted>() , 4usize , concat ! (
               "Size of: " , stringify ! ( Rooted ) ));
    assert_eq! (::std::mem::align_of::<Rooted>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( Rooted ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const Rooted ) ) . ptr as * const _ as usize }
                , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( Rooted ) , "::" ,
                stringify ! ( ptr ) ));
}
impl Clone for Rooted {
    fn clone(&self) -> Self { *self }
}
/// <div rustbindgen replaces="MaybeWrapped"></div>
pub type MaybeWrapped<a> = a;
