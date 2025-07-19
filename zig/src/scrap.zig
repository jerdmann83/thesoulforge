const std = @import("std");
const printf = std.debug.print;

const Data = struct {
    f: i32,

    pub fn init() Data {
        return Data { .f = 0 };
    }
};

fn print_data(d: ?Data) void {
    if (d) |data| {
        printf("{any}\n", .{ data });
    } else {
        printf("{any}\n", .{ d });
    }
}

fn tryme() void {
    var d : ?Data = null;
    print_data(d);

    d = Data.init();
    print_data(d);

    if (d) |*data| {
        data.f = 99;
    }
    print_data(d);

}

test "tryme" {
    tryme();
}
