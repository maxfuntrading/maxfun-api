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