# MARKDOWN.HTML

> 简体中文 | [English](README_EN.md)

本项目旨在提供低依赖、一键部署的 Markdown 网页渲染方案。

## 特性

- **依赖少**：仅需一个 HTML 文件，即可完成 Markdown 网页渲染。
- **代码高亮支持**：渲染 Markdown 中的代码块。
- **Markdown 支持**：将 Markdown 文件渲染为 HTML。
- **Mermaid 支持**：将 Markdown 中的 Mermaid 语法渲染为图表。
- **多语言支持**：支持多种语言间的机器互译。
- **深色模式**：支持浅色、深色、跟随系统。
- **Pjax 支持**：支持无刷新页面跳转。

## 使用方法

| 文件名 | 说明 |
| :----- | :--- |
| index.html | 默认 HTML 入口文件，使用 CDN 引入依赖。 |
| index.min.html | 压缩后的 HTML 入口文件，使用 CDN 引入依赖。 |
| index.allinone.html | 全合一 HTML 入口文件，默认包含所有可被脚本下载的依赖。 |
| index.allinone.min.html | 压缩后的全合一 HTML 入口文件，默认包含所有可被脚本下载的依赖。 |

### 手动部署

1. 在[发行页面](https://github.com/PJ-568/MARKDOWN.HTML/releases)下载并重命名您需要的 HTML 文件到你的网站根目录或任何其他目录中；
2. 当下载并重命名后的 HTML 被访问时，将自动渲染同目录下的 `index.md`，如果 `index.md` 不存在，则渲染 `README.md`。

### Github Actions

编译并把 HTML 放入指定目录。使用时须将 `<version>` 替换为版本号。

```yaml
- name: Deploy markdown.html
  uses: PJ-568/MARKDOWN.HTML@v<version>
  with:
    target-dir: '.'
    file-name: 'index.html'
    use-minified: 'true'
    use-all-in-one: 'false'
```

## 其他说明

`本项目使用的开源项目允许修改，但请保留原作者信息。确需去除，请联系作者，以免失去技术支持。`详情请参照[许可证](#许可证)。

## 代码或内容贡献

欢迎每一位对本项目感兴趣的朋友贡献代码和内容。
请参阅[程序逻辑](doc/logic.md)。

较为简单的提交贡献方法可查阅[贡献文档](CONTRIBUTING.md)以了解相关信息。

## 许可证

本项目遵循 [CC BY-SA 4.0 International](https://creativecommons.org/licenses/by-sa/4.0/) 许可协议。如果想了解完整许可信息，请查阅 [`LICENSE`](LICENSE) 文件。

### 鸣谢

#### 项目

|名称|协议|地址|
|:-:|:--|:--|
|tabler-icons|[MIT License](//mit-license.org)|[Github](https://github.com/tabler/tabler-icons)、[Website](https://tabler.io/icons)|
|highlightjs/highlight.js|[BSD-3-Clause license](https://github.com/highlightjs/highlight.js/raw/refs/heads/main/LICENSE)|[Github](https://github.com/highlightjs/highlight.js)、[Website](https://highlightjs.org)|
|markedjs/marked|[marked License](https://github.com/markedjs/marked/blob/master/LICENSE.md)|[Github](https://github.com/markedjs/marked)、[Website](https://marked.js.org)|
|mermaidjs/mermaid|[MIT License](https://github.com/mermaid-js/mermaid/blob/develop/LICENSE)|[Github](https://github.com/mermaid-js/mermaid)、[Website](https://mermaid.js.org)|
|translate.js|[Apache License Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)|[Github](https://github.com/xnx3/translate)、[Gitee](https://gitee.com/mail_osc/translate)、[Website](https://translate.zvo.cn)|
|darkmode.js|[MIT license](//mit-license.org)|[Github](https://github.com/sandoche/Darkmode.js)、[Website](https://darkmodejs.learn.uno)|
|MoOx/pjax|[MIT license](//mit-license.org)|[Github](https://github.com/MoOx/pjax)|
|instant.page|[MIT license](//mit-license.org)|[Github](https://github.com/instantpage/instant.page)、[Website](https://instant.page)|
|Maple Mono|[OFL-1.1 license](https://openfontlicense.org/open-font-license-official-text/)|[Github](https://github.com/subframe7536/maple-font)、[Website](https://font.subf.dev)|

#### 服务

|图标|名称|
|:-:|:--|
|无|[360 前端静态资源库](https://cdn.baomitu.com/)|
|![字节跳动静态资源公共库图标](https://cdn.bytedance.com/src/res/logo.svg)|[字节跳动静态资源公共库](https://cdn.bytedance.com/)|
|![中文网字计划](https://chinese-font.netlify.app/favicon.ico)|[中文网字计划](https://chinese-font.netlify.app)|

## 星

[![Stargazers over time](https://starchart.cc/PJ-568/MARKDOWN.HTML.svg?variant=adaptive)](https://starchart.cc/PJ-568/MARKDOWN.HTML)
