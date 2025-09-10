# @see - 引用其他符号

引用相关的类、函数或文档。

## 语法

```lua
---@see <符号名>
```

## 示例

```lua
-- 引用相关函数
---@see User
---@see createUser
---@param userData table 用户数据
function validateUser(userData)
    -- 验证用户数据
end

-- 引用相关类
---@see DatabaseConnection
---@see QueryBuilder
---@param sql string SQL语句
---@return table[] 查询结果
function executeQuery(sql)
    -- 执行SQL查询
end

-- 多重引用
---@see Logger.info
---@see Logger.error
---@see Logger.debug
---@param level string 日志级别
---@param message string 日志消息
function writeLog(level, message)
    -- 写入日志
end

-- 引用外部文档
---@see https://lua.org/manual/5.4/manual.html#6.4
---@param pattern string 正则表达式
---@param string string 目标字符串
---@return string[] 匹配结果
function findMatches(pattern, string)
    -- 查找匹配项
end
```

## 特性

1. **符号引用**
2. **文档链接**
3. **API关联**
4. **代码导航**
5. **文档生成**
