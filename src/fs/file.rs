use crate::fs::errors::RecordHeaderError;
use anyhow::Result;
use bson::serialize_to_vec;
use crc32fast::Hasher;
use serde::{Serialize, de::DeserializeOwned};
use std::{
    fs::File,
    io::{Read, Seek, SeekFrom, Write},
};

pub(super) const HEADER_SIZE: u64 = 32;
pub(super) const MAGIC: u32 = 0xDEADBEEF;
pub(super) const CURRENT_VERSION: u8 = 1;

// Record types
pub(super) const RECORD_TYPE_ACTIVE: u8 = 0x01;
pub(super) const RECORD_TYPE_DELETED: u8 = 0x02;

// Flags
// pub(super) const FLAG_COMPRESSED: u16 = 0x0001;
// pub(super) const FLAG_ENCRYPTED: u16 = 0x0002;
pub(super) const FLAG_HAS_VECTOR: u16 = 0x0010; // Relevant for your RAG use case!

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(super) struct RecordHeader {
    pub(super) magic: u32,      // 4 bytes
    pub(super) version: u8,     // 1 bytes
    pub(super) record_type: u8, // 1 byte
    pub(super) flags: u16,      // 4 bytes
    pub(super) length: u64,     // 8 bytes
    pub(super) timestamp: u64,  // 8 bytes
    pub(super) crc32: u32,      // 4 bytes
    pub(super) reserved: u32,   // 4 bytes

                                // 32 bytes
}

impl RecordHeader {
    fn new(record_type: u8, data_length: u64) -> Self {
        Self {
            magic: MAGIC,
            version: CURRENT_VERSION,
            record_type,
            flags: 0,
            length: HEADER_SIZE + data_length,
            timestamp: current_timestamp_micros(),
            crc32: 0, // Set after computing data CRC
            reserved: 0,
        }
    }

    pub(super) fn read<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buf = [0u8; HEADER_SIZE as usize];
        reader.read_exact(&mut buf)?;

        let header = Self {
            magic: u32::from_le_bytes(buf[0..4].try_into()?),
            version: buf[4],
            record_type: buf[5],
            flags: u16::from_le_bytes(buf[6..8].try_into()?),
            length: u64::from_le_bytes(buf[8..16].try_into()?),
            timestamp: u64::from_le_bytes(buf[16..24].try_into()?),
            crc32: u32::from_le_bytes(buf[24..28].try_into()?),
            reserved: u32::from_le_bytes(buf[28..32].try_into()?),
        };

        // Validate magic number
        if header.magic != MAGIC {
            return Err(anyhow::anyhow!(RecordHeaderError::InvalidMagic {
                magic: header.magic
            }));
        }

        // Check version
        if header.version > CURRENT_VERSION {
            return Err(anyhow::anyhow!(RecordHeaderError::UnsupportedVersion {
                version: header.version
            }));
        }

        Ok(header)
    }

    pub(super) fn write<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&self.magic.to_le_bytes())?;
        writer.write_all(&[self.version])?;
        writer.write_all(&[self.record_type])?;
        writer.write_all(&self.flags.to_le_bytes())?;
        writer.write_all(&self.length.to_le_bytes())?;
        writer.write_all(&self.timestamp.to_le_bytes())?;
        writer.write_all(&self.crc32.to_le_bytes())?;
        writer.write_all(&self.reserved.to_le_bytes())?;
        Ok(())
    }

    // fn has_flag(&self, flag: u16) -> bool {
    //     self.flags & flag != 0
    // }

    fn set_flag(&mut self, flag: u16) {
        self.flags |= flag;
    }

    fn data_size(&self) -> u64 {
        self.length - HEADER_SIZE
    }
}

fn current_timestamp_micros() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_micros() as u64
}

fn compute_crc32(data: &[u8]) -> u32 {
    let mut hasher = Hasher::new();
    hasher.update(data);
    hasher.finalize()
}

pub(super) fn write_active_record<T: Serialize>(
    file: &mut File,
    record_type: u8,
    data: &T,
    has_vector: bool,
) -> Result<u64> {
    let bson_bytes = serialize_to_vec(&data)?;
    // Compute CRC
    let crc = compute_crc32(&bson_bytes);

    // Create header
    let mut header = RecordHeader::new(record_type, bson_bytes.len() as u64);
    header.crc32 = crc;

    if has_vector {
        header.set_flag(FLAG_HAS_VECTOR);
    }

    let offset = file.seek(SeekFrom::End(0))?;
    header.write(file)?;
    file.write_all(&bson_bytes)?;

    Ok(offset)
}

pub(super) fn read_record<T: DeserializeOwned>(
    file: &mut File,
    offset: u64,
) -> Result<(RecordHeader, T)> {
    file.seek(SeekFrom::Start(offset))?;
    let header = RecordHeader::read(file)?;

    // Read data
    let data_size = header.data_size();
    let mut data = vec![0u8; data_size as usize];
    file.read_exact(&mut data)?;

    // Verify CRC
    let computed_crc = compute_crc32(&data);
    if computed_crc != header.crc32 {
        return Err(anyhow::anyhow!(RecordHeaderError::CorruptedData {
            offset,
            expected: header.crc32,
            actual: computed_crc,
        }));
    }

    // Deserialize
    let model: T = bson::deserialize_from_slice(&data)?;

    Ok((header, model))
}
