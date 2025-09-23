
all coming  .....  
# lifedoor -- safe life tools
## chat
### 实现概要
#### use flatbuffers, not http/s
#### udp,not tcp
#### server impl for go and rust
#### window linux mac...
#### add io-uring in second version
#### security or crypto
[libsignal-protocol](https://github.com/signalapp/libsignal/blob/main/rust/protocol/Cargo.toml)

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