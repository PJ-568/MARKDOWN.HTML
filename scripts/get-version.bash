#!/usr/bin/env bash
# 从 ../index.html 中获取版本号 | Get version number from ../index.html

# 设置定量 | Quantities
## 当前脚本所在目录 | Current Script Directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
## 仓库目录 | Repository Directory
REPO_DIR="$(dirname "$SCRIPT_DIR")"
## 文件路径 | File Path
FILE_PATH="$REPO_DIR/index.html"
## 当前语言 | Current Language
CURRENT_LANG=0 ### 0: en-US, 1: zh-Hans-CN

# 语言检测 | Language Detection
if [ $(echo ${LANG/_/-} | grep -Ei "\\b(zh|cn)\\b") ]; then CURRENT_LANG=1;  fi

# 本地化 | Localization
recho() {
  if [ $CURRENT_LANG == 1 ]; then
    ## zh-Hans-CN
    echo $1;
  else
    ## en-US
    echo $2;
  fi
}

# 提取版本号并打印 | Extract version number and print
if [ ! -f "$FILE_PATH" ]; then
  recho "错误：文件 $FILE_PATH 不存在" "Error: File $FILE_PATH does not exist"
  exit 1
fi

## 从 <markdown-html> 标签中提取第一个 version 属性的值
VERSION=$(grep -oP '<markdown-html\s+version="\K[^"]+' "$FILE_PATH" | head -1)
if [ -z "$VERSION" ]; then
  recho "错误：未找到版本号" "Error: Version not found"
  exit 1
fi

echo "$VERSION"