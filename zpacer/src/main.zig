const std = @import("std");
const c = @cImport({
    @cInclude("/usr/include/postgresql/libpq-fe.h");
});

pub fn main() !void {
    const conn_str = "host=localhost user=postgres password=postgres";

    const conn = c.PQconnectdb(conn_str).?;
    defer c.PQfinish(conn);

    if (c.PQstatus(conn) != c.CONNECTION_OK) {
        std.debug.print("Connection failed: {s}\n", .{c.PQerrorMessage(conn)});
        return error.ConnectionFailed;
    }
    std.debug.print("Connected to PostgreSQL!\n", .{});

    const create_query =
        \\ CREATE TABLE IF NOT EXISTS pacer (
        \\     id SERIAL PRIMARY KEY,
        \\     col1 TEXT, col2 TEXT, col3 TEXT, col4 TEXT, col5 TEXT,
        \\     col6 TEXT, col7 TEXT, col8 TEXT, col9 TEXT, col10 TEXT
        \\ );
    ;
    try run_query(conn, create_query);

    _ = c.PQexec(conn, "BEGIN");
    const lim = 1;
    for (0..lim) |i| {
        var query_buffer: [512]u8 = undefined;
        const query = try std.fmt.bufPrint(&query_buffer, "INSERT INTO pacer (col1, col2, col3, col4, col5, col6, col7, col8, col9, col10) VALUES ('val_{d}', 'val_{d}', 'val_{d}', 'val_{d}', 'val_{d}', 'val_{d}', 'val_{d}', 'val_{d}', 'val_{d}', 'val_{d}')", .{ i, i, i, i, i, i, i, i, i, i });

        try run_query(conn, query);
    }
    _ = c.PQexec(conn, "COMMIT");
    std.debug.print("Inserted {} rows.\n", .{lim});

    _ = c.PQexec(conn, "BEGIN");
    for (0..lim) |i| {
        var update_query_buffer: [512]u8 = undefined;
        const update_query = try std.fmt.bufPrint(&update_query_buffer, "UPDATE pacer SET col1 = 'upd_{d}' WHERE id = {d}", .{ i, i + 1 });
        std.debug.print("insert query is {s}\n", .{update_query});

        try run_query(conn, update_query);
    }
    _ = c.PQexec(conn, "COMMIT");
    std.debug.print("Updated {} rows.\n", .{lim});
}

const QueryError = error{SomeError};

fn run_query(conn: *c.PGconn, query: []const u8) anyerror!void {
    const res = c.PQexec(conn, query.ptr);
    defer c.PQclear(res);

    const msg = c.PQerrorMessage(conn);
    const rc = c.PQresultStatus(res);
    std.debug.print("query={s} msg={s} res={any} rc={any}\n", .{ query, msg, res, rc });
    if (rc != c.PGRES_COMMAND_OK) {
        return QueryError.SomeError;
    }
}
