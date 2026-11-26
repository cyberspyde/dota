-- cyberspyde enemy intel bridge for Dota 2.
--
-- Drop this file into your Dota 2 scripts folder.
-- It reads the enemy team, enemy wards, Roshan, runes and the enemy courier
-- each game update and POSTs a JSON snapshot to the cyberspyde agent, which
-- shares it with the rest of the team.
--
-- No setup: the agent listens on 127.0.0.1:4647 by default. Just start the
-- agent, then Dota.

local URL = "http://127.0.0.1:4647/"

-- Send at most once per second; OnUpdate fires every game tick.
local SEND_EVERY = 1.0
local last_send = 0

local script = {}

-- ---------------------------------------------------------------- helpers --

local function jstr(s)
    s = tostring(s)
    s = s:gsub("\\", "\\\\")
    s = s:gsub('"', '\\"')
    return '"' .. s .. '"'
end

local function jnum(n)
    n = tonumber(n) or 0
    return string.format("%.0f", n)
end

local function hero_name(h)
    local n = Entity.GetUnitName(h)
    if type(n) == "string" then
        return n:gsub("^npc_dota_hero_", "")
    end
    return ""
end

local function pos_of(u)
    local p = Entity.GetAbsOrigin(u)
    if p == nil then
        return 0, 0
    end
    return p.x or 0, p.y or 0
end

-- ------------------------------------------------------------- snapshots --

