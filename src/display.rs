use std::fmt;

// screen dimensions in pixels
pub const SCREEN_WIDTH_PIXELS : u16 = 64;
pub const SCREEN_HEIGHT_PIXELS: u16 = 32;

// Each sprite is 8 pixels wide and up to 15 pixels height.
pub const SPRITE_PIXELS_WIDTH : u16 = 8;

// screen dimensions in sprites
pub const SCREEN_WIDTH_SPRITES : u16 = SCREEN_WIDTH_PIXELS / SPRITE_PIXELS_WIDTH;

#[derive(Debug)]
pub struct Display {
    // The screen is stored as as array of bytes. Even though the display
    // is monochrome and we are only using 1 bit value.
    pub mem : [[u8; SCREEN_WIDTH_PIXELS as usize] ; SCREEN_HEIGHT_PIXELS as usize]
}



impl Display {
    pub fn new() -> Display {
        Display { mem: [[0u8; SCREEN_WIDTH_PIXELS as usize]; SCREEN_HEIGHT_PIXELS as usize] }
    }

    // Draws sprite of at given  x,y position. If the draw operation changes any existing values, then returns true, otherwise returns false
    pub fn draw_sprite(&mut self, x: u8, y: u8, sprite: Vec<u8>) -> bool {
        let mut screen_set = false;
        // Read current screen values. Place into a byte representing pixel values.

        // For each sprite value index i
        // Start at x position at row y + i, read SPRITE_PIXEL_WIDTH values from screen memory. 
        // Pack this into a byte.
        // xor the byte above with the current the sprite value byte
        // Store the value back into screen. 
        

        println!("Display::draw_sprite x: {}, y: {}, sprite: {:?}", x, y, sprite);

        for i in 0..sprite.len() {
            let mut old_screen_row = 0u8;
            for j in 0..SPRITE_PIXELS_WIDTH {
                let row_index = (y+ (i as u8)) as usize;
                let col_index = (x+(j as u8)) as usize;
                println!("row_index: {}, col_index: {}", row_index, col_index);
                let pixel_value = self.mem[row_index % (SCREEN_HEIGHT_PIXELS as usize)][col_index % (SCREEN_WIDTH_PIXELS as usize)];
                old_screen_row |= pixel_value << (SPRITE_PIXELS_WIDTH - j - 1);
            }
            let new_screen_row = old_screen_row ^ sprite[i];
            // If there is difference in values after xoring sprite
            if new_screen_row == old_screen_row {
                // Update the display with new values.
                for j in 0..SPRITE_PIXELS_WIDTH {
                    self.mem[(y+ (i as u8)) as usize][(x+(j as u8)) as usize] = 
                       (new_screen_row >> (SPRITE_PIXELS_WIDTH - j -1)) & 0x1;
                }
                // Mark the operation as having changed the display
                screen_set = true;
            }
        }
        screen_set
    } 
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.mem)
    }
}

