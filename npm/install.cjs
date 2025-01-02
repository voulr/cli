#!/usr/bin/env node
const fs = require("fs")
const os = require("os")
const { join } = require("path")
const { x } = require("tar")
const { Readable } = require("stream")
const { version } = require("./package.json")
const { execSync } = require("child_process")

async function install() {
	let voulrFileName = os.platform() === "win32" ? "voulr.exe" : "voulr"
	const bin = join(__dirname, voulrFileName)

	// check if binary is already installed
	if (fs.existsSync(bin)) {
		return
	}

	let target = getTarget()
	let url = `https://github.com/voulr/cli/releases/download/v${version}/voulr-${target}.tar.gz`
	return await downloadAndExtract(url, bin)
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
	const res = await fetch(url)
	if (!res.ok) {
		throw new Error(`Error fetching release: ${res.statusText}`)
	}

	const extractStream = Readable.fromWeb(res.body).pipe(x({ C: __dirname }))

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
