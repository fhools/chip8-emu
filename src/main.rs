/* Emulator for CHIP8 CPU */

use std::io::BufWriter;
use std::path::Path;
use std::time::Duration;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;

use clap::{App, Arg};

mod cpu;
mod rom;
mod system;
mod display;
use system::System;

/*
fn decode_instr(instr: u16) -> String {
    let bits15_12 = instr >> 12;
    let bits11_8 = (instr >> 8) & 0xF;
    let bits7_4 = (instr >> 4) & 0xF;
    let bits3_0 = (instr) & 0xF;
    let nnn = (bits11_8 << 8) | (bits7_4 << 4) | bits3_0;
    let result : String;
    match bits15_12 {
        // CLR
        // RET
        0x0 => {
            if bits11_8  == 0 {
               if bits7_4 == 0xE {
                   if bits3_0 == 0 {
                       result = "CLS".to_string()
                   } else if bits3_0 == 0xE {
                       result = "RET".to_string()
                   } else {
                       result = "UNSUPPORTED".to_string()
                   }
               } else {
                   result = "UNSUPPORTED".to_string()
                }
            } else {
                 let sys = format!("SYS {:x}",
                                         nnn);
                 result = sys
            }
        },
        // JP NNN
        0x1 => {
            let jp = format!("JP {:x}", nnn);
            result = jp
        },
        // CALL NNN
        0x2 => {
            let call = format!("CALL {:x}", nnn);
            result = call
        },
        // SE Vx, kk --- Skip next instruction if Vx == kk
        0x3 => {
            let byte = bits7_4 << 4 | bits3_0;
            let skipeq = format!("SE V{:x}, {:x}", bits11_8, byte);
            result = skipeq
        },
        // SNE Vx, kk --- Skip next instruction if Vx != kk
        0x4 => {
            let byte = bits7_4 << 4 | bits3_0;
            let skipneq = format!("SNE V{:x}, {:x}", bits11_8, byte);
            result = skipneq
        },
        0x5 => {
            if bits3_0 == 0 {
                let se_vx_vy = format!("SE V{:x}, V{:x}", bits11_8, bits7_4);
                result = se_vx_vy;
            } else {
                let unsupported_5xxx = format!("unsupported 5 instr:{:x}", instr);
                result = unsupported_5xxx;
            }
        }
        // LD Vx, KK
        0x6 => {
            let byte = bits7_4 << 4 | bits3_0;
            let ld = format!("LD V{:X}, {:x}", bits11_8, byte);
            result = ld
        },
        0x7 => {
            let byte = bits7_4 << 4 | bits3_0;
            let add = format!("ADD V{:X}, {:x}", bits11_8, byte);
            result = add
        },
        // LD Vx,Vy and ALU instructions
        0x8 => {
            match bits3_0 {
                0x0 => {
                    let ld_vx_vy = format!("LD V{:x}, V{:x}", bits11_8, bits7_4);
                    result = ld_vx_vy;
                },
                0x1 => {
                    let or = format!("OR V{:x}, V{:x}", bits11_8, bits7_4);
                    result = or;
                },
                0x2 => {
                    let or = format!("AND V{:x}, V{:x}", bits11_8, bits7_4);
                    result = or;
                },
                0x3 => {
                    let or = format!("XOR V{:x}, V{:x}", bits11_8, bits7_4);
                    result = or;
                },
                0x4 => {
                    let or = format!("ADD V{:x}, V{:x}", bits11_8, bits7_4);
                    result = or;
                },
                0x5 => {
                    let or = format!("SUB V{:x}, V{:x}", bits11_8, bits7_4);
                    result = or;
                },
                0x6 => {
                    let or = format!("SHR V{:x}, V{:x}", bits11_8, bits7_4);
                    result = or;
                },
                0x7 => {
                    let or = format!("SUBN V{:x}, V{:x}", bits11_8, bits7_4);
                    result = or;
                },
                0xE => {
                    let or = format!("SHL V{:x}, V{:x}", bits11_8, bits7_4);
                    result = or;
                },
                _ => {
                    let unsupported = format!("unsupported {:x}", instr);
                    result = unsupported
                }
            }
        },
        0x9 => {
            let sne_vx_vy = format!("SNE V{:x}, V{:x}", bits11_8, bits7_4);
            result = sne_vx_vy
        },
        0xA => {
            let ld_i = format!("LD I, {:x}", nnn);
            result = ld_i
        },
        0xD => {
            let draw = format!("DRW V{:X}, V{:X}, {:x}", bits11_8, bits7_4, bits3_0);
            result = draw;
        },
        0xE => {
            let lsbyte = bits7_4 << 4 | bits3_0;
            match lsbyte {
                0x9E => {
                    let skip_key_vx = format!("SKP V{:x}", bits11_8);
                    result = skip_key_vx
                },
                0xA1 => {
                    let skip_key_vx = format!("SKP V{:x}", bits11_8);
                    result = skip_key_vx
                },
                _ => {
                    let unsupported_e = format!("unsupported E instr {}", instr);
                    result = unsupported_e
                }
            }
        },


        0xF => {
            let f_opcode = bits7_4 << 4 | bits3_0;
            match f_opcode {
                0x07 => {
                    let ld_dt = format!("LD V{:x}, DT", bits11_8);
                    result = ld_dt
                },
                0x0A => {
                    let ld_key = format!("LD V{:x}, K", bits11_8);
                    result = ld_key
                },
                0x15 => {
                    let store_dt = format!("LD DT, V{:x}", bits11_8);
                    result = store_dt
                },
                0x18 => {
                    let store_st = format!("LD ST, V{:x}", bits11_8);
                    result = store_st
                },
                0x1E => {
                    let add_i = format!("ADD I, V{:x}", bits11_8);
                    result = add_i
                },
                0x29 => {
                    let store_sprite_i = format!("LD F, V{:x}", bits11_8);
                    result = store_sprite_i
                },
                0x33 => {
                    let store_bcd_i = format!("LD B, V{:x}", bits11_8);
                    result = store_bcd_i
                },
                0x55 => {
                    let ld_vx_into_i = format!("LD [I], V{:x}", bits11_8);
                    result = ld_vx_into_i;
                },
                0x65 => {
                    let ld_i_into_vx = format!("LD V{:x}, [i]", bits11_8);
                    result = ld_i_into_vx;
                },
                _ => {
                    let unsupported_f = format!("unsupported F instr: {}", instr);
                    result = unsupported_f
                }
            }
        },

        _ => {
            let unsupported = format!("unsupported {:x}", instr);
            result = unsupported
        }
    }
    result
}
*/

