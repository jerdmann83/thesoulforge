const std = @import("std");
const http_server = @import("http_server.zig");
const http_cfg = @import("http_config.zig");
const http_buf = @import("http_buffer.zig");
const http_request = @import("http_request.zig");
const Map = std.static_string_map.StaticStringMap;

const printf = std.debug.print;
const testing = std.testing;

const Method = enum {
    GET,
    pub fn init(str: []const u8) !Method {
        return MethodMap.get(str).?;
    }
    pub fn is_supported(str: []const u8) bool {
        const method = MethodMap.get(str);
        if (method) |_| {
            return true;
        }
        return false;
    }
};
const MethodMap = Map(Method).initComptime(.{
    .{ "GET", Method.GET },
});

test "method_map" {
    // get => ?V
    try testing.expectEqual(Method.GET, MethodMap.get("GET"));
    try testing.expectEqual(null, MethodMap.get("FOO"));
}

// terrible name for an allocator/hashmap-backed request
// this doesn't really feel like idiomatic zig...
//
// just trying to follow the Unmanaged naming convention which
// indicates a struct requires you to pass an allocator for every
// call that needs it, rather than storing one at init-time
const RequestManaged = struct {
    alloc: std.mem.Allocator,
    method: Method,
    uri: ?[]const u8,
    version: ?[]const u8,
    headers: std.StringHashMap([]const u8),
    body: ?[]const u8,

    pub fn init(alloc: std.mem.Allocator,
                method: Method,
                uri: []const u8,
                version: []const u8) Request {
        return RequestManaged {
            .alloc   = alloc,
            .method  = method,
            .uri     = uri,
            .version = version,
            .headers = std.StringHashMap([]const u8).init(alloc),
            .body = null,
        };
    }
};

const HeadersT = [24][2][]const u8;
const Request = struct {
    method: Method,
    uri: []const u8,
    version: []const u8,
    headers: HeadersT,
    body: ?[]const u8,

    pub fn init(method: Method,
                uri: []const u8,
                version: []const u8) Request {
        const out : Request = .{
            .method  = method,
            .uri     = uri,
            .version = version,
            .headers = undefined,
            .body = null,
        };
        return out;
    }
};

const RequestSection = enum {
    Request,
    Headers,
    Body
};

fn parse_request(text: []const u8) !Request {
    var lineIt = std.mem.splitSequence(u8, text, "\n");
    var lineno : u32 = 0;
    var out : Request = undefined;
    var sect = RequestSection.Request;
    var hdrno : u32 = 0;
    while (lineIt.next()) |line| {
        if (lineno == 0) {
            var tokIt = std.mem.tokenizeSequence(u8, line, " ");
            const method = try Method.init(tokIt.next().?);
            const uri = tokIt.next().?;
            const version = tokIt.next().?;
            out = Request.init(method, uri, version);

            lineno += 1;
            sect = RequestSection.Headers;
            continue;
        }
        if (std.mem.eql(u8, line, "")) {
            sect = RequestSection.Body;
            continue;
        }

        if (sect == RequestSection.Headers) {
            var tokIt = std.mem.tokenizeSequence(u8, line, ": ");
            const k = tokIt.next() orelse "";
            const v = tokIt.next() orelse "";
            out.headers[hdrno][0] = k;
            out.headers[hdrno][1] = v;
            hdrno += 1;
            continue;
        }
    }

    for (hdrno..out.headers.len) |i| {
        out.headers[i][0] = "";
        out.headers[i][1] = "";
    }

    return out;
}

test "parse" {
    const text = 
    \\GET / HTTP/1.1
    \\Host: localhost:3490
    \\User-Agent: curl/7.58.0
    \\Accept: */*
   ; 
    const r = try parse_request(text);
    const expect = testing.expectEqualStrings;
    try expect(r.headers[0][0], "Host");
    try expect(r.headers[0][1], "localhost:3490");
    try expect(r.headers[1][0], "User-Agent");
    try expect(r.headers[1][1], "curl/7.58.0");
    try expect(r.headers[2][0], "Accept");
    try expect(r.headers[2][1], "*/*");
}

pub fn main() !u8 {
    const socket = try http_cfg.Socket.init();
    printf("Socket {any}\n", .{ socket });
    var server = try socket.address.listen(.{ .reuse_address = true });
    var buf = http_buf.make_buffer(1024);
    while (true) {
        const conn = try server.accept();
        _ = try http_request.read_request(conn, &buf);
        printf("Conn {any} got request {s}\n", .{ conn, buf });
        conn.stream.close();
    }
    return 0;
}
