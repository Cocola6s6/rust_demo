# rust_demo

this is a rust demo

## Http_Server

![image-20230512183452812](https://note-1305755407.cos.ap-nanjing.myqcloud.com/note/image-20230512183452812.png)





## WebAssembly

![image-20230515173201949](https://note-1305755407.cos.ap-nanjing.myqcloud.com/note/image-20230515173201949.png)





![image-20230515173305959](https://note-1305755407.cos.ap-nanjing.myqcloud.com/note/image-20230515173305959.png)



### 一些问题

【问题】：为什么浏览器能解析webassembly代码？难道它也和javascript一样，浏览器内置对应的引擎？

![image-20230519211845945](https://note-1305755407.cos.ap-nanjing.myqcloud.com/note/image-20230519211845945.png)



【问题】：那现在的浏览器都内置了哪些语言的引擎呢？

* 浏览器的引擎是不断发展和更新的，因此具体支持的语言和技术可能会随着时间的推移而有所变化。

![image-20230519212009447](https://note-1305755407.cos.ap-nanjing.myqcloud.com/note/image-20230519212009447.png)



【问题】为什么浏览器不内置C、C++等语言的引擎，以使得它们能在浏览器上运行，来提高浏览器应用的效率呢？

![image-20230519212139550](https://note-1305755407.cos.ap-nanjing.myqcloud.com/note/image-20230519212139550.png)



### 总结

综上，浏览器没有内置C、C++等语言的引擎，因为作为原生语言的它们是和底层做交互的，在不同CPU架构上被翻译成的机器码是不一样的，而浏览器也就需要在不同的CPU架构上做接口适配，这不简单。同时，它们能操作系统底层，如果在浏览器应用中运行原生代码，如果没有很好的代码权限和访问范围，会带来安全隐患。

所以，为了解决以上两点主要问题，可以通过浏览器和系统之间引入一个中间体，通过这个中间体，帮助浏览器进行代码权限和访问范围的控制，帮助浏览器解析C、C++等原生语言并且适配不同CPU架构。WebAssembly做到了。

WebAssembly可以帮原生语言代码编译成WebAssembly模块，在不同CPU架构下的浏览器应用可以加载该模块，前提是内置了WebAssembly引擎。然后调用WebAssembly模块里的接口。