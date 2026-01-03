
# Doors -- Entry to Safe and efficient tools ---- 进入安全高效工具的入口

all coming  .....  
ui in Flutter/Compose Multiplatform/Fyne/Egui ---- 界面使用Flutter/Compose Multiplatform/Fyne/Egui  
lib in Zig/Rust/Go ---- 动态库使用Zig/Rust/Go  
the frist version in Flutter&Rust ---- 第一版本使用Flutter&Rust  

## chat in local net(dont sent message to internet) ---- 局域网通信（不使用外网）

### build ---- 编译

lib in rust, app in flutter, for linux ---- 在linux上，动态库rust, app使用flutter  

```sh
make debug_linux_flutter_rust
# make release_linux_flutter_rust
```

### 实现概要

#### use flatbuffers, not http/s

#### udp,not tcp

#### server impl for go and rust

#### window linux mac android ios

#### security or crypto

#### message in memory

[see libsignal-protocol](https://github.com/signalapp/libsignal/blob/main/rust/protocol/Cargo.toml)

## tools ------

all coming  .....  
## ffi_rpc
通过ffi,flatbuffer来实现，zero-copy的跨语言调用  
  
ffi接口,  call(methodId uint64, inParameter flatbuffer) outParameter flatbuffer  
methodId: 高32位是class的整数标识，低32位是方法的整数标识，合起来唯一标识一个方法，

methodId由于代码生成工具（gen_ffi_rpc）自动生成的

gen_ffi_rpc使用golang语言及go template实现
  
通过一个ffi函数接口来调用其它的方法，相当于把rpc的网络换成ffi接口  

### zig生成与实现

### dart生成与实现