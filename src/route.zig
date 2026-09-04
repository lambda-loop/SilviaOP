//

const std = @import("std");
const BitSet = std.bit_set.StaticBitSet;

const problem = @import("problem.zig");
const Least = problem.Least;

pub fn Route(num_points: usize) type {
    const T = Least(num_points);
    return struct {
        idx: T,
        buffer: [num_points]T,
    };
}

// constructive heuristic layout
pub fn cheuristic_layout(p: anytype, r: anytype, picker: anytype) void {
    std.debug.assert(@TypeOf(r.idx) == p.NUM_POINTS_T);
    var bitset = BitSet(p.num_points).initEmpty();
    var iter = bitset.iterator(.{ .Type = .unset });
    while (picker(p, &iter)) |picked| {}
}

// greedy
pub fn greedy(p: anytype, iter: anytype) ?p.NUM_POINTS_T {}

// constructive multiheuristic layout
// improvement heuristic layout
// improvement multiheuristic layuout
