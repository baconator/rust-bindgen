/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
pub struct nsISupports__bindgen_vtable {
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct nsISupports {
    pub vtable_: *const nsISupports__bindgen_vtable,
}
#[test]
fn bindgen_test_layout_nsISupports() {
    assert_eq!(::std::mem::size_of::<nsISupports>() , 8usize , concat ! (
               "Size of: " , stringify ! ( nsISupports ) ));
    assert_eq! (::std::mem::align_of::<nsISupports>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( nsISupports ) ));
}
impl Clone for nsISupports {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct nsIRunnable {
    pub _base: nsISupports,
}
#[test]
fn bindgen_test_layout_nsIRunnable() {
    assert_eq!(::std::mem::size_of::<nsIRunnable>() , 8usize , concat ! (
               "Size of: " , stringify ! ( nsIRunnable ) ));
    assert_eq! (::std::mem::align_of::<nsIRunnable>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( nsIRunnable ) ));
}
impl Clone for nsIRunnable {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Runnable {
    pub _base: nsIRunnable,
}
#[test]
fn bindgen_test_layout_Runnable() {
    assert_eq!(::std::mem::size_of::<Runnable>() , 8usize , concat ! (
               "Size of: " , stringify ! ( Runnable ) ));
    assert_eq! (::std::mem::align_of::<Runnable>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( Runnable ) ));
}
impl Clone for Runnable {
    fn clone(&self) -> Self { *self }
}
