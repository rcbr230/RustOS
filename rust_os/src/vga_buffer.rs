// vga_buffer.rs

// Purpose: Write strings to the screen with VGA_Buffer in a "simpler" manner.


#[allow(dead_code)] // ignore warnings for unused stuff bc annoying
#[derive(Debug, Clone, Copy, PartialEq, Eq)] // enable copy semantics for the type and make it printable and comparable. (bit unclear)
#[repr(u8)]

// enumerator for colors based on the VGA buffer info.
pub enum Color{
	Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode{
    fn new(foreground: Color, background: Color) -> ColorCode{
        ColorCode((foreground as u8) | (background as u8))
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)] // field ordering is usually undefined in rust, this sets up the struct to be like C to get correct ordering. (RAM stuff that isn't a huge deal here but whatever)
struct ScreenChar{
    ascii_character: u8, // actual character in our text
    color_code: ColorCode, // represent our bg and fg
}

// these values come from the actual vga buffer restrictions
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

// array that records what the user is writing
#[repr(transparent)] // ensure that it has the same memory layout as its single field
struct Buffer{
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}


struct Writer{
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer, // 'static means it's valid for whole program run time.
}

impl Writer{
    // write a single byte
    fn write_byte(&mut self, byte: u8){
        match byte{
            b'\n' => self.new_line(),       // new line case
            byte => {                       // every other case

                // check if at the end of the buffer length, if so then move to the next line
                if self.column_position >= BUFFER_WIDTH{
                    self.new_line();
                }

                // get current positions.
                let col = self.column_position;
                let row = BUFFER_HEIGHT - 1;

                let color_code = self.color_code;
                self.buffer.chars[row][col] = ScreenChar{
                    ascii_character: byte,
                    color_code
                };
                self.column_position += 1;
            }
        }
    }


    fn new_line(&mut self){ /* TBD */ }
}


impl Writer{
    // write entire strings by using write_byte
    fn write_string(&mut self, s: &str){
        for byte in s.bytes(){
            match byte{
                0x20..=0x7e | b'\n' => self.write_byte(byte), // printable ascii or new line
                _ => self.write_byte(0xfe) // 0xfe = ■
            }
        }
    }
}


pub fn WriteSomething(s: &str){
    let mut writer = Writer{
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_string(s);
}