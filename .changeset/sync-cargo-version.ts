/**
 * Syncs version stored in package.json (which is managed by changesets) with version stored in Cargo.toml
 */
import { writeFileSync, readFileSync } from "fs";
import { join } from "path";

function rewriteVersionInCargoConfig(config: string, newVersion: string) {
  const regex = /version = "(\d+\.\d+\.\d+)"/; // note: no global flag -- we want to replace the first instance only
  return config.replace(regex, `version = "${newVersion}"`);
}

const currentVersion = require("../package.json").version;
console.log(`Syncing Cargo.toml version to ${currentVersion}`);

const cargoConfigRaw = readFileSync(join(__dirname, "../Cargo.toml"), "utf-8");

const newCargoConfig = rewriteVersionInCargoConfig(cargoConfigRaw, currentVersion);

writeFileSync(join(__dirname, "../Cargo.toml"), newCargoConfig);
