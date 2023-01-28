#![no_std]

/*
	pub constants as specified by the HS310140A and AiP31068 Data sheets
	Ported from lcd1602_RGB_Module.h from waveshare pico C demo
*/




pub const WHITE: u8 = 0;
pub const RED: u8 = 1;
pub const GREEN: u8 = 2;
pub const BLUE: u8 = 3;

pub const REG_RED: u8 = 0x04;        // pwm2
pub const REG_GREEN: u8 = 0x03;        // pwm1
pub const REG_BLUE: u8 = 0x02;        // pwm0

pub const REG_MODE1: u8 = 0x00;
pub const REG_MODE2: u8 = 0x01;
pub const REG_OUTPUT: u8 = 0x08;


///  commands
pub const LCD_CLEARDISPLAY: u8 = 0x01;
pub const LCD_RETURNHOME: u8 = 0x02;
pub const LCD_ENTRYMODESET: u8 = 0x04;
pub const LCD_DISPLAYCONTROL: u8 = 0x08;
pub const LCD_CURSORSHIFT: u8 = 0x10;
pub const LCD_FUNCTIONSET: u8 = 0x20;
pub const LCD_SETCGRAMADDR: u8 = 0x40;
pub const LCD_SETDDRAMADDR: u8 = 0x80;

/// flags for display entry mode
pub const LCD_ENTRYRIGHT: u8 = 0x00;
pub const LCD_ENTRYLEFT: u8 = 0x02;
pub const LCD_ENTRYSHIFTINCREMENT: u8 = 0x01;
pub const LCD_ENTRYSHIFTDECREMENT: u8 = 0x00;

/// flags for display on/off control
pub const LCD_DISPLAYON: u8 = 0x04;
pub const LCD_DISPLAYOFF: u8 = 0x00;
pub const LCD_CURSORON: u8 = 0x02;
pub const LCD_CURSOROFF: u8 = 0x00;
pub const LCD_BLINKON: u8 = 0x01;
pub const LCD_BLINKOFF: u8 = 0x00;

/// flags for display/cursor shift
pub const LCD_DISPLAYMOVE: u8 = 0x08;
pub const LCD_CURSORMOVE: u8 = 0x00;
pub const LCD_MOVERIGHT: u8 = 0x04;
pub const LCD_MOVELEFT: u8 = 0x00;

/// flags for function set
pub const LCD_8BITMODE: u8 = 0x10;
pub const LCD_4BITMODE: u8 = 0x00;
pub const LCD_2LINE: u8 = 0x08;
pub const LCD_1LINE: u8 = 0x00;
pub const LCD_5X8DOTS: u8 = 0x00;