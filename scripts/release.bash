#!/usr/bin/env bash
# 根据参数生成版本号，并推送标签到远程仓库 | Automatically generate version number and push tags to remote repository

# 设置定量 | Quantities
## 当前脚本所在目录 | Current Script Directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
## 仓库目录 | Repository Directory
REPO_DIR="$(dirname "$SCRIPT_DIR")"
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

# 参数解析 | Argument Parsing
version=""
while [[ $# -gt 0 ]]; do
  case "$1" in
    -v|--version)
      version="$2"
      shift 2
      ;;
    *)
      recho "错误: 未知参数 $1" "Error: Unknown argument $1"
      exit 1
      ;;
  esac
done

# 版本号验证函数 | Version Validation Function
validate_version() {
  if [[ ! "$1" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    recho "错误: 版本号格式不正确" "Error: Invalid version format"
    exit 1
  fi
}

# 如果未通过参数指定版本号，则交互式输入 | Interactive input if version not specified
if [ -z "$version" ]; then
  while true; do
    recho "请输入版本号 (格式: X.X.X): " "Please enter version number (format: X.X.X): "
    read -r version
    validate_version "$version" && break
  done
else
  validate_version "$version"
fi

# 切换到仓库目录 | Change to repository directory
recho "切换到仓库目录..." "Changing to repository directory..."
cd "$REPO_DIR" || {
  recho "错误: 无法切换到仓库目录" "Error: Failed to change to repository directory"
  exit 1
}

# 创建并推送标签 | Create and push tag
recho "创建标签 v$version..." "Creating tag v$version..."
git tag "v$version" || {
  recho "错误: 创建标签失败" "Error: Failed to create tag"
  exit 1
}

recho "推送标签到远程仓库..." "Pushing tag to remote repository..."
git push origin "v$version" || {
  recho "错误: 推送标签失败" "Error: Failed to push tag"
  exit 1
}

recho "版本 $version 发布成功！" "Version $version released successfully!"