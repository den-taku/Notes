-- print("Hello World!")
-- print('Hello World!')
-- print "Hello World!"
-- print([[Hello World!]])
-- print([[Hello
-- World!
-- こんにちは世界
-- ]])
-- test
--[[
--
hogehoge
]]
-- hoge = 10
-- foo = 10 bar = 20
-- foo, bar = 10, 20
-- print(hoge)
-- print(hoge, foo, bar)
-- x = 10
-- y = 20
-- print("x+y=" .. x + y .. ".")
-- hoge = nil
-- print("type is" .. type(hoge))
-- hoge = "Hello"
-- print("type is" .. type(hoge))
-- hoge = 22
-- print("type is" .. type(hoge))
-- hoge = true
-- print("type is" .. type(hoge))
-- print("Input: ")
-- hoge = io.read()
-- print("input is " .. hoge)
-- result = 70
-- if result >= 60 then
--     print("OK")
-- else 
--     print("NO")
-- end
-- i = 0
-- while i <= 10 do
--     print(i .. "times: Hello World!")
--     i = i + 1
-- end
--for i = 1, 10, 1 do
--    print(i .. "times: Hello")
--end
-- for i = 1, 9 do
--     for j = 1, 9 do
--         io.write( string.format("%3d", i * j))
--     end
--     io.write("\n")
-- end
-- 
-- function sum(x, y)
--     return x + y
-- end
-- 
-- hoge = 10
-- piyo = 20
-- 
-- result = sum(hoge, piyo)
-- 
-- print(hoge .. " + " .. piyo .. " = " .. result)
-- 
-- hoge = sum
-- print("hoge is " .. type(hoge))
-- print("10 + 20 is " .. hoge(10, 20))
-- 
-- function func(x)
--     local function get()
--         return x
--     end
-- 
--     local function add(value)
--         x = x + value
--     end
--     return get, add
-- end
-- 
-- firstGetValue, firstAddValue = func(10)
-- secondGetValue, secondAddValue = func(30)
-- 
-- print("first value : " .. firstGetValue())
-- print("second value : " .. secondGetValue())
-- 
-- firstAddValue(15)
-- secondAddValue(20)
-- 
-- print("first value : " .. firstGetValue())
-- print("second value : " .. secondGetValue())

function createSquare()
    return function(x)
        return x * x
    end
end

square = createSquare()
print("10 * 10 is " .. square(10))

dofile("file.lua")

hoge = testFunc()
print(hoge)
