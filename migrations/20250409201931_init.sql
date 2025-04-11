-- Add migration script here
create table if not exists users
(
    id uuid primary key not null,
    username text not null unique,
    password text not null
);

insert into users (id, username, password)
values ('1376cb9e-29f0-4309-a76a-65f6f6f018b2', 'ferris', '$argon2id$v=19$m=19456,t=2,p=1$VE0e3g7DalWHgDwou3nuRA$uC6TER156UQpk0lNQ5+jHM0l5poVjPA1he/Tyn9J4Zw');
