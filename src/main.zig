//

const std = @import("std");
const Io = std.Io;
const File = Io.File;
const print = std.debug.print;

const problem = @import("problem.zig");

pub fn main(init: std.process.Init) !void {
    _ = init;
    // print("explica", .{});

    const p = problem.Problem(42);
    print("{any} {any}", .{ p.N, p.M });

    // const io = init.io;
    // const gpa = init.gpa;

    // const cwd = Io.Dir.cwd();
    // const dir = try cwd.openDir(io, "set_64_1/", .{});
    // const file_content = try dir.readFileAlloc(io, "set_64_1_15", gpa, .unlimited);
    // const N = getN(file_content);
    // print("{}", .{N});
}

const pow = std.math.pow;
pub fn Least(comptime N: usize) type {
    if (N < pow(usize, 2, 8))
        return u8
    else if (N < pow(usize, 2, 16))
        return u16
    else if (N < pow(usize, 2, 32))
        return u32
    else
        return u64;
}

pub fn getN(raw_input: []u8) usize {
    std.mem.countScalar(u8, raw_input, '\n');
    var lines = std.mem.splitScalar(u8, raw_input, '\n');
    var N: usize = 0;
    while (lines.next()) |_| : (N += 1) {}
}
