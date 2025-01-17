CREATE TABLE "user"
(
    address   CHAR(42) PRIMARY KEY,
    name      VARCHAR(255),
    avatar    VARCHAR(255),
    create_ts BIGINT
);

CREATE TABLE tag_info
(
    name      VARCHAR(255) PRIMARY KEY,
    sort      INT,
    create_ts BIGINT
);

CREATE TABLE raised_token
(
    address   CHAR(42) PRIMARY KEY,
    name      VARCHAR(100),
    symbol    VARCHAR(100),
    decimal   INT,
    icon      VARCHAR(255),
    price     DECIMAL(40, 18),
    create_ts BIGINT
);
INSERT INTO raised_token (address, name, symbol, decimal, icon, price,create_ts)
VALUES ('0x0f2D719407FdBeFF09D87557AbB7232601FD9F29', -- MAX token address
        'Maxity',
        'MAX',
        18,
        '/icon/cf228750-d8b8-43f6-aa89-965930820bf7.png', -- MAX token icon URL
        '1',
        EXTRACT(EPOCH FROM NOW())),
       ('0xdAC17F958D2ee523a2206206994597C13D831ec7', -- USDT token address
        'Tether USD',
        'USDT',
        6,
        '/icon/cf228750-d8b8-43f6-aa89-965930820bf7.png', -- USDT token icon URL
           '1',
        EXTRACT(EPOCH FROM NOW()));

CREATE TABLE token_info
(
    id             SERIAL PRIMARY KEY,
    token_address  CHAR(42) NOT NULL,
    user_address   CHAR(42) NOT NULL,
    name           VARCHAR(100) NOT NULL,
    icon           VARCHAR(255) NOT NULL,
    symbol         VARCHAR(100) NOT NULL,
    description    VARCHAR(1024) NOT NULL,
    tag            VARCHAR(100),
    website        VARCHAR(255),
    twitter        VARCHAR(255),
    telegram       VARCHAR(255),
    total_supply   DECIMAL(40, 18),
    raised_token   CHAR(42) NOT NULL,
    raised_amount  DECIMAL(40, 18),
    sale_ratio     DECIMAL(5, 2),
    reserved_ratio DECIMAL(5, 2),
    pool_ratio     DECIMAL(5, 2),
    launch_ts      BIGINT,
    create_ts      BIGINT,
    is_launched    BOOLEAN
);

CREATE UNIQUE INDEX idx_token_address_unique ON token_info (token_address);

CREATE TABLE token_comment
(
    id            SERIAL PRIMARY KEY,
    token_address CHAR(42) NOT NULL,
    user_address  CHAR(42) NOT NULL,
    comment       VARCHAR(255),
    create_ts     BIGINT
);

CREATE INDEX idx_token_address ON token_comment (token_address);
CREATE INDEX idx_user_address ON token_comment (user_address);

CREATE TABLE user_avatar
(
    id        SERIAL PRIMARY KEY,
    avatar    VARCHAR(255),
    create_ts BIGINT
);

INSERT INTO tag_info (name, sort, create_ts)
VALUES ('Ai', 1, EXTRACT(EPOCH FROM NOW()));
INSERT INTO tag_info (name, sort, create_ts)
VALUES ('Game', 2, EXTRACT(EPOCH FROM NOW()));
INSERT INTO tag_info (name, sort, create_ts)
VALUES ('Defi', 3, EXTRACT(EPOCH FROM NOW()));
INSERT INTO tag_info (name, sort, create_ts)
VALUES ('De-Sci', 4, EXTRACT(EPOCH FROM NOW()));
INSERT INTO tag_info (name, sort, create_ts)
VALUES ('Social', 5, EXTRACT(EPOCH FROM NOW()));
INSERT INTO tag_info (name, sort, create_ts)
VALUES ('Depin', 6, EXTRACT(EPOCH FROM NOW()));
INSERT INTO tag_info (name, sort, create_ts)
VALUES ('Others', 7, EXTRACT(EPOCH FROM NOW()));

INSERT INTO user_avatar (avatar, create_ts)
VALUES ('/avatar/1.svg', EXTRACT(EPOCH FROM NOW()));
INSERT INTO user_avatar (avatar, create_ts)
VALUES ('/avatar/2.svg', EXTRACT(EPOCH FROM NOW()));
INSERT INTO user_avatar (avatar, create_ts)
VALUES ('/avatar/3.svg', EXTRACT(EPOCH FROM NOW()));
INSERT INTO user_avatar (avatar, create_ts)
VALUES ('/avatar/4.svg', EXTRACT(EPOCH FROM NOW()));
INSERT INTO user_avatar (avatar, create_ts)
VALUES ('/avatar/5.svg', EXTRACT(EPOCH FROM NOW()));
INSERT INTO user_avatar (avatar, create_ts)
VALUES ('/avatar/6.svg', EXTRACT(EPOCH FROM NOW()));
INSERT INTO user_avatar (avatar, create_ts)
VALUES ('/avatar/7.svg', EXTRACT(EPOCH FROM NOW()));
INSERT INTO user_avatar (avatar, create_ts)
VALUES ('/avatar/8.svg', EXTRACT(EPOCH FROM NOW()));
INSERT INTO user_avatar (avatar, create_ts)
VALUES ('/avatar/9.svg', EXTRACT(EPOCH FROM NOW()));
INSERT INTO user_avatar (avatar, create_ts)
VALUES ('/avatar/10.svg', EXTRACT(EPOCH FROM NOW()));
INSERT INTO user_avatar (avatar, create_ts)
VALUES ('/avatar/11.svg', EXTRACT(EPOCH FROM NOW()));
INSERT INTO user_avatar (avatar, create_ts)
VALUES ('/avatar/12.svg', EXTRACT(EPOCH FROM NOW()));
INSERT INTO user_avatar (avatar, create_ts)
VALUES ('/avatar/13.svg', EXTRACT(EPOCH FROM NOW()));
INSERT INTO user_avatar (avatar, create_ts)
VALUES ('/avatar/14.svg', EXTRACT(EPOCH FROM NOW()));
INSERT INTO user_avatar (avatar, create_ts)
VALUES ('/avatar/15.svg', EXTRACT(EPOCH FROM NOW()));
INSERT INTO user_avatar (avatar, create_ts)
VALUES ('/avatar/16.svg', EXTRACT(EPOCH FROM NOW()));
INSERT INTO user_avatar (avatar, create_ts)
VALUES ('/avatar/17.svg', EXTRACT(EPOCH FROM NOW()));
INSERT INTO user_avatar (avatar, create_ts)
VALUES ('/avatar/18.svg', EXTRACT(EPOCH FROM NOW()));
INSERT INTO user_avatar (avatar, create_ts)
VALUES ('/avatar/19.svg', EXTRACT(EPOCH FROM NOW()));
INSERT INTO user_avatar (avatar, create_ts)
VALUES ('/avatar/20.svg', EXTRACT(EPOCH FROM NOW()));