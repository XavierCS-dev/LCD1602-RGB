#![no_std]

mod consts;

use consts::*;
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::blocking::i2c;

// display settings on LCD startup
// N = 1, 2 line display
// F=0, 5x8 dots font
// D=1, Display on

// reset routine triggered automatically upon startup, with settings:
// N=1, 2-line display
// Display clear
// DL=1, 8-bit interface
// F=0, 5x8 dot character font
// D=0, Display off
// C=0, Cursor off
// B=0, Blinking off
// I/D=1, increment by 1
// S=0, No shift
// Make sure to set the basic settings

pub struct Display<I, D>
where
    I: i2c::Write,
    D: DelayMs<u16>,
{
    device: I,
    lcd_addr: u8,
    rgb_addr: u8,
    pub delay: D,
}

impl<I, D> Display<I, D>
where
    I: i2c::Write,
    D: DelayMs<u16>,
{
    /// Creates a display struct and initialises the physical i2c display.
    pub fn new(i2c_dev: I, delay: D) -> Result<Display<I, D>, I::Error> {
        let mut display = Display {
            device: i2c_dev,
            lcd_addr: 62,
            rgb_addr: 96,
            delay,
        };
        display.initialise()?;
        Ok(display)
    }

    fn initialise(&mut self) -> Result<(), I::Error> {
        self.try_write_init_values()?;
        self.write_lcd(LCD_DISPLAYCONTROL | LCD_DISPLAYON)?;

        self.delay.delay_ms(10);

        // initialise mode1 rgb register, PCA9633 does not respond to I2C subaddresses or All Call address, and is set to normal mode
        self.write_rgb(REG_MODE1, 0x00)?;

        // TODO: IDENTIFY DIFFERENCE BETWEEN BLINKING MODE AND DIMMING MODE (SET to 0x00 for dimming)
        // set group control to blinking mode
        self.write_rgb(REG_MODE2, 0x20)?;
        // LDR0-3 set to 11, meaning each driver can be controlled through its PWM and GRPPWM registers
        self.write_rgb(REG_OUTPUT, 0xFF)?;
        //self.on()
        self.clear()?;
        self.backlight_colour(0, 0, 0)?;
        Ok(())
    }

    fn try_write_init_values(&mut self) -> Result<(), I::Error> {
        self.delay.delay_ms(50);
        // Try three times
        for _ in 0..3 {
            // Set display to move left to right
            self.write_lcd(LCD_ENTRYMODESET | LCD_ENTRYLEFT | LCD_ENTRYSHIFTDECREMENT)?;
            self.delay.delay_ms(10);
            self.write_lcd(LCD_FUNCTIONSET | LCD_5X8DOTS | LCD_8BITMODE | LCD_2LINE)?;
            self.delay.delay_ms(10);
        }
        Ok(())
    }

    /// Write data directly to registers of the device.
    /// Specify a write address, then sub-address with data.
    /// Marked as unsafe as writing to the incorrect memory addresses can have unintended side effects.
    pub unsafe fn write_data(&mut self, address: u8, bytes: &[u8]) -> Result<(), I::Error> {
        self.device.write(address, bytes)?;
        Ok(())
    }

    /// Write command to the LCD.
    fn write_lcd(&mut self, cmd: u8) -> Result<(), I::Error> {
        self.device.write(self.lcd_addr, &[LCD_SETDDRAMADDR, cmd])?;
        Ok(())
    }

    /// Insert a string at the current cursor position, can overflow.
    pub fn write_string(&mut self, string: &str) -> Result<(), I::Error> {
        for character in string.chars() {
            self.write_char(character)?;
        }
        Ok(())
    }

    /// Insert a character into the current position.
    pub fn write_char(&mut self, character: char) -> Result<(), I::Error> {
        self.device.write(self.lcd_addr, &[0x40, character as u8])?;
        self.delay.delay_ms(5);
        Ok(())
    }

    /// Write the the RGB register
    fn write_rgb(&mut self, register: u8, data: u8) -> Result<(), I::Error> {
        self.device.write(self.rgb_addr, &[register, data])?;
        Ok(())
    }

    /// Clear the LCD
    pub fn clear(&mut self) -> Result<(), I::Error> {
        self.write_lcd(LCD_CLEARDISPLAY)?;
        self.delay.delay_ms(5);
        Ok(())
    }

    /// Set the values of the RGB backlight: 0-255.
    pub fn backlight_colour(&mut self, r: u8, g: u8, b: u8) -> Result<(), I::Error> {
        self.write_rgb(REG_RED, r)?;
        self.write_rgb(REG_GREEN, g)?;
        self.write_rgb(REG_BLUE, b)?;
        Ok(())
    }

    /// Write the the first line, can overflow.
    pub fn write_line_one(&mut self, string: &str) -> Result<(), I::Error> {
        self.device
            .write(self.lcd_addr, &[LCD_SETDDRAMADDR, 0x80])?;
        self.write_string(string)?;
        Ok(())
    }

    /// Write to the second line, can overflow.
    pub fn write_line_two(&mut self, string: &str) -> Result<(), I::Error> {
        self.device
            .write(self.lcd_addr, &[LCD_SETDDRAMADDR, 0xc0])?;
        self.write_string(string)?;
        Ok(())
    }

    /// Set the current cursor position, to be used in conjuction with `write_char()` and `write_string()`.
    pub fn set_cursor(&mut self, row: u8, column: u8) -> Result<(), I::Error> {
        if column > 15 {
            panic!("Column index out of range, expected 0-15, got {}", column);
        }
        if row == 0 {
            self.device
                .write(self.lcd_addr, &[LCD_SETDDRAMADDR, column | 0x80])
        } else if row == 1 {
            self.device
                .write(self.lcd_addr, &[LCD_SETDDRAMADDR, column | 0xc0])
        } else {
            panic!("Row index out of range, expected 0-1, got {}", row);
        }
    }

    // check whether writing out of bounds when pushing characters matters
    /// Writes text to the display utilising both lines. Will not order words so they fit completely on the same line.
    pub fn write_text(&mut self, string: &str) -> Result<(), I::Error> {
        self.device
            .write(self.lcd_addr, &[LCD_SETDDRAMADDR, 0x80])?;
        let string_len = string.len();
        let mut str_iter = string.chars();
        if string_len <= 16 {
            for character in str_iter {
                self.write_char(character)?;
            }
        } else {
            for _ in 0..16 {
                self.write_char(str_iter.next().unwrap())?;
            }
            self.device
                .write(self.lcd_addr, &[LCD_SETDDRAMADDR, 0xc0])?;
            for character in str_iter {
                self.write_char(character)?;
            }
        }
        Ok(())
    }
}
