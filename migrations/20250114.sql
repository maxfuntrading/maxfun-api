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
    create_ts BIGINT
);
INSERT INTO raised_token (address, name, symbol, decimal, icon, create_ts)
VALUES ('0x0f2D719407FdBeFF09D87557AbB7232601FD9F29', -- MAX token address
        'Maxity',
        'MAX',
        18,
        'https://example.com/max.png', -- MAX token icon URL
        EXTRACT(EPOCH FROM NOW())),
       ('0xdAC17F958D2ee523a2206206994597C13D831ec7', -- USDT token address
        'Tether USD',
        'USDT',
        6,
        'https://example.com/usdt.png', -- USDT token icon URL
        EXTRACT(EPOCH FROM NOW()));

CREATE TABLE token_info
(
    id             SERIAL PRIMARY KEY,
    token_address  CHAR(42) NOT NULL,
    user_address   CHAR(42),
    name           VARCHAR(100),
    icon           VARCHAR(255),
    symbol         VARCHAR(100),
    description    VARCHAR(1024),
    tag            VARCHAR(100),
    website        VARCHAR(255),
    twitter        VARCHAR(255),
    telegram       VARCHAR(255),
    total_supply   DECIMAL(40, 18),
    raised_token   CHAR(42),
    raised_amount  DECIMAL(40, 18),
    sale_ratio     DECIMAL(5, 2),
    reserved_ratio DECIMAL(5, 2),
    pool_ratio     DECIMAL(5, 2),
    launch_ts      BIGINT,
    maxbuy_amount  DECIMAL(40, 18),
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
VALUES ('1.svg', EXTRACT(EPOCH FROM NOW()));
INSERT INTO user_avatar (avatar, create_ts)
VALUES ('2.svg', EXTRACT(EPOCH FROM NOW()));
INSERT INTO user_avatar (avatar, create_ts)
VALUES ('3.svg', EXTRACT(EPOCH FROM NOW()));
INSERT INTO user_avatar (avatar, create_ts)
VALUES ('4.svg', EXTRACT(EPOCH FROM NOW()));
INSERT INTO user_avatar (avatar, create_ts)
VALUES ('5.svg', EXTRACT(EPOCH FROM NOW()));
INSERT INTO user_avatar (avatar, create_ts)
VALUES ('6.svg', EXTRACT(EPOCH FROM NOW()));
INSERT INTO user_avatar (avatar, create_ts)
VALUES ('7.svg', EXTRACT(EPOCH FROM NOW()));
INSERT INTO user_avatar (avatar, create_ts)
VALUES ('8.svg', EXTRACT(EPOCH FROM NOW()));
INSERT INTO user_avatar (avatar, create_ts)
VALUES ('9.svg', EXTRACT(EPOCH FROM NOW()));
INSERT INTO user_avatar (avatar, create_ts)
VALUES ('10.svg', EXTRACT(EPOCH FROM NOW()));
INSERT INTO user_avatar (avatar, create_ts)
VALUES ('11.svg', EXTRACT(EPOCH FROM NOW()));
INSERT INTO user_avatar (avatar, create_ts)
VALUES ('12.svg', EXTRACT(EPOCH FROM NOW()));
INSERT INTO user_avatar (avatar, create_ts)
VALUES ('13.svg', EXTRACT(EPOCH FROM NOW()));
INSERT INTO user_avatar (avatar, create_ts)
VALUES ('14.svg', EXTRACT(EPOCH FROM NOW()));
INSERT INTO user_avatar (avatar, create_ts)
VALUES ('15.svg', EXTRACT(EPOCH FROM NOW()));
INSERT INTO user_avatar (avatar, create_ts)
VALUES ('16.svg', EXTRACT(EPOCH FROM NOW()));
INSERT INTO user_avatar (avatar, create_ts)
VALUES ('17.svg', EXTRACT(EPOCH FROM NOW()));
INSERT INTO user_avatar (avatar, create_ts)
VALUES ('18.svg', EXTRACT(EPOCH FROM NOW()));
INSERT INTO user_avatar (avatar, create_ts)
VALUES ('19.svg', EXTRACT(EPOCH FROM NOW()));
INSERT INTO user_avatar (avatar, create_ts)
VALUES ('20.svg', EXTRACT(EPOCH FROM NOW()));