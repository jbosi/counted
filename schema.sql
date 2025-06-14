-- Schéma pour une application de type Tricount

-- Active l'extension pour générer des UUIDs si nécessaire
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Table pour les groupes de dépenses
CREATE TABLE groups (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    -- Le share_token est un identifiant unique et secret pour rejoindre un groupe via une URL
    share_token UUID DEFAULT uuid_generate_v4() NOT NULL UNIQUE,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL
);

-- Table pour les utilisateurs (membres des groupes)
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    group_id INTEGER NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    -- S'assure qu'un nom d'utilisateur est unique au sein d'un même groupe
    UNIQUE(group_id, name)
);

-- Table pour les dépenses
CREATE TABLE expenses (
    id SERIAL PRIMARY KEY,
    group_id INTEGER NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
    description VARCHAR(255) NOT NULL,
    -- Utilise le type NUMERIC pour une précision financière
    amount NUMERIC(10, 2) NOT NULL,
    -- Qui a payé cette dépense
    paid_by_user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    date TIMESTAMPTZ DEFAULT NOW() NOT NULL
);

-- Table de liaison pour savoir quels utilisateurs sont concernés par une dépense
-- C'est le cœur du système de répartition.
CREATE TABLE expense_participants (
    expense_id INTEGER NOT NULL REFERENCES expenses(id) ON DELETE CASCADE,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    -- La part que cet utilisateur doit pour cette dépense.
    -- Par exemple, si une dépense de 30€ est partagée entre 3 personnes, la part de chacun est de 10€.
    share NUMERIC(10, 2) NOT NULL,
    -- La clé primaire est une combinaison de l'ID de dépense et de l'ID d'utilisateur
    PRIMARY KEY (expense_id, user_id)
);

-- Index pour accélérer les recherches courantes
CREATE INDEX idx_users_group_id ON users(group_id);
CREATE INDEX idx_expenses_group_id ON expenses(group_id);
CREATE INDEX idx_expense_participants_user_id ON expense_participants(user_id);

-- Ajout de quelques données de test pour commencer
-- 1. Créer un groupe de test
INSERT INTO groups (name) VALUES ('Vacances à la mer') RETURNING id, share_token;

-- NOTE: Pour utiliser les 'id' et 'share_token' retournés, il faudrait un script.
-- Pour cet exemple, nous allons supposer que le premier groupe a l'ID 1.

-- 2. Ajouter des membres au groupe 1
INSERT INTO users (group_id, name) VALUES
(1, 'Alice'),
(1, 'Bob'),
(1, 'Charlie');

-- 3. Ajouter une dépense payée par Alice pour tout le monde
-- Dépense : "Courses" de 90€, payée par Alice (ID utilisateur 1), pour Alice, Bob, et Charlie.
INSERT INTO expenses (group_id, description, amount, paid_by_user_id) VALUES
(1, 'Courses', 90.00, 1) RETURNING id;

-- Répartir cette dépense (ID 1) entre les 3 membres (parts égales de 30€)
INSERT INTO expense_participants (expense_id, user_id, share) VALUES
(1, 1, 30.00), -- Alice
(1, 2, 30.00), -- Bob
(1, 3, 30.00); -- Charlie

-- 4. Ajouter une autre dépense payée par Bob, mais seulement pour Bob et Charlie
-- Dépense : "Billets de train" de 50€, payée par Bob (ID 2), pour Bob et Charlie.
INSERT INTO expenses (group_id, description, amount, paid_by_user_id) VALUES
(1, 'Billets de train', 50.00, 2) RETURNING id;

-- Répartir cette dépense (ID 2) entre Bob et Charlie (parts égales de 25€)
INSERT INTO expense_participants (expense_id, user_id, share) VALUES
(2, 2, 25.00), -- Bob
(2, 3, 25.00); -- Charlie
