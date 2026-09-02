const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // ------------------------------------------------------------
    // Generate src/table
    // ------------------------------------------------------------

    const manager_tool = b.addExecutable(.{
        .name = "manager",
        .root_module = b.createModule(.{
            .root_source_file = b.path("data/manager.zig"),
            .target = b.graph.host,
            .optimize = .ReleaseSmall,
        }),
    });

    const run_manager = b.addRunArtifact(manager_tool);

    run_manager.addArg("data");
    run_manager.addArg("src");
    run_manager.addArg("table");
    // ------------------------------------------------------------
    // Library
    // ------------------------------------------------------------

    const mod = b.addModule("OrienteeringProblemSILVIA", .{
        .root_source_file = b.path("src/root.zig"),
        .target = target,
    });

    // ------------------------------------------------------------
    // Executable
    // ------------------------------------------------------------

    const exe = b.addExecutable(.{
        .name = "OrienteeringProblemSILVIA",
        .root_module = b.createModule(.{
            .root_source_file = b.path("src/main.zig"),
            .target = target,
            .optimize = optimize,
            .imports = &.{
                .{
                    .name = "OrienteeringProblemSILVIA",
                    .module = mod,
                },
            },
        }),
    });

    // src/table must exist before building the executable.
    exe.step.dependOn(&run_manager.step);

    b.installArtifact(exe);

    // ------------------------------------------------------------
    // Run
    // ------------------------------------------------------------

    const run_step = b.step("run", "Run the app");

    const run_cmd = b.addRunArtifact(exe);
    run_step.dependOn(&run_cmd.step);

    run_cmd.step.dependOn(b.getInstallStep());

    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    // ------------------------------------------------------------
    // Tests
    // ------------------------------------------------------------

    const mod_tests = b.addTest(.{
        .root_module = mod,
    });

    const run_mod_tests = b.addRunArtifact(mod_tests);

    const exe_tests = b.addTest(.{
        .root_module = exe.root_module,
    });

    const run_exe_tests = b.addRunArtifact(exe_tests);

    const test_step = b.step("test", "Run tests");
    test_step.dependOn(&run_mod_tests.step);
    test_step.dependOn(&run_exe_tests.step);
}
