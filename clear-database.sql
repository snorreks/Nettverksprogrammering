DROP DATABASE snorreks;
CREATE DATABASE snorreks;
CREATE TABLE bank_user
(
    kontonummer int not null,
    saldo int not null,
    eier text
);