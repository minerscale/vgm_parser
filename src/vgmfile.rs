use std::{
    fs::{self, File},
    io::{BufReader, Read},
};

use crate::command::{parse_commands, write_commands, Command};
use crate::header::HeaderData;
use crate::metadata::VgmMetadata;
use bytes::{Buf, Bytes};
use flate2::bufread::GzDecoder;

#[derive(Debug)]
pub struct VgmFile {
    pub header: HeaderData,
    pub commands: Vec<Command>,
    pub metadata: VgmMetadata,
}

impl VgmFile {
    pub fn from_path_gz(path: &str) -> Self {
        Self::from_bytes(&mut bytes::Bytes::from_iter(
            GzDecoder::new(BufReader::new(File::open(path).unwrap()))
                .bytes()
                .map(|x| x.unwrap()),
        ))
    }

    pub fn from_path(path: &str) -> Self {
        let file_data = fs::read(path).unwrap();
        let mut data = Bytes::from(file_data);
        VgmFile::from_bytes(&mut data)
    }

    pub fn from_bytes(data: &mut bytes::Bytes) -> Self {
        let len_data = data.len();
        let header_data = HeaderData::from_bytes(data);
        let vgm_start_pos = header_data.vgm_data_offset as usize + 0x34;

        while len_data - data.len() < vgm_start_pos {
            data.get_u8();
        }

        Self {
            header: header_data,
            commands: parse_commands(data),
            metadata: VgmMetadata::from_bytes(data),
        }
    }

    pub fn to_bytes(&self, buffer: &mut bytes::BytesMut) {
        self.header.to_bytes(buffer);
        write_commands(buffer, &self.commands);
        self.metadata.to_bytes(buffer);
    }
}
