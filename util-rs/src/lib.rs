use autocxx::prelude::*;

include_cpp! {
    #include "rocksdb/plugin/zenfs/fs/fs_zenfs.h"
    block!("rocksdb::MemoryMappedFileBuffer")
    generate!("rocksdb::Status")
}

pub use ffi::*;
