#!/usr/bin/env node
const fs = require("fs")
const os = require("os")
const { join } = require("path")
const https = require("https")
const tar = require("tar")
const { version } = require("./package.json")

async function install() {
	let voulrFileName = os.platform() === "win32" ? "voulr.exe" : "voulr"
	const targetExecutablePath = join(__dirname, voulrFileName)

	// check if binary already exists
	if (fs.existsSync(targetExecutablePath)) {
		return
	}

	let target = getTarget()
	const url = `https://github.com/voulr/cli/releases/download/v${version}/voulr-${target}.tar.gz`

	// download and extract from release
	await new Promise((resolve, reject) => {
		https
			.get(url, (response) => {
				response
					.pipe(tar.x({ C: __dirname }))
					.on("end", resolve)
					.on("error", reject)
			})
			.on("error", reject)
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

install()
