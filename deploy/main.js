import init, { run_app } from "./js/yew_todolist.js";
async function main() {
  await init("/js/yew_todolist_bg.wasm");
  run_app();
}
main();
