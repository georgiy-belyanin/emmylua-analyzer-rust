# @async - 异步函数标记

标记函数为异步函数，用于异步编程提示和类型检查。

## 语法

```lua
---@async
```

## 示例

```lua
-- 基础异步函数
---@async
---@param url string 请求URL
---@return string 响应内容
function fetchData(url)
    -- 模拟异步HTTP请求
    return coroutine.wrap(function()
        print("开始请求:", url)
        -- 模拟网络延迟
        local co = coroutine.running()
        timer.setTimeout(function()
            coroutine.resume(co, "响应数据: " .. url)
        end, 1000)
        return coroutine.yield()
    end)()
end

-- 异步文件操作
---@async
---@param filepath string 文件路径
---@return string 文件内容
function readFileAsync(filepath)
    return coroutine.wrap(function()
        local file = io.open(filepath, "r")
        if not file then
            error("无法打开文件: " .. filepath)
        end

        local content = file:read("*a")
        file:close()

        -- 模拟异步读取
        coroutine.yield()
        return content
    end)()
end

---@async
---@param filepath string 文件路径
---@param content string 文件内容
---@return boolean 是否成功
function writeFileAsync(filepath, content)
    return coroutine.wrap(function()
        local file = io.open(filepath, "w")
        if not file then
            return false
        end

        file:write(content)
        file:close()

        -- 模拟异步写入
        coroutine.yield()
        return true
    end)()
end

-- 异步数据库操作
---@async
---@param sql string SQL查询语句
---@param params? any[] 查询参数
---@return table[] 查询结果
function queryDatabaseAsync(sql, params)
    return coroutine.wrap(function()
        print("执行SQL:", sql)

        -- 模拟数据库查询延迟
        local co = coroutine.running()
        timer.setTimeout(function()
            local mockResults = {
                {id = 1, name = "张三", email = "zhangsan@example.com"},
                {id = 2, name = "李四", email = "lisi@example.com"}
            }
            coroutine.resume(co, mockResults)
        end, 500)

        return coroutine.yield()
    end)()
end

-- 异步批处理
---@async
---@param items any[] 要处理的项目列表
---@param processor fun(item: any): any 处理函数
---@return any[] 处理结果
function processBatchAsync(items, processor)
    return coroutine.wrap(function()
        local results = {}

        for i, item in ipairs(items) do
            results[i] = processor(item)

            -- 每处理10个项目就让出控制权
            if i % 10 == 0 then
                coroutine.yield()
            end
        end

        return results
    end)()
end

-- Promise风格的异步函数
---@async
---@generic T
---@param executor fun(resolve: fun(value: T), reject: fun(error: string))
---@return T
function createPromiseAsync(executor)
    return coroutine.wrap(function()
        local resolved = false
        local result = nil
        local error = nil

        local function resolve(value)
            if not resolved then
                resolved = true
                result = value
            end
        end

        local function reject(err)
            if not resolved then
                resolved = true
                error = err
            end
        end

        executor(resolve, reject)

        -- 等待Promise完成
        while not resolved do
            coroutine.yield()
        end

        if error then
            error(error)
        end

        return result
    end)()
end

-- 异步重试机制
---@async
---@param operation fun(): any 要执行的操作
---@param maxRetries number 最大重试次数
---@param delay number 重试间隔（毫秒）
---@return any 操作结果
function retryAsync(operation, maxRetries, delay)
    return coroutine.wrap(function()
        local lastError = nil

        for attempt = 1, maxRetries + 1 do
            local success, result = pcall(operation)

            if success then
                return result
            else
                lastError = result

                if attempt <= maxRetries then
                    print(string.format("重试 %d/%d 失败: %s", attempt, maxRetries, result))

                    -- 等待重试间隔
                    local co = coroutine.running()
                    timer.setTimeout(function()
                        coroutine.resume(co)
                    end, delay)
                    coroutine.yield()
                end
            end
        end

        error("操作失败，已达到最大重试次数: " .. tostring(lastError))
    end)()
end

-- 异步并发控制
---@async
---@param tasks fun()[] 任务列表
---@param concurrency number 并发数
---@return any[] 结果列表
function runConcurrentAsync(tasks, concurrency)
    return coroutine.wrap(function()
        local results = {}
        local running = {}
        local completed = 0
        local taskIndex = 1

        -- 启动初始任务
        while #running < concurrency and taskIndex <= #tasks do
            local co = coroutine.create(tasks[taskIndex])
            running[taskIndex] = co
            taskIndex = taskIndex + 1
        end

        -- 等待所有任务完成
        while completed < #tasks do
            for i, co in pairs(running) do
                if coroutine.status(co) == "dead" then
                    local success, result = coroutine.resume(co)
                    results[i] = success and result or nil
                    running[i] = nil
                    completed = completed + 1

                    -- 启动新任务
                    if taskIndex <= #tasks then
                        local newCo = coroutine.create(tasks[taskIndex])
                        running[taskIndex] = newCo
                        taskIndex = taskIndex + 1
                    end
                end
            end

            coroutine.yield()  -- 让出控制权
        end

        return results
    end)()
end

-- 使用示例
---@async
function main()
    -- 异步获取数据
    local data = fetchData("https://api.example.com/users")
    print("获取到数据:", data)

    -- 异步文件操作
    local content = readFileAsync("config.txt")
    local success = writeFileAsync("output.txt", content .. "\n处理完成")

    -- 异步数据库查询
    local users = queryDatabaseAsync("SELECT * FROM users WHERE active = ?", {true})

    -- 批处理
    local numbers = {1, 2, 3, 4, 5}
    local squares = processBatchAsync(numbers, function(n) return n * n end)

    -- Promise风格
    local promiseResult = createPromiseAsync(function(resolve, reject)
        timer.setTimeout(function()
            resolve("Promise完成")
        end, 2000)
    end)

    -- 重试操作
    local retryResult = retryAsync(function()
        if math.random() > 0.7 then
            return "操作成功"
        else
            error("操作失败")
        end
    end, 3, 1000)

    print("所有异步操作完成")
end

-- 启动异步主函数
local mainCoroutine = coroutine.create(main)
coroutine.resume(mainCoroutine)
```

## 特性

1. **协程支持**
2. **异步文件操作**
3. **Promise风格**
4. **重试机制**
5. **并发控制**
