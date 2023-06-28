# 创建 wsam 项目流程

1. 创建 wasm 项目

   ```bash
   cargo generate --git https://github.com/rustwasm/wasm-pack-template.git --name wasm-game-of-life
   ```

2. 下载 wasm-pack

   ```bash
   cargo install wasm-pack
   ```

3. 构建打包 wasm，打包后生成了 web 依赖

   ```bash
   wasm-pack build
   ```

4. 创建 web 项目

   ```bash
   npm install wasm-app www
   ```

5. web 项目的引入 wasm 依赖，在 package.json 中添加 wasm 构建的 package

6. 调用 wasm 依赖接口，在 index.js 中引入接口并调用

7. 启动 web 项目

   ```bash
   npm install && npm run start
   ```

   

# 一些问题

#### 1、\#[wasm_bindgen] 注解的作用

* 可以实现 wasm 项目和 web 项目之间的通信。

![image-20230628130242079](https://note-1305755407.cos.ap-nanjing.myqcloud.com/note/image-20230628130242079.png)



# 总结

总体来看，浏览器支持了 wasm 引擎，可以渲染 wasm。wasm 需要做兼容：

1. 为了使得  JS  可以访问到 wasm 对外的 API，实现了统一的方式是 \#[wasm_bindgen] 注解。
2. 为了使得 JS 可以映入 wasm，实现了wasm-pack 工具，可以将其打包为 JS 可以引入的依赖。

