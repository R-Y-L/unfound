======================================
提交ID: d677e2c
作者: unfound <t202510336998092@eduxiji.net>
时间: Sun Jan 4 14:34:14 2026 +0800
说明: axfs修复
======================================
 modules/axfs/Cargo.toml               |   2 +-
 modules/axfs/src/api/file.rs          |  68 +++-
 modules/axfs/src/api/mod.rs           |  18 +-
 modules/axfs/src/dev.rs               |  72 +++-
 modules/axfs/src/fops.rs              | 197 ++++++++--
 modules/axfs/src/fs/fatfs.rs          | 195 ++++++++--
 modules/axfs/src/fs/lwext4_rust.rs    | 690 ++++++++++++++++++++++++++++++++++
 modules/axfs/src/fs/mod.rs            |  28 +-
 modules/axfs/src/lib.rs               |  77 +++-
 modules/axfs/src/mounts.rs            |  82 ++--
 modules/axfs/src/path.rs              | 234 ++++++++++++
 modules/axfs/src/root.rs              | 188 +++++++--
 modules/axfs/tests/test_common/mod.rs |   6 +-
 13 files changed, 1698 insertions(+), 159 deletions(-)

======================================
提交ID: dbfd455
作者: unfound <t202510336998092@eduxiji.net>
时间: Sun Jan 4 03:10:51 2026 +0800
说明: doc 0
======================================
 doc/README.md                        |  47 ------
 doc/build.md                         |  80 ----------
 doc/figures/ArceOS.svg               |   4 -
 doc/figures/draw_jtag_connected.jpg  | Bin 121786 -> 0 bytes
 doc/figures/image_jtag_connected.jpg | Bin 240596 -> 0 bytes
 doc/figures/phytium_ok.png           | Bin 413228 -> 0 bytes
 doc/figures/phytium_select_dtb.png   | Bin 43675 -> 0 bytes
 doc/figures/phytium_uart.png         | Bin 266547 -> 0 bytes
 doc/grub_boot.md                     |  51 -------
 doc/ixgbe.md                         |  52 -------
 doc/jtag_debug_in_raspi4.md          | 276 -----------------------------------
 doc/platform_phytium_pi.md           |  52 -------
 doc/platform_raspi4.md               |  28 ----
 13 files changed, 590 deletions(-)

