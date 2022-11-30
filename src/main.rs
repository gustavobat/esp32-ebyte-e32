use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::units::Hertz;
use esp_idf_hal::serial::*;
use esp_idf_hal::delay;

use std::thread;
use std::time::Duration;

use ebyte_e32::{
    mode::Normal,
    parameters::{AirBaudRate, Persistence},
    Ebyte,
};
use embedded_hal::{
    blocking::delay::DelayMs,
    digital::v2::{InputPin, OutputPin},
    serial,
};

use std::fmt::Debug;

#[cfg(feature = "receiver")]
pub fn receive_lora_message<S, AUX, M0, M1, D>(mut ebyte: Ebyte<S, AUX, M0, M1, D, Normal>)
    where
    S: serial::Read<u8> + serial::Write<u8>,
    <S as serial::Read<u8>>::Error: Debug,
    <S as serial::Write<u8>>::Error: Debug,
    AUX: InputPin,
    M0: OutputPin,
    M1: OutputPin,
    D: DelayMs<u32>,
{
    let model_data = ebyte.model_data().unwrap();
    let mut params = ebyte.parameters().unwrap();

    println!("{model_data:#?}");

    params.air_rate = AirBaudRate::Bps300;
    params.channel = 23;

    ebyte.set_parameters(&params, Persistence::Temporary).unwrap();

    let params = ebyte.parameters().unwrap();
    println!("{:#?}", params);

    loop {
        let mut received_message_buf: [u8; 9] = [0; 9];
        let result = ebyte.read_buffer(&mut received_message_buf);
        if result.is_ok() {
            let string = String::from_utf8_lossy(&received_message_buf);
            println!("Message received: {}", string);
        }
        else {
            println!("Message not received");
        }
        thread::sleep(Duration::from_millis(100));
    }
}

#[cfg(feature = "transmitter")]
pub fn transmit_lora_message<S, AUX, M0, M1, D>(mut ebyte: Ebyte<S, AUX, M0, M1, D, Normal>)
    where
    S: serial::Read<u8> + serial::Write<u8>,
    <S as serial::Read<u8>>::Error: Debug,
    <S as serial::Write<u8>>::Error: Debug,
    AUX: InputPin,
    M0: OutputPin,
    M1: OutputPin,
    D: DelayMs<u32>,
{
    let model_data = ebyte.model_data().unwrap();
    let mut params = ebyte.parameters().unwrap();

    println!("{model_data:#?}");
    println!("{params:#?}");

    params.air_rate = AirBaudRate::Bps300;
    params.channel = 23;

    ebyte
        .set_parameters(&params, Persistence::Temporary)
        .unwrap();

    let params = ebyte.parameters().unwrap();
    println!("{:#?}", params);

    let mut i : u32 = 0;
    loop {
        let str = format!("Hello {}", i);
        println!("Sending message: '{}'", str);
        ebyte.write_buffer(str.as_bytes()).unwrap();
        i = i + 1;
        thread::sleep(Duration::from_millis(5000));
    }
}

fn main() {
    esp_idf_sys::link_patches();
    let peripherals = Peripherals::take().unwrap();

    let pins = peripherals.pins;


    let config = config::Config::default().baudrate(Hertz(9_600));

    let serial: Serial<UART2, _, _> = Serial::new(
        peripherals.uart2,
        Pins {
            tx: pins.gpio17,
            rx: pins.gpio16,
            cts: None,
            rts: None,
        },
        config
        ).unwrap();

    let aux = pins.gpio4.into_input().unwrap();
    let m0 = pins.gpio18.into_output().unwrap();
    let m1 = pins.gpio5.into_output().unwrap();

    let mut d = delay::Ets;
    d.delay_ms(500 as u32);
    let ebyte = Ebyte::new(serial, aux, m0, m1, d).unwrap();
    
    #[cfg(feature = "transmitter")]
    transmit_lora_message(ebyte);

    #[cfg(feature = "receiver")]
    receive_lora_message(ebyte);
}

