import * as fs from "fs";

const args = process.argv.slice(2);
const command = args[0];

switch (command) {
  case "bytes":
    const data = fs.readFileSync(args[1]);
    console.log(JSON.stringify(Array.from(data)));
    break;
  default:
    throw new Error(`Unknown command ${command}`);
}
