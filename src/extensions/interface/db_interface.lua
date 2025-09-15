local terminal = require("src.extensions.constants.terminal")
local colors = require("src.extensions.constants.colors")

local DATABASE_ON = true
local RESET = "\27[0m"

terminal.showBanner(colors.GREEN)

terminal.show([[
    ========================================
                MENU PRINCIPAL
    ========================================
    [1] ➜ Adicionar valor
    [2] ➜ Buscar valor
    [3] ➜ Ajuda
    ]], colors.CIAN)

while DATABASE_ON do

    terminal.show("Escolha uma opção: ", colors.YELLOW)
    local userInput = io.read()

    if userInput == "1" then

        terminal.show("Digite a chave: ", colors.BLUE)
        local addKey = io.read()

        terminal.show("Digite o valor: ", colors.BLUE)
        local addValor = io.read()

        ADD(addKey .. " " .. addValor)

        terminal.show("Valor inserido com sucesso!", colors.GREEN)

    elseif userInput == "2" then

        terminal.show("Digite a chave para buscar",colors.YELLOW)
        local key = io.read()
        local result = GET(key)
        terminal.show("Valor encontrado: " .. result, colors.GREEN)

    elseif userInput == "3" then
        terminal.show([[
        - Opção [1]: Adiciona uma nova chave/valor.
        - Opção [2]: Busca um valor pela chave.
        - Opção [3]: Mostra esta ajuda.
        ]], colors.PURPLE)

    else
        terminal.show("Opção inválida, tente novamente.", colors.RED)
    end

    terminal.show([[
    [1] ➜ Adicionar valor
    [2] ➜ Buscar valor
    [3] ➜ Ajuda
    ]], colors.YELLOW)
end