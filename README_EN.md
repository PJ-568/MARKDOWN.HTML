![MARKDOWN.HTML](data:image/svg+xml;charset=utf-8;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSI1MTIiIGhlaWdodD0iNTEyIiB2aWV3Qm94PSIwIDAgNTEyIDUxMiI+CiAgPCEtLSDog4zmma/mlrnlnZfku6PooajmlofmoaMgLS0+CiAgPHJlY3QgeD0iNjQiIHk9IjY0IiB3aWR0aD0iMzg0IiBoZWlnaHQ9IjM4NCIgcng9IjI0IiBmaWxsPSIjMkU3RDMyIiBzdHJva2U9IiMxQjVFMjAiIHN0cm9rZS13aWR0aD0iOCIvPgogIAogIDwhLS0g5paH5qGj5YaF5a6557q/5p2hIC0tPgogIDxyZWN0IHg9Ijk2IiB5PSIxMjgiIHdpZHRoPSIzMjAiIGhlaWdodD0iMTYiIHJ4PSI0IiBmaWxsPSIjRThGNUU5Ii8+CiAgPHJlY3QgeD0iOTYiIHk9IjE2OCIgd2lkdGg9IjI4MCIgaGVpZ2h0PSIxNiIgcng9IjQiIGZpbGw9IiNFOEY1RTkiLz4KICA8cmVjdCB4PSI5NiIgeT0iMjA4IiB3aWR0aD0iMjQwIiBoZWlnaHQ9IjE2IiByeD0iNCIgZmlsbD0iI0U4RjVFOSIvPgogIDxyZWN0IHg9Ijk2IiB5PSIyNDgiIHdpZHRoPSIzMjAiIGhlaWdodD0iMTYiIHJ4PSI0IiBmaWxsPSIjRThGNUU5Ii8+CiAgPHJlY3QgeD0iOTYiIHk9IjI4OCIgd2lkdGg9IjI4MCIgaGVpZ2h0PSIxNiIgcng9IjQiIGZpbGw9IiNFOEY1RTkiLz4KICA8cmVjdCB4PSI5NiIgeT0iMzI4IiB3aWR0aD0iMjQwIiBoZWlnaHQ9IjE2IiByeD0iNCIgZmlsbD0iI0U4RjVFOSIvPgogIAogIDwhLS0g6aG555uu5ZCN56ew57yp5YaZIC0tPgogIDx0ZXh0IHg9IjI1NiIgeT0iNDIwIiBmb250LWZhbWlseT0iU2FyYXNhIFRlcm0gU0MgTmVyZCcsICdNYXBsZSBNb25vIENOJywgVWJ1bnR1LCBSb2JvdG8sICdPcGVuIFNhbnMnLCAnTWljcm9zb2Z0IFlhSGVpJywgQXJpYWwsIHNhbnMtc2VyaWYiIGZvbnQtc2l6ZT0iNDgiIGZvbnQtd2VpZ2h0PSJib2xkIiB0ZXh0LWFuY2hvcj0ibWlkZGxlIiBmaWxsPSIjRThGNUU5Ij4mbHQ7TUgmZ3Q7PC90ZXh0Pgo8L3N2Zz4=)

# MARKDOWN.HTML

> [简体中文](README.md) | English

This project provides a low-dependency, one-click deployment solution for rendering Markdown to web pages.

## Features

- **Low Dependencies**: Only one HTML file is required to render Markdown to web pages.
- **Syntax Highlighting**: Render code blocks in Markdown.
- **Markdown Support**: Convert Markdown files to HTML.
- **Mermaid Support**: Render Mermaid diagrams in Markdown.
- **Multi-language Support**: Machine translation between multiple languages.
- **Dark Mode**: Support light, dark, and system default themes.
- **Pjax Support**: Page navigation without full reload.

## Usage

| File Name                                          | Description                                                    |
| :------------------------------------------------- | :------------------------------------------------------------- |
| [index.html](index.html)                           | Default HTML entry file using CDN dependencies                 |
| [index.min.html](index.min.html)                   | Minified HTML entry file using CDN dependencies                |
| [index.allinone.html](index.allinone.html)         | All-in-one HTML entry file with embedded dependencies          |
| [index.allinone.min.html](index.allinone.min.html) | Minified all-in-one HTML entry file with embedded dependencies |

