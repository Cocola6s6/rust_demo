// src/**/*.rs 下的 css 样式都会被加载到 tailwind 中，不需要提前定义，直接动态加载
// Tailwind CSS 的工作原理是扫描所有 HTML、JavaScript 组件和任何其他模板文件中的类名，然后为这些样式生成所有相应的 CSS
module.exports = {
  content: ['./src/**/*.rs'],
  theme: {
    extend: {},
  },
  plugins: [],
}