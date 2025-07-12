const std = @import("std");
const http_server = @import("http_server.zig");
const http_cfg = @import("http_config.zig");
const http_buf = @import("http_buffer.zig");
const http_request = @import("http_request.zig");
const printf = std.debug.print;

pub fn main() !u8 {
    const socket = try http_cfg.Socket.init();
    printf("Socket {any}\n", .{ socket });
    var server = try socket.address.listen(.{ .reuse_address = true });
    var buf = http_buf.make_buffer(1024);
    while (true) {
        const conn = try server.accept();
        printf("Conn {any} wait request\n", .{ conn });
        _ = try http_request.read_request(conn, &buf);
        printf("Conn {any} got request {s}\n", .{ conn, buf });
        conn.stream.close();
    }
    return 0;
}
