#!/usr/bin/env node
const fs = require("fs")
const os = require("os")
const { join } = require("path")
const { x: extract } = require("tar")
const { version } = require("./package.json")

async function install() {
	let voulrFileName = os.platform() === "win32" ? "voulr.exe" : "voulr"
	const targetExecutablePath = join(__dirname, voulrFileName)

	// check if binary already exists
	if (fs.existsSync(targetExecutablePath)) {
		return
	}

	let target = getTarget()
	const url = `https://github.com/voulr/cli/releases/download/v${version}/create-o7-app-${target}.tar.gz`

	// download and extract binaries from release
	const res = await fetch(url)
	if (!res.ok) {
		console.error(`Error fetching release: ${res.statusText}`)
		process.exit(1)
	}
	const sink = Readable.fromWeb(res.body).pipe(extract({ strip: 1, C: dir }))
	return new Promise((resolve) => {
		sink.on("finish", () => resolve())
		sink.on("error", (err) => {
			console.error(`Error fetching release: ${err.message}`)
			process.exit(1)
		})
	})
}

function getTarget() {
	const platform = os.platform()
	const arch = os.arch()

	if (platform == "darwin") {
		return arch == "arm64" ? "aarch64-apple-darwin" : "x86_64-apple-darwin"
	}
	if (platform === "win32" && arch === "x64") {
		return "x86_64-pc-windows-msvc"
	}
	if (platform === "linux" && arch === "x64") {
		return "x86_64-unknown-linux-musl"
	}

	throw new Error("Unsupported platform")
}

install().catch((error) => {
	console.error("Installation failed:", error)
	process.exit(1)
})
