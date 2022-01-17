#![no_std]
#![no_main]

extern crate panic_halt;

mod shell;

use orange_crab_hal::led::*;
use orange_crab_hal::rgbled::*;
use orange_crab_hal::rt::entry;
use orange_crab_hal::uart1::*;
use orange_crab_hal::{arch, interrupt, prelude::*, serial};
use shell::*;
use ushell::{autocomplete::*, history::*, *};

static mut STATE: Option<(Shell, Env)> = None;

#[entry]
fn main() -> ! {
    let mut led = RGBLED::take().unwrap().led();
    led.enable();
    led.set_color(RGB12::new(0, 0, 8));

    let mut uart = UART1::take().unwrap().serial(serial::Config::default());
    uart.rx().listen();

    let autocomplete = StaticAutocomplete(["color", "clear", "help", "status", "on", "off", "mw", "mr"]);
    let history = LRUHistory::default();
    let shell = UShell::new(uart, autocomplete, history);

    interrupt::free(|_| unsafe {
        STATE.replace((shell, Env::new(led)));
        arch::register::mstatus::set_mie();
        arch::register::mie::set_mext();
    });

    loop {}
}

#[export_name = "MachineExternal"]
fn uart_interrupt() {
    interrupt::free(|_| unsafe {
        STATE.as_mut().map(|(shell, env)| shell.spin(env));
    });
}