======================================
提交ID: e56acee
作者: unfound <t202510336998092@eduxiji.net>
时间: Sun Jan 4 03:08:55 2026 +0800
说明: crates add
======================================
 crates/axfs_crates/.github/workflows/ci.yml        |   53 +
 crates/axfs_crates/.gitignore                      |    4 +
 crates/axfs_crates/Cargo.toml                      |   21 +
 crates/axfs_crates/axfs_devfs/Cargo.toml           |   17 +
 crates/axfs_crates/axfs_devfs/src/chrdev.rs        |   51 +
 crates/axfs_crates/axfs_devfs/src/dir.rs           |  152 +
 crates/axfs_crates/axfs_devfs/src/lib.rs           |   88 +
 crates/axfs_crates/axfs_devfs/src/null.rs          |   55 +
 crates/axfs_crates/axfs_devfs/src/tests.rs         |  115 +
 crates/axfs_crates/axfs_devfs/src/zero.rs          |   56 +
 crates/axfs_crates/axfs_procfs/Cargo.toml          |   15 +
 crates/axfs_crates/axfs_procfs/src/dir.rs          |  308 ++
 crates/axfs_crates/axfs_procfs/src/file.rs         |   80 +
 crates/axfs_crates/axfs_procfs/src/lib.rs          |   61 +
 crates/axfs_crates/axfs_procfs/src/tests.rs        |  142 +
 crates/axfs_crates/axfs_ramfs/Cargo.toml           |   17 +
 crates/axfs_crates/axfs_ramfs/src/dir.rs           |  189 ++
 crates/axfs_crates/axfs_ramfs/src/file.rs          |   68 +
 crates/axfs_crates/axfs_ramfs/src/lib.rs           |   62 +
 crates/axfs_crates/axfs_ramfs/src/tests.rs         |  136 +
 crates/axfs_crates/axfs_vfs/Cargo.toml             |   20 +
 crates/axfs_crates/axfs_vfs/src/lib.rs             |  217 ++
 crates/axfs_crates/axfs_vfs/src/macros.rs          |   66 +
 crates/axfs_crates/axfs_vfs/src/path.rs            |   97 +
 crates/axfs_crates/axfs_vfs/src/structs.rs         |  655 ++++
 .../workflows/actions/setup-musl/action.yml        |   38 +
 .../workflows/actions/setup-qemu/action.yml        |   46 +
 crates/lwext4_rust/.github/workflows/build.yml     |   35 +
 crates/lwext4_rust/.github/workflows/test.yml      |   39 +
 crates/lwext4_rust/.gitignore                      |    2 +
 crates/lwext4_rust/.gitmodules                     |    3 +
 crates/lwext4_rust/Cargo.toml                      |   26 +
 crates/lwext4_rust/LICENSE.GPLv2                   |  339 ++
 crates/lwext4_rust/README.json                     |   11 +
 crates/lwext4_rust/README.md                       |   66 +
 crates/lwext4_rust/build.rs                        |  141 +
 crates/lwext4_rust/c/ext_images.7z                 |  Bin 0 -> 51195 bytes
 crates/lwext4_rust/c/lwext4-make.patch             |   69 +
 crates/lwext4_rust/c/lwext4/.clang-format          |    8 +
 crates/lwext4_rust/c/lwext4/.gitignore             |   12 +
 crates/lwext4_rust/c/lwext4/.travis.yml            |   35 +
 crates/lwext4_rust/c/lwext4/CHANGELOG              |   69 +
 crates/lwext4_rust/c/lwext4/CMakeLists.txt         |  106 +
 crates/lwext4_rust/c/lwext4/LICENSE                |  345 +++
 crates/lwext4_rust/c/lwext4/Makefile               |   93 +
 crates/lwext4_rust/c/lwext4/README.md              |  244 ++
 crates/lwext4_rust/c/lwext4/_config.yml            |    1 +
 .../lwext4_rust/c/lwext4/blockdev/CMakeLists.txt   |   13 +
 crates/lwext4_rust/c/lwext4/blockdev/blockdev.c    |   99 +
 crates/lwext4_rust/c/lwext4/blockdev/blockdev.h    |   41 +
 .../lwext4_rust/c/lwext4/blockdev/linux/file_dev.c |  142 +
 .../lwext4_rust/c/lwext4/blockdev/linux/file_dev.h |   43 +
 .../c/lwext4/blockdev/windows/file_windows.c       |  170 +
 .../c/lwext4/blockdev/windows/file_windows.h       |   44 +
 crates/lwext4_rust/c/lwext4/fs_test.mk             |  664 ++++
 crates/lwext4_rust/c/lwext4/fs_test/CMakeLists.txt |   32 +
 .../c/lwext4/fs_test/common/test_lwext4.c          |  379 +++
 .../c/lwext4/fs_test/common/test_lwext4.h          |   60 +
 .../lwext4_rust/c/lwext4/fs_test/lwext4_client.c   |  220 ++
 .../lwext4_rust/c/lwext4/fs_test/lwext4_generic.c  |  275 ++
 crates/lwext4_rust/c/lwext4/fs_test/lwext4_mbr.c   |  180 ++
 crates/lwext4_rust/c/lwext4/fs_test/lwext4_mkfs.c  |  223 ++
 .../lwext4_rust/c/lwext4/fs_test/lwext4_server.c   | 1180 +++++++
 crates/lwext4_rust/c/lwext4/include/ext4.h         |  628 ++++
 crates/lwext4_rust/c/lwext4/include/ext4_balloc.h  |  120 +
 crates/lwext4_rust/c/lwext4/include/ext4_bcache.h  |  296 ++
 crates/lwext4_rust/c/lwext4/include/ext4_bitmap.h  |  103 +
 .../c/lwext4/include/ext4_block_group.h            |  327 ++
 .../lwext4_rust/c/lwext4/include/ext4_blockdev.h   |  266 ++
 crates/lwext4_rust/c/lwext4/include/ext4_config.h  |  184 ++
 crates/lwext4_rust/c/lwext4/include/ext4_crc32.h   |   72 +
 crates/lwext4_rust/c/lwext4/include/ext4_debug.h   |  192 ++
 crates/lwext4_rust/c/lwext4/include/ext4_dir.h     |  301 ++
 crates/lwext4_rust/c/lwext4/include/ext4_dir_idx.h |  112 +
 crates/lwext4_rust/c/lwext4/include/ext4_errno.h   |   95 +
 crates/lwext4_rust/c/lwext4/include/ext4_extent.h  |   74 +
 crates/lwext4_rust/c/lwext4/include/ext4_fs.h      |  279 ++
 crates/lwext4_rust/c/lwext4/include/ext4_hash.h    |   76 +
 crates/lwext4_rust/c/lwext4/include/ext4_ialloc.h  |   85 +
 crates/lwext4_rust/c/lwext4/include/ext4_inode.h   |  354 +++
 crates/lwext4_rust/c/lwext4/include/ext4_journal.h |  148 +
 crates/lwext4_rust/c/lwext4/include/ext4_mbr.h     |   73 +
 crates/lwext4_rust/c/lwext4/include/ext4_misc.h    |  156 +
 crates/lwext4_rust/c/lwext4/include/ext4_mkfs.h    |   85 +
 crates/lwext4_rust/c/lwext4/include/ext4_oflags.h  |  103 +
 crates/lwext4_rust/c/lwext4/include/ext4_super.h   |  238 ++
 crates/lwext4_rust/c/lwext4/include/ext4_trans.h   |   90 +
 crates/lwext4_rust/c/lwext4/include/ext4_types.h   |  844 +++++
 crates/lwext4_rust/c/lwext4/include/ext4_xattr.h   |  108 +
 crates/lwext4_rust/c/lwext4/include/misc/queue.h   |  702 +++++
 crates/lwext4_rust/c/lwext4/include/misc/tree.h    |  809 +++++
 crates/lwext4_rust/c/lwext4/src/CMakeLists.txt     |   24 +
 crates/lwext4_rust/c/lwext4/src/ext4.c             | 3249 ++++++++++++++++++++
 crates/lwext4_rust/c/lwext4/src/ext4_balloc.c      |  669 ++++
 crates/lwext4_rust/c/lwext4/src/ext4_bcache.c      |  325 ++
 crates/lwext4_rust/c/lwext4/src/ext4_bitmap.c      |  162 +
 crates/lwext4_rust/c/lwext4/src/ext4_block_group.c |   94 +
 crates/lwext4_rust/c/lwext4/src/ext4_blockdev.c    |  475 +++
 crates/lwext4_rust/c/lwext4/src/ext4_crc32.c       |  187 ++
 crates/lwext4_rust/c/lwext4/src/ext4_debug.c       |   66 +
 crates/lwext4_rust/c/lwext4/src/ext4_dir.c         |  708 +++++
 crates/lwext4_rust/c/lwext4/src/ext4_dir_idx.c     | 1402 +++++++++
 crates/lwext4_rust/c/lwext4/src/ext4_extent.c      | 2140 +++++++++++++
 crates/lwext4_rust/c/lwext4/src/ext4_fs.c          | 1750 +++++++++++
 crates/lwext4_rust/c/lwext4/src/ext4_hash.c        |  327 ++
 crates/lwext4_rust/c/lwext4/src/ext4_ialloc.c      |  370 +++
 crates/lwext4_rust/c/lwext4/src/ext4_inode.c       |  405 +++
 crates/lwext4_rust/c/lwext4/src/ext4_journal.c     | 2291 ++++++++++++++
 crates/lwext4_rust/c/lwext4/src/ext4_mbr.c         |  208 ++
 crates/lwext4_rust/c/lwext4/src/ext4_mkfs.c        |  865 ++++++
 crates/lwext4_rust/c/lwext4/src/ext4_super.c       |  272 ++
 crates/lwext4_rust/c/lwext4/src/ext4_trans.c       |  107 +
 crates/lwext4_rust/c/lwext4/src/ext4_xattr.c       | 1564 ++++++++++
 .../lwext4_rust/c/lwext4/toolchain/arm-sim.cmake   |    9 +
 .../lwext4_rust/c/lwext4/toolchain/avrxmega7.cmake |    7 +
 .../c/lwext4/toolchain/common/arm-none-eabi.cmake  |   23 +
 .../c/lwext4/toolchain/common/avr-gcc.cmake        |   22 +
 .../c/lwext4/toolchain/common/bfin-elf.cmake       |   22 +
 .../c/lwext4/toolchain/common/msp430-gcc.cmake     |   22 +
 .../c/lwext4/toolchain/cortex-m0+.cmake            |    9 +
 .../lwext4_rust/c/lwext4/toolchain/cortex-m0.cmake |    9 +
 .../lwext4_rust/c/lwext4/toolchain/cortex-m3.cmake |    9 +
 .../lwext4_rust/c/lwext4/toolchain/cortex-m4.cmake |    9 +
 .../c/lwext4/toolchain/cortex-m4f.cmake            |    9 +
 .../lwext4_rust/c/lwext4/toolchain/cortex-m7.cmake |    9 +
 .../lwext4_rust/c/lwext4/toolchain/generic.cmake   |   28 +
 crates/lwext4_rust/c/lwext4/toolchain/mingw.cmake  |   32 +
 crates/lwext4_rust/c/lwext4/toolchain/msp430.cmake |    7 +
 .../c/lwext4/toolchain/musl-generic.cmake          |   45 +
 crates/lwext4_rust/c/musl-generic.cmake            |   79 +
 crates/lwext4_rust/c/ulibc.c                       |  158 +
 crates/lwext4_rust/c/wrapper.h                     |    5 +
 crates/lwext4_rust/doc/RefFS/RefFS-build.md        |  163 +
 crates/lwext4_rust/doc/RefFS/mount-reffs.png       |  Bin 0 -> 22667 bytes
 crates/lwext4_rust/doc/RefFS/output-ext2-check.png |  Bin 0 -> 261959 bytes
 .../doc/filesystem\346\216\245\345\217\243.md"     |  274 ++
 crates/lwext4_rust/doc/pic/build-bindgen.png       |  Bin 0 -> 279759 bytes
 crates/lwext4_rust/doc/pic/ext4-blockdev-box.png   |  Bin 0 -> 74967 bytes
 crates/lwext4_rust/doc/pic/ext4-rust-github.png    |  Bin 0 -> 169530 bytes
 crates/lwext4_rust/doc/pic/image.png               |  Bin 0 -> 167443 bytes
 crates/lwext4_rust/doc/pic/lwext4-seek.png         |  Bin 0 -> 32920 bytes
 crates/lwext4_rust/doc/pic/run-ext4-on-os.png      |  Bin 0 -> 305921 bytes
 crates/lwext4_rust/doc/rust-ext4-fs-support.md     | 1205 ++++++++
 crates/lwext4_rust/examples/.cargo/config          |    7 +
 crates/lwext4_rust/examples/.gitignore             |    2 +
 crates/lwext4_rust/examples/Cargo.toml             |   18 +
 crates/lwext4_rust/examples/Makefile               |   66 +
 crates/lwext4_rust/examples/rust-toolchain.toml    |    6 +
 crates/lwext4_rust/examples/src/boot/lang_items.rs |   14 +
 crates/lwext4_rust/examples/src/boot/linker64.ld   |   48 +
 crates/lwext4_rust/examples/src/boot/logger.rs     |   89 +
 crates/lwext4_rust/examples/src/boot/mod.rs        |   28 +
 crates/lwext4_rust/examples/src/boot/sbi.rs        |   69 +
 crates/lwext4_rust/examples/src/disk.rs            |  211 ++
 crates/lwext4_rust/examples/src/ext4fs.rs          |  316 ++
 crates/lwext4_rust/examples/src/main.rs            |  304 ++
 crates/lwext4_rust/examples/src/pci_impl.rs        |  142 +
 crates/lwext4_rust/examples/src/vfs_ops.rs         |  115 +
 crates/lwext4_rust/examples/src/virtio_impl.rs     |   51 +
 crates/lwext4_rust/src/bindings.rs                 | 2279 ++++++++++++++
 crates/lwext4_rust/src/blockdev.rs                 |  393 +++
 crates/lwext4_rust/src/file.rs                     |  700 +++++
 crates/lwext4_rust/src/lib.rs                      |   37 +
 crates/lwext4_rust/src/ulibc.rs                    |  117 +
 164 files changed, 39985 insertions(+)

======================================
提交ID: 15eb4b0
作者: unfound <t202510336998092@eduxiji.net>
时间: Sun Jan 4 03:03:23 2026 +0800
说明: axfs 修改
======================================
 modules/axfs/Cargo.toml | 33 +++++++++++++++++++++------------
 1 file changed, 21 insertions(+), 12 deletions(-)

======================================
提交ID: 82fb8de
作者: unfound <t202510336998092@eduxiji.net>
时间: Sun Jan 4 02:58:21 2026 +0800
说明: 配置修改
======================================
 Cargo.lock | 183 ++++++++++++++++++++++++++++++++++++++++++++++---------------
 Cargo.toml |  27 +++------
 2 files changed, 147 insertions(+), 63 deletions(-)

