const std = @import("std");
const net = std.net;
const builtin = @import("builtin");
const cfg = @import("http_config.zig");
const print = std.debug.print;

const http_server = @import("http_server.zig");
const http_cfg = @import("http_config.zig");
const http_buf = @import("http_buffer.zig");
const http_request = @import("http_request.zig");

const Connection = std.net.Server.Connection;
const StaticStringMap = std.static_string_map.StaticStringMap;

test "config" {
    const s = cfg.Socket.init();
    print("{any}\n", .{s});
}

