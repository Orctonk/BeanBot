use rusqlite::{params, Connection, Result};
use rusqlite::NO_PARAMS;

use std::result;

fn open_connection() -> Result<Connection> {
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
    println!("Currency module is using SQL version {:?}", rusqlite::version());
}

pub fn add_beans(user: u64, beans: u32){
    let conn = match open_connection() {
        Ok(connection) => connection,
        Err(why) => {
            println!("Failed to open wallet DB with error {:?}",why); 
            return;
        }
    };

    let res = conn.execute(
        "INSERT INTO Wallet VALUES(?1,?2) ON CONFLICT (id) DO UPDATE SET balance=balance+?2 WHERE id=?1",params![user as i64,beans]
    );
    if let Err(why) = res {
        println!("Failed to add beans to user with exception {:?}", why);
    }
}

pub fn withdraw_beans(user: u64, beans: u32) -> result::Result<(),String> {
    let conn = match open_connection() {
        Ok(connection) => connection,
        Err(why) => {
            println!("Failed to open wallet DB with error {:?}",why); 
            return Err("Database error".to_string());
        }
    };

    let res = conn.execute(
        "UPDATE Wallet SET balance=balance-?2 WHERE id=?1 AND balance >= ?2",params![user as i64,beans]
    );
    match res {
        Err(why) => {
            println!("Failed to add beans to user with exception {:?}", why);
            return Err("Database Error".to_string());
        },
        Ok(0) => return Err("Not enough beans".to_string()),
        _ => return Ok(())
    };
}

pub fn get_bean_balance(user: u64) -> result::Result<u32,String> {
    let conn = match open_connection() {
        Ok(connection) => connection,
        Err(why) => {
            println!("Failed to open wallet DB with error {:?}",why); 
            return Err("Database error".to_string());
        }
    };

    let res = conn.query_row("SELECT balance FROM Wallet WHERE id=?1",params![user as i64], |row| row.get(0));
    match res {
        Err(why) => {
            println!("Failed to get balance with error {:?}",why);
            Err("Failed to get balance".to_string())
        },
        Ok(balance) => Ok(balance)
    }
}

pub fn transfer_beans(from: u64, to: u64, amount: u32) -> result::Result<(),String> {
    let mut conn = match open_connection() {
        Ok(connection) => connection,
        Err(why) => {
            println!("Failed to open wallet DB with error {:?}",why); 
            return Err("Database error".to_string());
        }
    };

    let trans = match conn.transaction() {
        Ok(transaction) => transaction,
        Err(why) => {
            println!("Failed to create transaction with error {:?}",why); 
            return Err("Database error".to_string());
        }
    };

    let res = trans.execute("UPDATE Wallet SET balance=balance-?2 WHERE id=?1 AND balance >= ?2",params![from as i64,amount]);
    if let Err(why) = res {
        println!("Failed to withdraw with error {:?}",why); 
        let _ = trans.rollback();
        return Err("Database error".to_string());
    } else if let Ok(0) = res {
        return Err("Not enough beans".to_string());
    }

    let res2 = trans.execute("INSERT INTO Wallet VALUES(?1,?2) ON CONFLICT (id) DO UPDATE SET balance=balance+?2 WHERE id=?1",params![to as i64,amount]);
    if let Err(why) = res2 {
        println!("Failed to add balance with error {:?}",why); 
        let _ = trans.rollback();
        return Err("Database error".to_string());
    } 
    if let Err(why) = trans.commit() {
        println!("Failed to commit with error {:?}",why); 
        return Err("Database error".to_string());
    }
    Ok(())
}