======================================
提交ID: 3d13822
作者: unfound <t202510336998092@eduxiji.net>
时间: Sun Jan 4 02:49:06 2026 +0800
说明: axfs 修改
======================================
 {axfs => modules/axfs}/Cargo.toml                   | 0
 {axfs => modules/axfs}/resources/create_test_img.sh | 0
 {axfs => modules/axfs}/src/api/dir.rs               | 0
 {axfs => modules/axfs}/src/api/file.rs              | 0
 {axfs => modules/axfs}/src/api/mod.rs               | 0
 {axfs => modules/axfs}/src/dev.rs                   | 0
 {axfs => modules/axfs}/src/fops.rs                  | 0
 {axfs => modules/axfs}/src/fs/fatfs.rs              | 0
 {axfs => modules/axfs}/src/fs/mod.rs                | 0
 {axfs => modules/axfs}/src/fs/myfs.rs               | 0
 {axfs => modules/axfs}/src/lib.rs                   | 0
 {axfs => modules/axfs}/src/mounts.rs                | 0
 {axfs => modules/axfs}/src/root.rs                  | 0
 {axfs => modules/axfs}/tests/test_common/mod.rs     | 0
 {axfs => modules/axfs}/tests/test_fatfs.rs          | 0
 {axfs => modules/axfs}/tests/test_ramfs.rs          | 0
 16 files changed, 0 insertions(+), 0 deletions(-)

======================================
提交ID: c5b92b2
作者: unfound <t202510336998092@eduxiji.net>
时间: Sun Jan 4 02:48:08 2026 +0800
说明: axfs 修改
======================================
 {modules/axfs => axfs}/Cargo.toml                   | 0
 {modules/axfs => axfs}/resources/create_test_img.sh | 0
 {modules/axfs => axfs}/src/api/dir.rs               | 0
 {modules/axfs => axfs}/src/api/file.rs              | 0
 {modules/axfs => axfs}/src/api/mod.rs               | 0
 {modules/axfs => axfs}/src/dev.rs                   | 0
 {modules/axfs => axfs}/src/fops.rs                  | 0
 {modules/axfs => axfs}/src/fs/fatfs.rs              | 0
 {modules/axfs => axfs}/src/fs/mod.rs                | 0
 {modules/axfs => axfs}/src/fs/myfs.rs               | 0
 {modules/axfs => axfs}/src/lib.rs                   | 0
 {modules/axfs => axfs}/src/mounts.rs                | 0
 {modules/axfs => axfs}/src/root.rs                  | 0
 {modules/axfs => axfs}/tests/test_common/mod.rs     | 0
 {modules/axfs => axfs}/tests/test_fatfs.rs          | 0
 {modules/axfs => axfs}/tests/test_ramfs.rs          | 0
 16 files changed, 0 insertions(+), 0 deletions(-)

======================================
提交ID: 08cb2f0
作者: unfound <t202510336998092@eduxiji.net>
时间: Sun Jan 4 02:42:25 2026 +0800
说明: api
======================================
 api/arceos_posix_api/Cargo.toml                   |    3 +-
 api/arceos_posix_api/build.rs                     |   20 +-
 api/arceos_posix_api/src/ctype_my.rs              |   93 ++
 api/arceos_posix_api/src/imp/fd_ops.rs            |   11 +-
 api/arceos_posix_api/src/imp/fs.rs                | 1047 +++++++++++++++++-
 api/arceos_posix_api/src/imp/fs_complex_backup.rs | 1216 +++++++++++++++++++++
 api/arceos_posix_api/src/imp/io_mpx/epoll.rs      |   47 +
 api/arceos_posix_api/src/imp/mod.rs               |    2 +
 api/arceos_posix_api/src/imp/net.rs               |   47 +
 api/arceos_posix_api/src/imp/path_link.rs         |  380 +++++++
 api/arceos_posix_api/src/imp/pipe.rs              |   47 +
 api/arceos_posix_api/src/imp/stdio.rs             |   94 ++
 api/arceos_posix_api/src/lib.rs                   |   16 +-
 13 files changed, 2987 insertions(+), 36 deletions(-)

