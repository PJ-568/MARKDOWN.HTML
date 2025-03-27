# MARKDOWN.HTML

> [简体中文](README.md) | English

This project aims to provide a low-dependency, simple deployment solution for rendering Markdown web pages.

## Features

- **Low dependencies**: Only an HTML file, a small number of CDNs, and server programs are required to complete Markdown pages rendering.
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
  uses: PJ-568/MARKDOWN.HTML@v1.10.1
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

### Additional Licenses

|Name|License|Address|
|:-:|:--|:--|
|tabler-icons|[MIT License](//mit-license.org)|[https://github.com/tabler/tabler-icons](https://github.com/tabler/tabler-icons)|
|markedjs/marked|[marked License](https://github.com/markedjs/marked/blob/master/LICENSE.md)|[Github](https://github.com/markedjs/marked)|
|mermaidjs/mermaid|[MIT License](https://github.com/mermaid-js/mermaid/blob/develop/LICENSE)|[Github](https://github.com/mermaid-js/mermaid)|
|translate.js|[Apache License Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)|[Github](https://github.com/xnx3/translate)、[Gitee](https://gitee.com/mail_osc/translate)|
|darkmode.js|[MIT license](//mit-license.org)|[Github](https://github.com/sandoche/Darkmode.js)|
|MoOx/pjax|[MIT license](//mit-license.org)|[Github](https://github.com/MoOx/pjax)|
|instant.page|[MIT license](//mit-license.org)|[Github](https://github.com/instantpage/instant.page)|
