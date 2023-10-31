import { beforeAll } from "bun:test";
import { executeNy } from "./test-utils";

beforeAll(async () => {
  await executeNy("", { path: "../sandboxes/yarn" }); // install deps
});
