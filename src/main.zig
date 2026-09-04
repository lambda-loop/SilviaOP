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
        const pn = try p.new(io, dir, gpa);
        print("{any} {any} \n", .{ @TypeOf(p.num_points), @TypeOf(p.max_score) });
        print("instance: {s}\n\n", .{p.getInstance(p_idx)});
        print("instance: {any}\n\n", .{pn});
    }

    // const io = init.io;
    // const gpa = init.gpa;

    // const cwd = Io.Dir.cwd();
    // const dir = try cwd.openDir(io, "set_64_1/", .{});
    // const file_content = try dir.readFileAlloc(io, "set_64_1_15", gpa, .unlimited);
    // const N = getN(file_content);
    // print("{}", .{N});
}
