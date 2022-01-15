#![no_std]
#![no_main]

extern crate panic_halt;

mod shell;

use orange_crab_hal::gpio::*;
use orange_crab_hal::gpioa::*;
use orange_crab_hal::rt::entry;
use orange_crab_hal::timer1::*;
use orange_crab_hal::uart1::*;
use orange_crab_hal::{arch, interrupt, prelude::*, serial};
use shell::*;
use ushell::{autocomplete::*, history::*, *};

static mut STATE: Option<(Shell, Env)> = None;

#[entry]
fn main() -> ! {
    let mut timer = TIMER1::take().unwrap().timer();

    let gpio = GPIOA::take().unwrap().split();
    let mut led_red = gpio.pa0.into_output();
    let mut led_green = gpio.pa1.into_output();
    let mut led_blue = gpio.pa2.into_output();

    led_red.set_high().ok();
    led_green.set_high().ok();
    led_blue.set_high().ok();

    let mut uart = UART1::take().unwrap().serial(serial::Config::default());
    uart.rx().listen();

    let autocomplete = StaticAutocomplete(["clear", "help", "status", "on", "off"]);
    let history = LRUHistory::default();
    let shell = UShell::new(uart, autocomplete, history);

    interrupt::free(|_| unsafe {
        STATE.replace((shell, Env::new(led_blue)));
        arch::register::mstatus::set_mie();
        arch::register::mie::set_mext();
    });

    loop {
        led_red.toggle().ok();
        timer.delay(250.ms());
    }
}

#[export_name = "MachineExternal"]
fn uart_interrupt() {
    interrupt::free(|_| unsafe {
        STATE.as_mut().map(|(shell, env)| shell.spin(env));
    });
}
