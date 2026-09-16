return {
    prefix = "data_",

    add = function(key, value)
        if not string.match(value, "^%d%d%d%d%-%d%d%-%d%d$") then
            return {
                ok = false,
                error = "data deve estar no formato aaaa-mm-dd"
            }
        end

        local year = tonumber(string.sub(value, 1, 4))
        local month = tonumber(string.sub(value, 6, 7))
        local day = tonumber(string.sub(value, 9, 10))

        if month < 1 or month > 12 then
            return {
                ok = false,
                error = "data inválida"
            }
        end

        local days = {
            [1] = 31,
            [2] = 28,
            [3] = 31,
            [4] = 30,
            [5] = 31,
            [6] = 30,
            [7] = 31,
            [8] = 31,
            [9] = 30,
            [10] = 31,
            [11] = 30,
            [12] = 31
        }

        local max_day = days[month]

        local leap =
            (year % 400 == 0) or
            ((year % 4 == 0) and (year % 100 ~= 0))

        if month == 2 and leap then
            max_day = 29
        end

        if day < 1 or day > max_day then
            return {
                ok = false,
                error = "data inválida"
            }
        end

        return {
            ok = true,
            value = value
        }
    end,

    get = function(key, value)
        local formatted =
            string.sub(value, 9, 10) .. "/" ..
            string.sub(value, 6, 7) .. "/" ..
            string.sub(value, 1, 4)

        return {
            ok = true,
            value = formatted
        }
    end
}