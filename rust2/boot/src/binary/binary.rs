use core::{
    mem::MaybeUninit,
    str::{self, Utf8Error},
};

use alloc::{boxed::Box, string::String, vec, vec::Vec};
use fs_impl::{bobby_fs::Error, BobbyFile};

use crate::symbols;

fn read_u32(file: &mut BobbyFile) -> Result<u32, Error> {
    let mut bytes = [MaybeUninit::uninit(); 4];
    file.read(&mut bytes)?;
    Ok(u32::from_le_bytes(unsafe {
        MaybeUninit::array_assume_init(bytes)
    }))
}

pub struct Binary {
    code: Box<[usize]>,
    export_symbols: Vec<String>,
}

#[derive(Debug)]
#[allow(unused)]
pub enum BinaryError {
    Io(Error),
    Utf8Error(Utf8Error),
    UnalignedAddr,
    MissingSymbol(String),
    SymbolAddrOutOfBounds(String, usize),
    ReladynOutOfBounds(usize, usize),
    Other,
}

impl From<Error> for BinaryError {
    fn from(value: Error) -> Self {
        Self::Io(value)
    }
}

impl Binary {
    pub fn from_file(file: &mut BobbyFile) -> Result<Binary, BinaryError> {
        // println_err!("Reading binary from file.");
        let code_size = read_u32(file)?;
        if code_size == 0 && code_size % 4 != 0 {
            return Err(BinaryError::Other);
        }
        // println_err!("Code size: {} bytes.", code_size);
        let mut code: Box<[MaybeUninit<usize>]> =
            Box::new_uninit_slice(code_size as usize / size_of::<usize>());
        // println_err!("Code is at {:#p}", code.as_ptr());
        let slice = MaybeUninit::slice_as_bytes_mut(&mut code);
        // print_err!("Reading code... ");
        let Ok(_) = file.read(slice) else {
            // println_err!("Failed.");
            return Err(Error::Eof.into());
        };
        // println_err!("OK.");
        let binary_offset = code.as_ptr() as usize;

        {
            let reladyn_len = read_u32(file)?;
            // println_err!("reladyn len: {} entries.", reladyn_len);
            let mut vec = vec![MaybeUninit::uninit(); (reladyn_len * 8) as usize];
            file.read(&mut vec)?;
            let mut slice = unsafe { MaybeUninit::slice_assume_init_mut(&mut vec) };
            while !slice.is_empty() {
                let offset_bytes = slice.get(0..4).ok_or(BinaryError::Other)?;
                let offset = usize::from_le_bytes(offset_bytes.try_into().unwrap());
                let address_bytes = slice.get(4..8).ok_or(BinaryError::Other)?;
                let address = usize::from_le_bytes(address_bytes.try_into().unwrap());
                // println_err!("reladyn: {:#x} should point to {:#x}", offset, address);
                if offset % 4 != 0 {
                    return Err(BinaryError::UnalignedAddr);
                }
                *code
                    .get_mut(offset / size_of::<usize>())
                    .ok_or_else(|| BinaryError::ReladynOutOfBounds(offset, address))? =
                    MaybeUninit::new(binary_offset + address);
                slice = &mut slice[8..];
            }
        }

        let mut export_symbols: Vec<String> = vec![];
        {
            let export_len = read_u32(file)?;
            // println_err!("Export len: {} bytes.", export_len);
            let mut vec = vec![MaybeUninit::uninit(); export_len as usize];
            file.read(&mut vec)?;
            let mut slice = unsafe { MaybeUninit::slice_assume_init_mut(&mut vec) };
            while !slice.is_empty() {
                let len_bytes = slice.get(0..4).ok_or(BinaryError::Other)?;
                let len = usize::from_le_bytes(len_bytes.try_into().unwrap());
                let symbol = slice.get(4..len + 4).ok_or(BinaryError::Other)?;
                let symbol = str::from_utf8(symbol).map_err(|err| BinaryError::Utf8Error(err))?;
                // println_err!("Export symbol \"{}\".", symbol);
                let addr_bytes = slice.get(len + 4..len + 8).ok_or(BinaryError::Other)?;
                let addr = usize::from_le_bytes(addr_bytes.try_into().unwrap());
                if addr % 4 != 0 {
                    return Err(BinaryError::UnalignedAddr);
                }
                symbols::add_symbol(symbol.into(), (binary_offset + addr) as _);
                export_symbols.push(symbol.into());
                slice = &mut slice[len + 8..];
            }
        }

        {
            let import_len = read_u32(file)?;
            // println_err!("Import len: {} bytes.", import_len);
            let mut vec = vec![MaybeUninit::uninit(); import_len as usize];
            file.read(&mut vec)?;
            let mut slice = unsafe { MaybeUninit::slice_assume_init_mut(&mut vec) };
            while !slice.is_empty() {
                let len_bytes = slice.get(0..4).ok_or(BinaryError::Other)?;
                let len = usize::from_le_bytes(len_bytes.try_into().unwrap());
                let symbol = slice.get(4..len + 4).ok_or(BinaryError::Other)?;
                let symbol = str::from_utf8(symbol).map_err(|err| BinaryError::Utf8Error(err))?;
                // println_err!("Import symbol \"{}\".", symbol);
                let addr_bytes = slice.get(len + 4..len + 8).ok_or(BinaryError::Other)?;
                let addr = usize::from_le_bytes(addr_bytes.try_into().unwrap());
                if addr % 4 != 0 {
                    return Err(BinaryError::UnalignedAddr);
                }
                let symbol_addr = symbols::get_symbol(symbol)
                    .ok_or_else(|| BinaryError::MissingSymbol(symbol.into()))?;
                *code
                    .get_mut(addr / size_of::<usize>())
                    .ok_or_else(|| BinaryError::SymbolAddrOutOfBounds(symbol.into(), addr))? =
                    MaybeUninit::new(symbol_addr as usize);
                slice = &mut slice[len + 8..];
            }
        }
        Ok(Binary {
            code: unsafe { code.assume_init() },
            export_symbols,
        })
    }

    pub fn run_main(&self) {
        let func: extern "C" fn() = unsafe { core::mem::transmute(self.code.as_ptr()) };
        func();
    }
}

impl Drop for Binary {
    fn drop(&mut self) {
        for symbol in self.export_symbols.iter() {
            symbols::remove_symbol(&symbol);
        }
    }
}
