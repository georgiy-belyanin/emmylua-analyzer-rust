# @cast - 类型转换

强制转换变量类型，用于类型收窄或扩展。

## 语法

```lua
---@cast <变量名> [+|-]<类型>[, [+|-]<类型>...]
```

## 示例

```lua
---@param value any
function processValue(value)
    if type(value) == "string" then
        ---@cast value string
        print("字符串长度:", #value)  -- value现在被确认为string类型
    end

    if type(value) == "table" and value.id then
        ---@cast value {id: number, name?: string}
        print("ID:", value.id)       -- value现在有id字段
    end
end

-- 添加类型到联合类型
---@type string | number
local mixedValue = getValue()

if needsBoolean() then
    ---@cast mixedValue +boolean  -- 添加boolean类型
    mixedValue = true
end

-- 移除类型从联合类型
---@type string | number | nil
local maybeValue = getMaybeValue()

if maybeValue then
    ---@cast maybeValue -nil      -- 移除nil类型
    print("值:", maybeValue)      -- maybeValue现在是 string | number
end

-- 复杂类型转换
---@type table
local data = parseJSON(jsonString)

-- 转换为具体结构
---@cast data {users: {id: number, name: string}[]}
for _, user in ipairs(data.users) do
    print("用户:", user.name, "ID:", user.id)
end

-- 类型收窄示例
---@param input string | number | boolean
function handleInput(input)
    if type(input) == "string" and input:match("^%d+$") then
        ---@cast input string  -- 确认是字符串类型
        local num = tonumber(input)
        ---@cast num number    -- tonumber的结果确认为number
        print("数字:", num)
    end
end

-- 添加多个类型
---@type string
local value = "initial"

---@cast value +number, +boolean  -- 添加number和boolean类型
-- value现在是 string | number | boolean

-- 移除多个类型
---@type string | number | boolean | nil
local multiValue = getMultiValue()

---@cast multiValue -boolean, -nil  -- 移除boolean和nil类型
-- multiValue现在是 string | number
```

## 特性

1. **类型收窄**
2. **类型扩展**
3. **联合类型操作**
4. **多类型同时操作**
5. **运行时类型确认**
