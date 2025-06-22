const std = @import("std");
const printf = std.debug.print;

const Base64 = struct {
    table: *const [64]u8,

    pub fn init() Base64 {
        const upper = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        const lower = "abcdefghijklmnopqrstuvwxyz";
        const rest = "0123456789+/";
        return Base64{
            .table = upper ++ lower ++ rest,
        };
    }

    pub fn char_at(self: Base64, index: usize) u8 {
        return self.table[index];
    }

    pub fn char_index(self: Base64, c: u8) u8 {
        const NO_INDEX : u8 = 64;
        var idx : u8 = 0;
        while (idx < self.table.len) {
            if (self.table[idx] == c) return idx;
            idx += 1;
        }
        return NO_INDEX;
    }

    pub fn encode(self: Base64, alloc: std.mem.Allocator, input: []const u8) ![]u8 {
        if (input.len == 0) return "";

        const n_out = try calc_encode_len(input);
        var out = try alloc.alloc(u8, n_out);
        var iout: u64 = 0;
        // encode requires a 3-byte input window
        // the window size as we iterate over the input is 3 bytes,
        // where a full 3-byte input produces 4 bytes of base64 encoded output
        var buf = [3]u8{ 0, 0, 0 };
        var count: u64 = 0;
        // consume as many 3-byte input slices as we can
        for (input, 0..) |_, i| {
            buf[count] = input[i];
            count += 1;
            if (count == 3) {
                const b1 = buf[0] >> 2;
                // slice up input bytes into 6-bit tokens
                // using binary literals to make it more visually apparent
                //
                // grouping matters!  we need the (byte & 0b...) mask pattern, 
                // then the bit shift applied to that result
                const b2 = ((buf[0] & 0b00000011) << 4) + (buf[1] >> 4);
                const b3 = ((buf[1] & 0b00001111) << 2) + (buf[2] >> 6);
                const b4 = buf[2] & 0b00111111;
                out[iout] = self.char_at(b1);
                iout += 1;
                out[iout] = self.char_at(b2);
                iout += 1;
                out[iout] = self.char_at(b3);
                iout += 1;
                out[iout] = self.char_at(b4);
                iout += 1;
                // window complete, reset input buffer
                count = 0;
            }
        }

        // handle 1 or 2 bytes leftover (if present)
        if (count == 1) {
            // one input byte means we just encode two 6-bit tokens...
            const b1 = buf[0] >> 2;
            // ...padding the second token with zeroes
            const b2 = (buf[0] & 0b00000011) << 4;
            out[iout] = self.char_at(b1);
            iout += 1;
            out[iout] = self.char_at(b2);
            iout += 1;
            // ...then padding the last two output bytes
            // with the null b64 character =
            out[iout] = '=';
            iout += 1;
            out[iout] = '=';
            iout += 1;
        }
        if (count == 2) {
            // two input bytes means we get 3 output tokens...
            const b1 = buf[0] >> 2;
            const b2 = ((buf[0] & 0b00000011) << 4) + (buf[1] >> 4);
            const b3 = (buf[1] & 0b00001111) << 2;
            out[iout] = self.char_at(b1);
            iout += 1;
            out[iout] = self.char_at(b2);
            iout += 1;
            out[iout] = self.char_at(b3);
            iout += 1;
            // ...and a final null
            out[iout] = '=';
            iout += 1;
        }
        return out;
    }

    fn decode(self: Base64, alloc: std.mem.Allocator, input: []const u8) ![]u8 {
        if (input.len == 0) {
            return "";
        }
        const n_out = try calc_decode_len(input);
        var out = try alloc.alloc(u8, n_out);
        var iout: u64 = 0;
        var count: u8 = 0;
        // decode requires a 4-byte window buffer, the reverse of encode
        // 4 6-bit slices produce 3 bytes of original binary data
        var buf = [4]u8{ 0, 0, 0, 0 };

        for (0..input.len) |i| {
            buf[count] = self.char_index(input[i]);
            count += 1;
            // once we have buffered up 4 bytes, we can decode
            if (count == 4) {
                // we must have at least two input slices
                // a single-byte input produces two encoded slices
                out[iout] = (buf[0] << 2) + (buf[1] >> 4);
                iout += 1;
                // we have to check these latter two for null/=
                if (buf[2] != 64) {
                    out[iout] = (buf[1] << 4) + (buf[2] >> 2);
                    iout += 1;
                }
                if (buf[3] != 64) {
                    out[iout] = (buf[2] << 6) + buf[3];
                    iout += 1;
                }
                count = 0;
            }
        }

        return out;
    }
};

fn calc_encode_len(input: []const u8) !usize {
    if (input.len < 3) return 4;

    const ngroups = try std.math.divCeil(usize, input.len, 3);
    return ngroups * 4;
}

fn calc_decode_len(input: []const u8) !usize {
    if (input.len < 4) return 3;

    const ngroups: usize = try std.math.divFloor(usize, input.len, 4);
    var out = ngroups * 3;
    var idx = input.len - 1;
    // ignore null characters at the end of input
    while (input[idx] == '=') {
        out -= 1;
        idx -= 1;
    }
    return out;
}

test "base64" {
    const b = Base64.init();
    try std.testing.expect(b.char_at(0) == 'A');
    try std.testing.expect(b.char_at(1) == 'B');

    const alloc = std.heap.page_allocator;
    const input = "zig is neat";
    // output from some web b64 encoder
    const expect = "emlnIGlzIG5lYXQ=";
    const actual_encode = try b.encode(alloc, input);
    // correct encodes...
    try std.testing.expect(std.mem.eql(u8, expect, actual_encode));

    // ...and decode takes us back to the original input
    const actual_decode = try b.decode(alloc, actual_encode);
    try std.testing.expect(std.mem.eql(u8, actual_decode, input));
}
