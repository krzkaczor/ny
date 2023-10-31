import { expect, test } from "bun:test";
import { executeNy } from "./test-utils";

test("'ny add esbuild-register' works", async () => {
  const { stdout, stderr, exitCode } = await executeNy("add esbuild-register", { path: "../sandboxes/yarn" });

  expect(stdout).not.toContain("Installing types for ");
  expect(stderr).toBe("");
  expect(exitCode).toBe(0);
});
