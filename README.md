## LCD1602RGB Driver
Driver for the LCD1602RGB segmented LCD, it is not intended for use with other segmented LCDs, however you may be able to use this driver for some basic functionality.

I wrote this driver for my own personal use and will maintain it and add implement missing instructions when I need them, however feel free to submit a pull request.
Some functions are marked unsafe as they allow writing of arbitrary values to arbitrary addresses, that can result in unexpected behaviour.

This my first real embedded Rust project, and I used the following resources:

[PCA9633 Datasheet](https://www.nxp.com/docs/en/data-sheet/PCA9633.pdf)
[AiP21068 Datasheet](https://support.newhavendisplay.com/hc/en-us/article_attachments/4414498095511/AiP31068.pdf)
[Embedded HAL Docs](https://docs.rs/embedded-hal/0.2.3/embedded_hal/blocking/i2c/trait.Write.html)
[Raspberry Pi Pico Pinout Datasheet](https://datasheets.raspberrypi.com/pico/Pico-R3-A4-Pinout.pdf)
[LCD1602-RGB Datasheet](https://www.waveshare.com/w/upload/2/2e/LCD1602_RGB_Module.pdf)
[LCD1602 i2c driver by JohnSL](https://github.com/JohnSL/lcd_1602_i2c)
[Pi Pico Project Template](https://github.com/rp-rs/rp2040-project-template)

### Usage:

```Rust
let mut scl_pin = pins.gpio15.into_mode::<gpio::FunctionI2C>();
let mut sda_pin = pins.gpio14.into_mode::<gpio::FunctionI2C>();
let i2c_dev = i2c::I2C::new_controller(pac.I2C1, sda_pin, scl_pin, 400_u32.kHz(), &mut pac.RESETS, clocks.system_clock.freq());
let mut display_controller = Display::new(i2c_dev, 62 as u8, 96 as u8, delay).unwrap();
display_controller.write_text("Hello, World! How are you?").unwrap();
let mut r: u8 = 0;
let mut g: u8 = 255;
let mut b: u8 = 128;
let mut r_set = true;
let mut g_set = true;
let mut b_set = true;

loop {
    if r == 255 {
        r_set = false;
    } else if r == 0 {
        r_set = true;
    }
    if r_set {
        r += 1;
    } else {
        r -= 1;
    }
    if g == 255 {
        g_set = false;
    } else if g == 0 {
        g_set = true;
    }
    if g_set {
        g += 1;
    } else {
        g -= 1;
    }
    if b == 255 {
        b_set = false;
    } else if b == 0 {
        b_set = true;
    }
    if b_set {
        b += 1;
    } else {
        b -= 1;
    }
    display_controller.delay.delay_ms(10);
    display_controller.backlight_colour(r, g, b).unwrap();
}
```