# MARKDOWN.HTML

> [简体中文](README.md) | English

This project aims to provide a low-dependency, simple deployment solution for rendering Markdown web pages.

## Features

- **Low dependencies**: Only an HTML file, a small number of CDNs, and server programs are required to complete Markdown pages rendering.
- **Syntax highlighting support**: Renders code blocks in Markdown.
- **Markdown support**: Renders Markdown files into HTML.
- **Mermaid support**: Renders Mermaid syntax in Markdown into diagrams.
- **Multilingual support**: Supports machine translation between multiple languages.
- **Dark mode support**: Supports light, dark, and system-following modes.
- **Pjax support**: Supports page navigation without refreshing the whole page.

## Usage

### Manual Deployment

1. Download and rename the `index.html` file from this repository and place to your website's root directory or any other directory;
2. When the html file is accessed, it will automatically render the `index.md` in the same directory. If `index.md` does not exist, it will render `README.md`.

### Github Actions

```yaml
- name: Deploy markdown.html
  uses: PJ-568/MARKDOWN.HTML@v<version>
  with:
    target-dir: '.'
    file-name: 'index.html'
```

## Additional Notes

`The open-source projects used in this project allow modifications, but please retain the original author information. If removal is necessary, please contact the author to avoid losing technical support.` For more details, refer to the license.

## Code or Content Contributions

We welcome contributions of code and content from anyone interested in this project.
Please checkout [程序逻辑](doc/logic.md).

For a simpler way to contribute, please refer to the [Contribution Guide](CONTRIBUTING.md) for relevant information.

## License

This project is licensed under the [CC BY-SA 4.0 International](https://creativecommons.org/licenses/by-sa/4.0/) license. For full license information, please refer to the [`LICENSE`](LICENSE) file.

### Credits

#### Projects

|Name|License|URL|
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

#### Services

|Icon|Name|
|:-:|:--|
|None|[360 Frontend Static Resource Library](https://cdn.baomitu.com/)|
|![ByteDance Static Resource CDN icon](https://cdn.bytedance.com/src/res/logo.svg)|[ByteDance Static Resource CDN](https://cdn.bytedance.com/)|
|![Chinese Font Network Project](https://chinese-font.netlify.app/favicon.ico)|[Chinese Font Network Project](https://chinese-font.netlify.app)|

## Stargazers over time

[![Stargazers over time](https://starchart.cc/PJ-568/MARKDOWN.HTML.svg?variant=adaptive)](https://starchart.cc/PJ-568/MARKDOWN.HTML)
