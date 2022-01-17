use crate::modify_reg;
use crate::rgbled::*;
use crate::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RGB12 {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl RGB12 {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}

impl From<RGB12> for u32 {
    fn from(color: RGB12) -> Self {
        (color.blue as u32 & 0xF) << 8 | (color.green as u32 & 0xF) << 4 | color.red as u32 & 0xF
    }
}

impl From<u32> for RGB12 {
    fn from(raw: u32) -> Self {
        Self {
            red: ((raw >> 8) & 0xf) as _,
            green: ((raw >> 4) & 0xf) as _,
            blue: (raw & 0xf) as _,
        }
    }
}

pub struct Led {
    instance: Instance,
}

pub trait LedExt {
    fn led(self) -> Led;
}

impl LedExt for Instance {
    fn led(self) -> Led {
        Led::new(self)
    }
}

impl Led {
    pub fn new(instance: Instance) -> Self {
        Led { instance }
    }

    pub fn enabled(&mut self) -> bool {
        read_reg!(led, self.instance, LED1, ENABLE) != 0
    }

    pub fn enable(&mut self) {
        modify_reg!(led, self.instance, LED1, ENABLE: 0x01);
    }

    pub fn disable(&mut self) {
        modify_reg!(led, self.instance, LED1, ENABLE: 0x00);
    }

    pub fn color(&mut self) -> RGB12 {
        read_reg!(led, self.instance, LED1, COLOR).into()
    }

    pub fn set_color(&mut self, color: RGB12) {
        let color: u32 = color.into();
        modify_reg!(led, self.instance, LED1, COLOR: color);
    }
}
