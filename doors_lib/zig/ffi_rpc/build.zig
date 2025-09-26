const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    const lib = b.addLibrary(.{
        .name = "ffi_rpc",
        .linkage = .dynamic,
        .version = .{ .major = 0, .minor = 0, .patch = 1 },
        .root_module = b.createModule(.{
            .root_source_file = b.path("src/ffi.zig"),
            .target = target,
            .optimize = optimize,
            .link_libc = true,
        }),
    });
    {
        lib.linkLibC();
        b.installArtifact(lib);
        // const install_lib = b.addInstallFile(lib.getEmittedH(), "");
        // b.getInstallStep().dependOn(&install_lib.step);
    }
    const mod = b.addModule("ffi_rpc", .{
        .root_source_file = b.path("src/root.zig"),
        .target = target,
    });
    const mod_tests = b.addTest(.{
        .root_module = mod,
    });

    // A run step that will run the test executable.
    const run_mod_tests = b.addRunArtifact(mod_tests);

    const test_step = b.step("test", "Run tests");
    test_step.dependOn(&run_mod_tests.step);
}
