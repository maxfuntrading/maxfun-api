CREATE TABLE evt_txn_log
(
    block_number BIGINT,
    txn_index    BIGINT,
    log_index    BIGINT,
    block_time   BIGINT,
    txn_hash     CHAR(66),
    address      CHAR(42),
    topic_0      VARCHAR(255),
    topic_1      VARCHAR(255),
    topic_2      VARCHAR(255),
    data         TEXT,
    PRIMARY KEY (block_number, txn_index, log_index)
);
CREATE INDEX idx_evt_txn_log_address ON evt_txn_log (address);


CREATE TABLE evt_token_log
(
    block_number  BIGINT,
    txn_index     BIGINT,
    log_index     BIGINT,
    block_time    BIGINT,
    txn_hash      CHAR(66),
    user_address  CHAR(42),
    token_address VARCHAR(255),
    PRIMARY KEY (block_number, txn_index, log_index)
);
CREATE INDEX idx_evt_token_log_user_address ON evt_token_log (user_address);
CREATE INDEX idx_evt_token_log_token_address ON evt_token_log (token_address);

CREATE TABLE token_summary
(
    token_address  CHAR(42) PRIMARY KEY,
    raised_token   CHAR(42),
    price          DECIMAL(40, 18),
    price_rate24h  DECIMAL(3, 2),
    volume_rate24h DECIMAL(3, 2),
    liquidity      DECIMAL(40, 18),
    total_supply   DECIMAL(40, 18),
    market_cap     DECIMAL(40, 18),
    uniswap_pool   VARCHAR(255),
    last_trade_ts  BIGINT,
);

CREATE TABLE evt_trade_log
(
    block_number  BIGINT,
    txn_index     BIGINT,
    log_index     BIGINT,
    block_time    BIGINT,
    txn_hash      CHAR(66),
    token_address CHAR(42),
    user_address  CHAR(42),
    trace_type    INT, -- 0 buy, 1 sell
    token0        CHAR(42),
    amount0       DECIMAL(40, 18),
    token1        CHAR(42),
    amount1       DECIMAL(40, 18),
    price         TEXT,
    PRIMARY KEY (block_number, txn_index, log_index)
);
CREATE INDEX idx_evt_trade_log_token_address ON evt_trade_log (token_address);
CREATE INDEX idx_evt_trade_log_user_address ON evt_trade_log (user_address);

CREATE TABLE evt_transfer_log
(
    block_number  BIGINT,
    txn_index     BIGINT,
    log_index     BIGINT,
    block_time    BIGINT,
    txn_hash      CHAR(66),
    token_address CHAR(42),
    from_address  CHAR(42),
    to_address    VARCHAR(255),
    amount        DECIMAL(40, 18),
    PRIMARY KEY (block_number, txn_index, log_index)
);
CREATE INDEX idx_evt_transfer_log_token_address ON evt_transfer_log (token_address);

CREATE TABLE evt_balance_log
(
    block_number  BIGINT,
    txn_index     BIGINT,
    log_index     BIGINT,
    user_address  CHAR(42),
    token_address CHAR(42),
    block_time    BIGINT,
    txn_hash      CHAR(66),
    delta_amount  DECIMAL(40, 18),
    total_amount  DECIMAL(40, 18),
    PRIMARY KEY (block_number, txn_index, log_index, user_address)
);

CREATE INDEX idx_evt_balance_log_token_address ON evt_balance_log (token_address);

CREATE TABLE user_summary
(
    user_address  CHAR(42),
    token_address CHAR(42),
    amount        DECIMAL(40, 18),
    PRIMARY KEY (user_address, token_address)
);

CREATE TABLE kline_5m
(
    token_address CHAR(42),
    open_ts       BIGINT,
    close_ts      BIGINT,
    open          DECIMAL(40, 18),
    high          DECIMAL(40, 18),
    low           DECIMAL(40, 18),
    close         DECIMAL(40, 18),
    volume        DECIMAL(40, 18),
    amount        DECIMAL(40, 18),
    txn_num       BIGINT,
    PRIMARY KEY (token_address, open_ts)
);