# @operator - 操作符重载

为类定义操作符行为。

## 语法

```lua
---@operator <操作符>[(参数类型)]:<返回类型>
```

## 示例

```lua
---@class Vector
---@field x number
---@field y number
local Vector = {}

---@operator add(Vector): Vector
---@operator sub(Vector): Vector
---@operator mul(number): Vector
---@operator unm: Vector
---@operator len: number
function Vector:__add(other)
    return Vector.new(self.x + other.x, self.y + other.y)
end

function Vector:__sub(other)
    return Vector.new(self.x - other.x, self.y - other.y)
end

function Vector:__mul(scalar)
    return Vector.new(self.x * scalar, self.y * scalar)
end

function Vector:__unm()
    return Vector.new(-self.x, -self.y)
end

function Vector:__len()
    return math.sqrt(self.x * self.x + self.y * self.y)
end

-- 复数类示例
---@class Complex
---@field real number
---@field imag number
local Complex = {}

---@operator add(Complex): Complex
---@operator sub(Complex): Complex
---@operator mul(Complex): Complex
---@operator div(Complex): Complex
---@operator eq(Complex): boolean
---@operator tostring: string

function Complex:__add(other)
    return Complex.new(self.real + other.real, self.imag + other.imag)
end

function Complex:__mul(other)
    return Complex.new(
        self.real * other.real - self.imag * other.imag,
        self.real * other.imag + self.imag * other.real
    )
end

function Complex:__eq(other)
    return self.real == other.real and self.imag == other.imag
end

function Complex:__tostring()
    return string.format("%.2f + %.2fi", self.real, self.imag)
end

-- 矩阵类示例
---@class Matrix
---@field data number[][]
local Matrix = {}

---@operator add(Matrix): Matrix
---@operator mul(Matrix): Matrix
---@operator mul(number): Matrix

function Matrix:__add(other)
    -- 矩阵加法实现
end

function Matrix:__mul(other)
    if type(other) == "number" then
        -- 标量乘法
    else
        -- 矩阵乘法
    end
end
```

## 特性

1. **算术操作符**
2. **比较操作符**
3. **一元操作符**
4. **元方法支持**
5. **类型安全**
