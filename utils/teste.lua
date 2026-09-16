return {
    prefix = "teste_",

    add = function(key, value)
        return {
            ok = true,
            value = value
        }
    end,

    get = function(key, value)
        return {
            ok = true,
            value = "Lua: " .. value
        }
    end
}