use fatfs::{IoBase, Read, Seek, SeekFrom, Write};

const FS_LEN: usize = 4 * 1024 * 1024;
const FS_PTR: *mut u8 = (1024 * 1024 * 12) as _;

#[derive(Debug)]
pub struct FsStorage {
    pos: usize,
}

impl FsStorage {
    pub const fn new() -> Self {
        Self { pos: 0 }
    }
}

impl IoBase for FsStorage {
    type Error = ();
}

impl Read for FsStorage {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let left = FS_LEN - self.pos;
        let len = buf.len().min(left);
        unsafe {
            FS_PTR.add(self.pos).copy_to(buf.as_mut_ptr(), len);
        }
        self.pos += len;
        Ok(len)
    }
}

impl Write for FsStorage {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        let left = FS_LEN - self.pos;
        let len = buf.len().min(left);
        unsafe {
            FS_PTR.add(self.pos).copy_from(buf.as_ptr(), len);
        }
        self.pos += len;
        Ok(len)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl Seek for FsStorage {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, Self::Error> {
        match pos {
            SeekFrom::Start(pos) => {
                if pos > FS_LEN as u64 {
                    Err(())
                } else {
                    self.pos = pos as usize;
                    Ok(pos)
                }
            }
            SeekFrom::End(pos) => {
                let new_pos = FS_LEN as i64 + pos;
                if new_pos < 0 || new_pos as usize > FS_LEN {
                    Err(())
                } else {
                    self.pos = new_pos as usize;
                    Ok(new_pos as u64)
                }
            }
            SeekFrom::Current(pos) => {
                let new_pos = self.pos as i64 + pos;
                if new_pos < 0 || new_pos as usize > FS_LEN {
                    Err(())
                } else {
                    self.pos = new_pos as usize;
                    Ok(new_pos as u64)
                }
            }
        }
    }
}
