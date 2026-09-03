//

const std = @import("std");
const ptable = @embedFile("table");

pub const num_problems: usize = blk: {
    @setEvalBranchQuota(1000 * 1000 * 1000);
    break :blk std.mem.countScalar(u8, ptable, '\n');
};

const Content = struct {
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
        const max_score_str = iter.next().?;

        return .{
            .max_score = try std.fmt.parseInt(usize, max_score_str, 10),
            .num_points = try std.fmt.parseInt(usize, num_points_str, 10),
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
