//
// sprocketnes/rom.rs
//
// Author: Patrick Walton
//

use util;

use std::fs::File;
use std::path::Path;
use std::vec::Vec;

pub struct Rom {
    pub header: INesHeader,
    pub prg: Vec<u8>,         // PRG-ROM
    pub chr: Vec<u8>,         // CHR-ROM
}

impl Rom {
    fn from_file(file: &mut File) -> Rom {
        let mut buffer = [ 0; 16 ];
        util::read_to_buf(&mut buffer, file).unwrap();

        let header = INesHeader {
            magic: [
                buffer[0],
                buffer[1],
                buffer[2],
                buffer[3],
            ],
            prg_rom_size: buffer[4],
            chr_rom_size: buffer[5],
            flags_6: buffer[6],
            flags_7: buffer[7],
            prg_ram_size: buffer[8],
            flags_9: buffer[9],
            flags_10: buffer[10],
            zero: [ 0; 5 ]
        };

        assert!(header.magic == [
            'N' as u8,
            'E' as u8,
            'S' as u8,
            0x1a,
        ]);

        let mut prg_rom = vec![ 0u8; header.prg_rom_size as usize * 16384 ];
        util::read_to_buf(&mut prg_rom, file).unwrap();
        let mut chr_rom = vec![ 0u8; header.chr_rom_size as usize * 8192 ];
        util::read_to_buf(&mut chr_rom, file).unwrap();

        Rom {
            header: header,
            prg: prg_rom,
            chr: chr_rom,
        }
    }

    pub fn from_path(path: &Path) -> Rom {
        Rom::from_file(&mut File::open(path).unwrap())
    }
}

pub struct INesHeader {
    pub magic: [u8; 4],   // 'N' 'E' 'S' '\x1a'
    pub prg_rom_size: u8,   // number of 16K units of PRG-ROM
    pub chr_rom_size: u8,   // number of 8K units of CHR-ROM
    pub flags_6: u8,
    pub flags_7: u8,
    pub prg_ram_size: u8,   // number of 8K units of PRG-RAM
    pub flags_9: u8,
    pub flags_10: u8,
    pub zero: [u8; 5],    // always zero
}

impl INesHeader {
    pub fn mapper(&self) -> u8 {
        (self.flags_7 & 0xf0) | (self.flags_6 >> 4)
    }
    pub fn ines_mapper(&self) -> u8 {
        self.flags_6 >> 4
    }
    pub fn trainer(&self) -> bool {
        (self.flags_6 & 0x04) != 0
    }

    pub fn to_str(&self) -> String {
        (format!("PRG-ROM size: {}\nCHR-ROM size: {}\nMapper: {}/{}\nTrainer: {}",
                 self.prg_rom_size as isize,
                 self.chr_rom_size as isize,
                 self.mapper() as isize,
                 self.ines_mapper() as isize,
                 if self.trainer() {
                     "Yes"
                 } else {
                     "No"
                 })).to_string()
    }
}
