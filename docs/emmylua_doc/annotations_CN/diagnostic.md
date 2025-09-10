# @diagnostic - 诊断控制

控制代码诊断和静态分析警告的显示。

## 语法

```lua
---@diagnostic <状态>:<诊断名>[, <诊断名>...]
```

## 状态选项

- `disable-next-line` - 禁用下一行的诊断
- `disable-line` - 禁用当前行的诊断
- `disable` - 禁用文件中的诊断
- `enable` - 启用文件中的诊断

## 示例

```lua
-- 禁用下一行的未定义全局变量警告
---@diagnostic disable-next-line: undefined-global
local value = SOME_GLOBAL_CONSTANT

-- 禁用当前行的类型检查
local mixedVar = "string" ---@diagnostic disable-line: assign-type-mismatch

-- 禁用整个文件的特定诊断
---@diagnostic disable: unused-local, unused-vararg

-- 临时禁用多个诊断
---@diagnostic disable: undefined-global, lowercase-global
_G.GLOBAL_CONFIG = {}
_G.debug_mode = true
---@diagnostic enable: undefined-global, lowercase-global

-- 代码段诊断控制
function processData(data)
    ---@diagnostic disable: need-check-nil
    local result = data.value.nested.property  -- 可能为nil但暂时忽略检查
    ---@diagnostic enable: need-check-nil

    return result
end

-- 外部库兼容性
---@diagnostic disable: undefined-field
local socket = require("socket")
socket.http.request("http://example.com")  -- socket.http可能未定义
---@diagnostic enable: undefined-field

-- 元编程相关诊断控制
---@diagnostic disable: inject-field
local MyClass = {}
MyClass.dynamicMethod = function(self)  -- 动态添加方法
    return "dynamic"
end
---@diagnostic enable: inject-field

-- 类型转换诊断控制
---@diagnostic disable: cast-local-type
local stringValue = "123"
stringValue = tonumber(stringValue)  -- 类型从string变为number
---@diagnostic enable: cast-local-type

-- 使用场景：第三方库适配
---@diagnostic disable: undefined-global, need-check-nil
if _G.THIRD_PARTY_LIB then
    THIRD_PARTY_LIB.configure({
        option1 = true,
        option2 = "value"
    })
end
---@diagnostic enable: undefined-global, need-check-nil
```

## 特性

1. **行级控制**
2. **文件级控制**
3. **多诊断同时控制**
4. **临时启用/禁用**
5. **第三方库兼容**
