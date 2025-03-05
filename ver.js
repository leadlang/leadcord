const { readFileSync, writeFileSync } = require("fs");
const toml = require("toml");

const data = toml.parse(readFileSync("./leadcord/Cargo.toml").toString());

console.log(data.package.version);

writeFileSync("./.version", data.package.version);
