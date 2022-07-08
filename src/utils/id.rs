use rusqlite::types::ToSqlOutput;
use rusqlite::ToSql;

pub fn parse_list(s: &str) -> Vec<u32> {
    let s = s.trim();
    let slice: Vec<&str> = s.split(",").collect();
    let mut vec: Vec<u32> = Vec::new();
    for piece in slice {
        vec.append(&mut parse_piece(piece))
    }
    vec
}

pub fn parse_piece(s: &str) -> Vec<u32> {
    let s = s.trim();
    let spl: Vec<&str> = s.split("-").collect();
    if spl.len() == 1 {
        let id = spl[0].parse::<u32>();
        if id.is_err() {
            return vec![];
        }
        return vec![id.unwrap()];
    }
    if spl.len() == 2 {
        let id_min = spl[0].parse::<u32>();
        if id_min.is_err() {
            return vec![];
        }

        let id_max = spl[1].parse::<u32>();
        if id_max.is_err() {
            return vec![];
        }
        return (id_min.unwrap()..id_max.unwrap()).collect::<Vec<u32>>();
    }
    vec![]
}
