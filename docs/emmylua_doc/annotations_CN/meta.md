# @meta - 元数据文件

标记文件为元数据文件，用于类型定义而非实际执行。

## 语法

```lua
---@meta [模块名]
```

## 示例

```lua
-- 基础元数据文件
---@meta

-- 标准库扩展定义
---@class string
---@field len number 字符串长度

-- 为string类型添加方法
---@param pattern string 匹配模式
---@param replacement string 替换字符串
---@return string 替换后的字符串
function string:gsub(pattern, replacement) end

-- 全局函数定义
---@param value any 要转换的值
---@return string 字符串表示
function tostring(value) end

-- 具名元数据文件
---@meta mylib

---@class MyLibrary
---@field version string 库版本
---@field config table 配置对象
local mylib = {}

---@param options table 配置选项
---@return boolean 是否成功
function mylib.init(options) end

---@param data any 要处理的数据
---@return any 处理结果
function mylib.process(data) end

return mylib

-- 内置类型扩展
---@meta table_extensions

---@generic T
---@param array T[] 数组
---@param predicate fun(item: T): boolean 过滤函数
---@return T[] 过滤后的数组
function table.filter(array, predicate) end

---@generic T, R
---@param array T[] 输入数组
---@param mapper fun(item: T): R 映射函数
---@return R[] 映射后的数组
function table.map(array, mapper) end

---@generic T, R
---@param array T[] 输入数组
---@param reducer fun(acc: R, item: T): R 归约函数
---@param initialValue R 初始值
---@return R 归约结果
function table.reduce(array, reducer, initialValue) end

-- 游戏引擎API定义
---@meta game_engine

---@class Vector3
---@field x number X坐标
---@field y number Y坐标
---@field z number Z坐标
local Vector3 = {}

---@param x number
---@param y number
---@param z number
---@return Vector3
function Vector3.new(x, y, z) end

---@param other Vector3
---@return Vector3
function Vector3:add(other) end

---@return number 向量长度
function Vector3:magnitude() end

---@class GameObject
---@field name string 对象名称
---@field transform Transform 变换组件
---@field active boolean 是否激活
local GameObject = {}

---@param name string 对象名称
---@return GameObject
function GameObject.new(name) end

function GameObject:destroy() end

-- Web API定义
---@meta web_api

---@class Window
---@field location Location 位置对象
---@field document Document 文档对象
local window = {}

---@class Location
---@field href string 完整URL
---@field host string 主机名
---@field pathname string 路径
local location = {}

---@class Document
---@field title string 文档标题
local document = {}

---@param selector string CSS选择器
---@return Element? 找到的元素
function document:querySelector(selector) end

-- 数据库ORM定义
---@meta orm

---@class Model
---@field id number 主键ID
---@field createdAt string 创建时间
---@field updatedAt string 更新时间
local Model = {}

---@param attributes table 属性
---@return Model 模型实例
function Model.create(attributes) end

---@param id number 主键ID
---@return Model? 模型实例
function Model.find(id) end

---@param conditions table 查询条件
---@return Model[] 模型列表
function Model.where(conditions) end

function Model:save() end
function Model:delete() end

-- 测试框架定义
---@meta test_framework

---@param description string 测试描述
---@param testFunction fun() 测试函数
function describe(description, testFunction) end

---@param description string 测试描述
---@param testFunction fun() 测试函数
function it(description, testFunction) end

---@param actual any 实际值
---@return Matcher 匹配器对象
function expect(actual) end

---@class Matcher
local Matcher = {}

---@param expected any 期望值
function Matcher:toBe(expected) end

---@param expected any 期望值
function Matcher:toEqual(expected) end

function Matcher:toBeNil() end
function Matcher:toBeTruthy() end
function Matcher:toBeFalsy() end
```

## 特性

1. **类型定义专用**
2. **标准库扩展**
3. **第三方库定义**
4. **API声明**
5. **框架集成**
