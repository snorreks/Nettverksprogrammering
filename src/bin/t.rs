#[macro_use]
extern crate mysql;
// cargo install diesel_cli --no-default-features --features mysql
//https://cdn.mysql.com/archives/mysql-connector-c/mysql-connector-c-6.1.11-winx64.msi
use mysql as my;

#[derive(Debug, PartialEq, Eq)]
struct Payment {
    kontonummer: i32,
    saldo: i32,
    eier: String,
}

fn main() {
    //host: mysql.stud.iie.ntnu.no
    //user: snorreks
    //password: yJct1XSh

    let pool = my::Pool::new("mysql://snorreks:yJct1XSh@mysql.stud.iie.ntnu.no/snorreks").unwrap();

    // Let's create payment table.
    // Unwrap just to make sure no error happened.
    pool.prep_exec(
        r"DROP TABLE payment;
        CREATE TABLE payment (
                         kontonummer int not null,
                         saldo int not null,
                         eier text
                     );",
        (),
    )
    .unwrap();

    let payments = vec![
        Payment {
            kontonummer: 1,
            saldo: 2,
            eier: String::from("Per"),
        },
        Payment {
            kontonummer: 2,
            saldo: 4,
            eier: String::from("Bob"),
        },
        Payment {
            kontonummer: 3,
            saldo: 6,
            eier: String::from("Karl"),
        },
        Payment {
            kontonummer: 4,
            saldo: 8,
            eier: String::from("Martin"),
        },
        Payment {
            kontonummer: 5,
            saldo: 10,
            eier: String::from("Sander"),
        },
    ];

    // Let's insert payments to the database
    // We will use into_iter() because we do not need to map Stmt to anything else.
    // Also we assume that no error happened in `prepare`.
    for mut stmt in pool
        .prepare(
            r"INSERT INTO payment
                                       (kontonummer, saldo, eier)
                                   VALUES
                                       (:kontonummer, :saldo, :eier)",
        )
        .into_iter()
    {
        for p in payments.iter() {
            // `execute` takes ownership of `params` so we pass account name by reference.
            // Unwrap each result just to make sure no errors happened.
            stmt.execute(params! {
                "kontonummer" => p.kontonummer,
                "saldo" => p.saldo,
                "eier" => &p.eier,
            })
            .unwrap();
        }
    }

    // Let's select payments from database
    let selected_payments: Vec<Payment> = pool
        .prep_exec("SELECT kontonummer, saldo, eier from payment", ())
        .map(|result| {
            // In this closure we will map `QueryResult` to `Vec<Payment>`
            // `QueryResult` is iterator over `MyResult<row, err>` so first call to `map`
            // will map each `MyResult` to contained `row` (no proper error handling)
            // and second call to `map` will map each `row` to `Payment`
            result
                .map(|x| x.unwrap())
                .map(|row| {
                    // ⚠️ Note that from_row will panic if you don't follow your schema
                    let (kontonummer, saldo, eier) = my::from_row(row);
                    Payment {
                        kontonummer: kontonummer,
                        saldo: saldo,
                        eier: eier,
                    }
                })
                .collect() // Collect payments so now `QueryResult` is mapped to `Vec<Payment>`
        })
        .unwrap(); // Unwrap `Vec<Payment>`

    // Now make sure that `payments` equals to `selected_payments`.
    // Mysql gives no guaranties on order of returned rows without `ORDER BY`
    // so assume we are lukky.
    assert_eq!(payments, selected_payments);
    println!("Yay!");
}
