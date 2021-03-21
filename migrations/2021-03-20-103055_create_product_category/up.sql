-- Your SQL goes here
create table product_category (
    id serial primary key,
    name varchar unique not null,
    version int not null default 0
);