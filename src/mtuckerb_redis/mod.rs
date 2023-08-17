extern crate redis;

pub fn check_redis(message_id: &str) -> Result<String, String> {
    use redis::Commands;
    match redis::Client::open("redis://127.0.0.1/") {
        Ok(client) => {
            let result = match client.get_connection() {
                Ok(mut con) => match con.get::<&str, bool>(message_id) {
                    Ok(v) => match v {
                        true => {
                            return Ok(format!("{} Found issue in Redis!", message_id).to_string())
                        }
                        false => Err(format!("{} not found in redis", message_id).to_string()),
                    },
                    Err(e) => return Err(e.to_string()),
                },
                Err(e) => return Err(format!("Couldn't connect to redis: {}", e.to_string())),
            };
            result
        }
        Err(e) => return Err(e.to_string()),
    }
}

pub fn set_redis(message_id: &str) -> Result<(), String> {
    use redis::Commands;
    match redis::Client::open("redis://127.0.0.1/") {
        Ok(client) => {
            match client.get_connection() {
                Ok(mut conn) => match conn.set_ex::<&str, bool, u32>(message_id, true, 604800) {
                    Ok(_) => return Ok(()),
                    Err(e) => return Err(e.to_string()),
                },
                Err(e) => return Err(format!("Failed to set value in Redis {}", e)),
            };
        }
        Err(e) => return Err(format!("{}", e.to_string())),
    }
}

#[cfg(test)]
pub fn del_redis(message_id: &str) -> Result<(), String> {
    use redis::Commands;
    match redis::Client::open("redis://127.0.0.1/") {
        Ok(client) => {
            match client.get_connection() {
                Ok(mut conn) => match conn.del::<&str, bool>(message_id) {
                    Ok(_) => return Ok(()),
                    Err(e) => return Err(e.to_string()),
                },
                Err(e) => return Err(format!("Failed to delete value in Redis {}", e)),
            };
        }
        Err(e) => return Err(e.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] #[ignore]
    fn it_works() {
        match set_redis("POR-666") {
            Ok(_) => (),
            Err(_) =>  (),
        }
        assert!(check_redis("POR-666").is_ok());
        assert_ne!(check_redis("POR-01"), Ok("It workded".to_string()));

        del_redis("{POR-01}").expect("Failed to delete value in Redis");
        del_redis("POR-666").expect("Failed to delete value in Redis");
    }
}
