# 配置

1. 下载 [tailwindcss](https://github.com/tailwindlabs/tailwindcss/releases/tag/v3.3.2) ，配置环境变量
2. 下载 [trunk](https://github.com/thedodd/trunk/releases) ，配置环境变量
3. trunk serve



# 详细设计

### 一、组件设计

1. 创建整体页面
2. 创建需要局部刷新的组件 component
3. 组件方法中的返回值是当前组件，即生成组件实在最后。组件中的正确流程应该是：创建组件-->获取组件-->动态填充组件内容。使用 sycamore 提供的异步方法【spawn_local_scoped】，可以实现子线程使用主线程变量在作用域范围。



### 二、获取设备媒体资源设计

web 怎么获取设备的媒体资源，mozilla 提供了官方的 API 接口文档。参考 https://developer.mozilla.org/zh-CN/docs/Web/API/MediaDevices/getUserMedia

1.  初始化 client
2. 组装请求参数：因为 web 的数据结构和 Rust 的数据结构是不一样的，需要转换。web 统一使用的是 json
3. 发起请求
4. 处理响应





# 一些问题

### 一、sycamore 和 vue 比较

* Sycamore 是一个基于 Rust 的前端框架，而 Vue 是一个基于 JavaScript 的前端框架。

![image-20230626160828997](https://note-1305755407.cos.ap-nanjing.myqcloud.com/note/image-20230626160828997.png)





### 二、sycamore 和 wasm 的关系

* Sycamore 是一个基于 Rust 的前端框架，而 Rust 代码是需要通过 wasm 才能编译成浏览器引擎解析。

![image-20230626161532130](https://note-1305755407.cos.ap-nanjing.myqcloud.com/note/image-20230626161532130.png)



![image-20230626161633285](https://note-1305755407.cos.ap-nanjing.myqcloud.com/note/image-20230626161633285.png)