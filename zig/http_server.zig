const std = @import("std");
const net = std.net;
const builtin = @import("builtin");
const cfg = @import("http_config.zig");
const printf = std.debug.print;

test "config" {
    const s = cfg.Socket.init();
    printf("{any}\n", .{s});
}
