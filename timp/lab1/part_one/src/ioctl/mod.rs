use nix::{ioctl_read,ioctl_write_ptr};
use std::os::unix::prelude::*;
const FS_XFLAG_IMMUTABLE: u32 = 0x8;
ioctl_read!(fs_ioc_fsgetxattr, 'X' as u8, 31 as u8, fsxattr);
ioctl_write_ptr!(fs_ioc_fssetxattr,'X' as u8, 32 as u8, fsxattr);
use std::fs::File;

#[allow(non_camel_case_types)]
#[derive(Debug, Default)]
#[repr(C)]
pub struct fsxattr {
    xflags: u32,
    extsize: u32,
    nextents: u32,
    projid: u32,
    cowextsize: u32,
    pad: [u8;8],
}

pub fn set_immut_atribute(file: &File) -> nix::Result<()> {
    let fd: RawFd = file.as_raw_fd();
    let mut attr: fsxattr = fsxattr::default();    
    unsafe { fs_ioc_fsgetxattr(fd, &mut attr)}?;
    attr.xflags |= FS_XFLAG_IMMUTABLE;
    unsafe { fs_ioc_fssetxattr(fd, &mut attr)}?;
    Ok(())
}
pub fn unset_immut_atribute(file: &File) -> nix::Result<()> {
    let fd: RawFd = file.as_raw_fd();
    let mut attr: fsxattr = fsxattr::default();    
    unsafe { fs_ioc_fsgetxattr(fd, &mut attr)}?;
    attr.xflags &= !FS_XFLAG_IMMUTABLE;
    unsafe { fs_ioc_fssetxattr(fd, &mut attr)}?;
    Ok(())
}
