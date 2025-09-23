const std = @import("std");

pub const ByteSlice = extern struct {
    bytes: [*c]u8,
    len: usize,
};

export fn call(methodId: u64, inParameter: ByteSlice) ByteSlice {}

export fn bytes_free(bytes: ByteSlice) void {
    std.c.free(bytes.bytes);
    return;
}
