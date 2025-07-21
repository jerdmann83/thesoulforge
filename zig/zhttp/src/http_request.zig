const std = @import("std");
const Connection = std.net.Server.Connection;
const StaticStringMap = std.static_string_map.StaticStringMap;

const print = std.debug.print;

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

pub fn read_request(conn: Connection,
    buffer: []u8) !void {
    const reader = conn.stream.reader();
    _ = try reader.read(buffer);
}

pub fn parse_request(text: []const u8) !Request {
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
