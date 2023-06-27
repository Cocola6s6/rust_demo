# 配置

1. 整合 [tauri](https://tauri.app/v1/guides/getting-started/setup/integrate/) 
2. 更新 rust，rustup update
3. 下载 tauri-cli，cargo install tauri-cli
4. 修改 tauri.config.json，devPath 改为 "http://localhost:8080",
5. 启动 sycamore，trunk serve
6. 启动 tauri，cargo tauri dev



# 详细设计

### 一、组件设计

创建 App、Video、Button 组件



#### 1、App 组件设计

1. 初始化。加载媒体资源并且存储到上下文中。
2. 加载 Video 组件。
3. 返回 App 组件。



#### 2、Video 组件设计

1. 根据上下文中的设备编号修改资源信息。
2. 获取 Video 组件并且设置资源信息【异步】。
3. 加载 Button 组件。
4. 返回 Video 组件。



#### 3、Button 组件设计

1. 根据上下文中的资源信息，绑定到 Select 标签上。
2. 根据 Video 组件传递的信息，修改 Select 标签是否展示的样式。
3. 返回 Button 组件。



### 二、Video 资源的动态修改

1. 在 Select 标签增加监听事件，将选择的 device_id 存储到上下文中。
2. Video 组件中使用【create_memo】函数处理【根据上下文中的设备编号修改资源信息】的业务逻辑。create_memo 内的 Signal 发生变化时，会重新执行 create_memo 的逻辑。这里的 Signal 指的是上下文中定义为 Signal 类型的属性或者父组件传递的属性。
3. Video 组件中使用【create_effect】函数处理【获取 Video 组件并且设置资源信息】的业务逻辑。create_effect 提供了一种通过订阅事件或信号的方式来执行副作用操作，create_effect 中订阅上一步修改资源信息，来实现动态资源的设置。



### 三、鼠标放到 Video 上时，Button 组件的展示

1. Video 组件增加鼠标监听事件，将监听结果存储到上下中或者直接传递给 Button 组件。
2. 根据 Video 组件传递的信息，修改 Select 标签是否展示的样式。





# 一些问题

# TODO

#### 1、将结构体转换成 JsValue

要转成 JsValue 的前提是结构体要实现 Serialize trait 和 Default trait，但是结构体中的某些字段能实现 Serialize trait。



#### 2、Video componet 中，以下写法不行

```rust
    // 获取 view 并且填充信息
    let video_ref = create_node_ref(ctx);
    let video_future = async move {
        info!("[video_future]===============>");
        let el = video_ref
            .get::<DomNode>()
            .unchecked_into::<HtmlVideoElement>();
        let video_stream = VideoStream::new(el);
        video_stream
            .set_video_src(&video_src_signal.get())
            .await;

        // 加载所有的 devices。后续需要需要将加载的 devices 放到 ctx 上下文中保管
        let devices = Devices::load().await;
        info!("[devices]===================>{:?}", devices);

        info!("[video_future done]===============>");
    };

    // 需要使用 sycamore 提供的异步执行，因为得先创建才能获取修改
    // wasm_bindgen_futures::spawn_local(video_future);
    spawn_local_scoped(ctx, video_future);

    let _ = create_effect(ctx, move || {
        // video_src_signal.track();
        spawn_local_scoped(ctx, video_future);
    });
```

