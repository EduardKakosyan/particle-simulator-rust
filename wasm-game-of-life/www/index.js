import init, { greet } from '../pkg/wasm_game_of_life.js';

async function run() {
  await init();
  const result = greet("WebAssembly");
  console.log(result);
}

run().catch(console.error);