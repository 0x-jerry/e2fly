import { buildThenCopyTunHelper } from "./shared";

process.chdir("./src-tauri");
await buildThenCopyTunHelper();

// finished
process.exit(0);
