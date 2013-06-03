#[doc(hidden)]
pub mod sfTypes {
    use core::libc::{c_int};
    
    pub type sfBool = c_int;
    pub static sfFalse : sfBool = 0;
    pub static sfTrue : sfBool = 1;
}

#[doc(hidden)]
pub trait Wrapper {
    pub fn wrap_color<T>(sfObj : T) -> Self;
    pub fn unwrap_color(&self) -> Self; 
}