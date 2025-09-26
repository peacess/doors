const std = @import("std");

pub const Bytes = extern struct {
    bytes: [*c]u8,
    len: usize,
};
//, _inParameter: Bytes
pub export fn call(_methodId: u64) void {
    if (_methodId < 10) {
        std.debug.print("method id {}", .{_methodId});
    }
    // return inParameter;
}

pub export fn bytes_free(bytes: Bytes) void {
    std.c.free(bytes.bytes);
    return;
}
