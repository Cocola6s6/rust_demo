# 需求分析

### 一、流程

Server 循环监听连接请求，将请求解析为 HttpRequest，经过 Router、Handler 处理后得到 HttpResponse，最后返回给 Client。

![image-20230512183452812](https://note-1305755407.cos.ap-nanjing.myqcloud.com/note/image-20230512183452812.png)



# 详细设计

### 一、模块设计

设计 Server、设计 Router、设计 Handler、设计 HttpRequest、设计 HttpResponse



#### 1、设计 Server

1. 定义 new 函数，初始化 Server 实例并返回。
2. 定义 run 方法，启动 TcpListener 进行监听，循环对连接请求进行处理。
3. 对连接请求进行解析为 HttpRequest。
4. 将 HttpRequest 转发到 Router 处理。



#### 2、设计 Router

1. 对 HttpRequest 进行匹配。
2. 根据匹配转发到对应的 Handler 处理，返回 HttpResponse。
3. 将 HttpResponse 写入 Stream 流。



#### 3、设计 Handler

1. 定义 Handler trait，handle 和 load_file 通用方法。用来处理请求和加载静态资源。
2. 定义 StaticPageHandler、PageNotFoundHandler、WebServiceHandler，并且重载 Handler trait 的方法。
3. 返回 HttpResponse。



#### 4、设计 HttpRequest

1. 定义属性：请求类型、请求地址、请求头、请求参数
2. 重载 From trait，将 String 转换为 HttpRequest。



#### 4、设计 HttpResponse

1. 定义属性：响应内容、code、status。
2. 定义 new 函数，初始化 HttpResponse实例并返回。
3. 定义 send_response 方法，将 HttpResponse 写入 Stream 流。
4. 重载 From trait，将 HttpResponse 转换为 String。



### 二、数据流设计

1. 输入：string
2. 处理：string -> HttpRequest
3. 处理：HttpResponse -> string
4. 输出：string



## 总结

自定义 web 框架好累呀，还是直接用 Actix 吧。