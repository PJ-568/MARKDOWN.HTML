# MARKDOWN.HTML

> [简体中文](README.md) | English

This project provides a low-dependency, one-click deployment solution for rendering Markdown to web pages.

## Features

- **Minimal dependencies**: Only requires a single HTML file to render Markdown web pages.
- **Code highlighting**: Supports syntax highlighting for code blocks in Markdown.
- **Markdown support**: Renders Markdown files to HTML.
- **Mermaid support**: Renders Mermaid diagrams in Markdown.
- **Multi-language support**: Provides machine translation between languages.
- **Dark mode**: Supports light, dark, and system theme modes.
- **Pjax support**: Enables page navigation without full refresh.

## Usage

| Filename | Description |
| :----- | :--- |
| index.html | Default HTML entry file using CDN dependencies. |
| index.min.html | Minified HTML entry file using CDN dependencies. |
| index.allinone.html | All-in-one HTML entry file with embedded dependencies which can be downloaded by script. |
| index.allinone.min.html | Minified all-in-one HTML entry file with embedded dependencies which can be downloaded by script. |

### Manual Deployment

1. Download the required HTML file from [Releases](https://github.com/PJ-568/MARKDOWN.HTML/releases) and place it in your website root directory;
2. The HTML file will automatically render `index.md` in the same directory, or `README.md` if `index.md` doesn't exist.

### Github Actions

Compiles and deploys HTML files to specified directory. Replace `<version>` with actual version number.

```yaml
- name: Deploy markdown.html
  uses: PJ-568/MARKDOWN.HTML@v<version>
  with:
    target-dir: '.'
    file-name: 'index.html'
    use-minified: 'true'
    use-all-in-one: 'false'
```

## Notes

`This open-source project allows modifications but please retain original author information. If removal is necessary, please contact the author to avoid losing technical support.` See [License](#license) for details.

## Contributions

We welcome contributions to code and content. Please refer to:

- [Logic Documentation](doc/logic.md)
- [Contribution Guidelines](CONTRIBUTING.md)

## License

This project is licensed under [CC BY-SA 4.0 International](https://creativecommons.org/licenses/by-sa/4.0/). For complete license information, see [`LICENSE`](LICENSE).

### Acknowledgments

#### Projects

|Name|License|Links|
|:-:|:--|:--|
|tabler-icons|[MIT License](//mit-license.org)|[Github](https://github.com/tabler/tabler-icons), [Website](https://tabler.io/icons)|
|highlightjs/highlight.js|[BSD-3-Clause license](https://github.com/highlightjs/highlight.js/raw/refs/heads/main/LICENSE)|[Github](https://github.com/highlightjs/highlight.js), [Website](https://highlightjs.org)|
|markedjs/marked|[marked License](https://github.com/markedjs/marked/blob/master/LICENSE.md)|[Github](https://github.com/markedjs/marked), [Website](https://marked.js.org)|
|mermaidjs/mermaid|[MIT License](https://github.com/mermaid-js/mermaid/blob/develop/LICENSE)|[Github](https://github.com/mermaid-js/mermaid), [Website](https://mermaid.js.org)|
|translate.js|[Apache License Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)|[Github](https://github.com/xnx3/translate), [Gitee](https://gitee.com/mail_osc/translate), [Website](https://translate.zvo.cn)|
|darkmode.js|[MIT license](//mit-license.org)|[Github](https://github.com/sandoche/Darkmode.js), [Website](https://darkmodejs.learn.uno)|
|MoOx/pjax|[MIT license](//mit-license.org)|[Github](https://github.com/MoOx/pjax)|
|instant.page|[MIT license](//mit-license.org)|[Github](https://github.com/instantpage/instant.page), [Website](https://instant.page)|
|Maple Mono|[OFL-1.1 license](https://openfontlicense.org/open-font-license-official-text/)|[Github](https://github.com/subframe7536/maple-font), [Website](https://font.subf.dev)|

#### Services

|Icon|Name|
|:-:|:--|
|-|[360 Frontend CDN](https://cdn.baomitu.com/)|
|![ByteDance Static CDN](https://cdn.bytedance.com/src/res/logo.svg)|[ByteDance Static CDN](https://cdn.bytedance.com/)|
|![Chinese Web Font Project](https://chinese-font.netlify.app/favicon.ico)|[Chinese Web Font Project](https://chinese-font.netlify.app)|

## Stars

[![Stargazers over time](https://starchart.cc/PJ-568/MARKDOWN.HTML.svg?variant=adaptive)](https://starchart.cc/PJ-568/MARKDOWN.HTML)
