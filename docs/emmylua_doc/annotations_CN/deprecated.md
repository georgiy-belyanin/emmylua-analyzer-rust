# @deprecated - 弃用标记

标记函数、类或字段为已弃用，提醒开发者使用替代方案。

## 语法

```lua
---@deprecated [替代方案说明]
```

## 示例

```lua
-- 简单弃用标记
---@deprecated
function oldFunction()
    print("这是一个旧的函数")
end

-- 带替代方案说明的弃用
---@deprecated 请使用 newCalculateSum 函数
---@param numbers number[] 数字数组
---@return number 总和
function calculateSum(numbers)
    local sum = 0
    for _, num in ipairs(numbers) do
        sum = sum + num
    end
    return sum
end

-- 新的替代函数
---@param numbers number[] 数字数组
---@return number 总和
function newCalculateSum(numbers)
    return table.reduce(numbers, function(acc, num) return acc + num end, 0)
end

-- 弃用的类
---@deprecated 请使用 ModernUser 类替代
---@class OldUser
---@field id number
---@field name string
local OldUser = {}

-- 新的替代类
---@class ModernUser
---@field id number
---@field name string
---@field email string
---@field createdAt string
local ModernUser = {}

-- 弃用的字段
---@class APIResponse
---@field success boolean
---@field data any
---@field message string
---@deprecated 请使用 errorMessage 字段
---@field error string

-- 弃用的方法
---@class FileManager
local FileManager = {}

---@deprecated 使用 readFileSync 或 readFileAsync 替代
---@param path string 文件路径
---@return string 文件内容
function FileManager:loadFile(path)
    local file = io.open(path, "r")
    if file then
        local content = file:read("*a")
        file:close()
        return content
    end
    return ""
end

---@param path string 文件路径
---@return string 文件内容
function FileManager:readFileSync(path)
    -- 新的同步读取实现
    return fs.readFileSync(path)
end

---@async
---@param path string 文件路径
---@return string 文件内容
function FileManager:readFileAsync(path)
    -- 新的异步读取实现
    return fs.readFileAsync(path)
end

-- 渐进式弃用示例
---@class DatabaseConnection
local DatabaseConnection = {}

---@deprecated 从 v2.0 开始弃用，将在 v3.0 中移除。请使用 executeQuery 替代
---@param sql string SQL语句
---@return table[] 查询结果
function DatabaseConnection:query(sql)
    print("警告: query 方法已弃用，请使用 executeQuery")
    return self:executeQuery(sql)
end

---@param sql string SQL语句
---@param params? any[] 查询参数
---@return table[] 查询结果
function DatabaseConnection:executeQuery(sql, params)
    -- 新的查询实现
    return database.execute(sql, params or {})
end

-- 配置选项的弃用
---@class ServerConfig
---@field host string 服务器地址
---@field port number 端口号
---@deprecated 请使用 connectionTimeout 替代
---@field timeout number 超时时间（毫秒）
---@field connectionTimeout number 连接超时时间（毫秒）
---@deprecated 请使用 ssl.enabled 替代
---@field useSSL boolean 是否使用SSL
---@field ssl {enabled: boolean, cert?: string, key?: string} SSL配置

-- 弃用的常量
---@deprecated 请使用 HTTP_STATUS.OK 替代
local HTTP_OK = 200

---@deprecated 请使用 HTTP_STATUS.NOT_FOUND 替代
local HTTP_NOT_FOUND = 404

-- 新的状态码枚举
---@enum HTTP_STATUS
local HTTP_STATUS = {
    OK = 200,
    CREATED = 201,
    BAD_REQUEST = 400,
    UNAUTHORIZED = 401,
    NOT_FOUND = 404,
    INTERNAL_ERROR = 500
}

-- 兼容性包装函数
---@deprecated 请直接使用 string.format
---@param template string 模板字符串
---@param ... any 格式化参数
---@return string 格式化后的字符串
function formatString(template, ...)
    return string.format(template, ...)
end

-- 使用示例和迁移指南
-- 旧代码（已弃用）:
-- local result = calculateSum({1, 2, 3, 4, 5})
-- local user = OldUser.new("张三")

-- 新代码（推荐）:
-- local result = newCalculateSum({1, 2, 3, 4, 5})
-- local user = ModernUser.new("张三", "zhangsan@example.com")

-- 处理弃用警告的工具函数
---@param message string 弃用警告信息
function emitDeprecationWarning(message)
    if _G.SHOW_DEPRECATION_WARNINGS then
        print("DEPRECATION WARNING: " .. message)
        if _G.STACK_TRACE_ON_DEPRECATION then
            print(debug.traceback())
        end
    end
end
```

## 特性

1. **简单弃用标记**
2. **替代方案说明**
3. **渐进式弃用**
4. **版本控制支持**
5. **兼容性包装**
