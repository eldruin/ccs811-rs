use ccs811::{prelude::*, Error};
use embedded_hal_mock::{
    i2c::Transaction as I2cTrans,
    pin::{Mock as PinMock, State as PinState, Transaction as PinTrans},
};
mod common;
use crate::common::{destroy, new, BitFlags as BF, Register, DEV_ADDR};

#[test]
fn can_start_app_mode() {
    let nwake = PinMock::new(&[PinTrans::set(PinState::Low), PinTrans::set(PinState::High)]);
    let transactions = [
        I2cTrans::write_read(DEV_ADDR, vec![Register::STATUS], vec![BF::APP_VALID]),
        I2cTrans::write(DEV_ADDR, vec![Register::APP_START]),
        I2cTrans::write_read(DEV_ADDR, vec![Register::STATUS], vec![0]),
    ];
    let sensor = new(&transactions, nwake);
    let sensor = sensor.start_application().ok().unwrap();
    destroy(sensor);
}

#[test]
fn cannot_start_app_mode_invalid_app() {
    let nwake = PinMock::new(&[PinTrans::set(PinState::Low), PinTrans::set(PinState::High)]);
    let transactions = [I2cTrans::write_read(
        DEV_ADDR,
        vec![Register::STATUS],
        vec![0],
    )];
    let sensor = new(&transactions, nwake);
    let result = sensor.start_application().err().unwrap();
    match result.error {
        Error::NoValidApp => (),
        _ => panic!("Invalid error"),
    }
    destroy(result.dev);
}

#[test]
fn can_verify_app() {
    let nwake = PinMock::new(&[
        PinTrans::set(PinState::Low),
        PinTrans::set(PinState::High),
        PinTrans::set(PinState::Low),
        PinTrans::set(PinState::High),
        PinTrans::set(PinState::Low),
        PinTrans::set(PinState::High),
    ]);
    let transactions = [
        I2cTrans::write_read(DEV_ADDR, vec![Register::STATUS], vec![0]),
        I2cTrans::write(DEV_ADDR, vec![Register::APP_VERIFY]),
        I2cTrans::write_read(DEV_ADDR, vec![Register::STATUS], vec![0]),
        I2cTrans::write_read(DEV_ADDR, vec![Register::STATUS], vec![BF::APP_VERIFY]),
    ];
    let mut sensor = new(&transactions, nwake);
    sensor
        .verify_application()
        .expect_err("Should have returned nb::Error::WouldBlock");
    sensor
        .verify_application()
        .expect_err("Should have returned nb::Error::WouldBlock");
    sensor.verify_application().unwrap();
    destroy(sensor);
}

#[test]
fn can_erase_app() {
    let nwake = PinMock::new(&[
        PinTrans::set(PinState::Low),
        PinTrans::set(PinState::High),
        PinTrans::set(PinState::Low),
        PinTrans::set(PinState::High),
        PinTrans::set(PinState::Low),
        PinTrans::set(PinState::High),
    ]);
    let transactions = [
        I2cTrans::write_read(DEV_ADDR, vec![Register::STATUS], vec![0]),
        I2cTrans::write(DEV_ADDR, vec![Register::APP_ERASE, 0xE7, 0xA7, 0xE6, 0x09]),
        I2cTrans::write_read(DEV_ADDR, vec![Register::STATUS], vec![0]),
        I2cTrans::write_read(DEV_ADDR, vec![Register::STATUS], vec![BF::APP_ERASE]),
    ];
    let mut sensor = new(&transactions, nwake);
    sensor
        .erase_application()
        .expect_err("Should have returned nb::Error::WouldBlock");
    sensor
        .erase_application()
        .expect_err("Should have returned nb::Error::WouldBlock");
    sensor.erase_application().unwrap();
    destroy(sensor);
}