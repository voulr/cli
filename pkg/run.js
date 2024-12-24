#!/usr/bin/env node
const os = require("os")
const { execFileSync } = require("child_process")
const path = require("path")

const platforms = [
	{
		type: "Darwin",
		arch: "arm64",
		file: "macos-silicon",
	},
	{
		type: "Darwin",
		arch: "x64",
		file: "macos-intel",
	},
	{
		type: "Windows_NT",
		arch: "x64",
		file: "windows.exe",
	},
	{
		type: "Linux",
		arch: "x64",
		file: "linux",
	},
]

const type = os.type()
const arch = os.arch()
const supported = platforms.find((p) => p.type === type && p.arch === arch)
if (!supported) {
	throw new Error(`unsupported platform: ${type} ${arch}`)
}

const binaryPath = path.join(__dirname, `bin/voulr-${supported.file}`)
execFileSync(binaryPath, process.argv.slice(2), { stdio: "inherit" })
