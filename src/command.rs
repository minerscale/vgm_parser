#![allow(non_camel_case_types)]

use bytes::{Buf, BufMut, Bytes, BytesMut};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Command {
    AY8910StereoMask {
        value: u8,
    },
    GameGearPSGStereo {
        value: u8,
    },
    PSGWrite {
        value: u8,
    },
    YM2413Write {
        register: u8,
        value: u8,
    },
    YM2612Port0Write {
        register: u8,
        value: u8,
    },
    YM2612Port1Write {
        register: u8,
        value: u8,
    },
    YM2151Write {
        register: u8,
        value: u8,
    },
    YM2203Write {
        register: u8,
        value: u8,
    },
    YM2608Port0Write {
        register: u8,
        value: u8,
    },
    YM2608Port1Write {
        register: u8,
        value: u8,
    },
    YM2610Port0Write {
        register: u8,
        value: u8,
    },
    YM2610Port1Write {
        register: u8,
        value: u8,
    },
    YM3812Write {
        register: u8,
        value: u8,
    },
    YM3526Write {
        register: u8,
        value: u8,
    },
    Y8950Write {
        register: u8,
        value: u8,
    },
    YMZ280BWrite {
        register: u8,
        value: u8,
    },
    YMF262Port0Write {
        register: u8,
        value: u8,
    },
    YMF262Port1Write {
        register: u8,
        value: u8,
    },
    WaitNSamples {
        n: u16,
    },
    Wait735Samples,
    Wait882Samples,
    EndOfSoundData,
    DataBlock {
        data_type: u8,
        data_size: u32,
        data: Vec<u8>,
    },
    PCMRAMWrite {
        offset: u32,
        data: Vec<u8>,
    },
    WaitNSamplesPlus1 {
        n: u8,
    },
    YM2612Port0Address2AWriteWait {
        n: u8,
    },
    DACStreamControlWrite {
        register: u8,
        value: u8,
    },
    AY8910Write {
        register: u8,
        value: u8,
    },
    RF5C68Write {
        register: u8,
        value: u8,
    },
    RF5C164Write {
        register: u8,
        value: u8,
    },
    PWMWrite {
        register: u8,
        value: u16,
    },
    GameBoyDMGWrite {
        register: u8,
        value: u8,
    },
    NESAPUWrite {
        register: u8,
        value: u8,
    },
    MultiPCMWrite {
        register: u8,
        value: u8,
    },
    uPD7759Write {
        register: u8,
        value: u8,
    },
    OKIM6258Write {
        register: u8,
        value: u8,
    },
    OKIM6295Write {
        register: u8,
        value: u8,
    },
    HuC6280Write {
        register: u8,
        value: u8,
    },
    K053260Write {
        register: u8,
        value: u8,
    },
    PokeyWrite {
        register: u8,
        value: u8,
    },
    WonderSwanWrite {
        register: u8,
        value: u8,
    },
    SAA1099Write {
        register: u8,
        value: u8,
    },
    ES5506Write {
        register: u8,
        value: u8,
    },
    GA20Write {
        register: u8,
        value: u8,
    },
    SegaPCMWrite {
        offset: u16,
        value: u8,
    },
    MultiPCMSetBank {
        channel: u8,
        offset: u16,
    },
    QSoundWrite {
        register: u8,
        value: u16,
    },
    SCSPWrite {
        offset: u16,
        value: u8,
    },
    WonderSwanWrite16 {
        offset: u16,
        value: u8,
    },
    VSUWrite {
        offset: u16,
        value: u8,
    },
    X1010Write {
        offset: u16,
        value: u8,
    },
    YMF278BWrite {
        port: u8,
        register: u8,
        value: u8,
    },
    YMF271Write {
        port: u8,
        register: u8,
        value: u8,
    },
    SCC1Write {
        port: u8,
        register: u8,
        value: u8,
    },
    K054539Write {
        register: u16,
        value: u8,
    },
    C140Write {
        register: u16,
        value: u8,
    },
    ES5503Write {
        register: u16,
        value: u8,
    },
    ES5506Write16 {
        register: u8,
        value: u16,
    },
    SeekPCM {
        offset: u32,
    },
    C352Write {
        register: u16,
        value: u16,
    },

    // offset write
    RF5C68WriteOffset {
        offset: u16,
        value: u8,
    },
    RF5C164WriteOffset {
        offset: u16,
        value: u8,
    },
}

