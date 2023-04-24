create table db_users(
  id varchar not null,
  power bigint not null default 0,
);

create unique index user_id on db_users (id);
