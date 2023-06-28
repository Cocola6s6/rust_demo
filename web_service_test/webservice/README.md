# 需求分析

自定义 web 框架太累了，还是选择使用 Actix  框架吧。

* Server、Router、HttpRequest、HttpRequest 这些都不用自己设计了。与其对应的是，Actix 提供了 HttpServer、route、HttpRequest、HttpRequest 等。
* 唯一要做的就是自己实现 Handler，这一部分是业务逻辑。Handler 得到结果 用 HttpResponse 提供的转换方法，转换为 HttpResponse 返回即可。