return {
    prefix = "ip_",

    add = function(key, value)
        local a, b, c, d = string.match(
            value,
            "^(%d+)%.(%d+)%.(%d+)%.(%d+)$"
        )

        if a == nil then
            return {
                ok = false,
                error = "IPv4 inválido"
            }
        end

        a = tonumber(a)
        b = tonumber(b)
        c = tonumber(c)
        d = tonumber(d)

        if a > 255 or b > 255 or c > 255 or d > 255 then
            return {
                ok = false,
                error = "IPv4 inválido"
            }
        end

        return {
            ok = true,
            value = value
        }
    end,

    get = function(key, value)
        local a, b, c, d = string.match(
            value,
            "^(%d+)%.(%d+)%.(%d+)%.(%d+)$"
        )

        a = tonumber(a)
        b = tonumber(b)

        local classification = "público"

        if a == 10 then
            classification = "privado"
        elseif a == 172 and b >= 16 and b <= 31 then
            classification = "privado"
        elseif a == 192 and b == 168 then
            classification = "privado"
        end

        return {
            ok = true,
            value = value .. " (" .. classification .. ")"
        }
    end
}