======================================
提交ID: aa9eb26
作者: unfound <t202510336998092@eduxiji.net>
时间: Sun Jan 4 02:39:43 2026 +0800
说明: crates add
======================================
 crates/axfs_crates/Cargo.lock                      | 157 ++++++
 crates/axfs_crates/target/.rustc_info.json         |   1 +
 crates/axfs_crates/target/CACHEDIR.TAG             |   3 +
 crates/axfs_crates/target/debug/.cargo-lock        |   0
 .../axerrno-0e942d60553ca33e/dep-lib-axerrno       | Bin 0 -> 14 bytes
 .../axerrno-0e942d60553ca33e/invoked.timestamp     |   1 +
 .../axerrno-0e942d60553ca33e/lib-axerrno           |   1 +
 .../axerrno-0e942d60553ca33e/lib-axerrno.json      |   1 +
 .../axerrno-771c795ddbc7f76c/dep-lib-axerrno       | Bin 0 -> 75 bytes
 .../axerrno-771c795ddbc7f76c/invoked.timestamp     |   1 +
 .../axerrno-771c795ddbc7f76c/lib-axerrno           |   1 +
 .../axerrno-771c795ddbc7f76c/lib-axerrno.json      |   1 +
 .../run-build-script-build-script-build            |   1 +
 .../run-build-script-build-script-build.json       |   1 +
 .../build-script-build-script-build                |   1 +
 .../build-script-build-script-build.json           |   1 +
 .../dep-build-script-build-script-build            | Bin 0 -> 14 bytes
 .../axerrno-fb4107a154223bac/invoked.timestamp     |   1 +
 .../axfs_devfs-6efe3e84897c72ad/dep-lib-axfs_devfs | Bin 0 -> 80 bytes
 .../axfs_devfs-6efe3e84897c72ad/invoked.timestamp  |   1 +
 .../axfs_devfs-6efe3e84897c72ad/lib-axfs_devfs     |   1 +
 .../lib-axfs_devfs.json                            |   1 +
 .../output-lib-axfs_devfs                          |   5 +
 .../dep-test-lib-axfs_devfs                        | Bin 0 -> 98 bytes
 .../axfs_devfs-b05265814a2ba911/invoked.timestamp  |   1 +
 .../output-test-lib-axfs_devfs                     |   5 +
 .../test-lib-axfs_devfs                            |   1 +
 .../test-lib-axfs_devfs.json                       |   1 +
 .../dep-test-lib-axfs_procfs                       | Bin 0 -> 81 bytes
 .../axfs_procfs-412e73fdc72fe197/invoked.timestamp |   1 +
 .../output-test-lib-axfs_procfs                    |   2 +
 .../test-lib-axfs_procfs                           |   1 +
 .../test-lib-axfs_procfs.json                      |   1 +
 .../dep-lib-axfs_procfs                            | Bin 0 -> 63 bytes
 .../axfs_procfs-c23fb5891ae292d6/invoked.timestamp |   1 +
 .../axfs_procfs-c23fb5891ae292d6/lib-axfs_procfs   |   1 +
 .../lib-axfs_procfs.json                           |   1 +
 .../dep-test-lib-axfs_ramfs                        | Bin 0 -> 81 bytes
 .../axfs_ramfs-29e0a6a9fe7389db/invoked.timestamp  |   1 +
 .../test-lib-axfs_ramfs                            |   1 +
 .../test-lib-axfs_ramfs.json                       |   1 +
 .../axfs_ramfs-4d8742370d4cc209/dep-lib-axfs_ramfs | Bin 0 -> 63 bytes
 .../axfs_ramfs-4d8742370d4cc209/invoked.timestamp  |   1 +
 .../axfs_ramfs-4d8742370d4cc209/lib-axfs_ramfs     |   1 +
 .../lib-axfs_ramfs.json                            |   1 +
 .../axfs_vfs-ea39133e87f2a9c3/dep-lib-axfs_vfs     | Bin 0 -> 86 bytes
 .../axfs_vfs-ea39133e87f2a9c3/invoked.timestamp    |   1 +
 .../axfs_vfs-ea39133e87f2a9c3/lib-axfs_vfs         |   1 +
 .../axfs_vfs-ea39133e87f2a9c3/lib-axfs_vfs.json    |   1 +
 .../axfs_vfs-ea39133e87f2a9c3/output-lib-axfs_vfs  |  28 ++
 .../bitflags-d6b73c956113621d/dep-lib-bitflags     | Bin 0 -> 14 bytes
 .../bitflags-d6b73c956113621d/invoked.timestamp    |   1 +
 .../bitflags-d6b73c956113621d/lib-bitflags         |   1 +
 .../bitflags-d6b73c956113621d/lib-bitflags.json    |   1 +
 .../heck-20e948fc986d72b6/dep-lib-heck             | Bin 0 -> 14 bytes
 .../heck-20e948fc986d72b6/invoked.timestamp        |   1 +
 .../.fingerprint/heck-20e948fc986d72b6/lib-heck    |   1 +
 .../heck-20e948fc986d72b6/lib-heck.json            |   1 +
 .../lock_api-2b15bcdf84600135/dep-lib-lock_api     | Bin 0 -> 14 bytes
 .../lock_api-2b15bcdf84600135/invoked.timestamp    |   1 +
 .../lock_api-2b15bcdf84600135/lib-lock_api         |   1 +
 .../lock_api-2b15bcdf84600135/lib-lock_api.json    |   1 +
 .../.fingerprint/log-853433100b10785a/dep-lib-log  | Bin 0 -> 14 bytes
 .../log-853433100b10785a/invoked.timestamp         |   1 +
 .../.fingerprint/log-853433100b10785a/lib-log      |   1 +
 .../.fingerprint/log-853433100b10785a/lib-log.json |   1 +
 .../build-script-build-script-build                |   1 +
 .../build-script-build-script-build.json           |   1 +
 .../dep-build-script-build-script-build            | Bin 0 -> 14 bytes
 .../proc-macro2-7dbfc7e450e58b2e/invoked.timestamp |   1 +
 .../dep-lib-proc_macro2                            | Bin 0 -> 14 bytes
 .../proc-macro2-a7089ebad384fc04/invoked.timestamp |   1 +
 .../proc-macro2-a7089ebad384fc04/lib-proc_macro2   |   1 +
 .../lib-proc_macro2.json                           |   1 +
 .../run-build-script-build-script-build            |   1 +
 .../run-build-script-build-script-build.json       |   1 +
 .../run-build-script-build-script-build            |   1 +
 .../run-build-script-build-script-build.json       |   1 +
 .../build-script-build-script-build                |   1 +
 .../build-script-build-script-build.json           |   1 +
 .../dep-build-script-build-script-build            | Bin 0 -> 14 bytes
 .../quote-f14c48d1c4fb57ce/invoked.timestamp       |   1 +
 .../quote-fd1f92da8e164e73/dep-lib-quote           | Bin 0 -> 14 bytes
 .../quote-fd1f92da8e164e73/invoked.timestamp       |   1 +
 .../.fingerprint/quote-fd1f92da8e164e73/lib-quote  |   1 +
 .../quote-fd1f92da8e164e73/lib-quote.json          |   1 +
 .../scopeguard-cfa69166bd4da734/dep-lib-scopeguard | Bin 0 -> 14 bytes
 .../scopeguard-cfa69166bd4da734/invoked.timestamp  |   1 +
 .../scopeguard-cfa69166bd4da734/lib-scopeguard     |   1 +
 .../lib-scopeguard.json                            |   1 +
 .../spin-2c0339ac059a6b12/dep-lib-spin             | Bin 0 -> 14 bytes
 .../spin-2c0339ac059a6b12/invoked.timestamp        |   1 +
 .../.fingerprint/spin-2c0339ac059a6b12/lib-spin    |   1 +
 .../spin-2c0339ac059a6b12/lib-spin.json            |   1 +
 .../strum-ce082a349a7c1d1c/dep-lib-strum           | Bin 0 -> 14 bytes
 .../strum-ce082a349a7c1d1c/invoked.timestamp       |   1 +
 .../.fingerprint/strum-ce082a349a7c1d1c/lib-strum  |   1 +
 .../strum-ce082a349a7c1d1c/lib-strum.json          |   1 +
 .../dep-lib-strum_macros                           | Bin 0 -> 14 bytes
 .../invoked.timestamp                              |   1 +
 .../strum_macros-06ce07b389808601/lib-strum_macros |   1 +
 .../lib-strum_macros.json                          |   1 +
 .../.fingerprint/syn-66b839b40b77498e/dep-lib-syn  | Bin 0 -> 14 bytes
 .../syn-66b839b40b77498e/invoked.timestamp         |   1 +
 .../.fingerprint/syn-66b839b40b77498e/lib-syn      |   1 +
 .../.fingerprint/syn-66b839b40b77498e/lib-syn.json |   1 +
 .../dep-lib-unicode_ident                          | Bin 0 -> 14 bytes
 .../invoked.timestamp                              |   1 +
 .../lib-unicode_ident                              |   1 +
 .../lib-unicode_ident.json                         |   1 +
 .../axerrno-9be53b139578f1b2/invoked.timestamp     |   1 +
 .../axerrno-9be53b139578f1b2/out/linux_errno.rs    | 556 +++++++++++++++++++++
 .../debug/build/axerrno-9be53b139578f1b2/output    |   0
 .../build/axerrno-9be53b139578f1b2/root-output     |   1 +
 .../debug/build/axerrno-9be53b139578f1b2/stderr    |   0
 .../axerrno-fb4107a154223bac/build-script-build    | Bin 0 -> 3921336 bytes
 .../build_script_build-fb4107a154223bac            | Bin 0 -> 3921336 bytes
 .../build_script_build-fb4107a154223bac.d          |   5 +
 .../build-script-build                             | Bin 0 -> 4013232 bytes
 .../build_script_build-7dbfc7e450e58b2e            | Bin 0 -> 4013232 bytes
 .../build_script_build-7dbfc7e450e58b2e.d          |   5 +
 .../proc-macro2-b976d1e370997e47/invoked.timestamp |   1 +
 .../build/proc-macro2-b976d1e370997e47/output      |  21 +
 .../build/proc-macro2-b976d1e370997e47/root-output |   1 +
 .../build/proc-macro2-b976d1e370997e47/stderr      |   0
 .../build/quote-e62d430bc112b9d4/invoked.timestamp |   1 +
 .../debug/build/quote-e62d430bc112b9d4/output      |   2 +
 .../debug/build/quote-e62d430bc112b9d4/root-output |   1 +
 .../debug/build/quote-e62d430bc112b9d4/stderr      |   0
 .../quote-f14c48d1c4fb57ce/build-script-build      | Bin 0 -> 3955912 bytes
 .../build_script_build-f14c48d1c4fb57ce            | Bin 0 -> 3955912 bytes
 .../build_script_build-f14c48d1c4fb57ce.d          |   5 +
 .../target/debug/deps/axerrno-0e942d60553ca33e.d   |   8 +
 .../target/debug/deps/axerrno-771c795ddbc7f76c.d   |  11 +
 .../debug/deps/axfs_devfs-6efe3e84897c72ad.d       |  10 +
 .../target/debug/deps/axfs_devfs-b05265814a2ba911  | Bin 0 -> 7367912 bytes
 .../debug/deps/axfs_devfs-b05265814a2ba911.d       |   9 +
 .../target/debug/deps/axfs_procfs-412e73fdc72fe197 | Bin 0 -> 7937656 bytes
 .../debug/deps/axfs_procfs-412e73fdc72fe197.d      |   8 +
 .../debug/deps/axfs_procfs-c23fb5891ae292d6.d      |   9 +
 .../target/debug/deps/axfs_ramfs-29e0a6a9fe7389db  | Bin 0 -> 7713640 bytes
 .../debug/deps/axfs_ramfs-29e0a6a9fe7389db.d       |   8 +
 .../debug/deps/axfs_ramfs-4d8742370d4cc209.d       |   9 +
 .../target/debug/deps/axfs_vfs-ea39133e87f2a9c3.d  |  10 +
 .../target/debug/deps/bitflags-d6b73c956113621d.d  |  13 +
 .../target/debug/deps/heck-20e948fc986d72b6.d      |  15 +
 .../debug/deps/libaxerrno-0e942d60553ca33e.rlib    | Bin 0 -> 16096 bytes
 .../debug/deps/libaxerrno-0e942d60553ca33e.rmeta   | Bin 0 -> 14579 bytes
 .../debug/deps/libaxerrno-771c795ddbc7f76c.rlib    | Bin 0 -> 344514 bytes
 .../debug/deps/libaxerrno-771c795ddbc7f76c.rmeta   | Bin 0 -> 182042 bytes
 .../debug/deps/libaxfs_devfs-6efe3e84897c72ad.rlib | Bin 0 -> 1774504 bytes
 .../deps/libaxfs_devfs-6efe3e84897c72ad.rmeta      | Bin 0 -> 38844 bytes
 .../deps/libaxfs_procfs-c23fb5891ae292d6.rlib      | Bin 0 -> 3048338 bytes
 .../deps/libaxfs_procfs-c23fb5891ae292d6.rmeta     | Bin 0 -> 51938 bytes
 .../debug/deps/libaxfs_ramfs-4d8742370d4cc209.rlib | Bin 0 -> 2511320 bytes
 .../deps/libaxfs_ramfs-4d8742370d4cc209.rmeta      | Bin 0 -> 37882 bytes
 .../debug/deps/libaxfs_vfs-ea39133e87f2a9c3.rlib   | Bin 0 -> 1417808 bytes
 .../debug/deps/libaxfs_vfs-ea39133e87f2a9c3.rmeta  | Bin 0 -> 368813 bytes
 .../debug/deps/libbitflags-d6b73c956113621d.rlib   | Bin 0 -> 427988 bytes
 .../debug/deps/libbitflags-d6b73c956113621d.rmeta  | Bin 0 -> 196766 bytes
 .../debug/deps/libheck-20e948fc986d72b6.rlib       | Bin 0 -> 309248 bytes
 .../debug/deps/libheck-20e948fc986d72b6.rmeta      | Bin 0 -> 76723 bytes
 .../debug/deps/liblock_api-2b15bcdf84600135.rlib   | Bin 0 -> 383148 bytes
 .../debug/deps/liblock_api-2b15bcdf84600135.rmeta  | Bin 0 -> 375119 bytes
 .../target/debug/deps/liblog-853433100b10785a.rlib | Bin 0 -> 291940 bytes
 .../debug/deps/liblog-853433100b10785a.rmeta       | Bin 0 -> 192418 bytes
 .../deps/libproc_macro2-a7089ebad384fc04.rlib      | Bin 0 -> 1203432 bytes
 .../deps/libproc_macro2-a7089ebad384fc04.rmeta     | Bin 0 -> 339104 bytes
 .../debug/deps/libquote-fd1f92da8e164e73.rlib      | Bin 0 -> 494520 bytes
 .../debug/deps/libquote-fd1f92da8e164e73.rmeta     | Bin 0 -> 268535 bytes
 .../debug/deps/libscopeguard-cfa69166bd4da734.rlib | Bin 0 -> 27118 bytes
 .../deps/libscopeguard-cfa69166bd4da734.rmeta      | Bin 0 -> 25587 bytes
 .../debug/deps/libspin-2c0339ac059a6b12.rlib       | Bin 0 -> 274858 bytes
 .../debug/deps/libspin-2c0339ac059a6b12.rmeta      | Bin 0 -> 257383 bytes
 .../debug/deps/libstrum-ce082a349a7c1d1c.rlib      | Bin 0 -> 28820 bytes
 .../debug/deps/libstrum-ce082a349a7c1d1c.rmeta     | Bin 0 -> 27307 bytes
 .../debug/deps/libstrum_macros-06ce07b389808601.so | Bin 0 -> 7617848 bytes
 .../target/debug/deps/libsyn-66b839b40b77498e.rlib | Bin 0 -> 6018470 bytes
 .../debug/deps/libsyn-66b839b40b77498e.rmeta       | Bin 0 -> 2196021 bytes
 .../deps/libunicode_ident-12c495887fb81e30.rlib    | Bin 0 -> 54492 bytes
 .../deps/libunicode_ident-12c495887fb81e30.rmeta   | Bin 0 -> 34049 bytes
 .../target/debug/deps/lock_api-2b15bcdf84600135.d  |  10 +
 .../target/debug/deps/log-853433100b10785a.d       |  10 +
 .../debug/deps/proc_macro2-a7089ebad384fc04.d      |  18 +
 .../target/debug/deps/quote-fd1f92da8e164e73.d     |  13 +
 .../debug/deps/scopeguard-cfa69166bd4da734.d       |   7 +
 .../target/debug/deps/spin-2c0339ac059a6b12.d      |  14 +
 .../target/debug/deps/strum-ce082a349a7c1d1c.d     |   8 +
 .../debug/deps/strum_macros-06ce07b389808601.d     |  28 ++
 .../target/debug/deps/syn-66b839b40b77498e.d       |  49 ++
 .../debug/deps/unicode_ident-12c495887fb81e30.d    |   8 +
 .../s-heekufs3by-0jvwxcx.lock                      |   0
 .../s-heekufs4nk-09x4sfu.lock                      |   0
 .../s-heekybm7nu-03319gk.lock                      |   0
 .../s-heekxlkwnk-13huja6.lock                      |   0
 .../s-heeku8towm-0mshk99.lock                      |   0
 .../s-heeku8tqdw-0lkmd18.lock                      |   0
 .../s-heektpvprg-13q7k1r.lock                      |   0
 198 files changed, 1154 insertions(+)

