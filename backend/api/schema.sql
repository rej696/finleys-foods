create table users (
    id integer primary key autoincrement not null,
    username varchar(255) unique not null,
    password varchar(255) not null
);

