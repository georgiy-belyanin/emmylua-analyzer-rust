# @field - 字段定义

为类定义字段，支持访问控制、可选性和索引签名。

## 语法

```lua
-- 具名字段
---@field [<访问控制>] <字段名>[?] <类型> [描述]

-- 索引签名字段
---@field [<访问控制>] [<键类型>] <值类型> [描述]
```

## 访问控制修饰符

- `public` - 公共字段（默认）
- `private` - 私有字段（仅类内部访问）
- `protected` - 受保护字段（类及子类可访问）
- `package` - 包内字段（同包内可访问）

## 示例

```lua
-- 基础字段定义
---@class User
---@field id number 用户ID
---@field name string 用户名
---@field email string 邮箱地址
---@field createdAt string 创建时间

-- 可选字段（使用 ? 标记）
---@class UserProfile
---@field avatar? string 头像URL（可选）
---@field bio? string 个人简介（可选）
---@field phone? string 电话号码（可选）

-- 访问控制示例
---@class BankAccount
---@field public accountNumber string 账户号码
---@field public balance number 账户余额
---@field private pin string PIN码
---@field protected accountType string 账户类型
---@field package internalId number 内部ID

-- 索引签名字段
---@class Configuration
---@field host string 主机地址
---@field port number 端口号
---@field [string] any 其他配置项（任意字符串键）

---@class ScoreBoard
---@field [string] number 学生姓名到分数的映射

---@class GenericContainer<T>
---@field [number] T 数组索引访问

-- 复杂字段类型
---@class APIResponse
---@field success boolean 请求是否成功
---@field data table | nil 响应数据
---@field error string | nil 错误信息
---@field meta {page: number, limit: number, total: number} 元数据

-- 函数类型字段
---@class EventEmitter
---@field listeners table<string, fun(...)> 事件监听器映射
---@field emit fun(self: EventEmitter, event: string, ...): boolean 发射事件
---@field on fun(self: EventEmitter, event: string, listener: fun(...)): nil 注册监听器

-- 嵌套类字段
---@class Address
---@field street string 街道
---@field city string 城市
---@field zipCode string 邮编

---@class Company
---@field name string 公司名称
---@field headquarters Address 总部地址
---@field branches Address[] 分部地址列表

-- 使用示例
---@type User
local user = {
    id = 1001,
    name = "张三",
    email = "zhangsan@example.com",
    createdAt = "2024-01-01"
}

---@type Configuration
local config = {
    host = "localhost",
    port = 8080,
    database = "myapp",  -- 通过索引签名支持
    cache = true         -- 通过索引签名支持
}

---@type ScoreBoard
local scores = {
    ["张三"] = 95,
    ["李四"] = 87,
    ["王五"] = 92
}
```

## 特性

1. **可选字段支持**
2. **访问控制**
3. **索引签名**
4. **复杂类型支持**
5. **嵌套结构**