======================================
提交ID: 32ec40e
作者: unfound <t202510336998092@eduxiji.net>
时间: Sun Jan 4 02:35:21 2026 +0800
说明: Update README with file system innovations
======================================
 README.md | 268 +++++++++++++++++++-------------------------------------------
 1 file changed, 83 insertions(+), 185 deletions(-)

======================================
提交ID: 53abb95
作者: R-Y-L <1171577592@qq.com>
时间: Sat Jan 3 21:45:49 2026 +0800
说明: Merge branch 'main' of https://gitlab.eduxiji.net/T202510336998092/project3035746-358028
======================================
======================================
提交ID: e9f8cd1
作者: R-Y-L <1171577592@qq.com>
时间: Wed Dec 31 16:11:08 2025 +0800
说明: 调整测试细节
======================================
 examples/allocators/src/main.rs               |  2 +-
 modules/axalloc/src/bin/allocator_test.rs     | 22 +++++------
 modules/axalloc/src/tests/allocator_tester.rs | 53 ++++++++++++++++-----------
 modules/axalloc/src/tests/mod.rs              | 46 +++++++++++++++++------
 modules/axalloc/src/tests/workloads.rs        | 18 ++++-----
 5 files changed, 88 insertions(+), 53 deletions(-)

======================================
提交ID: 0ee66b0
作者: unfound <t202510336998092@eduxiji.net>
时间: Wed Dec 31 15:46:47 2025 +0800
说明: add README
======================================
 README.md | 0
 1 file changed, 0 insertions(+), 0 deletions(-)

======================================
提交ID: 163d360
作者: R-Y-L <1171577592@qq.com>
时间: Tue Dec 16 16:42:56 2025 +0800
说明: 添加初步内核运行测试
======================================
 Cargo.lock                      | 50 ++++++++++++++++++-----------
 Cargo.toml                      |  1 +
 examples/allocators/Cargo.toml  |  9 ++++++
 examples/allocators/src/main.rs | 70 +++++++++++++++++++++++++++++++++++++++++
 4 files changed, 112 insertions(+), 18 deletions(-)

======================================
提交ID: 46771b2
作者: R-Y-L <1171577592@qq.com>
时间: Sun Dec 14 22:47:20 2025 +0800
说明: 添加get_stats
======================================
 modules/axalloc/src/allocators/bitmap.rs |  6 ++++++
 modules/axalloc/src/allocators/buddy.rs  | 21 ++++++++++++++++++++-
 modules/axalloc/src/allocators/hybrid.rs | 23 +++++++++++++++++++++++
 3 files changed, 49 insertions(+), 1 deletion(-)

======================================
提交ID: c36f013
作者: R-Y-L <1171577592@qq.com>
时间: Sun Dec 14 19:33:39 2025 +0800
说明: 测试使用统一接口PageAllocator，添加碎片率等测试接口
======================================
 modules/axalloc/src/allocators/bitmap.rs  |   2 +-
 modules/axalloc/src/allocators/mod.rs     |   6 +
 modules/axalloc/src/bin/allocator_test.rs | 262 ++++++++----------------------
 3 files changed, 72 insertions(+), 198 deletions(-)

======================================
提交ID: ad3b744
作者: R-Y-L <1171577592@qq.com>
时间: Sat Dec 13 23:37:49 2025 +0800
说明: buddy移植+测试初步
======================================
 modules/axalloc/Cargo.toml                    |   9 +-
 modules/axalloc/src/allocators/buddy.rs       |   6 +-
 modules/axalloc/src/allocators/hybrid.rs      | 319 +++++++++++++++++++++++++-
 modules/axalloc/src/bin/allocator_test.rs     | 319 ++++++++++++++++++++++++++
 modules/axalloc/src/tests/allocator_tester.rs |  81 +++++++
 modules/axalloc/src/tests/mod.rs              |  61 +++++
 modules/axalloc/src/tests/workloads.rs        |  50 ++++
 7 files changed, 830 insertions(+), 15 deletions(-)

