//

const std = @import("std");

const problem = @import("problem.zig");
const Least = problem.Least;

pub fn Route(num_points: usize) type {
    const T = Least(num_points);
    return struct {
        idx: T,
        buffer: [num_points]T,
    };
}
