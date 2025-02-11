#![doc(html_root_url = "https://docs.rs/bzip2-sys/0.1")]

extern crate libc;

use libc::{c_char, c_int, c_uint, c_void, FILE};

pub const BZ_RUN: c_int = 0;
pub const BZ_FLUSH: c_int = 1;
pub const BZ_FINISH: c_int = 2;

pub const BZ_OK: c_int = 0;
pub const BZ_RUN_OK: c_int = 1;
pub const BZ_FLUSH_OK: c_int = 2;
pub const BZ_FINISH_OK: c_int = 3;
pub const BZ_STREAM_END: c_int = 4;
pub const BZ_SEQUENCE_ERROR: c_int = -1;
pub const BZ_PARAM_ERROR: c_int = -2;
pub const BZ_MEM_ERROR: c_int = -3;
pub const BZ_DATA_ERROR: c_int = -4;
pub const BZ_DATA_ERROR_MAGIC: c_int = -5;
pub const BZ_IO_ERROR: c_int = -6;
pub const BZ_UNEXPECTED_EOF: c_int = -7;
pub const BZ_OUTBUFF_FULL: c_int = -8;
pub const BZ_CONFIG_ERROR: c_int = -9;

pub const BZ_MAX_UNSUED: c_int = 5000;

pub type BZFILE = c_void;

#[repr(C)]
pub struct bz_stream {
    pub next_in: *mut c_char,
    pub avail_in: c_uint,
    pub total_in_lo32: c_uint,
    pub total_in_hi32: c_uint,

    pub next_out: *mut c_char,
    pub avail_out: c_uint,
    pub total_out_lo32: c_uint,
    pub total_out_hi32: c_uint,

    pub state: *mut c_void,

    pub bzalloc: Option<extern "C" fn(*mut c_void, c_int, c_int) -> *mut c_void>,
    pub bzfree: Option<extern "C" fn(*mut c_void, *mut c_void)>,
    pub opaque: *mut c_void,
}

#[cfg(windows)]
extern "system" {
    /*-- Core (low-level) library functions --*/
    pub fn BZ2_bzCompressInit(
        stream: *mut bz_stream,
        blockSize100k: c_int,
        verbosity: c_int,
        workFactor: c_int,
    ) -> c_int;
    pub fn BZ2_bzCompress(stream: *mut bz_stream, action: c_int) -> c_int;
    pub fn BZ2_bzCompressEnd(stream: *mut bz_stream) -> c_int;
    pub fn BZ2_bzDecompressInit(stream: *mut bz_stream, verbosity: c_int, small: c_int) -> c_int;
    pub fn BZ2_bzDecompress(stream: *mut bz_stream) -> c_int;
    pub fn BZ2_bzDecompressEnd(stream: *mut bz_stream) -> c_int;

    /*-- High(er) level library functions --*/
    pub fn BZ2_bzReadOpen(
        bzerror: *mut c_int,
        f: *mut FILE,
        verbosity: c_int,
        small: c_int,
        unused: *mut *mut c_void,
        nUnused: c_int,
    ) -> *mut BZFILE;

    pub fn BZ2_bzReadClose(bzerror: *mut c_int, b: *mut BZFILE);

    pub fn BZ2_bzReadGetUnused(
        bzerror: *mut c_int,
        b: *mut BZFILE,
        unused: *mut *mut c_void,
        nUnused: *mut c_int,
    );

    pub fn BZ2_bzRead(bzerror: *mut c_int, b: *mut BZFILE, buf: *mut c_void, len: c_int) -> c_int;

    pub fn BZ2_bzWriteOpen(
        bzerror: *mut c_int,
        f: *mut FILE,
        blockSize100k: c_int,
        verbosity: c_int,
        workFactor: c_int,
    ) -> *mut BZFILE;

    pub fn BZ2_bzWrite(bzerror: *mut c_int, b: *mut BZFILE, buf: *mut c_void, len: c_int);

    pub fn BZ2_bzWriteClose(
        bzerror: *mut c_int,
        b: *mut BZFILE,
        abandon: c_int,
        nbytes_in: *mut c_uint,
        nbytes_out: *mut c_uint,
    );

    pub fn BZ2_bzWriteClose64(
        bzerror: *mut c_int,
        b: *mut BZFILE,
        abandon: c_int,
        nbytes_in_lo32: *mut c_uint,
        nbytes_in_hi32: *mut c_uint,
        nbytes_out_lo32: *mut c_uint,
        nbytes_out_hi32: *mut c_uint,
    );

    /*-- Utility functions --*/

    pub fn BZ2_bzBuffToBuffCompress(
        dest: *mut c_char,
        destLen: *mut c_uint,
        source: *const c_char,
        sourceLen: c_uint,
        blockSize100k: c_int,
        verbosity: c_int,
        workFactor: c_int,
    ) -> c_int;

    pub fn BZ2_bzBuffToBuffDecompress(
        dest: *mut c_char,
        destLen: *mut c_uint,
        source: *const c_char,
        sourceLen: c_uint,
        small: c_int,
        verbosity: c_int,
    ) -> c_int;

}

