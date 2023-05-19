<div align="center">
# 创建wsam项目流程



> #创建wasm初始化项目
>
> cargo generate --git https://github.com/rustwasm/wasm-pack-template.git --name wasm-game-of-life
>
> cd wasm-game-of-life
>
> 
>
> #构建wasm，生成wasm 模块文件
>
> wasm-pack build
>
> 
>
> #创建web初始化项目，模版是wasm-app
>
> npm install wasm-app www
>
> cd www
>
> 
>
> #修改依赖
>
> 在package.json中添加wasm模块
>
> 
>
> #调用wasm模块接口
>
> 在index.js中引入接口并调用
>
> 
>
> #安装web项目并运行
>
> npm install && npm run start