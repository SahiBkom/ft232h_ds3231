use ftdi_embedded_hal as hal;
use hal::x232h::FTx232H;
use ds323x::{Ds323x, NaiveDate, Rtcc};

fn main() {
    let dev = FTx232H::init(0x0403, 0x6014).unwrap();
    let i2c = dev.i2c(hal::i2c::I2cSpeed::CLK_400kHz).unwrap();

    let mut rtc = Ds323x::new_ds3231(i2c);
    let datetime = NaiveDate::from_ymd(2020, 5, 1).and_hms(19, 59, 58);
    rtc.set_datetime(&datetime).unwrap();
    // do something else...
    let time = rtc.get_time().unwrap();
    println!("Time: {}", time);

    let _dev = rtc.destroy_ds3231();
}
