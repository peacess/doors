# build

## build android

cargo apk build --bin dchat_android --target aarch64-linux-android
cargo apk build --bin dchat_android --target armv7-linux-androideabi
cargo apk build --bin dchat_android --target x86_64-linux-android
cargo apk build --bin dchat_android --target i686-linux-android

## build pc

cargo build --bin dchat_pc --target x86_64-pc-windows-gnu/x86_64-unknown-linux-gnu

## build web

cargo build --bin dchat_web --target wasm32-unknown-unknown

# flatbuffers

1. install the flatbuffers，find flatc.exe in path
2. cd src/idl
3. flatc --rust base.fbs

## install in windows

1. install vcpkg
2. vcpkg install flatbuffers:x64-windows
3. set path for "vcpkg/packages/flatbuffers_x64-windows/tools/flatbuffers"

## install in ubuntu

1. sudo apt install snapd
2. sudo snap install flatbuffers

# rocksdb

## windows

1. install llvm,
2. 设置path, ..llvm/bin
3. 设置环境变量， LIBCLANG_PATH=..llvm/bin

## ubuntu

1. sudo apt install clang llvm

# net

## see

1. [kbio base io uring & tokio](https://github.com/KuiBaDB/kbio)
2. [runtime base on io uring](https://github.com/bytedance/monoio)

# More detail

## ctrl-c in main

连续两次按下Ctrl-c的时间小于10秒，就退出程序

## exit the loop in start of ChatServer

在每次循环开始时检查AtomicBool变量，是否需要退循环，如果是就退出

## 多线程数据安全：原子操作、volatile、执行顺序（cpu/编译）

### 原子操作很好理解，是不可分的

### volatile是可见性，线程有自己的缓存，首先会出缓存中存取变量的值。这样在不同线程中同看到的同一变量的值就可能不同，这就是变量的可见性。它与原子操作没有关系。

### 执行顺序。这里有三个相关的

1. 代码顺序，源代码中的先后顺序
2. 编译顺序，经过编译器编译之后的顺序，由于编译器会优化，可能编译顺序与代码顺序是不一样的
3. 执行顺序，cpu在运行时，由于多核或优化，执行顺序可能与编译后的顺序不一样，这也叫乱序执行/错序执行/out-of-order execution

### 使用说明

1. volatile使用在可见性要求的地方，比如“线程是否要退出”的bool变量，这时多线程访问时只要加上volatile就可以了。像rust没有volatile，那么需要使用“store(true, Ordering::Relaxed)
   ”写入，“load(Ordering::Relaxed)”来读取。这里没有“顺序”的要求所以只要 Relaxed就好了
2. 

