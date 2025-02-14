// Copyright (c) 2021 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

// https://github.com/Majored/rs-async-zip/blob/main/SPECIFICATION.md#437
pub struct LocalFileHeader {
    pub version: u16,
    pub flags: GeneralPurposeFlag,
    pub compression: u16,
    pub mod_time: u16,
    pub mod_date: u16,
    pub crc: u32,
    pub compressed_size: u32,
    pub uncompressed_size: u32,
    pub file_name_length: u16,
    pub extra_field_length: u16,
}

// https://github.com/Majored/rs-async-zip/blob/main/SPECIFICATION.md#444
#[derive(Copy, Clone)]
pub struct GeneralPurposeFlag {
    pub encrypted: bool,
    pub data_descriptor: bool,
    pub filename_unicode: bool,
}

/// 2 byte header ids
/// Ref https://github.com/Majored/rs-async-zip/blob/main/SPECIFICATION.md#452
#[derive(Clone, Copy, Debug)]
pub enum HeaderId {
    Zip64ExtendedInformationExtraField,
    Other(u16),
}

/// Represents each extra field.
/// Not strictly part of the spec, but is the most useful way to represent the data.
#[derive(Clone, Debug)]
pub enum ExtraField {
    Zip64ExtendedInformationExtraField(Zip64ExtendedInformationExtraField),
    UnknownExtraField(UnknownExtraField),
}

/// An extended information header for Zip64.
/// This field is used both for local file headers and central directory records.
/// https://github.com/Majored/rs-async-zip/blob/main/SPECIFICATION.md#453
#[derive(Clone, Debug)]
pub struct Zip64ExtendedInformationExtraField {
    pub header_id: HeaderId,
    pub data_size: u16,
    pub uncompressed_size: u64,
    pub compressed_size: u64,
    // While not specified in the spec, these two fields are often left out in practice.
    pub relative_header_offset: Option<u64>,
    pub disk_start_number: Option<u64>,
}

/// Represents any unparsed extra field.
#[derive(Clone, Debug)]
pub struct UnknownExtraField {
    pub header_id: HeaderId,
    pub data_size: u16,
    pub content: Vec<u8>,
}

// https://github.com/Majored/rs-async-zip/blob/main/SPECIFICATION.md#4312
pub struct CentralDirectoryRecord {
    pub v_made_by: u16,
    pub v_needed: u16,
    pub flags: GeneralPurposeFlag,
    pub compression: u16,
    pub mod_time: u16,
    pub mod_date: u16,
    pub crc: u32,
    pub compressed_size: u32,
    pub uncompressed_size: u32,
    pub file_name_length: u16,
    pub extra_field_length: u16,
    pub file_comment_length: u16,
    pub disk_start: u16,
    pub inter_attr: u16,
    pub exter_attr: u32,
    pub lh_offset: u32,
}

// https://github.com/Majored/rs-async-zip/blob/main/SPECIFICATION.md#4316
#[derive(Debug)]
pub struct EndOfCentralDirectoryHeader {
    pub(crate) disk_num: u16,
    pub(crate) start_cent_dir_disk: u16,
    pub(crate) num_of_entries_disk: u16,
    pub(crate) num_of_entries: u16,
    pub(crate) size_cent_dir: u32,
    pub(crate) cent_dir_offset: u32,
    pub(crate) file_comm_length: u16,
}

// https://github.com/Majored/rs-async-zip/blob/main/SPECIFICATION.md#4314
#[derive(Debug, PartialEq)]
pub struct Zip64EndOfCentralDirectoryRecord {
    /// The size of this Zip64EndOfCentralDirectoryRecord.
    /// This is specified because there is a variable-length extra zip64 information sector.
    /// However, we will gleefully ignore this sector because it is reserved for use by PKWare.
    pub size_of_zip64_end_of_cd_record: u64,
    pub version_made_by: u16,
    pub version_needed_to_extract: u16,
    pub disk_number: u32,
    pub disk_number_start_of_cd: u32,
    pub num_entries_in_directory_on_disk: u64,
    pub num_entries_in_directory: u64,
    pub directory_size: u64,
    pub offset_of_start_of_directory: u64,
}

// https://github.com/Majored/rs-async-zip/blob/main/SPECIFICATION.md#4315
#[derive(Debug)]
pub struct Zip64EndOfCentralDirectoryLocator {
    pub number_of_disk_with_start_of_zip64_end_of_central_directory: u32,
    pub relative_offset: u64,
    pub total_number_of_disks: u32,
}
