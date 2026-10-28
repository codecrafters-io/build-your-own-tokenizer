import * as fs from "fs";

const args = process.argv.slice(2);
const command = args[0];

console.error("Logs from your program will appear here!");

switch (command) {
  case "bytes":
    // TODO: Uncomment the code below to pass the first stage
    // const data = fs.readFileSync(args[1]);
    // console.log("[" + Array.from(data).join(", ") + "]");
    break;
  default:
    throw new Error(`Unknown command ${command}`);
}
