#!/usr/bin/env node
const fs = require("fs")
const os = require("os")
const { join } = require("path")
const { x } = require("tar")
const { Readable } = require("stream")
const { version } = require("./package.json")

async function install() {
	let voulrFileName = os.platform() === "win32" ? "voulr.exe" : "voulr"
	const dir = join(__dirname, "node_modules", ".bin")
	const bin = join(dir, voulrFileName)

	// check if binary is already installed
	if (fs.existsSync(bin)) {
		return
	}

	// make sure directory is created
	if (!fs.existsSync(dir)) {
		mkdirSync(dir, { recursive: true })
	}

	let target = getTarget()
	let url = `https://github.com/voulr/cli/releases/download/v${version}/voulr-${target}.tar.gz`
	return await downloadAndExtract(url, dir, bin)
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

async function downloadAndExtract(url, dir, bin) {
	const res = await fetch(url)
	if (!res.ok) {
		throw new Error(`Error fetching release: ${res.statusText}`)
	}

	const extractStream = Readable.fromWeb(res.body).pipe(x({ C: dir }))

	return new Promise((resolve) => {
		extractStream.on("finish", () => {
			fs.chmodSync(bin, 0o755)
			resolve()
		})
		extractStream.on("error", (err) => {
			throw new Error(`Error extracting file: ${err.message}`)
		})
	})
}

install()
