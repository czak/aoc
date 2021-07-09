const std = @import("std");

const input = "R4, R1, L2, R1, L1, L1, R1, L5, R1, R5, L2, R3, L3, L4, R4, R4, R3, L5, L1, R5, R3, L4, R1, R5, L1, R3, L2, R3, R1, L4, L1, R1, L1, L5, R1, L2, R2, L3, L5, R1, R5, L1, R188, L3, R2, R52, R5, L3, R79, L1, R5, R186, R2, R1, L3, L5, L2, R2, R4, R5, R5, L5, L4, R5, R3, L4, R4, L4, L4, R5, L4, L3, L1, L4, R1, R2, L5, R3, L4, R3, L3, L5, R1, R1, L3, R2, R1, R2, R2, L4, R5, R1, R3, R2, L2, L2, L1, R2, L1, L3, R5, R1, R4, R5, R2, R2, R4, R4, R1, L3, R4, L2, R2, R1, R3, L5, R5, R2, R5, L1, R2, R4, L1, R5, L3, L3, R1, L4, R2, L2, R1, L1, R4, R3, L2, L3, R3, L2, R1, L4, R5, L1, R5, L2, L1, L5, L2, L5, L2, L4, L2, R3";

pub fn main() void {
    std.debug.print("Part 1: {}\n", .{distance(input)});
    std.debug.print("Part 2: {}\n", .{traverse(input)});
}

const Direction = enum(u2) {
    north,
    east,
    south,
    west,

    fn right(self: Direction) Direction {
        return @intToEnum(Direction, @enumToInt(self) +% 1);
    }

    fn left(self: Direction) Direction {
        return @intToEnum(Direction, @enumToInt(self) -% 1);
    }
};

fn distance(steps: []const u8) i32 {
    var x: i32 = 0;
    var y: i32 = 0;
    var dir: Direction = .north;

    var it = std.mem.split(steps, ", ");
    while (it.next()) |step| {
        dir = if (step[0] == 'R') dir.right() else dir.left();
        const dist = std.fmt.parseInt(i32, step[1..], 10) catch unreachable;

        switch (dir) {
            .north => y += dist,
            .east => x += dist,
            .south => y -= dist,
            .west => x -= dist,
        }
    }

    const abs = std.math.absInt;
    return (abs(x) catch unreachable) + (abs(y) catch unreachable);
}

const Point = struct {
    x: i32,
    y: i32,

    fn distance(self: Point) i32 {
        const abs = std.math.absInt;
        return (abs(self.x) catch unreachable) + (abs(self.y) catch unreachable);
    }
};

fn traverse(steps: []const u8) i32 {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();

    var pos = Point{ .x = 0, .y = 0 };
    var dir: Direction = .north;
    var visited = std.AutoHashMap(Point, void).init(&arena.allocator);

    var it = std.mem.split(steps, ", ");
    while (it.next()) |step| {
        dir = if (step[0] == 'R') dir.right() else dir.left();
        var dist = std.fmt.parseInt(i32, step[1..], 10) catch unreachable;

        while (dist > 0) {
            switch (dir) {
                .north => pos.y += 1,
                .east => pos.x += 1,
                .south => pos.y -= 1,
                .west => pos.x -= 1,
            }

            if (visited.contains(pos)) {
                return pos.distance();
            }

            visited.put(pos, {}) catch unreachable;

            dist -= 1;
        }
    }

    unreachable;
}

test "part 1, example 1" {
    try std.testing.expect(distance("R2, L3") == 5);
}

test "part 1, example 2" {
    try std.testing.expect(distance("R2, R2, R2") == 2);
}

test "part 1, example 3" {
    try std.testing.expect(distance("R5, L5, R5, R3") == 12);
}

test "part 2, example 1" {
    try std.testing.expect(traverse("R8, R4, R4, R8") == 4);
}
