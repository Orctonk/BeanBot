// Special Beans Database 
use rusqlite::{params, Connection};

pub enum SpecialBeansError {
    InternalDatabaseError
}

//Error messages
macro_rules! db_err {
    ($fmt:tt, $reason:tt) => {
        {
            println!($fmt,$reason);
            return Err(SpecialBeansError::InternalDatabaseError);
        }
    };
}

fn open_connection() -> rusqlite::Result<Connection> {
    Connection::open("beans.db")
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
            image STRING,
            weight INT
        );",[]);
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
        );",[]);
    if let Err(why) = res2{
        println!("Failed to create 'have beans' table with error {:?}",why);
    } 
    bean_insert();
    
    println!("Specialbeans module is using SQL version {:?}", rusqlite::version());
}

// Function for creating a new special bean. 
pub fn create_special_bean(id: u32, name: &str, about: &str, image_url: &str, weight: u32) -> Result<(),SpecialBeansError> {
    let conn = match open_connection() {
        Ok(connection) => connection,
        Err(why) => db_err!("Failed to open bean DB with error {:?}",why)
    };
   
    let res = conn.execute(
        "INSERT INTO SpecBeans 
        VALUES (?1,?2,?3, ?4,?5)", 
        params![id, name, about, image_url, weight]);
    if let Err(why) = res{
        println!("Failed to insert test data with error {:?}",why);
    }
    Ok(())
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
    let rows = res.query_map([] ,|row|Ok((row.get(0)?,row.get(1)?)));
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
pub fn get_info_from_name(name: &str) -> Result<(String,String),SpecialBeansError>{
    let conn = match open_connection() {
        Ok(connection) => connection,
        Err(why) => db_err!("Failed to open bean DB with error {:?}",why)
    };
    let res : rusqlite::Result<(String,String)> = conn.query_row("
    SELECT about,image
    FROM SpecBeans
    WHERE UPPER(name) = UPPER(?1)", params![name],|row| Ok((row.get(0)?,row.get(1)?)));
    match res {
        Err(why) => db_err!("Failed to get description about bean with error {:?}",why),
        Ok(res) => Ok(res)
    }
}


pub fn bean_insert() {

    if create_special_bean(
        1,
        "Basic Bean",
        "Hello! I am a basic bean. How do you do?",
        "https://cdn.discordapp.com/attachments/594624834714206216/842111748618321930/basic_bean.png",
        10
    ).is_err() {
        println!("Failed to create Basic Bean!");
    }
    
    if create_special_bean(
        2,
        "Cool Bean",
        "I am a cooool bean.",
        "https://beanscape.dev/beans/better_bean.png",
        5
    ).is_err() {
        println!("Failed to create Cool Bean!");
    }

    if create_special_bean(
        3,
        "Bumblebean",
        "Bzzzzzzzz!",
        "https://cdn.discordapp.com/attachments/594624834714206216/842467339029970974/bumblebean.png",
        3
    ).is_err() {
        println!("Failed to create Bumblebean!");
    }

    if create_special_bean(
        4,
        "Furbean",
        "The power of CHRIST flows through me.",
        "https://cdn.discordapp.com/attachments/594624834714206216/842781409810317373/furbean.png",
        1
    ).is_err() {
        println!("Failed to create Furbean!");
    }

    if create_special_bean(
        5,
        "Stinky",
        "Im a stinky, stinky, bean.",
        "https://cdn.discordapp.com/attachments/594624834714206216/842111756264669214/Stinky.png",
        3
    ).is_err() {
        println!("Failed to create Stinky!");
    }

    if create_special_bean(
        6,
        "The Beantles",
        "Well she was just 17, if you know what I bean!",
        "https://cdn.discordapp.com/attachments/594624834714206216/842474994244124722/Beantles.png",
        1
    ).is_err() {
        println!("Failed to create The Beantles!");
    }
}