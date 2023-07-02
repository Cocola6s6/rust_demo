# 配置

1. 下载 [tailwindcss](https://github.com/tailwindlabs/tailwindcss/releases/tag/v3.3.2) ，配置环境变量
2. 下载 [trunk](https://github.com/thedodd/trunk/releases) ，配置环境变量
3. trunk serve



# 详细设计

### 一、组件设计

创建 App、Video、Button 组件



#### 1、App 组件设计

1. 初始化。加载媒体资源并且存储到上下文中。
2. 加载 Video 组件。
3. 返回 App 组件。



【准备】

* webapi 的【enumerateDevices】

  初始化加载媒体资源使用的 webapi 的【enumerateDevices】，得到可用的媒体输入和输出设备的列表。但是需要注意不授权的话 mac 获取不到。

* webapi 是【【getDisplayMedia】

  授权的 webapi 是【【getDisplayMedia】，提示用户去选择和授权。

* webapi 是【【getUserMedia】

  授权并且得到数据流





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



#### 3、初始化的时候使用【getDisplayMedia】进行授权的时候，提示错误了

> Unhandled Promise Rejection: InvalidAccessError: getDisplayMedia must be called from a user gesture handler.

