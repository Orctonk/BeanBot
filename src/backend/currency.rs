use rusqlite::{params, Connection};

use chrono::prelude::*;
use chrono::Duration;

pub enum CurrencyError {
    InsufficientBalance,
    InternalDatabaseError,
    NotReadyYet(Duration)
}

macro_rules! db_err {
    ($fmt:tt, $reason:tt) => {
        {
            println!($fmt,$reason);
            return Err(CurrencyError::InternalDatabaseError);
        }
    };
}

fn open_connection() -> rusqlite::Result<Connection> {
    Connection::open("beans.db")
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
        )",[]);
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
        )",[]);
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
        Ok(0) => Err(CurrencyError::InsufficientBalance),
        _ => Ok(())
    }
}

pub fn get_bean_balance(user: u64) -> Result<u32,CurrencyError> {
    let conn = match open_connection() {
        Ok(connection) => connection,
        Err(why) => db_err!("Failed to open wallet DB with error {:?}",why)
    };

    let res = conn.query_row("SELECT balance FROM Wallet WHERE id=?1",params![user as i64], |row| row.get(0));
    match res {
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(0),
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

pub fn claim_daily(user: u64, amount: u32) -> Result<(),CurrencyError> {
    let mut conn = match open_connection() {
        Ok(connection) => connection,
        Err(why) => db_err!("Failed to open wallet DB with error {:?}",why)
    };

    let res: rusqlite::Result<DateTime<Utc>> = conn.query_row("SELECT daily FROM Claims WHERE id=?1",params![user as i64], |r| r.get(0));
    let cd = match res {
        Err(rusqlite::Error::QueryReturnedNoRows) => Duration::zero(),
        Err(why) => db_err!("Failed to claim daily with error {:?}",why),
        Ok(date) => date + Duration::days(1) - Utc::now()
    };

    if cd > Duration::zero() {
        return Err(CurrencyError::NotReadyYet(cd));
    }

    let trans = match conn.transaction() {
        Ok(transaction) => transaction,
        Err(why) => db_err!("Failed to create transaction with error {:?}",why)
    };

    let res = trans.execute(
        "INSERT INTO Claims VALUES(?1,datetime('now'), datetime('00:00'),datetime('00:00'),datetime('00:00')) 
        ON CONFLICT (id) DO UPDATE SET daily=datetime('now') WHERE id=?1",params![user as i64]);
    if let Err(why) = res {
        let _ = trans.rollback();
        db_err!("Failed update claims table with error {:?}",why); 
    } 
    let res2 = trans.execute(
        "INSERT INTO Wallet VALUES(?1,?2) 
        ON CONFLICT (id) DO UPDATE SET balance=balance+?2 WHERE id=?1",params![user as i64, amount]);
    if let Err(why) = res2 {
        let _ = trans.rollback();
        db_err!("Failed to add balance with error {:?}",why);
    } 
    if let Err(why) = trans.commit() {
        db_err!("Failed to commit with error {:?}",why);
    }
    Ok(())
}

pub fn claim_weekly(user: u64, amount: u32) -> Result<(),CurrencyError> {
    let mut conn = match open_connection() {
        Ok(connection) => connection,
        Err(why) => db_err!("Fi64ailed to open wallet DB with error {:?}",why)
    };

    let res: rusqlite::Result<DateTime<Utc>> = conn.query_row("SELECT weekly FROM Claims WHERE id=?1",params![user as i64], |r| r.get(0));
    let cd = match res {
        Err(rusqlite::Error::QueryReturnedNoRows) => Duration::zero(),
        Err(why) => db_err!("Failed to claim weekly with error {:?}",why),
        Ok(date) => date + Duration::weeks(1) - Utc::now()
    };

    if cd > Duration::zero() {
        return Err(CurrencyError::NotReadyYet(cd));
    }

    let trans = match conn.transaction() {
        Ok(transaction) => transaction,
        Err(why) => db_err!("Failed to create transaction with error {:?}",why)
    };

    let res = trans.execute(
        "INSERT INTO Claims VALUES(?1,datetime('00:00'),datetime('now'),datetime('00:00'),datetime('00:00')) 
        ON CONFLICT (id) DO UPDATE SET weekly=datetime('now') WHERE id=?1",params![user as i64]);
    if let Err(why) = res {
        let _ = trans.rollback();
        db_err!("Failed update claims table with error {:?}",why); 
    } 
    let res2 = trans.execute(
        "INSERT INTO Wallet VALUES(?1,?2) 
        ON CONFLICT (id) DO UPDATE SET balance=balance+?2 WHERE id=?1",params![user as i64, amount]);
    if let Err(why) = res2 {
        let _ = trans.rollback();
        db_err!("Failed to add balance with error {:?}",why);
    } 
    if let Err(why) = trans.commit() {
        db_err!("Failed to commit with error {:?}",why);
    }
    Ok(())
}

pub fn claim_monthly(user: u64, amount: u32) -> Result<(),CurrencyError> {
    let mut conn = match open_connection() {
        Ok(connection) => connection,
        Err(why) => db_err!("Failed to open wallet DB with error {:?}",why)
    };

    let res: rusqlite::Result<DateTime<Utc>> = conn.query_row("SELECT monthly FROM Claims WHERE id=?1",params![user as i64], |r| r.get(0));
    let cd = match res {
        Err(rusqlite::Error::QueryReturnedNoRows) => Duration::zero(),
        Err(why) => db_err!("Failed to claim monthly with error {:?}",why),
        Ok(date) => date + Duration::days(30) - Utc::now()
    };

    if cd > Duration::zero() {
        return Err(CurrencyError::NotReadyYet(cd));
    }

    let trans = match conn.transaction() {
        Ok(transaction) => transaction,
        Err(why) => db_err!("Failed to create transaction with error {:?}",why)
    };

    let res = trans.execute(
        "INSERT INTO Claims VALUES(?1,datetime('00:00'),datetime('00:00'),datetime('now'),datetime('00:00')) 
        ON CONFLICT (id) DO UPDATE SET monthly=datetime('now') WHERE id=?1",params![user as i64]);
    if let Err(why) = res {
        let _ = trans.rollback();
        db_err!("Failed update claims table with error {:?}",why); 
    } 
    let res2 = trans.execute(
        "INSERT INTO Wallet VALUES(?1,?2) 
        ON CONFLICT (id) DO UPDATE SET balance=balance+?2 WHERE id=?1",params![user as i64, amount]);
    if let Err(why) = res2 {
        let _ = trans.rollback();
        db_err!("Failed to add balance with error {:?}",why);
    } 
    if let Err(why) = trans.commit() {
        db_err!("Failed to commit with error {:?}",why);
    }
    Ok(())
}

pub fn claim_yearly(user: u64, amount: u32) -> Result<(),CurrencyError> {
    let mut conn = match open_connection() {
        Ok(connection) => connection,
        Err(why) => db_err!("Failed to open wallet DB with error {:?}",why)
    };

    let res: rusqlite::Result<DateTime<Utc>> = conn.query_row("SELECT yearly FROM Claims WHERE id=?1",params![user as i64], |r| r.get(0));
    let cd = match res {
        Err(rusqlite::Error::QueryReturnedNoRows) => Duration::zero(),
        Err(why) => db_err!("Failed to claim yearly with error {:?}",why),
        Ok(date) => date + Duration::days(365) - Utc::now()
    };

    if cd > Duration::zero() {
        return Err(CurrencyError::NotReadyYet(cd));
    }

    let trans = match conn.transaction() {
        Ok(transaction) => transaction,
        Err(why) => db_err!("Failed to create transaction with error {:?}",why)
    };

    let res = trans.execute(
        "INSERT INTO Claims VALUES(?1,datetime('00:00'),datetime('00:00'),datetime('00:00'),datetime('now')) 
        ON CONFLICT (id) DO UPDATE SET yearly=datetime('now') WHERE id=?1",params![user as i64]);
    if let Err(why) = res {
        let _ = trans.rollback();
        db_err!("Failed update claims table with error {:?}",why); 
    } 
    let res2 = trans.execute(
        "INSERT INTO Wallet VALUES(?1,?2) 
        ON CONFLICT (id) DO UPDATE SET balance=balance+?2 WHERE id=?1",params![user as i64, amount]);
    if let Err(why) = res2 {
        let _ = trans.rollback();
        db_err!("Failed to add balance with error {:?}",why);
    } 
    if let Err(why) = trans.commit() {
        db_err!("Failed to commit with error {:?}",why);
    }
    Ok(())
}

pub fn get_highest_balance() ->  Result<u64,CurrencyError> {
    let conn = match open_connection() {
        Ok(connection) => connection,
        Err(why) => db_err!("Failed to open wallet DB with error {:?}",why)
    };
    let res : rusqlite::Result<i64> = conn.query_row("SELECT id FROM Wallet ORDER BY balance DESC LIMIT 1",params![], |row| row.get(0));
    match res {
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(0),
        Err(why) => db_err!("Failed to get balance with error {:?}",why),
        Ok(id) => Ok(id as u64)
    } 
}

pub fn get_scores() ->  Result<Vec<(u64, u32)>,CurrencyError> {
    let conn = match open_connection() {
        Ok(connection) => connection,
        Err(why) => db_err!("Failed to open wallet DB with error {:?}",why)
    };
    let mut res = match conn.prepare("SELECT id,balance FROM Wallet ORDER BY balance DESC LIMIT 10"){
        Err(why) => db_err!("Failed to get balance with error {:?}",why),
        Ok(res) => res
    };
    let rows = res.query_map(NO_PARAMS,|row|Ok((row.get(0)?,row.get(1)?)));
    match rows {
        Err(why) => db_err!("Failed to get balance with error {:?}",why),
        Ok(scores_mapped) => {          
            let mut scores = Vec::new();
            for score_result in scores_mapped {
                match score_result{
                    Err(_) => {},
                    Ok(score_result_elem) => {
                        let result_type : (i64, u32) = score_result_elem; 
                        scores.push((result_type.0 as u64, result_type.1 as u32))
                    }
                };
            };
            Ok(scores)
        }
        
    }
}

