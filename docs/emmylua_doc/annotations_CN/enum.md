# @enum - 枚举定义

将 Lua 表标记为枚举类型，提供运行时可用的枚举值。

## 语法

```lua
-- 值枚举（使用表的值）
---@enum <枚举名>

-- 键枚举（使用表的键）
---@enum (key) <枚举名>
```

## 示例

```lua
-- 基础值枚举
---@enum HTTPStatus
local HTTPStatus = {
    OK = 200,
    NOT_FOUND = 404,
    INTERNAL_ERROR = 500,
    BAD_REQUEST = 400,
    UNAUTHORIZED = 401
}

-- 字符串值枚举
---@enum LogLevel
local LogLevel = {
    DEBUG = "debug",
    INFO = "info",
    WARN = "warn",
    ERROR = "error",
    FATAL = "fatal"
}

-- 键枚举（使用表的键作为枚举值）
---@enum (key) Permission
local Permission = {
    READ = true,
    WRITE = true,
    DELETE = true,
    ADMIN = true
}

-- 混合类型枚举
---@enum TaskStatus
local TaskStatus = {
    PENDING = 0,
    RUNNING = "running",
    COMPLETED = true,
    FAILED = false
}

-- 使用枚举的函数
---@param status HTTPStatus HTTP状态码
---@return string 状态描述
function getStatusMessage(status)
    if status == HTTPStatus.OK then
        return "请求成功"
    elseif status == HTTPStatus.NOT_FOUND then
        return "资源未找到"
    elseif status == HTTPStatus.INTERNAL_ERROR then
        return "服务器内部错误"
    else
        return "未知状态"
    end
end

---@param level LogLevel 日志级别
---@param message string 日志消息
function writeLog(level, message)
    local timestamp = os.date("%Y-%m-%d %H:%M:%S")
    print(string.format("[%s] %s: %s", timestamp, level, message))
end

---@param user table 用户对象
---@param permission Permission 权限类型
---@return boolean 是否有权限
function hasPermission(user, permission)
    return user.permissions and user.permissions[permission] == true
end

-- 复杂枚举示例
---@enum DatabaseAction
local DatabaseAction = {
    -- CRUD操作
    CREATE = "create",
    READ = "read",
    UPDATE = "update",
    DELETE = "delete",

    -- 批量操作
    BATCH_INSERT = "batch_insert",
    BATCH_UPDATE = "batch_update",
    BATCH_DELETE = "batch_delete",

    -- 查询操作
    QUERY = "query",
    COUNT = "count",
    EXISTS = "exists"
}

---@enum EventType
local EventType = {
    -- 用户事件
    USER_LOGIN = "user.login",
    USER_LOGOUT = "user.logout",
    USER_REGISTER = "user.register",

    -- 系统事件
    SYSTEM_START = "system.start",
    SYSTEM_STOP = "system.stop",
    SYSTEM_ERROR = "system.error",

    -- 数据事件
    DATA_CHANGED = "data.changed",
    DATA_DELETED = "data.deleted"
}

-- 带有方法的枚举
---@enum Color
local Color = {
    RED = "#FF0000",
    GREEN = "#00FF00",
    BLUE = "#0000FF",
    BLACK = "#000000",
    WHITE = "#FFFFFF"
}

-- 为枚举添加方法
---@param color Color 颜色值
---@return number 红色分量
function Color.getRed(color)
    return tonumber(color:sub(2, 3), 16) or 0
end

---@param color Color 颜色值
---@return number 绿色分量
function Color.getGreen(color)
    return tonumber(color:sub(4, 5), 16) or 0
end

---@param color Color 颜色值
---@return number 蓝色分量
function Color.getBlue(color)
    return tonumber(color:sub(6, 7), 16) or 0
end

-- 数值枚举
---@enum Priority
local Priority = {
    LOW = 1,
    NORMAL = 2,
    HIGH = 3,
    URGENT = 4,
    CRITICAL = 5
}

---@param p1 Priority 优先级1
---@param p2 Priority 优先级2
---@return Priority 更高的优先级
function getHigherPriority(p1, p2)
    return p1 > p2 and p1 or p2
end

-- 位标志枚举
---@enum FileMode
local FileMode = {
    READ = 1,      -- 0001
    WRITE = 2,     -- 0010
    EXECUTE = 4,   -- 0100
    DELETE = 8     -- 1000
}

---@param mode1 FileMode 文件模式1
---@param mode2 FileMode 文件模式2
---@return FileMode 组合模式
function combineFileMode(mode1, mode2)
    return mode1 | mode2
end

---@param mode FileMode 文件模式
---@param permission FileMode 要检查的权限
---@return boolean 是否包含该权限
function hasFilePermission(mode, permission)
    return (mode & permission) ~= 0
end

-- 使用示例
print(getStatusMessage(HTTPStatus.OK))           -- "请求成功"
writeLog(LogLevel.ERROR, "系统错误")              -- 写入错误日志

-- 权限检查
local user = {
    permissions = {
        [Permission.READ] = true,
        [Permission.WRITE] = true
    }
}
print(hasPermission(user, Permission.READ))      -- true
print(hasPermission(user, Permission.DELETE))    -- false

-- 颜色操作
local red = Color.getRed(Color.RED)               -- 255
local green = Color.getGreen(Color.GREEN)         -- 255

-- 优先级比较
local highestPriority = getHigherPriority(Priority.LOW, Priority.HIGH)  -- Priority.HIGH

-- 文件权限组合
local readWrite = combineFileMode(FileMode.READ, FileMode.WRITE)         -- 3
print(hasFilePermission(readWrite, FileMode.READ))                      -- true
print(hasFilePermission(readWrite, FileMode.EXECUTE))                   -- false
```

## 特性

1. **值枚举**
2. **键枚举**
3. **混合类型支持**
4. **位标志枚举**
5. **带方法的枚举**
