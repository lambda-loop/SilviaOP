// will provide all the matrixes infos in the data directory as table which will
// be then used by the project. use it from the root of the project

const std = @import("std");
const Io = std.Io;
const Dir = std.Io.Dir;
const File = std.Io.File;
const Allocator = std.mem.Allocator;

// const config = @import("../src/config.zig");
pub const Instance = struct {
    file_name: []const u8,
    total_sum: usize,
    max_score: usize,
    tmax: f32,
    N: usize,

    const Self = @This();
    fn init(file_name: []const u8, input: []const u8) Self {
        // const N = std.mem.countScalar(u8, input, '\n') - 1;
        var lines = std.mem.splitScalar(u8, input, '\n');
        const fst_line = lines.next() orelse unreachable;
        var fst_line_iter = std.mem.tokenizeAny(u8, fst_line, "\r\t ");
        const tmax_str = fst_line_iter.next() orelse unreachable;

        var total_sum: usize = 0;
        var line_count: usize = 0;
        var max_score: usize = 0;
        while (lines.next()) |line| : (line_count += 1) {
            if (line.len == 0) continue;
            var iter = std.mem.tokenizeAny(u8, line, "\t\r ");
            _ = iter.next();
            _ = iter.next();
            const s_str = iter.next() orelse unreachable;
            // print("\n{s}\n", .{s_str});
            // print("\n{s}\n", .{s_str});
            // print("\n{s}\n", .{s_str});
            const s = std.fmt.parseInt(usize, s_str, 10) catch unreachable;
            total_sum += s;
            if (s > max_score) max_score = s;
        }

        return .{
            .file_name = file_name,
            .total_sum = total_sum,
            .max_score = max_score,
            .N = line_count - 1,
            .tmax = std.fmt.parseFloat(f32, tmax_str) catch unreachable,
        };
    }

    fn deinit(self: Self, allocator: Allocator) void {
        allocator.free(self.file_name);
    }

    fn format(self: Self, allocator: Allocator) ![]u8 {
        return std.fmt.allocPrint(allocator, "{s},{d},{d},{d},{}", .{ self.file_name, self.N, self.total_sum, self.max_score, self.tmax });
    }
};

pub fn getAllInstancesFromDir(io: Io, dir: Dir, allocator: Allocator) ![]Instance {
    var is = std.ArrayList(Instance).empty;
    var dir_iter = dir.iterate();
    while (try dir_iter.next(io)) |entry| {
        if (entry.kind == .file) continue;
        const inner_dir = try dir.openDir(io, entry.name, .{ .iterate = true });
        const inner_is = try getAllInstancesFromSubDir(io, inner_dir, allocator);
        defer allocator.free(inner_is);
        try is.appendSlice(allocator, inner_is);

        // print("\n\n{s}", .{entry.name});
    }

    return is.toOwnedSlice(allocator);
}

pub fn getAllInstancesFromSubDir(io: Io, dir: Dir, allocator: Allocator) ![]Instance {
    var is = std.ArrayList(Instance).empty;

    var dir_iter = dir.iterate();
    while (try dir_iter.next(io)) |entry| {
        if (entry.kind != .file) continue;
        if (!std.mem.endsWith(u8, entry.name, ".txt")) continue;

        const file_name = std.fs.path.stem(entry.name);
        // defer allocator.free(file_name);
        const safe_name = try allocator.dupe(u8, file_name);

        const safe_name_with_ext = try std.mem.concat(
            allocator,
            u8,
            &[_][]const u8{ safe_name, ".txt" },
        );
        defer allocator.free(safe_name_with_ext);
        const input = try dir.readFileAlloc(io, safe_name_with_ext, allocator, .unlimited);
        defer allocator.free(input);

        const i = Instance.init(safe_name, input);
        try is.append(allocator, i);
    }

    return is.toOwnedSlice(allocator);
}

const print = std.debug.print;
pub fn main(init: std.process.Init) !void {
    const io = init.io;
    const gpa = init.gpa;

    const args = try init.minimal.args.toSlice(gpa);
    defer gpa.free(args);

    const cwd = try Io.Dir.cwd().openDir(io, ".", .{ .iterate = true });
    defer cwd.close(io);

    const input_path = args[1];
    const input_dir = try cwd.openDir(io, input_path, .{
        .iterate = true,
    });
    defer input_dir.close(io);

    const is = try getAllInstancesFromSubDir(io, input_dir, gpa);
    defer {
        for (is) |i| i.deinit(gpa);
        gpa.free(is);
    }

    const path_to_write = args[2];
    const target_name = args[3];

    const target_dir = try Io.Dir.openDir(cwd, io, path_to_write, .{});
    defer target_dir.close(io);
    const target_file = try target_dir.createFile(io, target_name, .{});
    defer target_file.close(io);

    const buffer = try gpa.alloc(u8, 1024);
    defer gpa.free(buffer);

    var w = target_file.writer(io, buffer);
    defer w.flush() catch {};

    var table = std.ArrayList(u8).empty;
    for (is) |i| {
        const ii = try i.format(gpa);
        defer gpa.free(ii);

        try table.appendSlice(gpa, ii);
        try table.append(gpa, '\n');

        // print("{s}\n", .{ii});
        // print("{any}\n", .{table});
    }

    const ftable = try table.toOwnedSlice(gpa);
    defer gpa.free(ftable);

    // print("{s}\n", .{ftable});
    try w.interface.print("{s}", .{ftable});
}
