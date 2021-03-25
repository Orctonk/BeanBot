use rusqlite::{params, Connection};
use rusqlite::NO_PARAMS;

pub enum CurrencyError {
    InsufficientBalance,
    InternalDatabaseError,
    NotReadyYet(Duration)
}

macro_rules! db_err {
    ($fmt:tt, $reason:tt) => {
        {
            println!($fmt,$reason);
            return Err(CurrencyError::InternalDatabaseError)?;
        }
    };
}

fn open_connection() -> rusqlite::Result<Connection> {
    return Connection::open("beans.db");
}

pub fn create_wallet_table() {
    let conn = match open_connection() {
        Ok(connection) => connection,
        Err(why) => {
            println!("Failed to open wallet DB with error {:?}",why); 
            return;
        }
    };
    let res = conn.execute(
        "CREATE TABLE IF NOT EXISTS Wallet (
            id INTEGER PRIMARY KEY,
            balance INTEGER
        )",NO_PARAMS);
    if let Err(why) = res{
        println!("Failed to create wallet table with error {:?}",why);
    } 

    let res2 = conn.execute(
        "CREATE TABLE IF NOT EXISTS Claims (
            id INTEGER PRIMARY KEY,
            daily DATETIME,
            weekly DATETIME,
            monthly DATETIME,
            yearly DATETIME
        )",NO_PARAMS);
    if let Err(why) = res2{
        println!("Failed to create wallet table with error {:?}",why);
    } 
    println!("Currency module is using SQL version {:?}", rusqlite::version());
}

pub fn add_beans(user: u64, beans: u32) -> Result<(),CurrencyError> {
    let conn = match open_connection() {
        Ok(connection) => connection,
        Err(why) => db_err!("Failed to open wallet DB with error {:?}",why)
    };

    match conn.execute(
        "INSERT INTO Wallet VALUES(?1,?2) 
        ON CONFLICT (id) DO UPDATE SET balance=balance+?2 WHERE id=?1",params![user as i64,beans]) {
        Err(why) => db_err!("Failed to add beans to user with exception {:?}", why),
        Ok(_) => Ok(())
    }
}

pub fn withdraw_beans(user: u64, beans: u32) -> Result<(),CurrencyError> {
    let conn = match open_connection() {
        Ok(connection) => connection,
        Err(why) => db_err!("Failed to open wallet DB with error {:?}",why)
    };

    let res = conn.execute(
        "UPDATE Wallet SET balance=balance-?2 WHERE id=?1 AND balance >= ?2",params![user as i64,beans]
    );
    match res {
        Err(why) => db_err!("Failed to add beans to user with exception {:?}", why),
        Ok(0) => return Err(CurrencyError::InsufficientBalance),
        _ => return Ok(())
    };
}

pub fn get_bean_balance(user: u64) -> Result<u32,CurrencyError> {
    let conn = match open_connection() {
        Ok(connection) => connection,
        Err(why) => db_err!("Failed to open wallet DB with error {:?}",why)
    };

    let res = conn.query_row("SELECT balance FROM Wallet WHERE id=?1",params![user as i64], |row| row.get(0));
    match res {
        Err(why) => db_err!("Failed to get balance with error {:?}",why),
        Ok(balance) => Ok(balance)
    }
}

pub fn transfer_beans(from: u64, to: u64, amount: u32) -> Result<(),CurrencyError> {
    let mut conn = match open_connection() {
        Ok(connection) => connection,
        Err(why) => db_err!("Failed to open wallet DB with error {:?}",why)
    };

    let trans = match conn.transaction() {
        Ok(transaction) => transaction,
        Err(why) => db_err!("Failed to create transaction with error {:?}",why)
    };

    let res = trans.execute("UPDATE Wallet SET balance=balance-?2 WHERE id=?1 AND balance >= ?2",params![from as i64,amount]);
    if let Err(why) = res {
        let _ = trans.rollback();
        db_err!("Failed to withdraw with error {:?}",why);
    } else if let Ok(0) = res {
        return Err(CurrencyError::InsufficientBalance);
    }

    let res2 = trans.execute("INSERT INTO Wallet VALUES(?1,?2) ON CONFLICT (id) DO UPDATE SET balance=balance+?2 WHERE id=?1",params![to as i64,amount]);
    if let Err(why) = res2 {
        let _ = trans.rollback();
        db_err!("Failed to add balance with error {:?}",why);
    } 
    if let Err(why) = trans.commit() {
        db_err!("Failed to commit with error {:?}",why);
    }
    Ok(())
}