### Manual Deployment

1. Download the required HTML file from the [Releases page](https://github.com/PJ-568/MARKDOWN.HTML/releases) and place it in your website directory;
2. When accessed, the HTML file will automatically render `index.md` in the same directory. If `index.md` doesn't exist, it will render `README.md`.

### GitHub Actions

Compile and deploy HTML files to the target directory. Replace `<version>` with the actual version number.

```yaml
- name: Deploy markdown.html
  uses: PJ-568/MARKDOWN.HTML@v<version>
  with:
    target-dir: "."
    file-name: "index.html"
    use-minified: "true"
    use-all-in-one: "false"
```

## Additional Notes

`The open-source projects used in this work may be modified, but please retain the original author information. Contact the author if removal is necessary to avoid losing technical support.` For details, see [License](#license).

## Contributing

We welcome contributions to this project. Please review:

- [Program Logic](doc/logic.md)
- [Script Design](doc/scripts.md)
- [To-do List](doc/TODO.md)
- [Workflow](doc/workflow.md)

Ensure your contributions follow project specifications. For contribution guidelines, see [CONTRIBUTING.md](CONTRIBUTING.md).

## License

This project is licensed under [CC BY-SA 4.0 International](https://creativecommons.org/licenses/by-sa/4.0/). The full license text is available in [`LICENSE`](LICENSE).

### Acknowledgements

#### Projects

| Name         | License                                                                      | Links                                                                                                                           |
| :----------- | :--------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------ |
| tabler-icons | [MIT](//mit-license.org)                                                     | [GitHub](https://github.com/tabler/tabler-icons), [Website](https://tabler.io/icons)                                            |
| highlight.js | [BSD-3-Clause](https://github.com/highlightjs/highlight.js/raw/main/LICENSE) | [GitHub](https://github.com/highlightjs/highlight.js), [Website](https://highlightjs.org)                                       |
| marked       | [marked License](https://github.com/markedjs/marked/blob/master/LICENSE.md)  | [GitHub](https://github.com/markedjs/marked), [Website](https://marked.js.org)                                                  |
| mermaid      | [MIT](https://github.com/mermaid-js/mermaid/blob/develop/LICENSE)            | [GitHub](https://github.com/mermaid-js/mermaid), [Website](https://mermaid.js.org)                                              |
| translate.js | [Apache 2.0](http://www.apache.org/licenses/LICENSE-2.0)                     | [GitHub](https://github.com/xnx3/translate), [Gitee](https://gitee.com/mail_osc/translate), [Website](https://translate.zvo.cn) |
| darkmode.js  | [MIT](//mit-license.org)                                                     | [GitHub](https://github.com/sandoche/Darkmode.js), [Website](https://darkmodejs.learn.uno)                                      |
| pjax         | [MIT](//mit-license.org)                                                     | [GitHub](https://github.com/MoOx/pjax)                                                                                          |
| instant.page | [MIT](//mit-license.org)                                                     | [GitHub](https://github.com/instantpage/instant.page), [Website](https://instant.page)                                          |
| Maple Mono   | [OFL-1.1](https://openfontlicense.org)                                       | [GitHub](https://github.com/subframe7536/maple-font), [Website](https://font.subf.dev)                                          |

#### Services

| Icon                                                           | Service                                               |
| :------------------------------------------------------------- | :---------------------------------------------------- |
| -                                                              | [360 Frontend CDN](https://cdn.baomitu.com/)          |
| ![ByteDance CDN](https://cdn.bytedance.com/src/res/logo.svg)   | [ByteDance Static CDN](https://cdn.bytedance.com/)    |
| ![Chinese Fonts](https://chinese-font.netlify.app/favicon.ico) | [Chinese Web Fonts](https://chinese-font.netlify.app) |

## Stargazers

[![Stargazers over time](https://starchart.cc/PJ-568/MARKDOWN.HTML.svg?variant=adaptive)](https://starchart.cc/PJ-568/MARKDOWN.HTML)
