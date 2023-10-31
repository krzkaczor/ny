import { expect, test } from "bun:test";
import { executeNy } from "./test-utils";

test("'ny test' works", async () => {
  const { stdout, stderr, exitCode } = await executeNy("test", { path: "../sandboxes/yarn" });

  expect(stdout).toContain("some-output");
  expect(stderr).toBe("");
  expect(exitCode).toBe(0);
});

test("'ny test extra-arg' works - passing extra args to scripts", async () => {
  const { stdout, stderr, exitCode } = await executeNy("test extra-arg", { path: "../sandboxes/yarn" });

  expect(stdout).toContain("some-output extra-arg");
  expect(stderr).toBe("");
  expect(exitCode).toBe(0);
});

test("'ny run test' works", async () => {
  const { stdout, stderr, exitCode } = await executeNy("run test", { path: "../sandboxes/yarn" });

  expect(stdout).toContain("some-output");
  expect(stderr).toBe("");
  expect(exitCode).toBe(0);
});

test("'ny run test extra-arg' works - passing extra args to scripts", async () => {
  const { stdout, stderr, exitCode } = await executeNy("run test extra-arg", { path: "../sandboxes/yarn" });

  expect(stdout).toContain("some-output extra-arg");
  expect(stderr).toBe("");
  expect(exitCode).toBe(0);
});
