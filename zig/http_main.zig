const std = @import("std");

const Connection = std.net.Server.Connection;
const StaticStringMap = std.static_string_map.StaticStringMap;

const http_server = @import("http_server.zig");
const http_cfg = @import("http_config.zig");
const http_buf = @import("http_buffer.zig");
const http_request = @import("http_request.zig");

const print = std.debug.print;
const bufPrint = std.fmt.bufPrint;

const Method = enum {
    GET,
    POST,
    pub fn init(str: []const u8) ?Method {
        return MethodMap.get(str);
    }
    pub fn is_supported(str: []const u8) bool {
        const method = MethodMap.get(str);
        if (method) |_| {
            return true;
        }
        return false;
    }
};
const MethodMap = StaticStringMap(Method).initComptime(.{
    .{ "GET",  Method.GET },
    .{ "POST", Method.POST },
});

test "method_map" {
    // get => ?V
    try std.testing.expectEqual(Method.GET,  MethodMap.get("GET"));
    try std.testing.expectEqual(Method.POST, MethodMap.get("POST"));
    try std.testing.expectEqual(null,        MethodMap.get("FOO"));
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

const ParseError = error {
    NoMethod,
};

fn parse_request(text: []const u8) !Request {
    var lineIt = std.mem.splitSequence(u8, text, "\n");
    var lineno : u32 = 0;
    var out : Request = undefined;
    var sect = RequestSection.Request;
    var hdrno : u32 = 0;
    while (lineIt.next()) |line| {
        if (lineno == 0) {
            print("{s}\n", .{line});
            var tokIt = std.mem.tokenizeSequence(u8, line, " ");
            const smethod = tokIt.next() orelse "";
            const method = Method.init(smethod);
            if (method == null) return ParseError.NoMethod;

            const uri = tokIt.next() orelse "";
            const version = tokIt.next() orelse "";
            out = Request.init(method.?, uri, version);

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
    const expect = std.testing.expectEqualStrings;
    try expect(r.headers[0][0], "Host");
    try expect(r.headers[0][1], "localhost:3490");
    try expect(r.headers[1][0], "User-Agent");
    try expect(r.headers[1][1], "curl/7.58.0");
    try expect(r.headers[2][0], "Accept");
    try expect(r.headers[2][1], "*/*");
}

// var ResponseCodeMap = 

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

pub fn main() !u8 {
    const socket = try http_cfg.Socket.init();
    print("Socket {any}\n", .{ socket });
    var server = try socket.address.listen(.{ .reuse_address = true });
    var buf = http_buf.make_buffer(1024);
    while (true) {
        const conn = try server.accept();
        _ = try http_request.read_request(conn, &buf);
        const req = parse_request(&buf) catch |err| {
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
    return 0;
}
