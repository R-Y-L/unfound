======================================
提交ID: b0a3968
作者: R-Y-L <1171577592@qq.com>
时间: Sun Jan 4 18:37:00 2026 +0800
说明: 上传文档
======================================
 doc/UnfoundOS.pdf | Bin 0 -> 846746 bytes
 1 file changed, 0 insertions(+), 0 deletions(-)

======================================
提交ID: a9e46aa
作者: R-Y-L <1171577592@qq.com>
时间: Sun Jan 4 17:13:37 2026 +0800
说明: 碎片测试修正
======================================
 88554ffaa6c6e4dca6d5227ea4297a16.png | Bin 0 -> 90117 bytes
 README.md                            |  16 +++-
 image.png                            | Bin 0 -> 67133 bytes
 modules/axalloc/src/tests/README.md  |   6 +-
 modules/axalloc/src/tests/config.rs  |  29 ++++--
 modules/axalloc/src/tests/metrics.rs |  35 +++++---
 modules/axalloc/src/tests/suites.rs  | 165 ++++++++++++++++++++++++++---------
 7 files changed, 188 insertions(+), 63 deletions(-)

======================================
提交ID: 70926c3
作者: R-Y-L <1171577592@qq.com>
时间: Sun Jan 4 14:38:55 2026 +0800
说明: readme更新
======================================
 6682d6b984b4a175591c9534935e62c7.png | Bin 0 -> 104629 bytes
 README.md                            |  69 ++++-----
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
 modules/axalloc/src/tests/README.md  | 255 --------------------------------
 readme.md                            |   1 -
 17 files changed, 36 insertions(+), 879 deletions(-)

======================================
提交ID: b030a79
作者: R-Y-L <1171577592@qq.com>
时间: Sun Jan 4 13:14:28 2026 +0800
说明: 内部测试修改
======================================
 README.md                                     | 284 ++++---------
 modules/axalloc/Cargo.toml                    |   5 +-
 modules/axalloc/src/allocators/bitmap.rs      |  14 +-
 modules/axalloc/src/allocators/buddy.rs       |  80 ++--
 modules/axalloc/src/allocators/hybrid.rs      |  42 +-
 modules/axalloc/src/bin/allocator_test.rs     | 478 ++++++++++++++-------
 modules/axalloc/src/lib.rs                    |   4 +
 modules/axalloc/src/tests/README.md           | 570 +++++++++++++++++++++++++
 modules/axalloc/src/tests/allocator_tester.rs |   7 +-
 modules/axalloc/src/tests/config.rs           | 282 +++++++++++++
 modules/axalloc/src/tests/kernel_tests.rs     | 470 +++++++++++++++++++++
 modules/axalloc/src/tests/metrics.rs          | 379 +++++++++++++++++
 modules/axalloc/src/tests/mod.rs              | 481 ++++++++++++++++++---
 modules/axalloc/src/tests/suites.rs           | 573 ++++++++++++++++++++++++++
 modules/axalloc/src/tests/time_dimension.rs   | 493 ++++++++++++++++++++++
 modules/axalloc/src/tests/workloads.rs        |   3 +
 modules/axruntime/src/lib.rs                  |  14 +-
 17 files changed, 3706 insertions(+), 473 deletions(-)

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
