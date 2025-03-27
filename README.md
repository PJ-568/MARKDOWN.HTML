# MARKDOWN.HTML

> 简体中文 | [English](README_EN.md)

本项目旨在提供低依赖、一键部署的 Markdown 网页渲染方案。

## 特性

- **少依赖**：仅需一个 HTML 文件、少量 CDN 和服务器程序，即可完成 Markdown 网页渲染。
- **Markdown 支持**：将 Markdown 文件渲染为 HTML。
- **Mermaid 支持**：将 Markdown 中的 Mermaid 语法渲染为图表。
- **多语言支持**：支持多种语言间的机器互译。
- **深色模式**：支持浅色、深色、跟随系统。
- **Pjax 支持**：支持无刷新页面跳转。

## 使用方法

### 手动部署

1. 下载并重命名本仓库的 `index.html` 文件到你的网站根目录或任何其他目录中；
2. 当下载并重命名后的 `index.html` 被访问时，将自动渲染同目录下的 `index.md`，如果 `index.md` 不存在，则渲染 `README.md`。

### Github Actions

```yaml
- name: Deploy markdown.html
  uses: PJ-568/MARKDOWN.HTML@v1.10.1
  with:
    target-dir: '.'
    file-name: 'index.html'
```

## 其他说明

`本项目使用的开源项目允许修改，但请保留原作者信息。确需去除，请联系作者，以免失去技术支持。`详情请参照许可证。

## 代码或内容贡献

欢迎每一位对本项目感兴趣的朋友贡献代码和内容。
请参阅[程序逻辑](doc/logic.md)。

较为简单的提交贡献方法可查阅[贡献文档](CONTRIBUTING.md)以了解相关信息。

## 许可证

本项目遵循 [CC BY-SA 4.0 International](https://creativecommons.org/licenses/by-sa/4.0/) 许可协议。如果想了解完整许可信息，请查阅 [`LICENSE`](LICENSE) 文件。

### 附加许可证

|名称|协议|地址|
|:-:|:--|:--|
|tabler-icons|[MIT License](//mit-license.org)|[https://github.com/tabler/tabler-icons](https://github.com/tabler/tabler-icons)|
|markedjs/marked|[marked License](https://github.com/markedjs/marked/blob/master/LICENSE.md)|[Github](https://github.com/markedjs/marked)|
|mermaidjs/mermaid|[MIT License](https://github.com/mermaid-js/mermaid/blob/develop/LICENSE)|[Github](https://github.com/mermaid-js/mermaid)|
|translate.js|[Apache License Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)|[Github](https://github.com/xnx3/translate)、[Gitee](https://gitee.com/mail_osc/translate)|
|darkmode.js|[MIT license](//mit-license.org)|[Github](https://github.com/sandoche/Darkmode.js)|
|MoOx/pjax|[MIT license](//mit-license.org)|[Github](https://github.com/MoOx/pjax)|
|instant.page|[MIT license](//mit-license.org)|[Github](https://github.com/instantpage/instant.page)|
