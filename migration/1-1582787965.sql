create unique index if not exists service_app_secret_index on service_app (secret);

alter table service_app
  add column url json not null default '{}';