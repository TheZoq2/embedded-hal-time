#![no_std]
#![feature(never_type)]

extern crate nb;

extern crate embedded_hal;
use embedded_hal::timer::CountDown;

pub trait RealCountDown<Unit>: CountDown {
    fn start_real(&mut self, delay: Unit);
}

#[derive(Clone, Copy)]
pub struct Second(pub u32);
#[derive(Clone, Copy)]
pub struct Millisecond(pub u32);
#[derive(Clone, Copy)]
pub struct Microsecond(pub u32);
