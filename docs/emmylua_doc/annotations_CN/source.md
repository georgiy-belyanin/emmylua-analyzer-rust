# @source - 源代码引用

指定符号的源代码位置。

## 语法

```lua
---@source <路径>
```

## 示例

```lua
-- 文件路径引用
---@source file:///src/utils/string.lua
function stringUtils() end

-- 相对路径引用
---@source ./helpers/math.lua
function mathHelpers() end

-- URL引用
---@source https://github.com/user/repo/blob/main/src/module.lua
function externalFunction() end

-- 带行号的引用
---@source file:///src/core.lua:42
function specificFunction() end
```

## 特性

1. **源码定位**
2. **文件引用**
3. **URL支持**
4. **行号定位**
5. **工具集成**
