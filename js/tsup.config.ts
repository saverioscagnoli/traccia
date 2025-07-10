import { defineConfig } from "tsup";

export default defineConfig({
  entry: ["src/index.ts"],
  format: ["esm"],
  dts: true,
  clean: true,
  minify: true,
  sourcemap: true,
  external: ["chalk", "fs"],
});
