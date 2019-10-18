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
create index if not exists service_token_user_id_index on service_token (user_id);

create table if not exists service_app(
  id serial not null constraint service_app_pkey primary key,
  name varchar(32) unique not null,
  description varchar(256) not null,
  secret varchar(128) not null,

  public boolean not null default true,
  enable boolean not null default true,
  deleted boolean not null default false,

  create_time timestamp with time zone not null,
  update_time timestamp with time zone not null
);
create index if not exists service_app_name_index on service_app (name);

create table if not exists service_app_use(
  id serial not null constraint service_app_use_pkey primary key,
  user_id integer not null constraint service_app_use_user_id_key references service_user on update cascade on delete cascade deferrable initially deferred,
  app_id integer not null constraint service_app_use_app_id_key references service_app on update cascade on delete cascade deferrable initially deferred,

  last_use timestamp with time zone null,

  create_time timestamp with time zone not null,
  update_time timestamp with time zone not null
);
create index if not exists service_app_use_user_id_index on service_app_use (user_id);
create index if not exists service_app_use_app_id_index on service_app_use (app_id);

create table if not exists service_global_setting(
  id serial not null constraint service_global_setting_pkey primary key,
  register_mode varchar(8) not null,
  effective_max bigint null,
  effective_default bigint not null
);