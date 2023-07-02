// js 中引入 tauri 预制 api 的方式有两种：配置文件和 npm 方式
// 1、使用 npm install @tauri-apps/api
// import { invoke } from '@tauri-apps/api';
// 2、tauri.config.json 中添加了相应的配置
const { invoke } = window.__TAURI__.tauri


// TODO：测试 sycamore->js
// 结果：通过，sycamore 能调用到 js
export function js_api() {
    return console.log("[sycamore->js][js]===============================>");
}

// TODO：测试 sycamore->js->tauri
// 结果：不通过
export async function tauri_api() {
    console.log("[sycamore->js->tauri][js]===============================>");
    return await invoke('tauri_api');
}


// 分析：
// 1.tarur
// 2.js
// 3.sycamore
// 目标是 3->2->1，现在 3->2 可以, 3->2->1, 2->1 不行，所以 2->1 存在问题


// 设置 decorations
export async function tauri_set_window_decorations_api(decorations) {
    console.log("[tauri_set_window_decorations_api]===============================>");
    return await invoke('tauri_set_window_decorations_api', {decorations});
}