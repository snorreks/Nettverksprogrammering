use mysql as my;
use mysql::Error;
#[derive(Debug, PartialEq, Eq)]
pub struct BankUser {
    pub kontonummer: i32,
    pub saldo: i32,
    pub eier: String,
}

impl BankUser {
    pub fn eier(&self) -> &String {
        &self.eier
    }
    pub fn kontonummer(&self) -> &i32 {
        &self.kontonummer
    }
    pub fn saldo(&self) -> &i32 {
        &self.saldo
    }

    pub fn set_eier(&mut self, eier: String, pool: &my::Pool) {
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

    pub fn set_saldo(&mut self, saldo: i32, pool: &my::Pool) -> Result<i32,Error> {
        self.saldo = saldo;
        let mut stmt = pool
            .prepare(r"UPDATE bank_user SET saldo = :saldo WHERE kontonummer = :kontonummer")
            .unwrap();
        stmt.execute(params! {
            "kontonummer" => self.kontonummer,
            "saldo" => self.saldo(),
        })
        .unwrap();
        Ok(1)
    }

    pub fn trekk(&mut self, antall: i32, pool: &my::Pool) -> Result<i32,Error> {
        self.saldo = self.saldo - antall;
        let mut stmt = pool
            .prepare(r"UPDATE bank_user SET saldo = :saldo WHERE kontonummer = :kontonummer")
            .unwrap();
        stmt.execute(params! {
            "kontonummer" => self.kontonummer,
            "saldo" => self.saldo(),
        })
        .unwrap();
        Ok(1)
    }
}
