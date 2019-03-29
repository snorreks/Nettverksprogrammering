// cargo install diesel_cli --no-default-features --features mysql
//https://cdn.mysql.com/archives/mysql-connector-c/mysql-connector-c-6.1.11-winx64.msi
//host: mysql.stud.iie.ntnu.no
//user: snorreks
//password: yJct1XSh
#[macro_use]
extern crate mysql;
use mysql as my;

mod o5;

fn main() {
    let pool = my::Pool::new("mysql://snorreks:yJct1XSh@mysql.stud.iie.ntnu.no/snorreks").unwrap();

    pool.prep_exec(r"DROP TABLE IF EXISTS bank_user", ()).unwrap();

    pool.prep_exec(r"CREATE TABLE bank_user(
        kontonummer int not null,
        saldo int not null,
        eier text
        )", ()).unwrap();

    let mut bank_users = vec![
        o5::BankUser {
            kontonummer: 1,
            saldo: 200,
            eier: String::from("Per"),
        },
        o5::BankUser {
            kontonummer: 2,
            saldo: 400,
            eier: String::from("Bob"),
        },
        o5::BankUser {
            kontonummer: 3,
            saldo: 600,
            eier: String::from("Karl"),
        },
        o5::BankUser {
            kontonummer: 4,
            saldo: 20,
            eier: String::from("Martin"),
        },
        o5::BankUser {
            kontonummer: 5,
            saldo: 1000,
            eier: String::from("Sander"),
        },
    ];

    // Setter inn i databasen:
    for mut stmt in pool
        .prepare(
            r"INSERT INTO bank_user (kontonummer, saldo, eier) VALUES (:kontonummer, :saldo, :eier)",
        )
        .into_iter()
    {
        for p in bank_users.iter() {
            stmt.execute(params! {
                "kontonummer" => p.kontonummer(),
                "saldo" => p.saldo(),
                "eier" => p.eier(),
            })
            .unwrap(); //unwrap for error sjekking
        }
    }

    println!("init database");
}