======================================
提交ID: 01b3633
作者: R-Y-L <1171577592@qq.com>
时间: Wed Dec 10 15:05:54 2025 +0800
说明: buddy移植
======================================
 modules/axalloc/src/allocators/bitmap.rs |  43 +++++---
 modules/axalloc/src/allocators/buddy.rs  | 175 +++++++++++++++++++++++++++----
 modules/axalloc/src/allocators/mod.rs    |  89 +++++++++++++++-
 modules/axalloc/src/lib.rs               |  54 ++++++++++
 4 files changed, 323 insertions(+), 38 deletions(-)

======================================
提交ID: 0341f0e
作者: dj <2253075320@qq.com>
时间: Wed Dec 3 15:18:01 2025 +0800
说明: 基于arceos+内存管理模块模块布局初步修改
======================================
 .clang-format                                      |   16 +
 .gitattributes                                     |    1 +
 .gitignore                                         |   12 +
 Cargo.lock                                         | 2396 ++++++++++++++++++++
 Cargo.toml                                         |   72 +
 Dockerfile                                         |   39 +
 LICENSE.Apache2                                    |  201 ++
 LICENSE.GPLv3                                      |  674 ++++++
 LICENSE.MulanPSL2                                  |  127 ++
 LICENSE.MulanPubL2                                 |  183 ++
 Makefile                                           |  232 ++
 README.md                                          |  217 ++
 api/arceos_api/Cargo.toml                          |   47 +
 api/arceos_api/src/imp/display.rs                  |   11 +
 api/arceos_api/src/imp/fs.rs                       |   87 +
 api/arceos_api/src/imp/mem.rs                      |   25 +
 api/arceos_api/src/imp/mod.rs                      |   54 +
 api/arceos_api/src/imp/net.rs                      |  131 ++
 api/arceos_api/src/imp/task.rs                     |  138 ++
 api/arceos_api/src/lib.rs                          |  413 ++++
 api/arceos_api/src/macros.rs                       |  110 +
 api/arceos_posix_api/.gitignore                    |    1 +
 api/arceos_posix_api/Cargo.toml                    |   58 +
 api/arceos_posix_api/build.rs                      |  112 +
 api/arceos_posix_api/ctypes.h                      |   15 +
 api/arceos_posix_api/src/imp/fd_ops.rs             |  139 ++
 api/arceos_posix_api/src/imp/fs.rs                 |  218 ++
 api/arceos_posix_api/src/imp/io.rs                 |   85 +
 api/arceos_posix_api/src/imp/io_mpx/epoll.rs       |  205 ++
 api/arceos_posix_api/src/imp/io_mpx/mod.rs         |   16 +
 api/arceos_posix_api/src/imp/io_mpx/select.rs      |  165 ++
 api/arceos_posix_api/src/imp/mod.rs                |   20 +
 api/arceos_posix_api/src/imp/net.rs                |  581 +++++
 api/arceos_posix_api/src/imp/pipe.rs               |  214 ++
 api/arceos_posix_api/src/imp/pthread/mod.rs        |  154 ++
 api/arceos_posix_api/src/imp/pthread/mutex.rs      |   70 +
 api/arceos_posix_api/src/imp/resources.rs          |   51 +
 api/arceos_posix_api/src/imp/stdio.rs              |  175 ++
 api/arceos_posix_api/src/imp/sys.rs                |   43 +
 api/arceos_posix_api/src/imp/task.rs               |   40 +
 api/arceos_posix_api/src/imp/time.rs               |   92 +
 api/arceos_posix_api/src/lib.rs                    |   60 +
 api/arceos_posix_api/src/utils.rs                  |   61 +
 api/axfeat/Cargo.toml                              |   87 +
 api/axfeat/src/lib.rs                              |   41 +
 configs/custom/x86_64-pc-oslab.toml                |   61 +
 configs/defconfig.toml                             |    6 +
 configs/dummy.toml                                 |   58 +
 doc/README.md                                      |   47 +
 doc/build.md                                       |   80 +
 doc/figures/ArceOS.svg                             |    4 +
 doc/figures/draw_jtag_connected.jpg                |  Bin 0 -> 121786 bytes
 doc/figures/image_jtag_connected.jpg               |  Bin 0 -> 240596 bytes
 doc/figures/phytium_ok.png                         |  Bin 0 -> 413228 bytes
 doc/figures/phytium_select_dtb.png                 |  Bin 0 -> 43675 bytes
 doc/figures/phytium_uart.png                       |  Bin 0 -> 266547 bytes
 doc/grub_boot.md                                   |   51 +
 doc/ixgbe.md                                       |   52 +
 doc/jtag_debug_in_raspi4.md                        |  276 +++
 doc/platform_phytium_pi.md                         |   52 +
 doc/platform_raspi4.md                             |   28 +
 examples/helloworld-c/main.c                       |    7 +
 examples/helloworld-myplat/Cargo.toml              |   33 +
 examples/helloworld-myplat/src/main.rs             |   31 +
 examples/helloworld/Cargo.toml                     |   10 +
 examples/helloworld/src/main.rs                    |   10 +
 examples/httpclient-c/axbuild.mk                   |    1 +
 examples/httpclient-c/features.txt                 |    3 +
 examples/httpclient-c/httpclient.c                 |   56 +
 examples/httpclient/Cargo.toml                     |   14 +
 examples/httpclient/src/main.rs                    |   40 +
 examples/httpserver-c/axbuild.mk                   |    1 +
 examples/httpserver-c/features.txt                 |    3 +
 examples/httpserver-c/httpserver.c                 |   90 +
 examples/httpserver/Cargo.toml                     |   10 +
 examples/httpserver/src/main.rs                    |   96 +
 examples/shell/Cargo.toml                          |   17 +
 examples/shell/src/cmd.rs                          |  299 +++
 examples/shell/src/main.rs                         |   82 +
 examples/shell/src/ramfs.rs                        |   15 +
 modules/axalloc/Cargo.toml                         |   33 +
 modules/axalloc/src/allocators/bitmap.rs           |   39 +
 modules/axalloc/src/allocators/buddy.rs            |   39 +
 modules/axalloc/src/allocators/hybrid.rs           |   39 +
 modules/axalloc/src/allocators/mod.rs              |   61 +
 modules/axalloc/src/lib.rs                         |  257 +++
 modules/axalloc/src/page.rs                        |  108 +
 modules/axconfig/Cargo.toml                        |   13 +
 modules/axconfig/build.rs                          |    6 +
 modules/axconfig/src/lib.rs                        |   14 +
 modules/axdisplay/Cargo.toml                       |   17 +
 modules/axdisplay/src/lib.rs                       |   36 +
 modules/axdma/Cargo.toml                           |   22 +
 modules/axdma/src/dma.rs                           |  128 ++
 modules/axdma/src/lib.rs                           |  109 +
 modules/axdriver/Cargo.toml                        |   48 +
 modules/axdriver/build.rs                          |   75 +
 modules/axdriver/src/bus/mmio.rs                   |   23 +
 modules/axdriver/src/bus/mod.rs                    |    4 +
 modules/axdriver/src/bus/pci.rs                    |  122 +
 modules/axdriver/src/drivers.rs                    |  176 ++
 modules/axdriver/src/dummy.rs                      |  102 +
 modules/axdriver/src/ixgbe.rs                      |   39 +
 modules/axdriver/src/lib.rs                        |  184 ++
 modules/axdriver/src/macros.rs                     |   73 +
 modules/axdriver/src/prelude.rs                    |   10 +
 modules/axdriver/src/structs/dyn.rs                |   85 +
 modules/axdriver/src/structs/mod.rs                |   51 +
 modules/axdriver/src/structs/static.rs             |   75 +
 modules/axdriver/src/virtio.rs                     |  171 ++
 modules/axfs/Cargo.toml                            |   55 +
 modules/axfs/resources/create_test_img.sh          |   30 +
 modules/axfs/src/api/dir.rs                        |  150 ++
 modules/axfs/src/api/file.rs                       |  193 ++
 modules/axfs/src/api/mod.rs                        |   89 +
 modules/axfs/src/dev.rs                            |   92 +
 modules/axfs/src/fops.rs                           |  418 ++++
 modules/axfs/src/fs/fatfs.rs                       |  301 +++
 modules/axfs/src/fs/mod.rs                         |   13 +
 modules/axfs/src/fs/myfs.rs                        |   16 +
 modules/axfs/src/lib.rs                            |   46 +
 modules/axfs/src/mounts.rs                         |   82 +
 modules/axfs/src/root.rs                           |  313 +++
 modules/axfs/tests/test_common/mod.rs              |  262 +++
 modules/axfs/tests/test_fatfs.rs                   |   27 +
 modules/axfs/tests/test_ramfs.rs                   |   56 +
 modules/axhal/.gitignore                           |    1 +
 modules/axhal/Cargo.toml                           |   58 +
 modules/axhal/build.rs                             |   35 +
 modules/axhal/linker.lds.S                         |  101 +
 modules/axhal/src/dummy.rs                         |  116 +
 modules/axhal/src/irq.rs                           |   24 +
 modules/axhal/src/lib.rs                           |  143 ++
 modules/axhal/src/mem.rs                           |  126 +
 modules/axhal/src/paging.rs                        |   48 +
 modules/axhal/src/percpu.rs                        |  127 ++
 modules/axhal/src/time.rs                          |    9 +
 modules/axhal/src/tls.rs                           |  181 ++
 modules/axipi/Cargo.toml                           |   21 +
 modules/axipi/src/event.rs                         |   57 +
 modules/axipi/src/lib.rs                           |   80 +
 modules/axipi/src/queue.rs                         |   53 +
 modules/axlog/Cargo.toml                           |   30 +
 modules/axlog/src/lib.rs                           |  262 +++
 modules/axmm/Cargo.toml                            |   24 +
 modules/axmm/src/aspace.rs                         |  321 +++
 modules/axmm/src/backend/alloc.rs                  |  108 +
 modules/axmm/src/backend/linear.rs                 |   46 +
 modules/axmm/src/backend/mod.rs                    |   87 +
 modules/axmm/src/lib.rs                            |  114 +
 modules/axnet/Cargo.toml                           |   42 +
 modules/axnet/src/lib.rs                           |   47 +
 modules/axnet/src/smoltcp_impl/addr.rs             |    4 +
 modules/axnet/src/smoltcp_impl/bench.rs            |   74 +
 modules/axnet/src/smoltcp_impl/dns.rs              |   89 +
 modules/axnet/src/smoltcp_impl/listen_table.rs     |  155 ++
 modules/axnet/src/smoltcp_impl/mod.rs              |  337 +++
 modules/axnet/src/smoltcp_impl/tcp.rs              |  563 +++++
 modules/axnet/src/smoltcp_impl/udp.rs              |  295 +++
 modules/axns/Cargo.toml                            |   24 +
 modules/axns/src/lib.rs                            |  275 +++
 modules/axns/src/link.rs                           |  103 +
 modules/axns/tests/test_global.rs                  |   71 +
 modules/axns/tests/test_thread_local.rs            |   85 +
 modules/axruntime/Cargo.toml                       |   46 +
 modules/axruntime/src/lang_items.rs                |    7 +
 modules/axruntime/src/lib.rs                       |  282 +++
 modules/axruntime/src/mp.rs                        |   73 +
 modules/axsync/Cargo.toml                          |   24 +
 modules/axsync/src/lib.rs                          |   28 +
 modules/axsync/src/mutex.rs                        |  150 ++
 modules/axtask/Cargo.toml                          |   56 +
 modules/axtask/src/api.rs                          |  220 ++
 modules/axtask/src/api_s.rs                        |   22 +
 modules/axtask/src/lib.rs                          |   59 +
 modules/axtask/src/run_queue.rs                    |  668 ++++++
 modules/axtask/src/task.rs                         |  555 +++++
 modules/axtask/src/task_ext.rs                     |  201 ++
 modules/axtask/src/tests.rs                        |  130 ++
 modules/axtask/src/timers.rs                       |   66 +
 modules/axtask/src/wait_queue.rs                   |  247 ++
 rust-toolchain.toml                                |   10 +
 scripts/make/bsta1000b-fada.mk                     |    4 +
 scripts/make/build.mk                              |   74 +
 scripts/make/build_c.mk                            |   77 +
 scripts/make/cargo.mk                              |   55 +
 scripts/make/config.mk                             |   33 +
 scripts/make/deps.mk                               |   19 +
 scripts/make/features.mk                           |   66 +
 scripts/make/platform.mk                           |   59 +
 scripts/make/qemu.mk                               |  120 +
 scripts/make/raspi4.mk                             |   95 +
 scripts/make/utils.mk                              |   23 +
 scripts/net/create-bridge.sh                       |   26 +
 scripts/net/del-iface.sh                           |   14 +
 scripts/net/pci-bind.sh                            |   23 +
 scripts/net/qemu-ifup.sh                           |   18 +
 scripts/net/set-ip-forward.sh                      |   18 +
 scripts/net/unset-ip-forward.sh                    |   18 +
 tools/.gitignore                                   |    5 +
 tools/bsta1000b/bsta1000b-fada-arceos.its          |   50 +
 tools/bsta1000b/bsta1000b-fada.dtb                 |  Bin 0 -> 59621 bytes
 tools/bwbench_client/Cargo.toml                    |   13 +
 tools/bwbench_client/README.md                     |   37 +
 tools/bwbench_client/src/device.rs                 |  217 ++
 tools/bwbench_client/src/main.rs                   |  133 ++
 tools/deptool/Cargo.toml                           |   10 +
 tools/deptool/Makefile                             |   34 +
 tools/deptool/README.md                            |   17 +
 tools/deptool/src/cmd_builder.rs                   |   18 +
 tools/deptool/src/cmd_parser.rs                    |   91 +
 tools/deptool/src/d2_generator.rs                  |   39 +
 tools/deptool/src/lib.rs                           |  100 +
 tools/deptool/src/main.rs                          |   11 +
 tools/deptool/src/mermaid_generator.rs             |   37 +
 tools/phytium_pi/phytium.dts                       | 1523 +++++++++++++
 tools/phytium_pi/phytiumpi_firefly.dtb             |  Bin 0 -> 24293 bytes
 tools/raspi4/chainloader/.gitignore                |    1 +
 tools/raspi4/chainloader/.vscode/settings.json     |   10 +
 tools/raspi4/chainloader/Cargo.lock                |   26 +
 tools/raspi4/chainloader/Cargo.toml                |   33 +
 tools/raspi4/chainloader/Makefile                  |  236 ++
 tools/raspi4/chainloader/README.CN.md              |  116 +
 tools/raspi4/chainloader/README.md                 |  670 ++++++
 tools/raspi4/chainloader/build.rs                  |   20 +
 tools/raspi4/chainloader/src/_arch/aarch64/cpu.rs  |   37 +
 .../chainloader/src/_arch/aarch64/cpu/boot.rs      |   32 +
 .../chainloader/src/_arch/aarch64/cpu/boot.s       |   88 +
 tools/raspi4/chainloader/src/bsp.rs                |   13 +
 tools/raspi4/chainloader/src/bsp/device_driver.rs  |   12 +
 .../chainloader/src/bsp/device_driver/bcm.rs       |   11 +
 .../src/bsp/device_driver/bcm/bcm2xxx_gpio.rs      |  228 ++
 .../bsp/device_driver/bcm/bcm2xxx_pl011_uart.rs    |  402 ++++
 .../chainloader/src/bsp/device_driver/common.rs    |   38 +
 tools/raspi4/chainloader/src/bsp/raspberrypi.rs    |   26 +
 .../raspi4/chainloader/src/bsp/raspberrypi/cpu.rs  |   14 +
 .../chainloader/src/bsp/raspberrypi/driver.rs      |   71 +
 .../chainloader/src/bsp/raspberrypi/kernel.ld      |   83 +
 .../chainloader/src/bsp/raspberrypi/memory.rs      |   48 +
 tools/raspi4/chainloader/src/console.rs            |   83 +
 .../raspi4/chainloader/src/console/null_console.rs |   41 +
 tools/raspi4/chainloader/src/cpu.rs                |   19 +
 tools/raspi4/chainloader/src/cpu/boot.rs           |    9 +
 tools/raspi4/chainloader/src/driver.rs             |  154 ++
 tools/raspi4/chainloader/src/main.rs               |  219 ++
 tools/raspi4/chainloader/src/panic_wait.rs         |   64 +
 tools/raspi4/chainloader/src/print.rs              |   36 +
 tools/raspi4/chainloader/src/synchronization.rs    |   77 +
 tools/raspi4/chainloader/tests/chainboot_test.rb   |   78 +
 tools/raspi4/chainloader/update.sh                 |    8 +
 tools/raspi4/common/docker.mk                      |    1 +
 tools/raspi4/common/format.mk                      |   12 +
 tools/raspi4/common/operating_system.mk            |    9 +
 tools/raspi4/common/serial/minipush.rb             |  131 ++
 .../common/serial/minipush/progressbar_patch.rb    |   29 +
 tools/raspi4/common/serial/miniterm.rb             |  144 ++
 ulib/axlibc/.gitignore                             |    3 +
 ulib/axlibc/Cargo.toml                             |   64 +
 ulib/axlibc/build.rs                               |   29 +
 ulib/axlibc/c/assert.c                             |    8 +
 ulib/axlibc/c/ctype.c                              |   17 +
 ulib/axlibc/c/dirent.c                             |   98 +
 ulib/axlibc/c/dlfcn.c                              |   39 +
 ulib/axlibc/c/env.c                                |   31 +
 ulib/axlibc/c/fcntl.c                              |   56 +
 ulib/axlibc/c/flock.c                              |    9 +
 ulib/axlibc/c/fnmatch.c                            |   15 +
 ulib/axlibc/c/glob.c                               |  329 +++
 ulib/axlibc/c/ioctl.c                              |    9 +
 ulib/axlibc/c/libgen.c                             |   33 +
 ulib/axlibc/c/libm.c                               |   17 +
 ulib/axlibc/c/libm.h                               |  233 ++
 ulib/axlibc/c/locale.c                             |   42 +
 ulib/axlibc/c/log.c                                |  395 ++++
 ulib/axlibc/c/math.c                               |  571 +++++
 ulib/axlibc/c/mmap.c                               |   39 +
 ulib/axlibc/c/network.c                            |  226 ++
 ulib/axlibc/c/poll.c                               |    9 +
 ulib/axlibc/c/pow.c                                |  815 +++++++
 ulib/axlibc/c/printf.c                             | 1482 ++++++++++++
 ulib/axlibc/c/printf.h                             |  215 ++
 ulib/axlibc/c/printf_config.h                      |   14 +
 ulib/axlibc/c/pthread.c                            |  110 +
 ulib/axlibc/c/pwd.c                                |   14 +
 ulib/axlibc/c/resource.c                           |   10 +
 ulib/axlibc/c/sched.c                              |    9 +
 ulib/axlibc/c/select.c                             |   17 +
 ulib/axlibc/c/signal.c                             |   87 +
 ulib/axlibc/c/socket.c                             |   47 +
 ulib/axlibc/c/stat.c                               |   38 +
 ulib/axlibc/c/stdio.c                              |  412 ++++
 ulib/axlibc/c/stdlib.c                             |  385 ++++
 ulib/axlibc/c/string.c                             |  478 ++++
 ulib/axlibc/c/syslog.c                             |   16 +
 ulib/axlibc/c/time.c                               |  203 ++
 ulib/axlibc/c/unistd.c                             |  174 ++
 ulib/axlibc/c/utsname.c                            |    9 +
 ulib/axlibc/c/wait.c                               |   17 +
 ulib/axlibc/ctypes.h                               |    2 +
 ulib/axlibc/include/arpa/inet.h                    |   14 +
 ulib/axlibc/include/assert.h                       |   14 +
 ulib/axlibc/include/ctype.h                        |   20 +
 ulib/axlibc/include/dirent.h                       |   45 +
 ulib/axlibc/include/dlfcn.h                        |   41 +
 ulib/axlibc/include/endian.h                       |   68 +
 ulib/axlibc/include/errno.h                        |  150 ++
 ulib/axlibc/include/fcntl.h                        |  123 +
 ulib/axlibc/include/features.h                     |   11 +
 ulib/axlibc/include/float.h                        |  116 +
 ulib/axlibc/include/fnmatch.h                      |   24 +
 ulib/axlibc/include/glob.h                         |   34 +
 ulib/axlibc/include/inttypes.h                     |   10 +
 ulib/axlibc/include/langinfo.h                     |   82 +
 ulib/axlibc/include/libgen.h                       |   15 +
 ulib/axlibc/include/limits.h                       |   35 +
 ulib/axlibc/include/locale.h                       |   59 +
 ulib/axlibc/include/math.h                         |  321 +++
 ulib/axlibc/include/memory.h                       |    6 +
 ulib/axlibc/include/netdb.h                        |   85 +
 ulib/axlibc/include/netinet/in.h                   |  129 ++
 ulib/axlibc/include/netinet/tcp.h                  |   45 +
 ulib/axlibc/include/poll.h                         |   21 +
 ulib/axlibc/include/pthread.h                      |   84 +
 ulib/axlibc/include/pwd.h                          |   45 +
 ulib/axlibc/include/regex.h                        |    4 +
 ulib/axlibc/include/sched.h                        |   23 +
 ulib/axlibc/include/setjmp.h                       |   25 +
 ulib/axlibc/include/signal.h                       |  179 ++
 ulib/axlibc/include/stdarg.h                       |   11 +
 ulib/axlibc/include/stdbool.h                      |   11 +
 ulib/axlibc/include/stddef.h                       |   27 +
 ulib/axlibc/include/stdint.h                       |   66 +
 ulib/axlibc/include/stdio.h                        |  116 +
 ulib/axlibc/include/stdlib.h                       |   60 +
 ulib/axlibc/include/string.h                       |   50 +
 ulib/axlibc/include/strings.h                      |    4 +
 ulib/axlibc/include/sys/epoll.h                    |   60 +
 ulib/axlibc/include/sys/file.h                     |   23 +
 ulib/axlibc/include/sys/ioctl.h                    |   65 +
 ulib/axlibc/include/sys/mman.h                     |   52 +
 ulib/axlibc/include/sys/param.h                    |    6 +
 ulib/axlibc/include/sys/prctl.h                    |    4 +
 ulib/axlibc/include/sys/resource.h                 |   63 +
 ulib/axlibc/include/sys/select.h                   |   36 +
 ulib/axlibc/include/sys/socket.h                   |  311 +++
 ulib/axlibc/include/sys/stat.h                     |   78 +
 ulib/axlibc/include/sys/time.h                     |   41 +
 ulib/axlibc/include/sys/types.h                    |   20 +
 ulib/axlibc/include/sys/uio.h                      |   13 +
 ulib/axlibc/include/sys/un.h                       |   11 +
 ulib/axlibc/include/sys/utsname.h                  |   27 +
 ulib/axlibc/include/sys/wait.h                     |   12 +
 ulib/axlibc/include/syslog.h                       |   45 +
 ulib/axlibc/include/termios.h                      |    8 +
 ulib/axlibc/include/time.h                         |   43 +
 ulib/axlibc/include/unistd.h                       |  240 ++
 ulib/axlibc/src/errno.rs                           |   39 +
 ulib/axlibc/src/fd_ops.rs                          |   54 +
 ulib/axlibc/src/fs.rs                              |   67 +
 ulib/axlibc/src/io.rs                              |   32 +
 ulib/axlibc/src/io_mpx.rs                          |   54 +
 ulib/axlibc/src/lib.rs                             |  124 +
 ulib/axlibc/src/malloc.rs                          |   56 +
 ulib/axlibc/src/mktime.rs                          |   58 +
 ulib/axlibc/src/net.rs                             |  180 ++
 ulib/axlibc/src/pipe.rs                            |   14 +
 ulib/axlibc/src/pthread.rs                         |   59 +
 ulib/axlibc/src/rand.rs                            |   30 +
 ulib/axlibc/src/resource.rs                        |   17 +
 ulib/axlibc/src/setjmp.rs                          |  325 +++
 ulib/axlibc/src/strftime.rs                        |  252 ++
 ulib/axlibc/src/strtod.rs                          |  131 ++
 ulib/axlibc/src/sys.rs                             |   10 +
 ulib/axlibc/src/time.rs                            |   21 +
 ulib/axlibc/src/unistd.rs                          |   20 +
 ulib/axlibc/src/utils.rs                           |   10 +
 ulib/axstd/Cargo.toml                              |   89 +
 ulib/axstd/src/env.rs                              |   19 +
 ulib/axstd/src/fs/dir.rs                           |  154 ++
 ulib/axstd/src/fs/file.rs                          |  187 ++
 ulib/axstd/src/fs/mod.rs                           |   77 +
 ulib/axstd/src/io/mod.rs                           |   28 +
 ulib/axstd/src/io/stdio.rs                         |  172 ++
 ulib/axstd/src/lib.rs                              |   77 +
 ulib/axstd/src/macros.rs                           |   23 +
 ulib/axstd/src/net/mod.rs                          |   46 +
 ulib/axstd/src/net/socket_addr.rs                  |  190 ++
 ulib/axstd/src/net/tcp.rs                          |  105 +
 ulib/axstd/src/net/udp.rs                          |   96 +
 ulib/axstd/src/os.rs                               |    8 +
 ulib/axstd/src/process.rs                          |   10 +
 ulib/axstd/src/sync/mod.rs                         |   19 +
 ulib/axstd/src/sync/mutex.rs                       |   94 +
 ulib/axstd/src/thread/mod.rs                       |   51 +
 ulib/axstd/src/thread/multi.rs                     |  189 ++
 ulib/axstd/src/time.rs                             |   97 +
 396 files changed, 40751 insertions(+)

======================================
提交ID: ed49691
作者: dj <2253075320@qq.com>
时间: Wed Nov 12 19:52:07 2025 +0800
说明: update readme
======================================
 readme.md | 1 +
 1 file changed, 1 insertion(+)
