# 需求分析

### 一、流程

![image-20230628134340869](https://note-1305755407.cos.ap-nanjing.myqcloud.com/note/image-20230628134340869.png)



# 详细设计

### 一、模块设计

创建 wasm-client 作为前端项目、创建 webservice 作为后端项目



#### 1、创建 wasm-client 作为前端项目

1. 项目结构分层：common、models、managers。
2. 设计对外 API 接口，list_courses_api、insert_course_api、delete_course_api



#### 2、创建 webservice 作为后端项目

1. 使用 actix 作为 web 框架，处理 web 请求。
2. 使用 sqlx 作为数据库访问框架。



### 二、实现 list_courses_api 自动调用和渲染

1. wasm 中使用 [wasm_bindgen(start)] 注解。wasm 模块加载完成后自动执行，不需要 JS 通过 API 调用。
2. 使用 web_sys 获取网页元素，并且进行数据填充。



![image-20230630152537053](https://note-1305755407.cos.ap-nanjing.myqcloud.com/note/image-20230630152537053.png)



### 三、文件结构分层

* common：通用文件，如 constants、error 等
* models：结构体文件，web 中 JSON 需要转换为 rust 数据结构
* managers：外部接口文件，调用外部 webservice 的接口
* main.rs：提供给 web 的接口文件。用 \#[wasm_bindgen] 注解修饰，返回值为 JsValue。





# 总结

比较 Java 做 web 开发，使用 Rust 时，好像多此一举了，明明我可以在 web 上通过 JS 直接调用 webservice，为什么还要先调用 wasm 的 API，再通过 wasm 请求到 webservice 呢？

因为应用场景不对。

* wasm 是为了解决 js 的效率问题而引入，rust 的效率比 JS 要好，这些场景可以放在在 wasm 上进行实现。

* 作为 web 开发时 wasm 上的实现只是转发了 http 请求而已。 



【问题】那么，能用 wasm 做什么呢？

* 游戏开发，用 rust 开发的游戏不比用 JS 性能高么？
* Web 应用开发，用 rust 开发的 Web 应用不比用 JS 性能高么？如用 Sycamore 开发 Web 应用。

![image-20230628135244605](https://note-1305755407.cos.ap-nanjing.myqcloud.com/note/image-20230628135244605.png)



最后，使用原生 wasm 太累了，可以选择一些现有的 wasm 框架开发，如 Web 应用开发可以使用 Sycamore，桌面应用开发可以使用 Tauri。