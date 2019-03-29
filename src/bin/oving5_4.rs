use std::{thread, time};
#[macro_use]
extern crate mysql;
use mysql as my;

mod o5;

fn main() {
    let pool = my::Pool::new("mysql://snorreks:yJct1XSh@mysql.stud.iie.ntnu.no/snorreks").unwrap();

    // Get all bank_users with saldo > 300
    let mut init_users: Vec<o5::BankUser> = pool
        .prep_exec(
            "SELECT kontonummer, saldo, eier from bank_user",
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

            println!(
        "User 1 current saldo: {}",
        init_users[0].saldo()
    );
        println!(
        "User 2 current saldo: {}",
        init_users[1].saldo()
    );


   // let mut transaction = my::Transaction.query(query: T);
    
{
        let user1 = &mut init_users[0];
        user1.trekk(40, &pool);
        let user2 = &mut init_users[1];

        thread::sleep(time::Duration::from_millis(10));
        user2.set_saldo(user2.saldo() + 40,&pool);
}
    








    // Get users again
    let updated_users: Vec<o5::BankUser> = pool
        .prep_exec(
            "SELECT kontonummer, saldo, eier from bank_user WHERE kontonummer < 3",
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
        "User 1 updated saldo: {}",
        updated_users[0].saldo()
    );
        println!(
        "User 2 updated saldo: {}",
        updated_users[1].saldo()
    );
}