pub fn parse_commands(data: &mut Bytes) -> Vec<Command> {
    let mut commands = vec![];
    loop {
        let curr_command = Command::from_bytes(data);
        match curr_command {
            Ok(Command::EndOfSoundData) => {
                break;
            }
            Ok(c) => commands.push(c),
            Err(e) => panic!("unknown command: {e}"),
        }
    }

    commands
}

pub fn write_commands(buffer: &mut BytesMut, commands: &Vec<Command>) {
    for cmd in commands {
        buffer.put(&cmd.clone().to_bytes()[..]);
    }
}

impl Command {
    pub fn to_bytes(self) -> Vec<u8> {
        match self {
            Command::AY8910StereoMask { value } => {
                vec![0x31, value]
            }
            Command::GameGearPSGStereo { value } => {
                vec![0x4f, value]
            }
            Command::PSGWrite { value } => {
                vec![0x50, value]
            }
            Command::YM2413Write { register, value } => {
                vec![0x51, register, value]
            }
            Command::YM2612Port0Write { register, value } => {
                vec![0x52, register, value]
            }
            Command::YM2612Port1Write { register, value } => {
                vec![0x53, register, value]
            }
            Command::YM2151Write { register, value } => {
                vec![0x54, register, value]
            }
            Command::YM2203Write { register, value } => {
                vec![0x55, register, value]
            }
            Command::YM2608Port0Write { register, value } => {
                vec![0x56, register, value]
            }
            Command::YM2608Port1Write { register, value } => {
                vec![0x57, register, value]
            }
            Command::YM2610Port0Write { register, value } => {
                vec![0x58, register, value]
            }
            Command::YM2610Port1Write { register, value } => {
                vec![0x59, register, value]
            }
            Command::YM3812Write { register, value } => {
                vec![0x5A, register, value]
            }
            Command::YM3526Write { register, value } => {
                vec![0x5B, register, value]
            }
            Command::Y8950Write { register, value } => {
                vec![0x5C, register, value]
            }
            Command::YMZ280BWrite { register, value } => {
                vec![0x5D, register, value]
            }
            Command::YMF262Port0Write { register, value } => {
                vec![0x5E, register, value]
            }
            Command::YMF262Port1Write { register, value } => {
                vec![0x5F, register, value]
            }
            Command::WaitNSamples { n } => {
                let temp = n.to_le_bytes();
                vec![0x61, temp[0], temp[1]]
            }
            Command::Wait735Samples => {
                vec![0x62]
            }
            Command::Wait882Samples => {
                vec![0x63]
            }
            Command::EndOfSoundData => {
                vec![0x66]
            }

            Command::DataBlock {
                data_type,
                data_size,
                data,
            } => {
                let mut out_data: Vec<u8> = vec![data_type];
                out_data.extend(data_size.to_le_bytes());
                out_data.extend(data);
                out_data
            }
            Command::PCMRAMWrite { offset: _, data: _ } => {
                panic!("not implemented")
            }

            Command::WaitNSamplesPlus1 { n } => vec![0x70 + n],

            Command::YM2612Port0Address2AWriteWait { n } => vec![0x80 + n],

            Command::DACStreamControlWrite {
                register: _,
                value: _,
            } => {
                panic!("not implemented")
            }

            Command::AY8910Write { register, value } => {
                vec![0xA0, register, value]
            }
            Command::RF5C68Write { register, value } => {
                vec![0xB0, register, value]
            }
            Command::RF5C164Write { register, value } => {
                vec![0xB1, register, value]
            }
            Command::PWMWrite { register, value } => {
                let temp = value.to_le_bytes();
                vec![0xB2, register, temp[0], temp[1]]
            }
            Command::GameBoyDMGWrite { register, value } => {
                vec![0xB3, register, value]
            }
            Command::NESAPUWrite { register, value } => {
                vec![0xB4, register, value]
            }
            Command::MultiPCMWrite { register, value } => {
                vec![0xB5, register, value]
            }
            Command::uPD7759Write { register, value } => {
                vec![0xB6, register, value]
            }
            Command::OKIM6258Write { register, value } => {
                vec![0xB7, register, value]
            }
            Command::OKIM6295Write { register, value } => {
                vec![0xB8, register, value]
            }
            Command::HuC6280Write { register, value } => {
                vec![0xB9, register, value]
            }
            Command::K053260Write { register, value } => {
                vec![0xBA, register, value]
            }
            Command::PokeyWrite { register, value } => {
                vec![0xBB, register, value]
            }
            Command::WonderSwanWrite { register, value } => {
                vec![0xBC, register, value]
            }
            Command::SAA1099Write { register, value } => {
                vec![0xBD, register, value]
            }
            Command::ES5506Write { register, value } => {
                vec![0xBE, register, value]
            }
            Command::GA20Write { register, value } => {
                vec![0xBF, register, value]
            }
            Command::SegaPCMWrite { offset, value } => {
                let temp = offset.to_le_bytes();
                vec![0xC0, temp[0], temp[1], value]
            }
            Command::MultiPCMSetBank { channel, offset } => {
                let temp = offset.to_le_bytes();
                vec![0xC3, temp[0], temp[1], channel]
            }

            Command::QSoundWrite { register, value } => {
                let temp = value.to_le_bytes();
                vec![0xC4, temp[1], temp[0], register]
            }
            Command::SCSPWrite { offset, value } => {
                let temp = offset.to_le_bytes();
                vec![0xC5, temp[1], temp[0], value]
            }
            Command::WonderSwanWrite16 { offset, value } => {
                let temp = offset.to_le_bytes();
                vec![0xC6, temp[1], temp[0], value]
            }
            Command::VSUWrite { offset, value } => {
                let temp = offset.to_le_bytes();
                vec![0xC7, temp[1], temp[0], value]
            }
            Command::X1010Write { offset, value } => {
                let temp = offset.to_le_bytes();
                vec![0xC8, temp[1], temp[0], value]
            }

            Command::YMF278BWrite {
                port,
                register,
                value,
            } => {
                vec![0xD0, port, register, value]
            }

            Command::YMF271Write {
                port,
                register,
                value,
            } => {
                vec![0xD1, port, register, value]
            }
            Command::SCC1Write {
                port,
                register,
                value,
            } => {
                vec![0xD2, port, register, value]
            }
            Command::K054539Write { register, value } => {
                let temp = register.to_le_bytes();
                vec![0xD3, temp[0], temp[1], value]
            }
            Command::C140Write { register, value } => {
                let temp = register.to_le_bytes();
                vec![0xD4, temp[0], temp[1], value]
            }

            Command::ES5503Write { register, value } => {
                let temp = register.to_le_bytes();
                vec![0xD5, temp[0], temp[1], value]
            }
            Command::ES5506Write16 { register, value } => {
                let temp = value.to_le_bytes();
                vec![0xD6, register, temp[0], temp[1]]
            }
            Command::SeekPCM { offset } => {
                let mut rslt = vec![0xE0];
                rslt.extend(offset.to_le_bytes());
                rslt
            }
            Command::C352Write { register, value } => {
                let mut rslt = vec![0xE1];
                rslt.extend(register.to_le_bytes());
                rslt.extend(value.to_le_bytes());
                rslt
            }

            // offset write
            Command::RF5C68WriteOffset { offset, value } => {
                let mut rslt = vec![0xC1];
                rslt.extend(offset.to_le_bytes());
                rslt.extend(value.to_le_bytes());
                rslt
            }
            Command::RF5C164WriteOffset { offset, value } => {
                let mut rslt = vec![0xC1];
                rslt.extend(offset.to_le_bytes());
                rslt.extend(value.to_le_bytes());
                rslt
            }
        }
    }

