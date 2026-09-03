//

const std = @import("std");
const Io = std.Io;
const File = Io.File;
const print = std.debug.print;

const problem = @import("problem.zig");

pub fn main(init: std.process.Init) !void {
    const io = init.io;
    const gpa = init.gpa;
    // print("explica", .{});

    const cwd = std.Io.Dir.cwd();
    const dir = try cwd.openDir(io, "data", .{});

    const n = problem.num_problems;
    print("num_problems: {any}\n", .{n});

    inline for (0..problem.num_problems) |p_idx| {
        const p = problem.Problem(p_idx);
        try p.new(io, dir, gpa);
        print("{any} {any} \n", .{ @TypeOf(p.N), @TypeOf(p.M) });
        print("{any} {any} \n", .{ p.N, p.M });
        print("instance: {s}\n\n", .{p.getInstance(p_idx)});
    }

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
