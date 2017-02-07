/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy)]
pub struct C {
    pub _bitfield_1: u8,
}
#[test]
fn bindgen_test_layout_C() {
    assert_eq!(::std::mem::size_of::<C>() , 1usize , concat ! (
               "Size of: " , stringify ! ( C ) ));
    assert_eq! (::std::mem::align_of::<C>() , 1usize , concat ! (
                "Alignment of " , stringify ! ( C ) ));
}
impl Clone for C {
    fn clone(&self) -> Self { *self }
}
impl C {
    #[inline]
    pub fn a(&self) -> bool {
        unsafe {
            ::std::mem::transmute(((self._bitfield_1 & (1usize as u8)) >>
                                       0u32) as u8)
        }
    }
    #[inline]
    pub fn set_a(&mut self, val: bool) {
        self._bitfield_1 &= !(1usize as u8);
        self._bitfield_1 |= ((val as u8 as u8) << 0u32) & (1usize as u8);
    }
    #[inline]
    pub fn b(&self) -> bool {
        unsafe {
            ::std::mem::transmute(((self._bitfield_1 & (254usize as u8)) >>
                                       1u32) as u8)
        }
    }
    #[inline]
    pub fn set_b(&mut self, val: bool) {
        self._bitfield_1 &= !(254usize as u8);
        self._bitfield_1 |= ((val as u8 as u8) << 1u32) & (254usize as u8);
    }
}
