use std::path::PathBuf;

use rusqlite::{Connection, Error};

struct Block {
    block_number: i64,
    slot_number: i64,
    hash: String,
    prev_hash: String,
    leader_vrf: String,
    orphaned: bool,
}

pub fn validate_block(db_path: &PathBuf, hash: &String) {
    let like = format!("{}%", hash);
    match query_block(db_path, like) {
        Ok(block) => {
            println!("{{\n\
            \x20\"status\": \"{}\",\n\
            \x20\"block_number\": \"{}\",\n\
            \x20\"slot_number\": \"{}\",\n\
            \x20\"hash\": \"{}\",\n\
            \x20\"prev_hash\": \"{}\",\n\
            \x20\"leader_vrf\": \"{}\"\n\
            }}",
                     if block.orphaned { "orphaned" } else { "ok" },
                     block.block_number,
                     block.slot_number,
                     block.hash,
                     block.prev_hash,
                     block.leader_vrf
            );
        }
        Err(error) => {
            println!("{{\n\
            \x20\"status\": \"error\",\n\
            \x20\"errorMessage\": \"{}\"\n\
            }}", error);
        }
    }
}

fn query_block(db_path: &PathBuf, like: String) -> Result<Block, Error> {
    let db = Connection::open(db_path)?;
    let query_result = db.query_row("SELECT block_number,slot_number,hash,prev_hash,leader_vrf_0,orphaned FROM chain WHERE hash LIKE ?",
                                    &[&like],
                                    |row| Ok(
                                        Block {
                                            block_number: row.get(0)?,
                                            slot_number: row.get(1)?,
                                            hash: row.get(2)?,
                                            prev_hash: row.get(3)?,
                                            leader_vrf: row.get(4)?,
                                            orphaned: row.get(5)?,
                                        }
                                    ));

    match db.close() {
        Err(error) => {
            return Err(error.1);
        }
        _ => {}
    }

    query_result
}