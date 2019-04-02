#[macro_use]
extern crate mysql;
use mysql as my;

mod o5;

/*
fn do_transaction(user1: &o5::BankUser, user2 &o5::BankUser, antall: i32, pool: &my::Pool) ->Result<f64,MyError> {

    Ok();

}
*/

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

    setup_transaction(&pool);
    set_auto_commit(false);
        let user1 = &mut init_users[0];
        let res1 = user1.trekk(40, &pool).unwrap();
        let user2 = &mut init_users[1];
        let res2 = user2.set_saldo(user2.saldo() + 40, &pool).unwrap();

        if res1 == 1 || res2  == 1 {
            commit();
        } else {
            rollback();
        }
    set_auto_commit(true);


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

/* 
    pub fn set_saldo(&mut self, saldo: i32, pool: &my::Pool) {
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

    pub fn trekk(&mut self, antall: i32, pool: &my::Pool) {
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

*/







fn set_auto_commit(bol: bool) {

}
fn setup_transaction(pool: &my::Pool){

}

fn rollback() {
    print!("rollbacked")
}

fn commit() {
    println!("commited");
}
