# 外部格式化工具选项

emmyLua_ls支持使用外部格式化工具来格式化 Lua 代码。通过配置 `.emmyrc.json` 文件，你可以集成任何支持命令行的代码格式化工具。

## 配置格式

在 `.emmyrc.json` 文件中，你可以配置外部格式化工具：

文档格式化配置:
```json
{
  "format" : {
      "externalTool": {
          "program": "stylua",
          "args": [
              "-",
              "--stdin-filepath",
              "${file}",
          ],
          "timeout": 5000
      }
  }
}

```
范围格式化配置:
```json
{
    "format" : {
        "externalToolRangeFormat": {
            "program": "stylua",
            "args": [
                "-",
                "--stdin-filepath",
                "${file}",
                "--indent-width=${indent_size}",
                "--indent-type",
                "${use_tabs?Tabs:Spaces}",
                "--range-start=${start_offset}",
                "--range-end=${end_offset}"
            ],
            "timeout": 5000
        }
    }
}
```


## 配置项说明

- **program**: 外部格式化工具的可执行文件路径
- **args**: 传递给格式化工具的参数列表
- **timeout**: 格式化操作的超时时间（毫秒），默认值为 5000ms

## 变量替换

在 `args` 参数中，你可以使用以下变量，它们会在运行时被实际值替换：

### 简单变量

| 变量 | 描述 | 示例值 |
|------|------|--------|
| `${file}` | 当前文件的完整路径 | `/path/to/script.lua` |
| `${indent_size}` | 缩进大小（空格数） | `4` |
| `${start_offset}` | 选定范围的起始byte偏移量 | `0` |
| `${end_offset}` | 选定范围的结束byte偏移量 | `100` |
| `${start_line}` | 当前文件的起始行 | `1` |
| `${end_line}` | 当前文件的结束行 | `10` |

### 条件变量

条件变量使用 `${variable?true_value:false_value}` 的格式，根据条件返回不同的值：

| 变量 | 描述 | true 时返回 | false 时返回 |
|------|------|-------------|--------------|
| `${use_tabs?--use-tabs:--use-spaces}` | 是否使用制表符缩进 | `--use-tabs` | `--use-spaces` |
| `${insert_final_newline?--final-newline:}` | 是否在文件末尾插入换行符 | `--final-newline` | 空字符串 |
| `${non_standard_symbol?--allow-non-standard}` | 是否允许非标准符号 | `--allow-non-standard` | 空字符串 |

## 变量语法说明

### 基本语法
- `${variable}` - 简单变量替换
- `${variable?value}` - 条件变量，当条件为真时返回 value，否则返回空字符串
- `${variable?true_value:false_value}` - 条件变量，根据条件返回不同的值

### 特殊处理
- 如果条件变量的结果为空字符串，该参数将不会传递给外部工具
- 变量名区分大小写
- 未知的变量将保持原样不被替换

## 配置示例

### 使用 Stylua 格式化器

```json
{
    "format" : {
        "externalTool": {
            "program": "stylua",
            "args": [
                "-",
                "--stdin-filepath",
                "${file}",
                "--indent-width=${indent_size}",
                "--indent-type",
                "${use_tabs?Tabs:Spaces}"
            ]
        },
        "externalToolRangeFormat": {
            "program": "stylua",
            "args": [
                "-",
                "--stdin-filepath",
                "${file}",
                "--indent-width=${indent_size}",
                "--indent-type",
                "${use_tabs?Tabs:Spaces}",
                "--range-start=${start_offset}",
                "--range-end=${end_offset}"
            ],
            "timeout": 5000
        }
    }
}
```

## 工作流程

1. 当用户触发代码格式化时，EmmyLua 分析器会读取配置的外部工具设置
2. 解析 `args` 中的变量，将它们替换为实际值
3. 启动外部格式化工具，并将当前代码通过 stdin 传递给它
4. 等待外部工具完成处理，读取格式化后的代码
5. 如果格式化成功，将结果应用到编辑器中

## 错误处理

- 如果外部工具不存在或无法执行，会记录错误日志
- 如果格式化过程超时，会终止进程并记录超时错误
- 如果外部工具返回非零退出码，会记录错误信息
- 如果输出不是有效的 UTF-8 文本，会记录编码错误
