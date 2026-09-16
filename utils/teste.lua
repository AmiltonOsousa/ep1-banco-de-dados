-- return {
--     prefix = "teste_",

--     add = function(key, value)
--         local existing_key = db.find_by_value(value, key)

--         if existing_key ~= nil then
--             return {
--                 ok = false,
--                 error = "valor já cadastrado na chave " .. existing_key
--             }
--         end

--         return {
--             ok = true,
--             value = value
--         }
--     end,

--     get = function(key, value)
--         return {
--             ok = true,
--             value = "TESTE: " .. value
--         }
--     end
-- }