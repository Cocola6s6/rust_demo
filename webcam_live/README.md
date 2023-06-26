# 配置

1. 下载 [tailwindcss](https://github.com/tailwindlabs/tailwindcss/releases/tag/v3.3.2) ，配置环境变量
2. 下载 [trunk](https://github.com/thedodd/trunk/releases) ，配置环境变量
3. trunk serve



# 详细设计

1. 创建整体页面
2. 创建需要局部刷新的组件 component
3. 组件方法中的返回值是当前组件，即生成组件实在最后。组件中的正确流程应该是：创建组件-->获取组件-->动态填充组件内容。使用 sycamore 提供的异步方法【spawn_local_scoped】，可以实现子线程使用主线程变量在作用域范围。