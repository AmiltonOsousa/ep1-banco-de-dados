return {
    prefix = "cpf_",

    add = function(key, value)
        if not string.match(value, "^%d%d%d%d%d%d%d%d%d%d%d$") then
            return {
                ok = false,
                error = "CPF deve conter exatamente 11 dígitos"
            }
        end

        local first = string.sub(value, 1, 1)

        local all_equal = true

        for i = 2, 11 do
            if string.sub(value, i, i) ~= first then
                all_equal = false
                break
            end
        end

        if all_equal then
            return {
                ok = false,
                error = "CPF inválido"
            }
        end

        local sum = 0

        for i = 1, 9 do
            sum = sum + tonumber(string.sub(value, i, i)) * (11 - i)
        end

        local remainder = sum % 11
        local digit1 = 11 - remainder

        if digit1 >= 10 then
            digit1 = 0
        end

        if digit1 ~= tonumber(string.sub(value, 10, 10)) then
            return {
                ok = false,
                error = "CPF inválido"
            }
        end

        sum = 0

        for i = 1, 10 do
            sum = sum + tonumber(string.sub(value, i, i)) * (12 - i)
        end

        remainder = sum % 11
        local digit2 = 11 - remainder

        if digit2 >= 10 then
            digit2 = 0
        end

        if digit2 ~= tonumber(string.sub(value, 11, 11)) then
            return {
                ok = false,
                error = "CPF inválido"
            }
        end

        local existing_key = db.find_by_value(value, key)

        if existing_key ~= nil then
            return {
                ok = false,
                error = "CPF já cadastrado na chave " .. existing_key
            }
        end

        return {
            ok = true,
            value = value
        }
    end,

    get = function(key, value)
        local formatted =
            string.sub(value, 1, 3) .. "." ..
            string.sub(value, 4, 6) .. "." ..
            string.sub(value, 7, 9) .. "-" ..
            string.sub(value, 10, 11)

        return {
            ok = true,
            value = formatted
        }
    end
}