const esbuild = require("esbuild");
const fs = require("fs");
const path = require("path");
const production = process.argv.includes("--production");
const watch = process.argv.includes("--watch");

/**
 * @type {import('esbuild').Plugin}
 */
const esbuildProblemMatcherPlugin = {
  name: "esbuild-problem-matcher",

  setup(build) {
    build.onStart(() => {
      console.log("[watch] build started");
    });
    build.onEnd((result) => {
      result.errors.forEach(({ text, location }) => {
        console.error(`✘ [ERROR] ${text}`);
        console.error(
          `    ${location.file}:${location.line}:${location.column}:`,
        );
      });
      console.log("[watch] build finished");
    });
  },
};

const copyWasmPlugin = {
  name: "copy-wasm",
  setup(build) {
    build.onEnd(() => {
      const srcDir = path.join(__dirname, "pkg");
      const distDir = path.join(__dirname, "dist");

      if (!fs.existsSync(distDir)) {
        fs.mkdirSync(distDir, { recursive: true });
      }

      // Alle .wasm Dateien aus pkg/ nach dist/ kopieren
      if (fs.existsSync(srcDir)) {
        fs.readdirSync(srcDir).forEach((file) => {
          if (file.endsWith(".wasm")) {
            fs.copyFileSync(path.join(srcDir, file), path.join(distDir, file));
          }
        });
      }
    });
  },
};

async function main() {
  const ctx = await esbuild.context({
    entryPoints: ["src/extension.ts"],
    bundle: true,
    format: "cjs",
    minify: production,
    sourcemap: !production,
    sourcesContent: false,
    platform: "node",
    outfile: "dist/extension.js",
    external: ["vscode"],
    logLevel: "silent",
    plugins: [esbuildProblemMatcherPlugin, copyWasmPlugin],
  });
  if (watch) {
    await ctx.watch();
  } else {
    await ctx.rebuild();
    await ctx.dispose();
  }
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
