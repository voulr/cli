const os = require("os")
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

const { join } = require("path")
const { existsSync, mkdirSync, chmodSync } = require("fs")
const { Readable } = require("stream")
const { x: extract } = require("tar")
const { spawnSync } = require("child_process")
const { version } = require("./package.json")

// Using a more reliable installation directory
const dir = join(os.homedir(), ".voulr", "bin")
const bin = join(dir, `voulr-${supported.file}`)

console.log("Installation directory:", dir)
console.log("Binary path:", bin)

async function install() {
	console.log("Starting installation...")
	console.log("Platform:", type, arch)
	console.log("Selected binary:", supported.file)

	try {
		if (!existsSync(dir)) {
			console.log("Creating installation directory...")
			mkdirSync(dir, { recursive: true })
		}

		const url = `https://github.com/voulr/cli/releases/download/${version}/voulr-${supported.file}.tar.gz`
		console.log("Downloading from:", url)

		const res = await fetch(url)

		if (!res.ok) {
			throw new Error(`Failed to download: ${res.status} ${res.statusText}`)
		}

		console.log("Download successful, extracting...")

		const sink = Readable.fromWeb(res.body).pipe(extract({ strip: 1, C: dir }))

		await new Promise((resolve, reject) => {
			sink.on("finish", () => {
				console.log("Extraction complete")
				resolve()
			})
			sink.on("error", (err) => {
				console.error("Extraction failed:", err)
				reject(err)
			})
		})

		// Make binary executable
		console.log("Setting executable permissions...")
		chmodSync(bin, "755")

		// Verify binary exists
		if (!existsSync(bin)) {
			throw new Error(`Binary not found at ${bin} after installation`)
		}

		console.log("Installation complete!")
	} catch (error) {
		console.error("Installation failed:", error)
		throw error
	}
}

async function run() {
	if (!existsSync(bin)) {
		console.log("Binary not found, installing...")
		await install()
	}

	console.log("Running binary:", bin)
	const args = process.argv.slice(2)
	console.log("Arguments:", args)

	const child = spawnSync(bin, args, {
		cwd: process.cwd(),
		stdio: "inherit",
		env: process.env,
	})

	if (child.error) {
		console.error("Execution failed:", child.error)
		process.exit(1)
	}

	process.exit(child.status ?? 0)
}

module.exports = { install, run }
