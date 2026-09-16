return {
    prefix = "teste_",

    add = function(key, value)
        return {
            ok = true,
            value = value
        }
    end,

    get = function(key, value)
        local existente = db.get("nome")

        if existente == nil then
            return {
                ok = false,
                error = "chave nome não encontrada"
            }
        end

        return {
            ok = true,
            value = existente
        }
    end
}