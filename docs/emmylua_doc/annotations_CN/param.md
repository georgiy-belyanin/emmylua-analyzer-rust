# @param - 参数定义

为函数参数定义类型和描述信息。

## 语法

```lua
---@param <参数名>[?] <类型表达式> [描述]
```

## 参数特性

- `?` - 可选参数标记
- `...` - 可变参数标记
- 支持联合类型
- 支持泛型参数

## 示例

```lua
-- 基础参数定义
---@param name string 用户名
---@param age number 年龄
function createUser(name, age)
    return {name = name, age = age}
end

-- 可选参数（使用 ? 标记）
---@param name string 用户名
---@param age? number 年龄（可选，默认为18）
---@param email? string 邮箱（可选）
function registerUser(name, age, email)
    age = age or 18
    return {
        name = name,
        age = age,
        email = email
    }
end

-- 联合类型参数
---@param id string | number 用户ID（字符串或数字）
---@param options table | nil 配置选项（可为空）
function getUserById(id, options)
    -- 处理不同类型的ID
    local normalizedId = tostring(id)
    -- 使用配置选项
    options = options or {}
end

-- 可变参数
---@param format string 格式字符串
---@param ... any 格式化参数
function printf(format, ...)
    print(string.format(format, ...))
end

-- 函数类型参数
---@param data table 数据
---@param callback fun(result: any, error: string?): nil 回调函数
---@param onProgress? fun(percent: number): nil 进度回调（可选）
function processDataAsync(data, callback, onProgress)
    -- 异步处理数据
    if onProgress then
        onProgress(50)  -- 报告进度
    end

    -- 处理完成后调用回调
    local success, result = pcall(function()
        return processData(data)
    end)

    if success then
        callback(result, nil)
    else
        callback(nil, result)  -- result是错误信息
    end
end

-- 复杂对象参数
---@param request {method: string, url: string, headers?: table<string, string>, body?: string}
---@param options? {timeout?: number, retries?: number}
function httpRequest(request, options)
    options = options or {}
    local timeout = options.timeout or 30
    local retries = options.retries or 3

    -- 发送HTTP请求
end

-- 泛型参数
---@generic T
---@param items T[] 项目列表
---@param predicate fun(item: T): boolean 过滤谓词
---@return T[] 过滤后的列表
function filter(items, predicate)
    local result = {}
    for _, item in ipairs(items) do
        if predicate(item) then
            table.insert(result, item)
        end
    end
    return result
end

-- 方法参数（self参数）
---@class Calculator
local Calculator = {}

---@param self Calculator
---@param x number 第一个数
---@param y number 第二个数
---@return number 计算结果
function Calculator:add(x, y)
    return x + y
end

-- 或者使用冒号语法（self自动推断）
---@param x number 第一个数
---@param y number 第二个数
---@return number 计算结果
function Calculator:multiply(x, y)
    return x * y
end

-- 使用示例
local user1 = createUser("张三", 25)
local user2 = registerUser("李四")  -- age使用默认值
local user3 = registerUser("王五", 30, "wangwu@example.com")

printf("Hello %s, you are %d years old", "Alice", 25)

processDataAsync({value = 100}, function(result, error)
    if error then
        print("Error:", error)
    else
        print("Result:", result)
    end
end, function(percent)
    print("Progress:", percent .. "%")
end)

httpRequest({
    method = "GET",
    url = "https://api.example.com/users",
    headers = {
        ["Authorization"] = "Bearer token123"
    }
}, {
    timeout = 60,
    retries = 5
})
```

## 特性

1. **可选参数支持**
2. **联合类型**
3. **泛型参数**
4. **函数类型**
5. **可变参数**
