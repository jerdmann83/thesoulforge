const std = @import("std");

pub fn init_buffer(buf: []u8) void {
    for (0..buf.len) |i| {
        buf[i] = 0;
    }
}

pub fn make_buffer(comptime len: usize) [len]u8 {
    var buf : [len]u8 = undefined;
    init_buffer(&buf);
    // TODO: this is probably a copy?
    // anything more idiomatic?  take an allocator?
    return buf;
}
