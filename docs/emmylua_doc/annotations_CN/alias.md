# @alias - 类型别名

定义类型别名，用于创建自定义类型、枚举类型或简化复杂类型表达式。

## 语法

```lua
-- 简单别名
---@alias <别名> <类型表达式>

-- 枚举别名
---@alias <别名>
---| '<值1>' [# 描述1]
---| '<值2>' [# 描述2]
---| ...

-- 泛型别名
---@alias <别名><<泛型参数列表>> <类型表达式>
```

## 示例

```lua
-- 简单类型别名
---@alias ID number
---@alias UserName string

-- 联合类型别名
---@alias StringOrNumber string | number
---@alias MaybeString string | nil

-- 枚举类型别名
---@alias HTTPMethod
---| 'GET'     # HTTP GET 请求
---| 'POST'    # HTTP POST 请求
---| 'PUT'     # HTTP PUT 请求
---| 'DELETE'  # HTTP DELETE 请求
---| 'PATCH'   # HTTP PATCH 请求

-- 状态枚举
---@alias TaskStatus
---| 'pending'   # 等待执行
---| 'running'   # 正在执行
---| 'completed' # 执行完成
---| 'failed'    # 执行失败

-- 泛型别名
---@alias Result<T, E> {success: boolean, data: T, error: E}
---@alias Array<T> T[]
---@alias Dictionary<K, V> table<K, V>

-- 复杂函数类型别名
---@alias EventHandler fun(event: string, ...): boolean
---@alias AsyncCallback<T> fun(error: string?, result: T?): nil

-- 使用示例
---@type HTTPMethod
local method = 'GET'

---@type Result<string, string>
local result = {success = true, data = "Hello", error = nil}

---@param status TaskStatus
function updateTaskStatus(status)
    print("Task status:", status)
end

updateTaskStatus('running')
```

## 使用场景

1. **简化复杂类型表达式**
2. **创建枚举类型**
3. **定义通用的数据结构**
4. **提高代码可读性和维护性**
