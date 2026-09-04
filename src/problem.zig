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
    const t = u0;
    sum_scores: usize,
    max_score: usize,
    num_points: usize,
    tmax: f16,

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
        const tmax_str = iter.next().?;

        return .{
            .num_points = try std.fmt.parseInt(usize, num_points_str, 10),
            .sum_scores = try std.fmt.parseInt(usize, sum_scores_str, 10),
            .max_score = try std.fmt.parseInt(usize, max_score_str, 10),
            .tmax = try std.fmt.parseFloat(f16, tmax_str),
        };
    }
};

pub fn Problem(problem_number: usize) type {
    @setEvalBranchQuota(1000 * 1000 * 1000);
    // const max_score = undefined
    const content: Content = comptime Content.new(problem_number) catch {} orelse unreachable;
    return struct {
        pub const MAX_SCORE_T = Least(content.max_score);
        pub const SUM_SCORES_T = Least(content.sum_scores);
        pub const NUM_POINTS_T = Least(content.num_points);

        pub const num_points: NUM_POINTS_T = @intCast(content.num_points);
        pub const max_score: MAX_SCORE_T = @intCast(content.max_score);

        // TODO: tmax should also be know at compile time..
        pub const tmax = content.tmax;
        scores: [num_points]MAX_SCORE_T,
        costs: [num_points][num_points]f16,

        const Self = @This();
        pub fn new(io: Io, dir: Dir, allocator: Allocator) !Self {
            const pline = getInstance(problem_number);
            var piter = std.mem.tokenizeScalar(u8, pline, ',');
            const problem_name = piter.next() orelse unreachable;

            const file_name = try std.mem.concat(
                allocator,
                u8,
                &[_][]const u8{ problem_name, ".txt" },
            );
            defer allocator.free(file_name);

            const input = try dir.readFileAlloc(io, file_name, allocator, .unlimited);
            defer allocator.free(input);

            var scores = std.ArrayList(MAX_SCORE_T).empty;
            defer scores.deinit(allocator);

            var points = std.ArrayList(Point2D).empty;
            defer points.deinit(allocator);

            var lines = std.mem.tokenizeScalar(u8, input, '\n');
            _ = lines.next();
            while (lines.next()) |line| {
                if (line.len == 0) continue;
                var iter = std.mem.tokenizeAny(u8, line, "\r\t ");
                const x_str = iter.next().?;
                const y_str = iter.next() orelse {
                    std.debug.print("problem y line: {s}", .{line});
                    unreachable;
                };
                const score_str = iter.next().?;

                const score = try std.fmt.parseInt(MAX_SCORE_T, score_str, 10);
                const point = Point2D{
                    .x = try std.fmt.parseFloat(f16, x_str),
                    .y = try std.fmt.parseFloat(f16, y_str),
                };

                try scores.append(allocator, score);
                try points.append(allocator, point);
            }

            var scores_f: [num_points]MAX_SCORE_T = undefined;
            var costs_f: [num_points][num_points]f16 = undefined;

            for (0..num_points) |i| {
                scores_f[i] = scores.items[i];
            }

            for (0..num_points) |i| {
                for (0..num_points) |j| {
                    costs_f[i][j] = points.items[i].distance_to(points.items[j]);
                }
            }

            return .{
                .scores = scores_f,
                .costs = costs_f,
            };
        }

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
    x: f16,
    y: f16,

    pub fn distance_to(self: Point2D, other: Point2D) f16 {
        const dx = other.x - self.x;
        const dy = other.y - self.y;

        return std.math.hypot(dx, dy);
    }
};

// TODO: write tests such that SOME of the costs are tested!
