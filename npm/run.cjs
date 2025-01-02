#!/usr/bin/env node
const fs = require("fs")
const os = require("os")
const { join } = require("path")
const { spawnSync } = require("child_process")

async function run() {
	let voulrFileName = os.platform() === "win32" ? "voulr.exe" : "voulr"
	const bin = join(__dirname, voulrFileName)

	// make sure binary is installed
	if (!fs.existsSync(bin)) {
		require("./install.cjs")
	}

	let res = spawnSync(bin, process.argv.slice(2), {
		cwd: process.cwd(),
		stdio: "inherit",
	})
	if (res.error) {
		throw new Error(`Error executing binary: ${res.error}`)
	}
	process.exit(res.status)
}

run()
