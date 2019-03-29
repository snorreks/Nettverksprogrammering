// cargo install diesel_cli --no-default-features --features mysql
//https://cdn.mysql.com/archives/mysql-connector-c/mysql-connector-c-6.1.11-winx64.msi
//host: mysql.stud.iie.ntnu.no
//user: snorreks
//password: yJct1XSh
#[macro_use]
extern crate mysql;
use mysql as my;

#[derive(Debug, PartialEq, Eq)]
struct BankUser {
    kontonummer: i32,
    saldo: i32,
    eier: String,
}

impl BankUser {
    fn eier(&self) -> &String {
        &self.eier
    }
    fn kontonummer(&self) -> &i32 {
        &self.kontonummer
    }
    fn saldo(&self) -> &i32 {
        &self.saldo
    }

    fn set_eier(&mut self, eier: String, pool: &my::Pool) {
        self.eier = eier;
        let mut stmt = pool
            .prepare(r"UPDATE bank_user SET eier = :eier WHERE kontonummer = :kontonummer")
            .unwrap();
        stmt.execute(params! {
            "kontonummer" => self.kontonummer,
            "eier" => self.eier(),
        })
        .unwrap();
    }

    fn set_saldo(&mut self, saldo: i32, pool: &my::Pool) {
        self.saldo = saldo;
        let mut stmt = pool
            .prepare(r"UPDATE bank_user SET saldo = :saldo WHERE kontonummer = :kontonummer")
            .unwrap();
        stmt.execute(params! {
            "kontonummer" => self.kontonummer,
            "saldo" => self.saldo(),
        })
        .unwrap();
    }

    fn trekk(&mut self, antall: i32, pool: &my::Pool) {
        self.saldo = self.saldo - antall;
        let mut stmt = pool
            .prepare(r"UPDATE bank_user SET saldo = :saldo WHERE kontonummer = :kontonummer")
            .unwrap();
        stmt.execute(params! {
            "kontonummer" => self.kontonummer,
            "saldo" => self.saldo(),
        })
        .unwrap();
    }
}

fn main() {
    let pool = my::Pool::new("mysql://snorreks:yJct1XSh@mysql.stud.iie.ntnu.no/snorreks").unwrap();

    let mut bank_users = vec![
        BankUser {
            kontonummer: 1,
            saldo: 200,
            eier: String::from("Per"),
        },
        BankUser {
            kontonummer: 2,
            saldo: 400,
            eier: String::from("Bob"),
        },
        BankUser {
            kontonummer: 3,
            saldo: 600,
            eier: String::from("Karl"),
        },
        BankUser {
            kontonummer: 4,
            saldo: 20,
            eier: String::from("Martin"),
        },
        BankUser {
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

    // Get all bank_users with saldo > 300
    let selected_users: Vec<BankUser> = pool
        .prep_exec(
            "SELECT kontonummer, saldo, eier from bank_user WHERE saldo > 300",
            (),
        )
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    let (kontonummer, saldo, eier) = my::from_row(row);
                    BankUser {
                        kontonummer: kontonummer,
                        saldo: saldo,
                        eier: eier,
                    }
                })
                .collect() // Collect payments so now `QueryResult` is mapped to `Vec<Payment>`
        })
        .unwrap(); // Unwrap `Vec<Payment>`
    println!("Users with saldo >300:\n{:?}", selected_users);

    let user = &mut bank_users[0];
    user.set_eier(String::from("Sander"), &pool);

    // Get all bank_users with saldo > 300
    let selected_user: Vec<BankUser> = pool
        .prep_exec(
            "SELECT kontonummer, saldo, eier from bank_user WHERE kontonummer = 1",
            (),
        )
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    let (kontonummer, saldo, eier) = my::from_row(row);
                    BankUser {
                        kontonummer: kontonummer,
                        saldo: saldo,
                        eier: eier,
                    }
                })
                .collect() // Collect payments so now `QueryResult` is mapped to `Vec<Payment>`
        })
        .unwrap(); //
    println!(
        "The first user's name is now: {:?}",
        selected_user[0].eier()
    );
}
