const std = @import("std");
const net = std.net;

pub const Socket = struct {
    address: std.net.Address,
    stream: std.net.Stream,

    pub fn init() !Socket {
        const host = [4]u8 { 127, 0, 0, 1 };
        const port = 3490;
        const addr = net.Address.initIp4(host, port);
        const socket = try std.posix.socket(
            addr.any.family,
            std.posix.SOCK.STREAM,
            std.posix.IPPROTO.TCP,
        );

        const stream = net.Stream{ .handle = socket };
        return Socket{ .address = addr, .stream = stream };

    }
};

