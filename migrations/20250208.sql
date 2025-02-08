alter table public.token_summary
    add price_token decimal(40, 18) default 0 not null;

alter table public.token_summary
    add pair_address char(42) default '' not null;

alter table public.token_summary
    drop column liquidity;

alter table public.evt_txn_log
    alter column topic_1 drop not null,
    alter column topic_2 drop not null,
    alter column data drop not null,
    add column topic_3 varchar(255) default '';

alter table public.evt_trade_log
    add price_token decimal(40, 18) default 0 not null;

alter table public.evt_token_log
    add pair_address char(42) default '' not null,
    add raised_address char(42) default '' not null;

alter table public.evt_token_log
    drop column user_address;

alter table public.raised_token
    add oracle char(42) default '';

comment on column public.raised_token.oracle is 'oracle price address';