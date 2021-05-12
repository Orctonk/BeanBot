// Special Beans Database 
use rusqlite::{params, Connection};
use rusqlite::NO_PARAMS;

use chrono::prelude::*;
use chrono::Duration;

pub enum SpecialBeansError {
    InternalDatabaseError
}

//Error messages
macro_rules! db_err {
    ($fmt:tt, $reason:tt) => {
        {
            println!($fmt,$reason);
            return Err(SpecialBeansError::InternalDatabaseError)?;
        }
    };
}

fn open_connection() -> rusqlite::Result<Connection> {
    return Connection::open("beans.db");
}

// Create 2 tables in the database.
pub fn create_spec_table() {
    let conn = match open_connection() {
        Ok(connection) => connection,
        Err(why) => {
            println!("Failed to open special beans DB with error {:?}",why); 
            return;
        }
    };
    // Table with special beans. Weitght should lie between 1-10
    let res = conn.execute(
        "CREATE TABLE IF NOT EXISTS SpecBeans (
            id INTEGER PRIMARY KEY, 
            name STRING,
            about STRING,
            weight INT
        );",NO_PARAMS);
    if let Err(why) = res{
        println!("Failed to create 'special beans' table with error {:?}",why);
    } 
    // Table with user-bean relation. 
    let res2 = conn.execute(
        "CREATE TABLE IF NOT EXISTS HaveBeans (
            user_id INTEGER, 
            amount INTEGER,
            bean_id INTEGER,
            FOREIGN KEY (bean_id) REFERENCES SpecBeans (id),
            PRIMARY KEY (bean_id, user_id) 
        );",NO_PARAMS);
    if let Err(why) = res2{
        println!("Failed to create 'have beans' table with error {:?}",why);
    } 
    //TEST DATA
    let res3 = conn.execute(
        "INSERT INTO SpecBeans 
        VALUES (?1,?2,?3, ?4)", 
        params![1,"Basic Bean", "I am a basic bean, I drink Star Bucks",8]);
    if let Err(why) = res3{
        println!("Failed to insert test data with error {:?}",why);
    } 
    let res4 = conn.execute(
        "INSERT INTO SpecBeans 
        VALUES (?1,?2,?3, ?4)", 
        params![2,"Better Bean", "I am a rare bean, you should feel luckey to have me!", 1]);
    if let Err(why) = res4{
        println!("Failed to insert test data with error {:?}",why);
    } 
    let res5 = conn.execute(
        "INSERT INTO SpecBeans 
        VALUES (?1,?2,?3, ?4)", params![3,"Stinky","I am a stinky, stinky bean", 7]);
    if let Err(why) = res5{
        println!("Failed to insert test data with error {:?}",why);
    } 
    // REMEMBER TO REMOVE TEST DATA
    println!("Specialbeans module is using SQL version {:?}", rusqlite::version());
}

// Add bean to user and return the name of the bean.
pub fn add_special_bean(user: u64, bean_id: u32) -> Result<String,SpecialBeansError> {
    let conn = match open_connection() {
        Ok(connection) => connection,
        Err(why) => db_err!("Failed to open bean DB with error {:?}",why)
    };
    match conn.execute(
        "INSERT INTO HaveBeans 
        VALUES (?1,?2,?3)
        ON CONFLICT (user_id, bean_id) 
        DO UPDATE SET amount=amount+1", params![user as i64, 1, bean_id as i64]){
            Err(why) => db_err!("Failed to add beans to user with exception {:?}", why),
            Ok(_) => {
                let get_name: rusqlite::Result<String> = conn.query_row("
                SELECT name 
                FROM SpecBeans 
                WHERE SpecBeans.id = ?1", params![bean_id as i64],|row| row.get(0));
                    match get_name {
                        Err(why) => db_err!("Failed to name of new bean with exception {:?}", why),
                        Ok(name) => Ok(name)
                    }
                }
            }
}

//Get special beans from user.
pub fn get_special_beans(user: u64) -> Result<Vec<(String,u32)>,SpecialBeansError> {
    let conn = match open_connection() {
        Ok(connection) => connection,
        Err(why) => db_err!("Failed to open bean DB with error {:?}",why)
    };

    let mut res = match conn.prepare("
    SELECT name,amount 
    FROM SpecBeans 
    INNER JOIN HaveBeans
    ON HaveBeans.bean_id = SpecBeans.id
    AND HaveBeans.user_id = ?1"){
        Err(why) => db_err!("Failed to get beans with error {:?}",why),
        Ok(res) => res
    };
    let rows = res.query_map(params![user as i64] ,|row|Ok((row.get(0)?,row.get(1)?)));
    match rows {
        Err(why) => db_err!("Failed to get beans with error {:?}",why),
        Ok(beans_mapped) => {          
            let mut beans = Vec::new();
            for beans_result in beans_mapped {
                match beans_result{
                    Err(_) => {},
                    Ok(beans_result_elem) => {
                        let result_type : (String, u32) = beans_result_elem; 
                        beans.push((result_type.0 as String, result_type.1 as u32))
                    }
                };
            };
            Ok(beans)
        }
    }
}

//Get special beans id with weight
pub fn get_all_beans() -> Result<Vec<(u32,u32)>,SpecialBeansError> {
    let conn = match open_connection() {
        Ok(connection) => connection,
        Err(why) => db_err!("Failed to open bean DB with error {:?}",why)
    };

    let mut res = match conn.prepare("
    SELECT id,weight 
    FROM SpecBeans"){
        Err(why) => db_err!("Failed to get beans with error {:?}",why),
        Ok(res) => res
    };
    let rows = res.query_map(NO_PARAMS ,|row|Ok((row.get(0)?,row.get(1)?)));
    match rows {
        Err(why) => db_err!("Failed to get beans with error {:?}",why),
        Ok(beans_mapped) => {          
            let mut beans = Vec::new();
            for beans_result in beans_mapped {
                match beans_result{
                    Err(_) => {},
                    Ok(beans_result_elem) => {
                        let result_type : (u32, u32) = beans_result_elem; 
                        beans.push((result_type.0 as u32, result_type.1 as u32))
                    }
                };
            };
            Ok(beans)
        }
    }
}

//Function to get about collumn from the name of a bean
pub fn get_about_from_name(name: &str) -> Result<String,SpecialBeansError>{
    let conn = match open_connection() {
        Ok(connection) => connection,
        Err(why) => db_err!("Failed to open bean DB with error {:?}",why)
    };
    let res = match conn.query_row("
    SELECT about 
    FROM SpecBeans
    WHERE name = ?1", params![name], |row| row.get(0)){
        Err(why) => db_err!("Failed to get description about bean with error {:?}",why),
        Ok(res) => res
    };
    Ok(res)
}