local function hero_items(h)
    local out = {}
    for i = 0, 5 do
        local it = NPC.GetItemByIndex(h, i)
        if it ~= nil then
            local name = Ability.GetName(it)
            if type(name) == "string" and name ~= "" and name ~= "empty" then
                local charges = nil
                if Item.RequiresCharges(it) then
                    charges = Item.GetCurrentCharges(it)
                end
                local cd = 0
                local t = Item.GetEnableTime(it)
                if type(t) == "number" and t > GameRules.GetDOTATime() then
                    cd = math.floor(t - GameRules.GetDOTATime() + 0.5)
                end
                local s = jstr(name)
                if charges ~= nil then
                    s = s .. ', "charges": ' .. jnum(charges)
                end
                s = s .. ', "cooldown": ' .. jnum(cd)
                out[#out + 1] = "{" .. s .. "}"
            end
        end
    end
    return table.concat(out, ", ")
end

local function hero_ult(h)
    -- The ultimate is the last ability slot; -1 = not learned yet.
    local ab = NPC.GetAbilityByIndex(h, 5)
    if ab == nil then
        return -1
    end
    local level = Ability.GetLevel(ab)
    if level == nil or level <= 0 then
        return -1
    end
    -- Seconds until the ultimate can be cast again.
    local cd = Ability.GetCooldown(ab)
    if type(cd) ~= "number" or cd <= 0 then
        return 0
    end
    return math.floor(cd + 0.5)
end

local function hero_json(h)
    local x, y = pos_of(h)
    local name = hero_name(h)
    if name == "" then
        return nil
    end
    local level = NPC.GetCurrentLevel(h)
    if type(level) ~= "number" then
        level = 1
    end
    local parts = {
        '"hero": ' .. jstr(name),
        '"level": ' .. jnum(level),
        '"alive": ' .. (Hero.GetLifeState(h) and "true" or "false"),
        '"health": ' .. jnum(Entity.GetHealth(h)),
        '"max_health": ' .. jnum(Entity.GetMaxHealth(h)),
        '"x": ' .. jnum(x),
        '"y": ' .. jnum(y),
        '"items": [' .. hero_items(h) .. ']',
        '"ult_cooldown": ' .. jnum(hero_ult(h)),
        '"magic_immune": ' .. (NPC.HasState(h, Enum.ModifierState.MODIFIER_STATE_MAGIC_IMMUNE) and "true" or "false"),
        '"smoked": ' .. (NPC.HasState(h, Enum.ModifierState.MODIFIER_STATE_INVISIBLE) and "true" or "false"),
        '"stunned": ' .. (NPC.IsStunned(h) and "true" or "false"),
    }
    return "{" .. table.concat(parts, ", ") .. "}"
end

local function heroes_json()
    local mine = Entity.GetTeamNum(Heroes.GetLocal())
    local out = {}
    for _, h in ipairs(Heroes.GetAll()) do
        if Entity.GetTeamNum(h) ~= mine then
            local s = hero_json(h)
            if s then
                out[#out + 1] = s
            end
        end
    end
    return table.concat(out, ", ")
end

local function wards_json()
    local out = {}
    local n = NPCs.Count()
    for i = 0, n - 1 do
        local w = NPCs.Get(i)
        if w ~= nil and NPC.IsWard(w) then
            local x, y = pos_of(w)
            local kind = "observer"
            if Entity.GetUnitName(w) == "npc_dota_sentry_ward" then
                kind = "sentry"
            end
            out[#out + 1] = '{"kind": ' .. jstr(kind) .. ', "x": ' .. jnum(x) .. ', "y": ' .. jnum(y) .. ', "age": 0}'
        end
    end
    return table.concat(out, ", ")
end

local roshan_dead_at = nil

local function roshan_json()
    local n = NPCs.Count()
    for i = 0, n - 1 do
        local r = NPCs.Get(i)
        if r ~= nil and NPC.IsRoshan(r) then
            roshan_dead_at = nil
            local x, y = pos_of(r)
            return '{"alive": true, "health": ' .. jnum(Entity.GetHealth(r))
                .. ', "max_health": ' .. jnum(Entity.GetMaxHealth(r))
                .. ', "x": ' .. jnum(x) .. ', "y": ' .. jnum(y)
                .. ', "last_death": ' .. jnum(roshan_dead_at or 0) .. '}'
        end
    end
    if roshan_dead_at == nil then
        roshan_dead_at = GameRules.GetDOTATime()
    end
    return '{"alive": false, "health": 0, "max_health": 0, "x": 0, "y": 0, "last_death": ' .. jnum(roshan_dead_at) .. '}'
end

local function runes_json()
    local out = {}
    for _, r in ipairs(Runes.GetAll()) do
        local x, y = pos_of(r)
        local kind = "rune"
        local t = Rune.GetRuneType and Rune.GetRuneType(r)
        if t ~= nil then
            kind = tostring(t)
        end
        out[#out + 1] = '{"kind": ' .. jstr(kind) .. ', "x": ' .. jnum(x) .. ', "y": ' .. jnum(y) .. ', "available_in": 0}'
    end
    return table.concat(out, ", ")
end

local function courier_json()
    local mine = Entity.GetTeamNum(Heroes.GetLocal())
    local n = Couriers.Count()
    for i = 0, n - 1 do
        local c = Couriers.Get(i)
        if c ~= nil and NPC.IsCourier(c) and Entity.GetTeamNum(c) ~= mine then
            local x, y = pos_of(c)
            local state = "ok"
            local cs = Courier.GetCourierState and Courier.GetCourierState(c)
            if cs ~= nil then
                state = tostring(cs)
            end
            return '{"x": ' .. jnum(x) .. ', "y": ' .. jnum(y) .. ', "state": ' .. jstr(state) .. '}'
        end
    end
    return '{"x": 0, "y": 0, "state": ""}'
end

local function snapshot()
    -- Wall-clock ms since epoch, for staleness checks on the agent side.
    local now_ms = math.floor(os.time() * 1000)
    return '{'
        .. '"clock": ' .. jnum(GameRules.GetDOTATime())
        .. ', "captured_ms": ' .. jnum(now_ms)
        .. ', "heroes": [' .. heroes_json() .. ']'
        .. ', "wards": [' .. wards_json() .. ']'
        .. ', "roshan": ' .. roshan_json()
        .. ', "runes": [' .. runes_json() .. ']'
        .. ', "courier": ' .. courier_json()
        .. '}'
end

-- ------------------------------------------------------------------ send --

local function send()
    local now = GameRules.GetDOTATime()
    if now - last_send < SEND_EVERY then
        return
    end
    last_send = now
    local body = snapshot()
    HTTP.Request("POST", URL, {
        headers = { ["Content-Type"] = "application/json" },
        data = body,
    }, nil, "cyberspyde_enemy")
end

function script.OnUpdate()
    if GameRules.GetGameState() ~= Enum.GameState.DOTA_GAMERULES_STATE_GAME_IN_PROGRESS then
        return
    end
    send()
end

return script
