//! # GPIO 'Blinky' Example
//!
//! This application demonstrates how to control a GPIO pin on the RP2040.
//!
//! It may need to be adapted to your particular board layout and/or pin assignment.
//!
//! See the `Cargo.toml` file for Copyright and license details.

#![no_std]
#![no_main]
extern crate panic_halt;
extern crate embedded_hal;
extern crate rp2040_hal;
extern crate core as rust_core;
extern crate critical_section;
// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// Alias for our HAL crate
use rp2040_hal as hal;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access

use hal::{pac, gpio::Interrupt};
use rust_core::cell::RefCell;
use critical_section::Mutex;

// Some traits we need
use embedded_hal::digital::v2::{OutputPin, InputPin};
use rp2040_hal::clocks::Clock;

/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
/// Note: This boot block is not necessary when using a rp-hal based BSP
/// as the BSPs already perform this step.
#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

const XTAL_FREQ_HZ: u32 = 12_000_000u32;

type RedLedPin = hal::gpio::Pin<hal::gpio::bank0::Gpio13, hal::gpio::FunctionSioOutput, hal::gpio::PullNone>;
type YellowLedPin = hal::gpio::Pin<hal::gpio::bank0::Gpio12, hal::gpio::FunctionSioOutput, hal::gpio::PullNone>;
type GreenLedPin = hal::gpio::Pin<hal::gpio::bank0::Gpio11, hal::gpio::FunctionSioOutput, hal::gpio::PullNone>;

type ButtonPin = hal::gpio::Pin<hal::gpio::bank0::Gpio14, hal::gpio::FunctionSioInput, hal::gpio::PullUp>;

type LedsAndButton = (RedLedPin, YellowLedPin, GreenLedPin, ButtonPin);

static GLOBAL_PINS: Mutex<RefCell<Option<LedsAndButton>>> = Mutex::new(RefCell::new(None));

#[rp2040_hal::entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins to their default state
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Set high all pins exept 3
    let mut button = pins.gpio14.reconfigure();
    let mut green_led = pins.gpio11.reconfigure();
    let mut yellow_led = pins.gpio12.reconfigure();
    let mut red_led = pins.gpio13.reconfigure();
    let mut state = 0;

    button.set_interrupt_enabled(Interrupt::EdgeLow, true);

    critical_section::with(|cs| {
        GLOBAL_PINS.borrow(cs).replace(Some((red_led, yellow_led, green_led, button)));
    });

    loop {   
        
    }
}

// End of file
