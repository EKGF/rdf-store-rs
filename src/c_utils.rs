// Copyright (c) 2018-2023, agnos.ai UK Ltd, all rights reserved.
//---------------------------------------------------------------

use std::ffi::CStr;
use crate::RDFStoreError;

pub fn ptr_to_cstr<'b>(data: *const u8, len: usize) -> Result<&'b CStr, RDFStoreError> {
    unsafe {
        let slice = std::slice::from_raw_parts(data, len as usize);
        Ok(CStr::from_bytes_with_nul_unchecked(slice))
    }
}
