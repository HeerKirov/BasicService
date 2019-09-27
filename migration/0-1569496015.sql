create schema if not exists public;

create table if not exists service_user(
  id serial not null constraint service_user_pkey primary key,
  username varchar(255) unique not null,
  password varchar(128) not null,
  name varchar(64) not null,

  cover varchar(255) null,

  is_staff boolean not null default false,

  last_login timestamp with time zone null,
  last_login_ip varchar(64) null,
  create_time timestamp with time zone not null,
  create_path varchar(16) not null,

  enable boolean not null default true,
  deleted boolean not null default false
);
create index if not exists service_user_username_index on service_user (username);

create table if not exists service_registration_code(
  id serial not null constraint service_registration_code_pkey primary key,
  code varchar(255) not null unique,
  enable boolean not null,
  deadline timestamp with time zone null,
  used_time timestamp with time zone null,
  used_user varchar(255) null,
  create_time timestamp with time zone not null
);
create index if not exists service_registration_code_code_index on service_registration_code (code);

create table if not exists service_token(
  key varchar(64) not null constraint service_token_pkey primary key,
  user_id integer not null constraint service_token_user_id_key references service_user on update cascade on delete cascade deferrable initially deferred,

  expire_time timestamp with time zone null,

  create_time timestamp with time zone not null,
  update_time timestamp with time zone not null
);
create index if not exists service_token_key_index on service_token (key);

create table if not exists service_app(
  id serial not null constraint service_app_pkey primary key,
  user_id integer not null constraint service_app_user_id_key references service_user on update cascade on delete cascade deferrable initially deferred,
  application_name varchar(32) not null,

  create_time timestamp with time zone not null
);

create table if not exists service_global_setting(
  id serial not null constraint service_global_setting_pkey primary key,
  register_mode integer not null
);