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

    // Get all bank_users with saldo > 300
    let mut selected_users: Vec<o5::BankUser> = pool
        .prep_exec(
            "SELECT kontonummer, saldo, eier from bank_user WHERE saldo > 300",
            (),
        )
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    let (kontonummer, saldo, eier) = my::from_row(row);
                    o5::BankUser {
                        kontonummer: kontonummer,
                        saldo: saldo,
                        eier: eier,
                    }
                })
                .collect() // Collect payments so now `QueryResult` is mapped to `Vec<Payment>`
        })
        .unwrap(); // Unwrap `Vec<Payment>`
    println!("Users with saldo >300:\n{:?}", selected_users);

    let user = &mut selected_users[0];
    user.set_eier(String::from("Sander"), &pool);

    // Get all bank_users with saldo > 300
    let selected_user: Vec<o5::BankUser> = pool
        .prep_exec(
            "SELECT kontonummer, saldo, eier from bank_user WHERE kontonummer = 1",
            (),
        )
        .map(|result| {
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    let (kontonummer, saldo, eier) = my::from_row(row);
                    o5::BankUser {
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
