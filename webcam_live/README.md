# Sycamore Web 应用开发

前面说到，使用原生 wasm 太累了，可以选择一些现有的 wasm 框架开发，如 Web 应用开发可以使用 Sycamore，桌面应用开发可以使用 Tauri。

流程和 wasm 一样，都通过【web-sys】调用 webapi 进行页面操作：获取页面元素、修改页面元素、增加页面元素、增加页面监听等操作。



【问题】为什么要使用 Sycamore 呢，不直接使用 wasm 呢？

* 这就好比你提问，为什么要使用 vue 呢，不直接使用 js。
* Sycamore 提供了动态渲染、接口更高级抽象等等功能。不需要你重复造轮子..................



【问题】为什么使用 web-sys 呢？

* 因为 web-sys 对 webapi 进行了 rust 的实现，这样我们就可以在 rust 中直接使用 webapi 了。



到这里，你可能还会提问，什么是 webapi 呢？

* 首先我们的需求是，在 web 上展示摄像头的内容。于是我们就需要找到获取摄像头的 API，我们的操作系统肯定提供了这么一个 API，在操作系统的下层，浏览器应用基于此也提供了 API，也就是 webapi。接着更下层会有 Java-webapi，Js-webapi，Rust-webapi 等等。从【操作系统-->浏览器-->编程语言】，我们第一选择肯定是最下层，更抽象封装的 API 啦。
* 这里一样解释了，为什么使用 Sycamore 而不直接用 wasm 了。



# 配置

1. 下载 [tailwindcss](https://github.com/tailwindlabs/tailwindcss/releases/tag/v3.3.2) ，配置环境变量。

2. 配置 tailwindcss.conf.js。

   ```bash
   tailwindcss init
   ```

3. 下载 [trunk](https://github.com/thedodd/trunk/releases) ，配置环境变量。

4. 配置 trunk hook，使得构建 wasm 的时候构建 tailwindcss。

5. trunk serve。





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



### 三、Tailwindcss 和 Bootstrap 比较

* Tailwindcss 和 Bootstrap 一样，是一个 CSS 框架。
* 可以通过将 rust 文件配置到【module.exports】，然后为 rust  文件中的这些样式生成所有相应的 CSS。



### 四、Trunk 的作用是什么？

* Trunk 是一个用来简化 wasm web 应用构建的工具。

* 构建后的文件存放在 dist 目录下。



# TODO

#### 1、使用 trunk hook 构建 tailwindcss 并存放到 trunk 的默认目录 dist 下。但是 trunk 结束之后，dist 目录下的文件就会重置。

* 使用 trunk hook 的 pre_build，生成 tailwindcss 并且存放到当前目录，标记它作为 asset 让 trunk 一起打包。

![image-20230629154921580](https://note-1305755407.cos.ap-nanjing.myqcloud.com/note/image-20230629154921580.png)

![image-20230628200637248](https://note-1305755407.cos.ap-nanjing.myqcloud.com/note/image-20230628200637248.png)