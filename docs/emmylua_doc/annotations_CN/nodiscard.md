# @nodiscard - 不可忽略返回值

标记函数的返回值不应被忽略。

## 语法

```lua
---@nodiscard
```

## 示例

```lua
---@nodiscard
---@return boolean 操作是否成功
function criticalOperation()
    -- 执行关键操作
    return true
end

-- 正确使用
local success = criticalOperation()
if not success then
    error("关键操作失败")
end

-- 错误使用（会产生警告）
-- criticalOperation()  -- 忽略返回值会产生警告

-- 资源管理示例
---@nodiscard
---@param filepath string 文件路径
---@return File? 文件句柄
function openFile(filepath)
    local file = io.open(filepath, "r")
    return file
end

-- 正确使用
local file = openFile("config.txt")
if file then
    local content = file:read("*a")
    file:close()
    print(content)
end

-- 数据库连接示例
---@nodiscard
---@param connectionString string 连接字符串
---@return Connection? 数据库连接
function connectDatabase(connectionString)
    -- 建立数据库连接
    return createConnection(connectionString)
end

-- 正确使用
local conn = connectDatabase("localhost:5432/mydb")
if conn then
    local result = conn:query("SELECT * FROM users")
    conn:close()
end

-- 错误处理示例
---@nodiscard
---@param data table 要验证的数据
---@return boolean success 是否验证成功
---@return string? error 错误信息
function validateData(data)
    if not data.name then
        return false, "缺少name字段"
    end
    if not data.email then
        return false, "缺少email字段"
    end
    return true, nil
end

-- 正确使用
local valid, errorMsg = validateData({name = "张三"})
if not valid then
    print("验证失败:", errorMsg)
    return
end

-- 内存分配示例
---@nodiscard
---@param size number 缓冲区大小
---@return Buffer? 缓冲区对象
function allocateBuffer(size)
    if size <= 0 then
        return nil
    end
    return createBuffer(size)
end

-- 正确使用
local buffer = allocateBuffer(1024)
if buffer then
    buffer:write("Hello, World!")
    buffer:flush()
    buffer:free()  -- 释放资源
end
```

## 特性

1. **强制返回值检查**
2. **资源管理保护**
3. **错误处理提醒**
4. **API安全性**
5. **代码质量保障**
