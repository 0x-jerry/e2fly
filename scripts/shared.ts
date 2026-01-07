import { exec } from "@0x-jerry/utils/node";
import fs from "node:fs/promises";
import path from "node:path";

const platform = process.platform;
export const isMacOS = platform === "darwin";

export async function buildThenCopyTunHelper() {
  await exec("cargo build --package tun-helper --release");

  const extension = isMacOS ? "" : ".exe";

  const targetTriple = await getTargetTriple();
  const from = path.resolve(`target/release/tun-helper${extension}`);
  const to = path.resolve(`binaries/tun-helper-${targetTriple}${extension}`);
  await fs.copyFile(from, to);
}

async function getTargetTriple() {
  const rustInfo = await exec("rustc -vV", { collectOutput: true });
  const targetTriple = /host: (\S+)/g.exec(rustInfo)?.at(1);

  if (!targetTriple) {
    throw new Error("Failed to determine platform target triple");
  }

  return targetTriple;
}
