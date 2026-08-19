import { Keys } from "../dist/index.js";

const keys = Keys.generate();
console.log(keys.publicKey().toBech32());
