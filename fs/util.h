#pragma once

#include <cstdint>
#include <filesystem>

#include "fs_zenfs.h"
#include "rocksdb/status.h"

namespace ROCKSDB_NAMESPACE {

IOStatus zenfs_zbd_open(std::filesystem::path const &path, bool readonly,
                        bool exclusive,
                        std::unique_ptr<ZonedBlockDevice> &out_zbd);

Status zenfs_mount(std::unique_ptr<ZonedBlockDevice> &zbd, bool readonly,
                   std::unique_ptr<ZenFS> &out_zen_fs);

Status zenfs_mkfs(std::filesystem::path const &zbd_path,
                  std::filesystem::path const &aux_path,
                  uint32_t finish_threshold, bool force);

//TODO: autocxx cannot generate bindings for Status and IOStatus
bool status_is_ok(Status &status);
bool iostatus_is_ok(IOStatus &status);


  //TODO: autocxx cannot generate constructors for subclasses https://github.com/google/autocxx/issues/700
std::unique_ptr<ZenFS> construct_zenfs(std::unique_ptr<ZonedBlockDevice> zbd,
                                        std::shared_ptr<FileSystem> aux_fs,
                                       std::shared_ptr<Logger> logger);
}  // namespace ROCKSDB_NAMESPACE
