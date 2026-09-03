//

const std = @import("std");

const Io = std.Io;
const Dir = std.Io.Dir;
const Allocator = std.mem.Allocator;

const ptable = @embedFile("table");

pub const num_problems: usize = blk: {
    @setEvalBranchQuota(1000 * 1000 * 1000);
    break :blk std.mem.countScalar(u8, ptable, '\n');
};

const Content = struct {
    sum_scores: usize,
    max_score: usize,
    num_points: usize,

    fn new(problem_number: usize) !?Content {
        @setEvalBranchQuota(1000 * 1000 * 1000);
        var lines = std.mem.tokenizeScalar(u8, ptable, '\n');

        for (0..problem_number) |_| {
            _ = lines.next();
        }

        const line = lines.next().?;
        var iter = std.mem.tokenizeScalar(u8, line, ',');

        _ = iter.next();
        const num_points_str = iter.next().?;
        const sum_scores_str = iter.next().?;
        const max_score_str = iter.next().?;

        return .{
            .num_points = try std.fmt.parseInt(usize, num_points_str, 10),
            .sum_scores = try std.fmt.parseInt(usize, sum_scores_str, 10),
            .max_score = try std.fmt.parseInt(usize, max_score_str, 10),
        };
    }
};

pub fn Problem(problem_number: usize) type {
    @setEvalBranchQuota(1000 * 1000 * 1000);
    // const max_score = undefined
    const content: Content = comptime Content.new(problem_number) catch {} orelse unreachable;
    const MAX_SCORE_T = Least(content.max_score);
    const NUM_POINTS_T = Least(content.num_points);
    return struct {
        pub const N: NUM_POINTS_T = @intCast(content.num_points);
        pub const M: MAX_SCORE_T = @intCast(content.max_score);

        tmax: f32,
        // TODO: should it care the least type for the MAX score acctually?
        scores: [N]MAX_SCORE_T,
        costs: [N][N]f32,

        const Self = @This();
        // fn new(io: Io, dir: Dir, allocator: Allocator) !Self {
        //     const line = getInstance(problem_number);
        //     var iter = std.mem.tokenizeScalar(u8, line, ',');
        //     const problem_name = iter.next() orelse unreachable;

        //     const file_name = try std.mem.concat(
        //         allocator,
        //         u8,
        //         &[_][]const u8{ problem_name, ".txt" },
        //     );

        //     const input = try dir.readFileAlloc(io, file_name, allocator, .unlimited);

        // const file = try dir.openFile(io, file_name, .{});
        // defer file.close();

        // // const buffer = try allocator.alloc(u8, 1024);
        // // allocator.free(buffer);

        // defer writer.flush catch {};
        // }

        pub fn getInstance(idx: usize) []const u8 {
            var lines = std.mem.tokenizeScalar(u8, ptable, '\n');

            for (0..idx) |_| {
                _ = lines.next();
            }

            const line = lines.next().?;
            return line;
        }
    };
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

const Point2D = struct {
    x: f32,
    y: f32,
};
