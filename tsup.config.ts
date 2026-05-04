import { defineConfig } from "tsup";

export default defineConfig({
  entry: ["src/cli.tsx"],
  format: ["esm"],
  target: "node20",
  platform: "node",
  outDir: "dist",
  clean: true,
  shims: true,
  splitting: false,
  sourcemap: false,
  minify: false,
  banner: { js: "#!/usr/bin/env node" },
  external: [
    "telegram",
    "@anthropic-ai/sdk",
    "openai",
    "grammy",
    "ink",
    "react",
    "@modelcontextprotocol/sdk"
  ]
});
