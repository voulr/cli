#!/usr/bin/env node
const fs = require("fs")
const os = require("os")
const { join } = require("path")
const { spawnSync } = require("child_process")

async function run() {
	let voulrFileName = os.platform() === "win32" ? "voulr.exe" : "voulr"
	const dir = join(__dirname, "node_modules", ".bin")
	const bin = join(dir, voulrFileName)

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
