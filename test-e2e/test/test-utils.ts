import assert from "assert";
import { existsSync } from "fs";
import { join } from "path";

export async function executeNy(
  args: string,
  opts?: { path: string },
): Promise<{ stdout: string; stderr: string; exitCode: number }> {
  const cwd = opts?.path ? join(import.meta.dir, opts.path) : import.meta.dir;
  const nyPath = join(import.meta.dir, "../../target/debug/ny");
  assert(existsSync(nyPath), "ny binary not found. Did you forgot to run `cargo build`?");
  assert(existsSync(cwd), `cwd=${cwd} does not exist.`);

  const proc = Bun.spawn([nyPath, ...args.split(" ")], { cwd, stderr: "pipe" });

  const stdout = await new Response(proc.stdout).text();
  const stderr = await Bun.readableStreamToText(proc.stderr);
  const exitCode = await proc.exited;

  return { stdout, stderr, exitCode };
}
