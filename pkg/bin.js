const os = require("os")
const platforms = [
	{
		platform: "macos-silicon",
		type: "Darwin",
		arch: "arm64",
	},
	{
		platform: "macos-intel",
		type: "Darwin",
		arch: "x64",
	},
	{
		platform: "windows",
		type: "Windows_NT",
		arch: "x64",
	},
	{
		platform: "linux",
		type: "Linux",
		arch: "x64",
	},
]

const type = os.type()
const arch = os.arch()
const supported = platforms.find((p) => p.type === type && p.arch === arch)

if (!supported) {
	throw new Error(`unsupported platform: ${type} ${arch}`)
}

const { join } = require("path")
const { existsSync, mkdirSync } = require("fs")
const { Readable } = require("stream")
const { x: extract } = require("tar")
const { spawnSync } = require("child_process")
const { version } = require("./package.json")

const dir = join(__dirname, "node_modules", ".bin")
const bin = join(dir, `voulr${type === "Windows_NT" ? ".exe" : ""}`)
const exists = existsSync(bin)

async function install() {
	if (exists) return
	if (!existsSync(dir)) {
		mkdirSync(dir, { recursive: true })
	}

	const res = await fetch(
		`https://github.com/voulr/cli/releases/download/${version}/voulr-${supported.platform}.tar.gz`,
	)

	if (!res.ok) {
		console.error(`error fetching release: ${res.statusText}`)
		process.exit(1)
	}

	const sink = Readable.fromWeb(res.body).pipe(extract({ C: dir }))
	return new Promise((resolve) => {
		sink.on("finish", () => resolve())
		sink.on("error", (err) => {
			console.error(`error fetching release: ${err.message}`)
			process.exit(1)
		})
	})
}

async function run() {
	if (!exists) await install()
	const args = process.argv.slice(2)
	const child = spawnSync(bin, args, {
		cwd: process.cwd(),
		stdio: "inherit",
	})
	if (child.error) {
		console.error(child.error)
		process.exit(1)
	}
	process.exit(child.status)
}

module.exports = { install, run }
