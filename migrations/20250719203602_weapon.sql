CREATE TABLE weapon (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    weapon_category VARCHAR(255) NOT NULL, -- melee, ranged, magic, etc.
    weapon_type VARCHAR(255) NOT NULL, -- sword, axe, bow, etc
    level INT NOT NULL, -- minimum level avaiable
    durability INT NOT NULL, -- if -1 = infinite
    attack_speed DECIMAL(10, 2) NOT NULL, -- attack speed in seconds
    rarity VARCHAR(255),
    crafting_exp INT NOT NULL,
    description TEXT,
    sell_price INT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

create table weapon_damage_range (
    id SERIAL PRIMARY KEY,
    weapon_id INT NOT NULL REFERENCES weapon(id) ON DELETE CASCADE,
    level INT NOT NULL,
    damage_min INT NOT NULL,
    damage_max INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(weapon_id, level)  -- Assurer qu'un outil n'a qu'une plage de dégâts par niveau
);

create table weapon_effect
(
    id SERIAL PRIMARY KEY,
    weapon_id INT NOT NULL REFERENCES weapon(id) ON DELETE CASCADE,
    level INT NOT NULL,
    effect_type VARCHAR(255) NOT NULL,
    effect_value INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);