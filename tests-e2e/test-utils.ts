import assert from "assert";
import { existsSync } from "fs";
import { join } from "path";

export async function executeNy(
  args: string,
  _opts?: { path: string },
): Promise<{ stdout: string; stderr: string; exitCode: number }> {
  const cwd = _opts?.path ? join(import.meta.dir, _opts.path) : import.meta.dir;
  const nyPath = join(import.meta.dir, "../target/debug/ny");
  assert(existsSync(nyPath), "ny binary not found. Did you forgot to run `cargo build`?");

  const proc = Bun.spawn([nyPath, ...args.split(" ")], { cwd });

  const exitCode = await proc.exited;
  const stdout = await new Response(proc.stdout).text();
  const stderr = await new Response(proc.stderr).text();

  return { stdout, stderr, exitCode };
}
