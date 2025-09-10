# @overload - 函数重载

为函数定义多个调用签名，支持不同的参数组合。

## 语法

```lua
---@overload fun(<参数列表>): <返回值>
```

## 示例

```lua
-- 基础重载示例
---@overload fun(x: number, y: number): number
---@overload fun(x: string, y: string): string
---@param x number | string
---@param y number | string
---@return number | string
function add(x, y)
    if type(x) == "number" and type(y) == "number" then
        return x + y
    elseif type(x) == "string" and type(y) == "string" then
        return x .. y
    else
        error("参数类型不匹配")
    end
end

-- 可选参数重载
---@overload fun(name: string): User
---@overload fun(name: string, age: number): User
---@overload fun(name: string, age: number, email: string): User
---@param name string 用户名
---@param age? number 年龄（可选）
---@param email? string 邮箱（可选）
---@return User 用户对象
function createUser(name, age, email)
    return {
        name = name,
        age = age or 18,
        email = email or ""
    }
end

-- 不同返回类型的重载
---@overload fun(input: string): {type: "string", value: string}
---@overload fun(input: number): {type: "number", value: number}
---@overload fun(input: boolean): {type: "boolean", value: boolean}
---@param input any
---@return table
function wrapValue(input)
    local inputType = type(input)
    return {
        type = inputType,
        value = input
    }
end

-- HTTP请求重载
---@class HTTPOptions
---@field timeout? number 超时时间
---@field headers? table<string, string> 请求头
---@field retries? number 重试次数

---@overload fun(url: string): string
---@overload fun(url: string, options: HTTPOptions): string
---@overload fun(url: string, method: string): string
---@overload fun(url: string, method: string, options: HTTPOptions): string
---@param url string 请求URL
---@param methodOrOptions? string | HTTPOptions 方法或选项
---@param options? HTTPOptions 选项
---@return string 响应内容
function httpRequest(url, methodOrOptions, options)
    local method = "GET"
    local opts = {}

    if type(methodOrOptions) == "string" then
        method = methodOrOptions
        opts = options or {}
    elseif type(methodOrOptions) == "table" then
        opts = methodOrOptions
    end

    -- 实际HTTP请求逻辑
    return string.format("Response from %s %s", method, url)
end

-- 日志记录重载
---@overload fun(message: string)
---@overload fun(level: string, message: string)
---@overload fun(level: string, message: string, ...): nil
---@param levelOrMessage string 日志级别或消息
---@param message? string 日志消息
---@param ... any 格式化参数
function log(levelOrMessage, message, ...)
    local level, msg

    if message then
        level = levelOrMessage
        msg = string.format(message, ...)
    else
        level = "INFO"
        msg = levelOrMessage
    end

    print(string.format("[%s] %s", level, msg))
end

-- 数据库查询重载
---@class QueryOptions
---@field limit? number 限制数量
---@field offset? number 偏移量
---@field orderBy? string 排序字段

---@overload fun(sql: string): table[]
---@overload fun(sql: string, params: any[]): table[]
---@overload fun(sql: string, options: QueryOptions): table[]
---@overload fun(sql: string, params: any[], options: QueryOptions): table[]
---@param sql string SQL语句
---@param paramsOrOptions? any[] | QueryOptions 参数或选项
---@param options? QueryOptions 查询选项
---@return table[] 查询结果
function query(sql, paramsOrOptions, options)
    local params = {}
    local opts = {}

    if paramsOrOptions then
        if type(paramsOrOptions[1]) ~= nil and type(paramsOrOptions.limit) == nil then
            -- 是参数数组
            params = paramsOrOptions
            opts = options or {}
        else
            -- 是选项对象
            opts = paramsOrOptions
        end
    end

    -- 模拟查询逻辑
    return {{id = 1, name = "结果1"}, {id = 2, name = "结果2"}}
end

-- 事件监听器重载
---@class EventListener
local EventListener = {}

---@overload fun(self: EventListener, event: string, callback: fun())
---@overload fun(self: EventListener, event: string, callback: fun(), once: boolean)
---@overload fun(self: EventListener, events: table<string, fun()>)
---@param eventOrEvents string | table<string, fun()> 事件名或事件映射
---@param callback? fun() 回调函数
---@param once? boolean 是否只执行一次
function EventListener:on(eventOrEvents, callback, once)
    if type(eventOrEvents) == "string" then
        -- 单个事件
        self.listeners = self.listeners or {}
        self.listeners[eventOrEvents] = self.listeners[eventOrEvents] or {}
        table.insert(self.listeners[eventOrEvents], {
            callback = callback,
            once = once or false
        })
    elseif type(eventOrEvents) == "table" then
        -- 多个事件
        for event, cb in pairs(eventOrEvents) do
            self:on(event, cb, false)
        end
    end
end

-- 使用示例
local result1 = add(10, 20)           -- 30 (number)
local result2 = add("Hello", "World")  -- "HelloWorld" (string)

local user1 = createUser("张三")                              -- age=18, email=""
local user2 = createUser("李四", 25)                          -- email=""
local user3 = createUser("王五", 30, "wangwu@example.com")    -- 完整信息

local wrapped1 = wrapValue("hello")    -- {type: "string", value: "hello"}
local wrapped2 = wrapValue(42)         -- {type: "number", value: 42}

-- HTTP请求示例
local response1 = httpRequest("https://api.example.com")
local response2 = httpRequest("https://api.example.com", "POST")
local response3 = httpRequest("https://api.example.com", {timeout = 30})

-- 日志示例
log("系统启动")                        -- [INFO] 系统启动
log("ERROR", "连接失败")               -- [ERROR] 连接失败
log("DEBUG", "用户%s登录", "张三")      -- [DEBUG] 用户张三登录

-- 数据库查询示例
local results1 = query("SELECT * FROM users")
local results2 = query("SELECT * FROM users WHERE id = ?", {1})
local results3 = query("SELECT * FROM users", {limit = 10, offset = 0})
```

## 特性

1. **多种参数组合**
2. **不同返回类型**
3. **可选参数处理**
4. **类型安全检查**
5. **复杂重载模式**
