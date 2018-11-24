#![allow(dead_code)]
use std::ops::{AddAssign, SubAssign, BitAnd, BitOr};
use std::fmt;

#[derive(Clone, Default)]
pub struct State(u32, u32);
impl State {
    pub fn new(init: u32) -> Self {
        State(init,0)
    }
    pub fn update(&mut self) {
        self.1 = self.0;
    }
    fn activate<T: Into<u32>>(&mut self, o: T) {
        self.0 = self.0 | o.into();
    }
    fn deactivate<T: Into<u32>>(&mut self, o: T) {
        self.0 = self.0 & !o.into();
    }
    pub fn is_on<T: Into<u32>>(&self, o: T) -> bool {
        self.0 & o.into() != 0
    }
    pub fn only<T: Into<u32>>(&self, o: T) -> bool {
        let oo = o.into();
        (self.0 & oo != 0) && (self.0 & !oo == 0)
    }
    pub fn rising<T: Into<u32>>(&self, o: T) -> bool {
        let oo = o.into();
        (self.0 & oo != 0) && (self.1 & oo == 0)
    }
    pub fn falling<T: Into<u32>>(&self, o: T) -> bool {
        let oo = o.into();
        (!(self.0 & oo != 0)) && (self.1 & oo != 0)
    }
    pub fn any(&self) -> bool {
        (self.0 & !0) != 0
    }
    pub fn get(&self) -> u32 {
        self.0
    }
}
impl AddAssign<u32> for State {
    fn add_assign(&mut self, o: u32) {
        self.activate(o);
    }
}
impl SubAssign<u32> for State {
    fn sub_assign(&mut self, o: u32) {
        self.deactivate(o);
    }
}
impl AddAssign<VVal> for State {
    fn add_assign(&mut self, o: VVal) {
        self.activate(o);
    }
}
impl SubAssign<VVal> for State {
    fn sub_assign(&mut self, o: VVal) {
        self.deactivate(o);
    }
}
impl AddAssign<IVal> for State {
    fn add_assign(&mut self, o: IVal) {
        self.activate(o);
    }
}
impl SubAssign<IVal> for State {
    fn sub_assign(&mut self, o: IVal) {
        self.deactivate(o);
    }
}
impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:021b}", self.0)
    }
}
impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:032b}|{:032b}", self.0, self.1)
    }
}

#[derive(Clone, Copy)]
pub enum IVal {
    NoInput  = 0b0_0000_0000_0000_0000,
    JInput   = 0b1_0000_0000_0000_0000,
    LInput   = 0b0_1000_0000_0000_0000,
    RInput   = 0b0_0100_0000_0000_0000,
    UInput   = 0b0_0010_0000_0000_0000,
    DInput   = 0b0_0001_0000_0000_0000,
    BInput   = 0b0_0000_1000_0000_0000,
    AInput   = 0b0_0000_0100_0000_0000,
    SInput   = 0b0_0000_0010_0000_0000,
    ZInput   = 0b0_0000_0001_0000_0000,
    CRInput  = 0b0_0000_0000_1000_0000,
    CLInput  = 0b0_0000_0000_0100_0000,
    CDInput  = 0b0_0000_0000_0010_0000,
    CUInput  = 0b0_0000_0000_0001_0000,
    TRInput  = 0b0_0000_0000_0000_1000,
    TLInput  = 0b0_0000_0000_0000_0100,
    TDInput  = 0b0_0000_0000_0000_0010,
    TUInput  = 0b0_0000_0000_0000_0001,
}
impl Into<u32> for IVal {
    fn into(self) -> u32 {
        self as u32
    }
}
impl BitAnd<IVal> for IVal {
    type Output = u32;
    fn bitand(self, o: IVal) -> u32 {
        (self as u32) & (o as u32)
    }
}
impl BitOr<IVal> for IVal {
    type Output = u32;
    fn bitor(self, o: IVal) -> u32 {
        (self as u32) | (o as u32)
    }
}
impl BitAnd<u32> for IVal {
    type Output = u32;
    fn bitand(self, o: u32) -> u32 {
        (self as u32) & o
    }
}
impl BitOr<u32> for IVal {
    type Output = u32;
    fn bitor(self, o: u32) -> u32 {
        (self as u32) | o
    }
}
#[derive(Clone, Copy)]
pub enum VVal {
    Null            = 0b0_0000_0000_0000_0000,
    Grounded        = 0b1_0000_0000_0000_0000,
    ActiveInput     = 0b0_1000_0000_0000_0000,
    Interruptable   = 0b0_0100_0000_0000_0000,
    Helpless        = 0b0_0010_0000_0000_0000,
    //              = 0b0_0001_0000_0000_0000,
    //              = 0b0_0000_1000_0000_0000,
    //              = 0b0_0000_0100_0000_0000,
    //              = 0b0_0000_0010_0000_0000,
    //              = 0b0_0000_0001_0000_0000,
    //              = 0b0_0000_0000_1000_0000,
    //              = 0b0_0000_0000_0100_0000,
    //              = 0b0_0000_0000_0010_0000,
    //              = 0b0_0000_0000_0001_0000,
    //              = 0b0_0000_0000_0000_1000,
    //              = 0b0_0000_0000_0000_0100,
    //              = 0b0_0000_0000_0000_0010,
    FacingLeft      = 0b0_0000_0000_0000_0001,
}
impl Into<u32> for VVal {
    fn into(self) -> u32 {
        self as u32
    }
}
impl BitAnd<VVal> for VVal {
    type Output = u32;
    fn bitand(self, o: VVal) -> u32 {
        (self as u32) & (o as u32)
    }
}
impl BitOr<VVal> for VVal {
    type Output = u32;
    fn bitor(self, o: VVal) -> u32 {
        (self as u32) | (o as u32)
    }
}
impl BitAnd<u32> for VVal {
    type Output = u32;
    fn bitand(self, o: u32) -> u32 {
        (self as u32) & o
    }
}
impl BitOr<u32> for VVal {
    type Output = u32;
    fn bitor(self, o: u32) -> u32 {
        (self as u32) | o
    }
}
