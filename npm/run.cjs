#!/usr/bin/env node
const fs = require("fs")
const os = require("os")
const { join } = require("path")
const { spawnSync } = require("child_process")

async function run() {
	let voulrFileName = os.platform() === "win32" ? "voulr.exe" : "voulr"
	const targetExecutablePath = join(__dirname, voulrFileName)

	// make sure binary is installed
	if (!fs.existsSync(targetExecutablePath)) {
		require("./install.cjs")
	}

	let res = spawnSync(targetExecutablePath, process.argv.slice(2), {
		cwd: process.cwd(),
		stdio: "inherit",
	})
	if (res.error) {
		throw new Error(`Error executing binary: ${res.error}`)
	}
	process.exit(res.status)
}

run()
