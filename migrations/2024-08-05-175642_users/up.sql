-- Your SQL goes here
create table users (
    id uuid primary key ,
    email varchar(256) unique not null ,
    password varchar(2048) not null ,
    first_name varchar(128) not null , 
    last_name varchar(128) not null ,
    bio varchar(128)  ,
    dp_url varchar( 2048 ) 
) ;