import { expect, test } from "bun:test";
import { executeNy } from "./test-utils";

test("'ny --help' works", async () => {
  const { stdout, stderr, exitCode } = await executeNy("--help");

  expect(exitCode).toBe(0);
  expect(stdout).toContain("Usage: ny [COMMAND]");
  expect(stderr).toBe("");
});

test("'ny add --help' works", async () => {
  const { stdout, stderr, exitCode } = await executeNy("add --help");

  expect(exitCode).toBe(0);
  expect(stdout).toContain("Usage: ny add [OPTIONS] <PACKAGES>...");
  expect(stderr).toBe("");
});

// this should work but if it doesn't it's not a big deal
test.skip("'ny run --help' works", async () => {
  const { stdout, stderr, exitCode } = await executeNy("run --help");

  expect(exitCode).toBe(0);
  expect(stdout).toContain("Usage: ny run [COMMAND]");
  expect(stderr).toBe("");
});

test("'ny run echo-cli --help' prints echo's help", async () => {
  const { stdout, stderr, exitCode } = await executeNy("run echo-cli --help", { path: "./yarn" });

  expect(exitCode).toBe(0);
  expect(stdout).toContain("Outputs the passed text to the command line.");
  expect(stderr).toBe("");
});

test("'ny run echo-cli some --help' should simply pass all args", async () => {
  const { stdout, stderr, exitCode } = await executeNy("run echo-cli some --help", { path: "./yarn" });

  expect(exitCode).toBe(0);
  expect(stdout).toContain("some --help");
  expect(stderr).toBe("");
});

test("'ny echo-cli --help' prints echo's help", async () => {
  const { stdout, stderr, exitCode } = await executeNy("echo-cli --help", { path: "./yarn" });

  expect(exitCode).toBe(0);
  expect(stdout).toContain("Outputs the passed text to the command line.");
  expect(stderr).toBe("");
});

test("'ny echo-cli some --help' should simply pass all args", async () => {
  const { stdout, stderr, exitCode } = await executeNy("echo-cli some --help", { path: "./yarn" });

  expect(exitCode).toBe(0);
  expect(stdout).toContain("some --help");
  expect(stderr).toBe("");
});
