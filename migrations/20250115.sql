CREATE TABLE evt_txn_log
(
    block_number BIGINT NOT NULL DEFAULT 0,
    txn_index    BIGINT NOT NULL DEFAULT 0,
    log_index    BIGINT NOT NULL DEFAULT 0,
    block_time   BIGINT NOT NULL DEFAULT 0,
    txn_hash     CHAR(66) NOT NULL DEFAULT '',
    address      CHAR(42) NOT NULL DEFAULT '',
    topic_0      VARCHAR(255) NOT NULL DEFAULT '',
    topic_1      VARCHAR(255) NOT NULL DEFAULT '',
    topic_2      VARCHAR(255) NOT NULL DEFAULT '',
    data         TEXT NOT NULL DEFAULT '',
    PRIMARY KEY (block_number, txn_index, log_index)
);
CREATE INDEX idx_evt_txn_log_address ON evt_txn_log (address);


CREATE TABLE evt_token_log
(
    block_number  BIGINT NOT NULL DEFAULT 0,
    txn_index     BIGINT NOT NULL DEFAULT 0,
    log_index     BIGINT NOT NULL DEFAULT 0,
    block_time    BIGINT NOT NULL DEFAULT 0,
    txn_hash      CHAR(66) NOT NULL DEFAULT '',
    user_address  CHAR(42) NOT NULL DEFAULT '',
    token_address VARCHAR(255) NOT NULL DEFAULT '',
    PRIMARY KEY (block_number, txn_index, log_index)
);
CREATE INDEX idx_evt_token_log_user_address ON evt_token_log (user_address);
CREATE INDEX idx_evt_token_log_token_address ON evt_token_log (token_address);

CREATE TABLE token_summary
(
    token_address CHAR(42) PRIMARY KEY,
    raised_token  CHAR(42) NOT NULL DEFAULT '',
    price         DECIMAL(40, 18) NOT NULL DEFAULT 0,
    price_rate24h DECIMAL(3, 2) NOT NULL DEFAULT 0,
    volume_24h    DECIMAL(40, 18) NOT NULL DEFAULT 0,
    liquidity     DECIMAL(40, 18) NOT NULL DEFAULT 0,
    total_supply  DECIMAL(40, 18) NOT NULL DEFAULT 0,
    market_cap    DECIMAL(40, 18) NOT NULL DEFAULT 0,
    bonding_curve DECIMAL(4, 2) NOT NULL DEFAULT 0,
    uniswap_pool  VARCHAR(255) NOT NULL DEFAULT '',
    last_trade_ts BIGINT NOT NULL DEFAULT 0
);

CREATE TABLE evt_trade_log
(
    block_number  BIGINT NOT NULL DEFAULT 0,
    txn_index     BIGINT NOT NULL DEFAULT 0,
    log_index     BIGINT NOT NULL DEFAULT 0,
    block_time    BIGINT NOT NULL DEFAULT 0,
    txn_hash      CHAR(66) NOT NULL DEFAULT '',
    token_address CHAR(42) NOT NULL DEFAULT '',
    user_address  CHAR(42) NOT NULL DEFAULT '',
    trace_type    INT NOT NULL DEFAULT 0, -- 0 buy, 1 sell
    token0        CHAR(42) NOT NULL DEFAULT '',
    amount0       DECIMAL(40, 18) NOT NULL DEFAULT 0,
    token1        CHAR(42) NOT NULL DEFAULT '',
    amount1       DECIMAL(40, 18) NOT NULL DEFAULT 0,
    price         TEXT NOT NULL DEFAULT '',
    PRIMARY KEY (block_number, txn_index, log_index)
);
CREATE INDEX idx_evt_trade_log_token_address ON evt_trade_log (token_address);
CREATE INDEX idx_evt_trade_log_user_address ON evt_trade_log (user_address);

CREATE TABLE evt_transfer_log
(
    block_number  BIGINT NOT NULL DEFAULT 0,
    txn_index     BIGINT NOT NULL DEFAULT 0,
    log_index     BIGINT NOT NULL DEFAULT 0,
    block_time    BIGINT NOT NULL DEFAULT 0,
    txn_hash      CHAR(66) NOT NULL DEFAULT '',
    token_address CHAR(42) NOT NULL DEFAULT '',
    from_address  CHAR(42) NOT NULL DEFAULT '',
    to_address    VARCHAR(255) NOT NULL DEFAULT '',
    amount        DECIMAL(40, 18) NOT NULL DEFAULT 0,
    PRIMARY KEY (block_number, txn_index, log_index)
);
CREATE INDEX idx_evt_transfer_log_token_address ON evt_transfer_log (token_address);

CREATE TABLE evt_balance_log
(
    block_number  BIGINT NOT NULL DEFAULT 0,
    txn_index     BIGINT NOT NULL DEFAULT 0,
    log_index     BIGINT NOT NULL DEFAULT 0,
    user_address  CHAR(42) NOT NULL DEFAULT '',
    token_address CHAR(42) NOT NULL DEFAULT '',
    block_time    BIGINT NOT NULL DEFAULT 0,
    txn_hash      CHAR(66) NOT NULL DEFAULT '',
    delta_amount  DECIMAL(40, 18) NOT NULL DEFAULT 0,
    total_amount  DECIMAL(40, 18) NOT NULL DEFAULT 0,
    PRIMARY KEY (block_number, txn_index, log_index, user_address)
);

CREATE INDEX idx_evt_balance_log_token_address ON evt_balance_log (token_address);

CREATE TABLE user_summary
(
    user_address  CHAR(42) NOT NULL DEFAULT '',
    token_address CHAR(42) NOT NULL DEFAULT '',
    amount        DECIMAL(40, 18) NOT NULL DEFAULT 0,
    PRIMARY KEY (user_address, token_address)
);

CREATE TABLE kline_5m
(
    token_address CHAR(42) NOT NULL DEFAULT '',
    open_ts       BIGINT NOT NULL DEFAULT 0,
    close_ts      BIGINT NOT NULL DEFAULT 0,
    open          DECIMAL(40, 18) NOT NULL DEFAULT 0,
    high          DECIMAL(40, 18) NOT NULL DEFAULT 0,
    low           DECIMAL(40, 18) NOT NULL DEFAULT 0,
    close         DECIMAL(40, 18) NOT NULL DEFAULT 0,
    volume        DECIMAL(40, 18) NOT NULL DEFAULT 0,
    amount        DECIMAL(40, 18) NOT NULL DEFAULT 0,
    txn_num       BIGINT NOT NULL DEFAULT 0,
    PRIMARY KEY (token_address, open_ts)
);