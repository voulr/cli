#!/usr/bin/env node
const { execFileSync } = require("child_process")
const path = require("path")

const binaryPath = path.join(__dirname, "voulr")
execFileSync(binaryPath, process.argv.slice(2), { stdio: "inherit" })
