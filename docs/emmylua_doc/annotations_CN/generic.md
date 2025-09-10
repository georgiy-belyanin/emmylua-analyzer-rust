# @generic - 泛型定义

定义泛型参数，实现代码复用和类型安全。

## 语法

```lua
---@generic <泛型名1>[: <约束类型1>] [, <泛型名2>[: <约束类型2>]...]
```

## 示例

```lua
-- 基础泛型函数
---@generic T
---@param value T 输入值
---@return T 相同类型的输出值
function identity(value)
    return value
end

-- 使用示例
local str = identity("hello")      -- str的类型是string
local num = identity(42)           -- num的类型是number

-- 多泛型参数
---@generic K, V
---@param map table<K, V> 映射表
---@return K[] 所有键的数组
function getKeys(map)
    local keys = {}
    for k in pairs(map) do
        table.insert(keys, k)
    end
    return keys
end

---@generic K, V
---@param map table<K, V> 映射表
---@return V[] 所有值的数组
function getValues(map)
    local values = {}
    for _, v in pairs(map) do
        table.insert(values, v)
    end
    return values
end

-- 泛型约束
---@generic T : table
---@param obj T 表对象
---@return T 深拷贝的对象
function deepClone(obj)
    if type(obj) ~= "table" then
        return obj
    end

    local copy = {}
    for k, v in pairs(obj) do
        copy[k] = deepClone(v)
    end
    return copy
end

-- 数组操作泛型
---@generic T
---@param array T[] 输入数组
---@param predicate fun(item: T): boolean 过滤条件
---@return T[] 过滤后的数组
function filter(array, predicate)
    local result = {}
    for _, item in ipairs(array) do
        if predicate(item) then
            table.insert(result, item)
        end
    end
    return result
end

---@generic T, R
---@param array T[] 输入数组
---@param mapper fun(item: T): R 映射函数
---@return R[] 映射后的数组
function map(array, mapper)
    local result = {}
    for _, item in ipairs(array) do
        table.insert(result, mapper(item))
    end
    return result
end

---@generic T, R
---@param array T[] 输入数组
---@param reducer fun(acc: R, item: T): R 归约函数
---@param initialValue R 初始值
---@return R 归约结果
function reduce(array, reducer, initialValue)
    local accumulator = initialValue
    for _, item in ipairs(array) do
        accumulator = reducer(accumulator, item)
    end
    return accumulator
end

-- 泛型类定义
---@class List<T>
---@field private items T[] 存储的项目
local List = {}

---@generic T
---@return List<T>
function List.new()
    return setmetatable({items = {}}, {__index = List})
end

---@param item T
function List:add(item)
    table.insert(self.items, item)
end

---@param index number
---@return T?
function List:get(index)
    return self.items[index]
end

---@return number
function List:size()
    return #self.items
end

---@generic R
---@param mapper fun(item: T): R
---@return List<R>
function List:map(mapper)
    local result = List.new()
    for _, item in ipairs(self.items) do
        result:add(mapper(item))
    end
    return result
end

-- 泛型工厂函数
---@generic T
---@param constructor fun(): T 构造函数
---@return fun(): T 工厂函数
function createFactory(constructor)
    return function()
        return constructor()
    end
end

-- Promise类型泛型
---@class Promise<T>
---@field private value T?
---@field private state 'pending' | 'resolved' | 'rejected'
local Promise = {}

---@generic T
---@param executor fun(resolve: fun(value: T), reject: fun(reason: any))
---@return Promise<T>
function Promise.new(executor)
    local promise = setmetatable({
        value = nil,
        state = 'pending'
    }, {__index = Promise})

    local function resolve(value)
        if promise.state == 'pending' then
            promise.value = value
            promise.state = 'resolved'
        end
    end

    local function reject(reason)
        if promise.state == 'pending' then
            promise.value = reason
            promise.state = 'rejected'
        end
    end

    executor(resolve, reject)
    return promise
end

---@generic R
---@param onResolve fun(value: T): R
---@return Promise<R>
function Promise:then(onResolve)
    if self.state == 'resolved' then
        return Promise.new(function(resolve)
            resolve(onResolve(self.value))
        end)
    end
    -- 简化实现，实际应该处理异步情况
    return Promise.new(function() end)
end

-- 高阶函数泛型
---@generic T
---@param fn fun(...): T 要记忆化的函数
---@return fun(...): T 记忆化的函数
function memoize(fn)
    local cache = {}
    return function(...)
        local key = table.concat({...}, ",")
        if cache[key] == nil then
            cache[key] = fn(...)
        end
        return cache[key]
    end
end

-- 类型安全的泛型容器
---@class Container<T>
---@field private data T[]
---@field private maxSize number
local Container = {}

---@generic T
---@param maxSize number
---@return Container<T>
function Container.new(maxSize)
    return setmetatable({
        data = {},
        maxSize = maxSize or math.huge
    }, {__index = Container})
end

---@param item T
---@return boolean 是否添加成功
function Container:push(item)
    if #self.data < self.maxSize then
        table.insert(self.data, item)
        return true
    end
    return false
end

---@return T? 弹出的项目
function Container:pop()
    return table.remove(self.data)
end

---@return T? 首个项目
function Container:peek()
    return self.data[#self.data]
end

-- 使用示例
local numbers = {1, 2, 3, 4, 5}
local strings = {"a", "b", "c"}

-- 过滤偶数
local evenNumbers = filter(numbers, function(n) return n % 2 == 0 end)  -- {2, 4}

-- 字符串转换为大写
local upperStrings = map(strings, function(s) return s:upper() end)      -- {"A", "B", "C"}

-- 计算数组总和
local sum = reduce(numbers, function(acc, n) return acc + n end, 0)      -- 15

-- 创建字符串列表
---@type List<string>
local stringList = List.new()
stringList:add("hello")
stringList:add("world")

-- 映射为长度列表
local lengthList = stringList:map(function(s) return #s end)             -- List<number>

-- 创建用户容器
---@type Container<{name: string, age: number}>
local userContainer = Container.new(100)
userContainer:push({name = "张三", age = 25})
userContainer:push({name = "李四", age = 30})

local user = userContainer:pop()  -- {name: "李四", age: 30}
```

## 特性

1. **基础泛型**
2. **多泛型参数**
3. **泛型约束**
4. **泛型类**
5. **高阶函数泛型**
