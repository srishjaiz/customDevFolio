import type { NextConfig } from "next";
import path from "path";
import { fileURLToPath } from "url";

const templateRoot = path.dirname(fileURLToPath(import.meta.url));

const nextConfig: NextConfig = {
  reactStrictMode: true,
  // Avoid picking a parent lockfile when this repo sits under another project tree.
  outputFileTracingRoot: templateRoot,
};

export default nextConfig;
