//

const std = @import("std");
const print = std.debug.print;

pub fn main() void {
    const t1 = Least(5);
    const t2 = Least(50_000);
    const t3 = Least(500_000);

    print("t1: {any}\n", .{t1});
    print("t2: {any}\n", .{t2});
    print("t3: {any}\n", .{t3});
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