fn main() -> Result<(), String> {
    let sdl2_context = sdl2::init()?;
    let video_subsystem = sdl2_context.video()?;

    let window  = video_subsystem.window("CHIP8 Emulator", 320, 200)
        .position_centered()
        .build()
        .expect("could not initialize sdl2 video_subsystem");
    
    let mut canvas = window.into_canvas().build().expect("could not make sdl2 canvas");

    let mut event_pump = sdl2_context.event_pump()?;

    let mut buf = BufWriter::new(Vec::new());

    let mut app = App::new("CHIP8 Disassembler")
        .version("0.1.0")
        .author("fhools")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .help("filepath to ROM"),
        );

    app.write_long_help(&mut buf).unwrap();
    let argmatches = app.get_matches();
    let bytes = buf.into_inner().unwrap();
    let helpmessage = String::from_utf8(bytes).unwrap();

    let rom_filepath = argmatches.value_of("file");
    let mut system = System::new();
    if let None = rom_filepath {
        println!("{}", helpmessage);
        std::process::exit(1);
    } else if let Some(filepath) = rom_filepath {
        let rom = rom::read_rom(Path::new(filepath));
        if let Err(err) = rom {
            println!("err: {}", err);
        } else if let Ok(rom) = rom {
            println!("read rom successfully");
            println!("rom size is {}", rom.size());
            system.load_rom(&rom);
          
        }
    }
    
   
    'running: loop {
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running;
                },
                _ => {}
            }
        }
        
        system.run_tick();
        canvas.set_draw_color(Color::RED);
        canvas.draw_point((0,0))?;
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

    }
    Ok(())

}
