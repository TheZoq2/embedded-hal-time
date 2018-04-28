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


/**
  A monotonic timer that can be used to measure time passed. It should count from 0 to
  the max value of TickType and then wrap around.
*/
pub trait Stopwatch<TickType, Frequency> {
    /**
      Returns the amount of ticks that have passed since the last wraparound.
    */
    fn ticks_passed(&self) -> TickType;
    /**
      Returns the frequency of the timer. The total elapsed time should be
      ticks_passed * 1/frequency
    */
    fn frequency(&self) -> Frequency;
}
