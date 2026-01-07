import { exec } from "@0x-jerry/utils/node";
import { buildThenCopyTunHelper } from "./shared";

process.chdir("./src-tauri");
await buildThenCopyTunHelper();

process.chdir("..");
await exec("npm run build:tauri");

// finished
process.exit(0);
