#!/usr/bin/env node
const fs = require("fs")
const os = require("os")
const { join } = require("path")
const { x } = require("tar")
const { Readable } = require("stream")
const { version } = require("./package.json")
const { execSync } = require("child_process")

async function install() {
	try {
		let voulrFileName = os.platform() === "win32" ? "voulr.exe" : "voulr"
		const bin = join(__dirname, voulrFileName)

		if (fs.existsSync(bin)) {
			process.exit(0)
		}

		let target = getTarget()
		let url = `https://github.com/voulr/cli/releases/download/v${version}/voulr-${target}.tar.gz`
		await downloadAndExtract(url, bin)
		process.exit(0)
	} catch (error) {
		console.error("Installation failed:", error)
		process.exit(1)
	}
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
		return isMusl() ? "x86_64-unknown-linux-musl" : "x86_64-unknown-linux-gnu"
	}
	throw new Error("Unsupported platform")
}

function isMusl() {
	try {
		const command = "getconf GNU_LIBC_VERSION 2>&1 || true; ldd --version 2>&1 || true"
		const output = execSync(command, { encoding: "utf8" })
		const [_, ldd1] = output.split(/[\r\n]+/)
		return ldd1 && ldd1.includes("musl")
	} catch {
		return false
	}
}

async function downloadAndExtract(url, bin) {
	const res = await fetch(url, {
		timeout: 10000,
	})

	if (!res.ok) {
		throw new Error(`Error fetching release: ${res.statusText}`)
	}

	return new Promise((resolve, reject) => {
		const extractStream = Readable.fromWeb(res.body).pipe(x({ C: __dirname }))

		extractStream.on("finish", () => {
			try {
				fs.chmodSync(bin, 0o755)
				resolve()
			} catch (error) {
				reject(new Error(`Failed to set permissions: ${error.message}`))
			}
		})

		extractStream.on("error", (err) => {
			reject(new Error(`Error extracting file: ${err.message}`))
		})
	})
}

install().catch((error) => {
	console.error("Installation failed:", error)
	process.exit(1)
})