#[cfg(not(windows))]
extern "C" {
    /*-- Core (low-level) library functions --*/
    pub fn BZ2_bzCompressInit(
        stream: *mut bz_stream,
        blockSize100k: c_int,
        verbosity: c_int,
        workFactor: c_int,
    ) -> c_int;
    pub fn BZ2_bzCompress(stream: *mut bz_stream, action: c_int) -> c_int;
    pub fn BZ2_bzCompressEnd(stream: *mut bz_stream) -> c_int;
    pub fn BZ2_bzDecompressInit(stream: *mut bz_stream, verbosity: c_int, small: c_int) -> c_int;
    pub fn BZ2_bzDecompress(stream: *mut bz_stream) -> c_int;
    pub fn BZ2_bzDecompressEnd(stream: *mut bz_stream) -> c_int;

    /*-- High(er) level library functions --*/
    pub fn BZ2_bzReadOpen(
        bzerror: *mut c_int,
        f: *mut FILE,
        verbosity: c_int,
        small: c_int,
        unused: *mut *mut c_void,
        nUnused: c_int,
    ) -> *mut BZFILE;

    pub fn BZ2_bzReadClose(bzerror: *mut c_int, b: *mut BZFILE);

    pub fn BZ2_bzReadGetUnused(
        bzerror: *mut c_int,
        b: *mut BZFILE,
        unused: *mut *mut c_void,
        nUnused: *mut c_int,
    );

    pub fn BZ2_bzRead(bzerror: *mut c_int, b: *mut BZFILE, buf: *mut c_void, len: c_int) -> c_int;

    pub fn BZ2_bzWriteOpen(
        bzerror: *mut c_int,
        f: *mut FILE,
        blockSize100k: c_int,
        verbosity: c_int,
        workFactor: c_int,
    ) -> *mut BZFILE;

    pub fn BZ2_bzWrite(bzerror: *mut c_int, b: *mut BZFILE, buf: *mut c_void, len: c_int);

    pub fn BZ2_bzWriteClose(
        bzerror: *mut c_int,
        b: *mut BZFILE,
        abandon: c_int,
        nbytes_in: *mut c_uint,
        nbytes_out: *mut c_uint,
    );

    pub fn BZ2_bzWriteClose64(
        bzerror: *mut c_int,
        b: *mut BZFILE,
        abandon: c_int,
        nbytes_in_lo32: *mut c_uint,
        nbytes_in_hi32: *mut c_uint,
        nbytes_out_lo32: *mut c_uint,
        nbytes_out_hi32: *mut c_uint,
    );

    /*-- Utility functions --*/

    pub fn BZ2_bzBuffToBuffCompress(
        dest: *mut c_char,
        destLen: *mut c_uint,
        source: *const c_char,
        sourceLen: c_uint,
        blockSize100k: c_int,
        verbosity: c_int,
        workFactor: c_int,
    ) -> c_int;

    pub fn BZ2_bzBuffToBuffDecompress(
        dest: *mut c_char,
        destLen: *mut c_uint,
        source: *const c_char,
        sourceLen: c_uint,
        small: c_int,
        verbosity: c_int,
    ) -> c_int;

}

#[no_mangle]
pub fn bz_internal_error(errcode: c_int) {
    panic!("bz internal error: {}", errcode);
}
