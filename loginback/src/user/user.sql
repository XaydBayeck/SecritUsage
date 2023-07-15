drop table if exists users;

create table users (
  id integer primary key autoincrement,
  name text not null,
  email char(50) not null,
  password char(20) not null
);
