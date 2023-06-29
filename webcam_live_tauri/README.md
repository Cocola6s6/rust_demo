# 创建 tauri 项目流程

1. 整合 [tauri](https://tauri.app/v1/guides/getting-started/setup/integrate/) 
2. 更新 rust，rustup update
3. 下载 tauri-cli，cargo install tauri-cli
4. 修改 tauri.config.json，devPath 改为 "http://localhost:8080",
5. 启动 sycamore，trunk serve
6. 启动 tauri，cargo tauri dev



# 详细设计

### 一、Video 组件大小改变跟随窗口的改变而改变

1. 动态获取 window 的大小。
2. 根据 window 的大小设置 Video 组件的大小。





# 一些问题

# TODO

#### 1、启动 tauri 桌面应用时，Video 组件展示不出来，提示 Permission error。但是 web 可以正常

* 既然 web 没问题，推测是 tauri 的配置有问题，于是修改了 tauri.config.json。不行
* 打印报错位置的日志。发现是数据结构组装的有问题。加了 audio 参数的情况下桌面应用会提示错误。