CREATE TABLE users (
    id          SERIAL      NOT NULL    PRIMARY KEY                 ,
    user_id     UUID        NOT NULL    DEFAULT gen_random_uuid()   ,
    first_name  TEXT        NOT NULL                                ,
    last_name   TEXT        NOT NULL                                ,
    email       TEXT        NOT NULL                                ,
    password    TEXT        NOT NULL                                ,
    created_at  TIMESTAMP   NOT NULL                                ,
    updated_at  TIMESTAMP   NOT NULL                                ,
    UNIQUE (email)
);