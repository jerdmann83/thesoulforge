const std = @import("std");
const builtin = @import("builtin");
const net = std.net;

const http_server = @import("http_server.zig");
const http_config = @import("http_config.zig");
const http_buf = @import("http_buffer.zig");
const http_request = @import("http_request.zig");

const print = std.debug.print;

const Connection = std.net.Server.Connection;
const StaticStringMap = std.static_string_map.StaticStringMap;

test "config" {
    const s = http_config.Socket.init();
    print("{any}\n", .{s});
}

const ResponseMap = StaticStringMap([]const u8).initComptime(.{
    .{ "200", (
        "HTTP/1.1 200 OK\nContent-Length: 48"
        ++ "\nContent-Type: text/html\n"
        ++ "Connection: Closed\n\n<html><body>"
        ++ "<h1>Hello, World!</h1></body></html>"
    )},
    .{ "404", (
        "HTTP/1.1 404 Not Found\nContent-Length: 50"
        ++ "\nContent-Type: text/html\n"
        ++ "Connection: Closed\n\n<html><body>"
        ++ "<h1>File not found!</h1></body></html>"
    )},
});

pub fn get_response_message(code: []const u8) ?[]const u8 {
    return ResponseMap.get(code);
}

pub fn send_response(code: []const u8, conn: Connection) !void {
    const msg = get_response_message(code) orelse unreachable;
    _ = try conn.stream.write(msg);
}

pub const HttpServer = struct {
    // ridiculous:  server only stores port at present
    port: u16,

    pub fn init(port: u16) HttpServer {
        return HttpServer {
            .port = port,
        };
    }

    pub fn run(self: *HttpServer) !void {
        const socket = try http_config.Socket.init(self.port);
        var server = try socket.address.listen(.{ .reuse_address = true });
        var buf = http_buf.make_buffer(1024);
        while (true) {
            const conn = try server.accept();
            _ = try http_request.read_request(conn, &buf);
            const req = http_request.parse_request(&buf) catch |err| {
                print("error: caught {any}\n", .{ err });
                conn.stream.close();
                continue;
            };
            if (std.mem.eql(u8, req.uri, "/")) {
                try send_response("200", conn);
            } else {
                try send_response("404", conn);
            }
            conn.stream.close();
        }
    }
};
