create table db_users(
  id varchar not null,
  power int not null default 0,
);

create unique index user_id on dc_users (id);
