#!/usr/bin/env node
const fs = require("fs")
const os = require("os")
const { join } = require("path")
const { spawnSync } = require("child_process")

async function run() {
	let voulrFileName = os.platform() === "win32" ? "voulr.exe" : "voulr"
	const targetExecutablePath = join(__dirname, voulrFileName)

	// install the binary if it doesnt exist
	if (fs.existsSync(targetExecutablePath)) {
		await require("./install.cjs")
	}

	spawnSync(bin, process.argv.slice(2), {
		cwd: process.cwd(),
		stdio: "inherit",
	})
}

run()
