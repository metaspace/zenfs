use autocxx::prelude::*;

include_cpp! {
    #include "rocksdb/plugin/zenfs/fs/fs_zenfs.h"
    #include "rocksdb/plugin/zenfs/fs/util.h"
    block!("rocksdb::MemoryMappedFileBuffer")
    generate!("rocksdb::ZenFS")
    generate!("rocksdb::FileSystem")
    generate!("rocksdb::ZonedBlockDevice")
    generate!("rocksdb::status_is_ok")
    generate!("rocksdb::iostatus_is_ok")
    generate!("rocksdb::construct_zenfs")
    generate!("std::filesystem::path")
}

pub use ffi::rocksdb;
pub use ffi::std as cxxstd;