    pub fn from_bytes(bytes: &mut Bytes) -> Result<Command, u8> {
        Ok(match bytes.get_u8() {
            0x31 => {
                // handle AY8910 stereo mask command
                // `bytes.get(1)` gives you the `dd` value
                // create and return a `Command` variant
                Command::AY8910StereoMask {
                    value: bytes.get_u8(),
                }
            }
            0x4F => {
                // handle Game Gear PSG stereo command
                Command::GameGearPSGStereo {
                    value: bytes.get_u8(),
                }
            }
            0x50 => {
                // handle PSG write command
                Command::PSGWrite {
                    value: bytes.get_u8(),
                }
            }
            0x51 => {
                // handle YM2413 write command
                Command::YM2413Write {
                    register: bytes.get_u8(),
                    value: bytes.get_u8(),
                }
            }
            0x52 => {
                // handle YM2612 port 0 write command
                Command::YM2612Port0Write {
                    register: bytes.get_u8(),
                    value: bytes.get_u8(),
                }
            }
            0x53 => {
                // handle YM2612 port 1 write command
                Command::YM2612Port1Write {
                    register: bytes.get_u8(),
                    value: bytes.get_u8(),
                }
            }
            0x54 => {
                // handle YM2151 write command
                Command::YM2151Write {
                    register: bytes.get_u8(),
                    value: bytes.get_u8(),
                }
            }
            0x55 => {
                // handle YM2203 write command
                Command::YM2203Write {
                    register: bytes.get_u8(),
                    value: bytes.get_u8(),
                }
            }
            0x56 => {
                // handle YM2608 port 0 write command
                Command::YM2608Port0Write {
                    register: bytes.get_u8(),
                    value: bytes.get_u8(),
                }
            }
            0x57 => {
                // handle YM2608 port 1 write command
                Command::YM2608Port1Write {
                    register: bytes.get_u8(),
                    value: bytes.get_u8(),
                }
            }
            0x58 => {
                // handle YM2610 port 0 write command
                Command::YM2610Port0Write {
                    register: bytes.get_u8(),
                    value: bytes.get_u8(),
                }
            }
            0x59 => {
                // handle YM2610 port 1 write command
                Command::YM2610Port1Write {
                    register: bytes.get_u8(),
                    value: bytes.get_u8(),
                }
            }
            0x5A => {
                // handle YM3812 write command
                Command::YM3812Write {
                    register: bytes.get_u8(),
                    value: bytes.get_u8(),
                }
            }
            0x5B => {
                // handle YM3526 write command
                Command::YM3526Write {
                    register: bytes.get_u8(),
                    value: bytes.get_u8(),
                }
            }
            0x5C => {
                // handle Y8950 write command
                Command::Y8950Write {
                    register: bytes.get_u8(),
                    value: bytes.get_u8(),
                }
            }
            0x5D => {
                // handle YMZ280B write command
                Command::YMZ280BWrite {
                    register: bytes.get_u8(),
                    value: bytes.get_u8(),
                }
            }
            0x5E => {
                // handle YMF262 port 0 write command
                Command::YMF262Port0Write {
                    register: bytes.get_u8(),
                    value: bytes.get_u8(),
                }
            }
            0x5F => {
                // handle YMF262 port 1 write command
                Command::YMF262Port1Write {
                    register: bytes.get_u8(),
                    value: bytes.get_u8(),
                }
            }
            0x61 => {
                // handle wait command
                Command::WaitNSamples {
                    n: bytes.get_u16_le(),
                }
            }
            0x62 => {
                // handle wait 735 samples command
                Command::Wait735Samples
            }
            0x63 => {
                // handle wait 882 samples command
                Command::Wait882Samples
            }
            0x66 => {
                // handle end of sound data command
                Command::EndOfSoundData
            }
            0x67 => {
                // handle data block command
                // skip compatibility arg (0x66)
                bytes.get_u8();
                let data_type = bytes.get_u8();
                let data_size = bytes.get_u32_le();
                Command::DataBlock {
                    data_type,
                    data_size,
                    data: (0..data_size as usize).map(|_| bytes.get_u8()).collect(),
                }
            }
            0x68 => {
                // handle PCM RAM write command
                // TODO: not done
                Command::PCMRAMWrite {
                    offset: 0,
                    data: vec![],
                }
            }
            cmd @ 0x70..=0x7F => {
                // handle wait n+1 samples command
                Command::WaitNSamplesPlus1 { n: cmd - 0x70 }
            }
            cmd @ 0x80..=0x8F => {
                // handle YM2612 port 0 address 2A write command
                Command::YM2612Port0Address2AWriteWait { n: cmd - 0x80 }
            }
            0x90..=0x95 => {
                // handle DAC Stream Control Write command
                // TODO: not done
                Command::DACStreamControlWrite {
                    register: 0,
                    value: 0,
                }
            }
            0xA0 => {
                // handle AY8910 write command
                Command::AY8910Write {
                    register: bytes.get_u8(),
                    value: bytes.get_u8(),
                }
            }
            0xB0 => {
                // handle RF5C68 write command
                Command::RF5C68Write {
                    register: bytes.get_u8(),
                    value: bytes.get_u8(),
                }
            }
            0xB1 => {
                // handle RF5C164 write command
                Command::RF5C164Write {
                    register: bytes.get_u8(),
                    value: bytes.get_u8(),
                }
            }
            0xB2 => {
                // handle PWM write command
                // TODO: is not aadd but addd
                Command::PWMWrite {
                    register: bytes.get_u8(),
                    value: bytes.get_u16_le(),
                }
            }
            0xB3 => {
                // handle GameBoy DMG write command
                Command::GameBoyDMGWrite {
                    register: bytes.get_u8(),
                    value: bytes.get_u8(),
                }
            }
            0xB4 => {
                // handle NES APU write command
                Command::NESAPUWrite {
                    register: bytes.get_u8(),
                    value: bytes.get_u8(),
                }
            }
            0xB5 => {
                // handle MultiPCM write command
                Command::MultiPCMWrite {
                    register: bytes.get_u8(),
                    value: bytes.get_u8(),
                }
            }
            0xB6 => {
                // handle uPD7759 write command
                Command::uPD7759Write {
                    register: bytes.get_u8(),
                    value: bytes.get_u8(),
                }
            }
            0xB7 => Command::HuC6280Write {
                register: bytes.get_u8(),
                value: bytes.get_u8(),
            },
            0xB8 => Command::K053260Write {
                register: bytes.get_u8(),
                value: bytes.get_u8(),
            },
            0xB9 => Command::PokeyWrite {
                register: bytes.get_u8(),
                value: bytes.get_u8(),
            },
            0xBA => Command::WonderSwanWrite {
                register: bytes.get_u8(),
                value: bytes.get_u8(),
            },
            0xBB => Command::SAA1099Write {
                register: bytes.get_u8(),
                value: bytes.get_u8(),
            },
            0xBC => Command::ES5506Write {
                register: bytes.get_u8(),
                value: bytes.get_u8(),
            },
            0xBD => Command::GA20Write {
                register: bytes.get_u8(),
                value: bytes.get_u8(),
            },
            0xBE => Command::ES5506Write {
                register: bytes.get_u8(),
                value: bytes.get_u8(),
            },
            0xBF => Command::GA20Write {
                register: bytes.get_u8(),
                value: bytes.get_u8(),
            },
            0xC0 => Command::SegaPCMWrite {
                offset: bytes.get_u16_le(),
                value: bytes.get_u8(),
            },
            0xC1 => Command::RF5C68WriteOffset {
                offset: bytes.get_u16_le(),
                value: bytes.get_u8(),
            },
            0xC2 => Command::RF5C164WriteOffset {
                offset: bytes.get_u16_le(),
                value: bytes.get_u8(),
            },
            0xC3 => Command::MultiPCMSetBank {
                channel: bytes.get_u8(),
                offset: bytes.get_u16_le(),
            },
            0xC4 => {
                // TODO: weird stuff with the data
                let value = bytes.get_u16_le();
                Command::QSoundWrite {
                    register: bytes.get_u8(),
                    value,
                }
            }
            0xC5 => {
                // TODO: weird stuff with the data
                //let value = bytes.get_u16_le();
                Command::SCSPWrite {
                    offset: bytes.get_u16_le(),
                    value: bytes.get_u8(),
                }
            }
            0xC6 => {
                // TODO: check
                Command::WonderSwanWrite16 {
                    offset: bytes.get_u16_le(),
                    value: bytes.get_u8(),
                }
            }
            0xC7 => {
                // TODO: check
                Command::VSUWrite {
                    offset: bytes.get_u16_le(),
                    value: bytes.get_u8(),
                }
            }
            0xC8 => {
                // TODO: check
                Command::X1010Write {
                    offset: bytes.get_u16_le(),
                    value: bytes.get_u8(),
                }
            }
            0xD0 => Command::YMF278BWrite {
                port: bytes.get_u8(),
                register: bytes.get_u8(),
                value: bytes.get_u8(),
            },
            0xD1 => Command::YMF271Write {
                port: bytes.get_u8(),
                register: bytes.get_u8(),
                value: bytes.get_u8(),
            },
            0xD2 => Command::SCC1Write {
                port: bytes.get_u8(),
                register: bytes.get_u8(),
                value: bytes.get_u8(),
            },
            0xD3 => Command::K054539Write {
                register: bytes.get_u16_le(),
                value: bytes.get_u8(),
            },
            0xD4 => Command::C140Write {
                register: bytes.get_u16_le(),
                value: bytes.get_u8(),
            },
            0xD5 => Command::ES5503Write {
                register: bytes.get_u16_le(),
                value: bytes.get_u8(),
            },
            0xD6 => Command::ES5506Write16 {
                register: bytes.get_u8(),
                value: bytes.get_u16_le(),
            },
            0xE0 => Command::SeekPCM {
                offset: bytes.get_u32_le(),
            },
            0xE1 => Command::C352Write {
                register: bytes.get_u16_le(),
                value: bytes.get_u16_le(),
            },
            cmd => Err(cmd)?,
        })
    }
}
