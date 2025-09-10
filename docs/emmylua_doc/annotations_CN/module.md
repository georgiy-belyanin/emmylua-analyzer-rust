# @module - 模块声明

声明或引用模块，用于模块间的类型传递。

## 语法

```lua
---@module '<模块名>'
```

## 示例

```lua
-- 引用现有模块
---@module 'socket'
local socket = require('socket')

---@module 'json'
local json = require('json')

-- 复杂模块引用
---@module 'http.client'
local httpClient = require('http.client')

---@module 'database.mysql'
local mysql = require('database.mysql')

-- 条件模块加载
---@module 'logger'
local logger = nil
if _G.ENABLE_LOGGING then
    logger = require('logger')
end

-- 模块别名
---@module 'very.long.module.name'
local shortName = require('very.long.module.name')

-- 动态模块加载
---@param moduleName string 模块名
---@return table 模块对象
function loadModule(moduleName)
    ---@module 'dynamic'
    return require(moduleName)
end

-- 模块工厂
---@module 'config'
local configModule = require('config')

---@param environment string 环境名称
---@return table 配置对象
function createConfig(environment)
    return configModule.load(environment)
end

-- 插件系统
---@class PluginManager
local PluginManager = {}

---@param pluginName string 插件名称
---@return table? 插件对象
function PluginManager:loadPlugin(pluginName)
    local modulePath = "plugins." .. pluginName
    ---@module 'plugins.dynamic'
    local success, plugin = pcall(require, modulePath)
    return success and plugin or nil
end

-- 使用示例
local client = httpClient.new()
local response = client:get("http://example.com")

local connection = mysql.connect({
    host = "localhost",
    database = "myapp",
    user = "root",
    password = "password"
})

if logger then
    logger.info("应用启动")
end

-- 模块类型定义
---@module 'utils'
local utils = require('utils')

-- 使用模块中的类型
---@type utils.StringUtils
local stringUtils = utils.strings

---@type utils.MathUtils
local mathUtils = utils.math
```

## 特性

1. **模块引用**
2. **动态加载**
3. **条件加载**
4. **插件系统**
5. **类型传递**
