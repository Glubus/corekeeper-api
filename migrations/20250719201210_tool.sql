CREATE TABLE tool (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    tool_category VARCHAR(255) NOT NULL,
    tool_type VARCHAR(255) NOT NULL,
    level INT NOT NULL, -- minimum level avaiable
    durability INT NOT NULL, -- if -1 = infinite
    attack_speed FLOAT NOT NULL,
    rarity VARCHAR(255),
    crafting_exp INT NOT NULL,
    description TEXT,
    sell_price INT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE tool_damage_range (
    id SERIAL PRIMARY KEY,
    tool_id INT NOT NULL REFERENCES tool(id) ON DELETE CASCADE,
    level INT NOT NULL,  -- Niveau de l'outil
    damage_min INT NOT NULL,  -- Dégâts minimum
    damage_max INT NOT NULL,  -- Dégâts maximum
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(tool_id, level)  -- Assurer qu'un outil n'a qu'une plage de dégâts par niveau
);

create table tool_effect
(
    id SERIAL PRIMARY KEY,
    tool_id INT NOT NULL REFERENCES tool(id) ON DELETE CASCADE,
    level INT NOT NULL, -- if -1 = all level
    effect_type VARCHAR(255) NOT NULL,
    effect_value INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(tool_id, level)  -- Assurer qu'un outil n'a qu'un effecretet par niveau
);

