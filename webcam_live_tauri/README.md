# Tauri 桌面应用开发

前面说到，使用原生 wasm 太累了，可以选择一些现有的 wasm 框架开发，如 Web 应用开发可以使用 Sycamore，桌面应用开发可以使用 Tauri。



# 创建 tauri 项目流程

1. 整合 [tauri](https://tauri.app/v1/guides/getting-started/setup/integrate/) 
2. 更新 rust，rustup update
3. 下载 tauri-cli，cargo install tauri-cli
4. 修改 tauri.config.json，devPath 改为 "http://localhost:8080",
5. 启动 sycamore，trunk serve
6. 启动 tauri，cargo tauri dev 或者 cargo tauri dev --no-watch



cargo tauri dev 是动态加载，cargo tauri dev --no-watch 取消动态加载



# 详细设计

### 一、Video 组件大小改变跟随窗口的改变而改变

1. 动态获取 window 的大小。
2. 根据 window 的大小设置 Video 组件的大小。



#### 1、动态获取 window 的大小

【准备】

* web-sys 的 add_event_listener_with_callback() 方法
  * 对应的 webapi 是 【addEventListener】，参数是事件类型和事件通知，当事件类型触发时，收到事件通知。
  * 参考地址：https://developer.mozilla.org/zh-CN/docs/Web/API/EventTarget/addEventListener

* wasm 的 Closure struct
  * 事件通知可以是一个函数，需要将 rust 函数转换为 web 的数据结构 JsValue。使用 wasm Closure 可以将函数转为 JsValue



【注意】事件的生命周期是很长的，对应 rust 中就是需要一个 'static 的生命周期

1. 所以 add_event_listener_with_callback 的参数必须是 'static 的生命周期，否则会发生悬垂引用。事件通知函数作为它的一个参数，当函数退出后就会被回收。
2. 使用 wasm Closure 的 into_js_value() 的方法，让事件通知函数 forget 后，保证不被回收。
3. 注意监听事件等操作，要保证生命周期有效。





#### 2、根据 window 的大小设置 Video 组件的大小。

【遇到的问题】当获得 window 的大小后，需要将它存储到上下文中，这时候会提示 ctx 的声明周期存在问题。

* 由于监听事件的生命周期等价于 'static，而 ctx 的生命周期是一个普通的生命周期，监听事件中又需要使用到 ctx，ctx 是一个引用，所以会发生悬垂引用。



【解决办法】ctx 结束时，强制让监听事件结束

1. 将 ctx 传入监听事件中。
2. 使用 unsafe 方法替换 into_js_value，为了下一步可以 drop。
3. 判断 ctx 是否被清除，清除时手动 drop 监听事件。





### 二、Video 组件窗口栏的显示根据鼠标位置显示

1. tauri 提供 api，可以根据输入修改 window 的 decoration。
2. 创建 js 调用 tauri api。
3. 在 sycamore 通过 wasm_bindgen 调用 js 的 api。



API 调用图如下所示。

![image-20230705173554212](https://note-1305755407.cos.ap-nanjing.myqcloud.com/note/image-20230705173554212.png)



# 一些问题

【问题】为什么 sycamore 不能直接调用 tauri 的 api 呢，而是要先给到 js ？

* 因为 sycamore、tauri 都是跟 js 交互，所以 js 是一个中间体。



【问题】为什么不把鼠标监听事件的逻辑放在 tauri 上处理，反而绕了一圈呢？

* 因为 tauri 只是提供了操作桌面应用的接口，具体逻辑还需要在其他地方处理。也就是说你完全可以直接在 js 上处理，因为 rust 上的处理也是通过 web-sys 提供的 web api 进行处理，原理是一样的。



【问题】tauri 的 api 都有哪些？怎么引入 tauri 提供的 api 呢？

首先需要知道的是，引入 api 的位置是 js 文件。因为是根据 tauri 是和 js 进行交互。

tauri api 分为两种：

* @tauri-apps/api 是 tauri 预制的。使用 npm install @tauri-apps/api 进行安装，或者修改 tauri.config.js 配置文件进行安装。
* 使用 \#[tauri::command] 注解自定义实现，自定义的需要通过预制的 invoke 进行调用。





# TODO

#### 1、启动 tauri 桌面应用时，Video 组件展示不出来，提示 Permission error。但是 web 可以正常。

* 既然 web 没问题，推测是 tauri 的配置有问题，于是修改了 tauri.config.json。不行
* 打印报错位置的日志。发现是数据结构组装的有问题。加了 audio 参数的情况下桌面应用会提示错误。



#### 2、按理来说 ctx 的声明周期是比较长的，我不关闭应用程序是不会消失，但是这里确实报了 ctx 空，需要后续再看看，ctx 什么时候被消耗，ctx 在哪里被销毁了。