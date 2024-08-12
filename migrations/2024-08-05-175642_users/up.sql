-- user roles 
CREATE TYPE roles AS ENUM ('client', 'consultant', 'builder');

create table users (
    id uuid primary key ,
    email varchar(256) unique not null ,
    password varchar(2048) not null ,
    first_name varchar(128) not null , 
    last_name varchar(128) not null ,
    bio varchar(128)  ,
    dp_url varchar( 2048 ) 
);

-- create junction table with user and roles
CREATE TABLE user_roles (
    user_id UUID REFERENCES users(id),
    role roles,
    PRIMARY KEY (user_id, role)
);