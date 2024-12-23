#!/usr/bin/env node
import os from "os"
import { execFileSync } from "child_process"
import path from "path"

const platforms = [
	{
		type: "Windows_NT",
		arch: "x64",
		file: "win64.exe",
	},
	{
		type: "Windows_NT",
		arch: "ia32",
		file: "win32.exe",
	},
	{
		type: "Linux",
		arch: "x64",
		file: "linux",
	},
	{
		type: "Darwin",
		arch: "x64",
		file: "macos",
	},
	{
		type: "Darwin",
		arch: "arm64",
		file: "macos-arm64",
	},
]

async function run() {
	const type = os.type()
	const arch = os.arch()
	const supported = platforms.find((p) => p.type === type && p.arch === arch)
	if (!supported) {
		throw new Error(`unsupported platform: ${type} ${arch}`)
	}

	const binaryPath = path.join(__dirname, `bin/voulr-${supported.file}`)
	execFileSync(binaryPath, process.argv.slice(2), { stdio: "inherit" })
}

await run()
