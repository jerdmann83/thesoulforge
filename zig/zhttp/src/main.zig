const std = @import("std");

const Connection = std.net.Server.Connection;

const http_server = @import("http_server.zig");
const http_cfg = @import("http_config.zig");
const http_buf = @import("http_buffer.zig");
const http_request = @import("http_request.zig");

pub fn main() !u8 {
    var hs = http_server.HttpServer.init(3490);
    try hs.run();
    return 0;
}
