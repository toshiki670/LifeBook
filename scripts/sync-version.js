#!/usr/bin/env node

/**
 * Version Sync Script
 *
 * Synchronizes version from package.json to tauri.conf.json
 * This ensures frontend and backend use the same version number.
 */

import { readFileSync, writeFileSync } from "node:fs"
import { dirname, join } from "node:path"
import { fileURLToPath } from "node:url"

const __filename = fileURLToPath(import.meta.url)
const __dirname = dirname(__filename)
const rootDir = join(__dirname, "..")

// Color codes for terminal output
const colors = {
  reset: "\x1b[0m",
  green: "\x1b[32m",
  yellow: "\x1b[33m",
  red: "\x1b[31m",
  blue: "\x1b[36m",
}

function log(message, color = colors.reset) {
  console.log(`${color}${message}${colors.reset}`)
}

try {
  // Read package.json
  const packageJsonPath = join(rootDir, "package.json")
  const packageJson = JSON.parse(readFileSync(packageJsonPath, "utf8"))
  const version = packageJson.version

  if (!version) {
    throw new Error("Version not found in package.json")
  }

  log(`📦 Current version: ${version}`, colors.blue)

  // Read tauri.conf.json
  const tauriConfPath = join(rootDir, "src-tauri", "lifebook", "tauri.conf.json")
  const tauriConf = JSON.parse(readFileSync(tauriConfPath, "utf8"))
  const currentTauriVersion = tauriConf.version

  // Check if update is needed
  if (currentTauriVersion === version) {
    log(`✓ Versions are already in sync (${version})`, colors.green)
    process.exit(0)
  }

  // Update tauri.conf.json
  tauriConf.version = version
  writeFileSync(tauriConfPath, `${JSON.stringify(tauriConf, null, 2)}\n`, "utf8")

  log(`✓ Updated tauri.conf.json: ${currentTauriVersion} → ${version}`, colors.green)
  log(`  ${tauriConfPath}`, colors.yellow)
} catch (error) {
  log(`✗ Error syncing version: ${error.message}`, colors.red)
  process.exit(1)
}
