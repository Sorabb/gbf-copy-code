use crate::{
    db,
    models::{AddCodesResult, ExpiredItem, Item},
    services::now_millis,
};
use rusqlite::Connection;
use std::collections::HashSet;

const LIFETIME_MILLIS: i64 = 90 * 60 * 1000;

pub fn get_active_items(connection: &Connection, list_id: i64) -> Result<Vec<Item>, String> {
    if !db::lists::exists(connection, list_id).map_err(|error| error.to_string())? {
        return Err("列表不存在".into());
    }
    db::items::active_for_list(connection, list_id, now_millis()).map_err(|error| error.to_string())
}

pub fn add_codes(
    connection: &mut Connection,
    list_id: i64,
    candidates: Vec<String>,
) -> Result<AddCodesResult, String> {
    let transaction = connection
        .transaction()
        .map_err(|error| error.to_string())?;
    if !db::lists::exists(&transaction, list_id).map_err(|error| error.to_string())? {
        return Err("列表不存在".into());
    }

    let now = now_millis();
    db::items::delete_expired(&transaction, now).map_err(|error| error.to_string())?;
    let mut result = AddCodesResult {
        added: 0,
        duplicates: 0,
        invalid: 0,
    };
    let mut seen = HashSet::new();

    for code in candidates {
        if !is_valid_code(&code) {
            result.invalid += 1;
            continue;
        }
        if !seen.insert(code.clone())
            || db::items::contains(&transaction, list_id, &code, now)
                .map_err(|error| error.to_string())?
        {
            result.duplicates += 1;
            continue;
        }
        let item = Item {
            list_id,
            code,
            created_at: now,
            expires_at: now + LIFETIME_MILLIS,
        };
        db::items::insert(&transaction, &item).map_err(|error| error.to_string())?;
        tracing::info!(list_id, code = %item.code, "added code");
        result.added += 1;
    }

    transaction.commit().map_err(|error| error.to_string())?;
    Ok(result)
}

pub fn delete_code(connection: &Connection, list_id: i64, code: &str) -> Result<(), String> {
    if db::items::delete(connection, list_id, code).map_err(|error| error.to_string())? == 0 {
        return Err("数据不存在".into());
    }
    tracing::info!(list_id, code, "deleted code");
    Ok(())
}

pub fn expire_items(connection: &mut Connection) -> Result<Vec<ExpiredItem>, String> {
    let now = now_millis();
    let transaction = connection
        .transaction()
        .map_err(|error| error.to_string())?;
    let expired = db::items::expired(&transaction, now).map_err(|error| error.to_string())?;
    if !expired.is_empty() {
        db::items::delete_expired(&transaction, now).map_err(|error| error.to_string())?;
    }
    transaction.commit().map_err(|error| error.to_string())?;
    for item in &expired {
        tracing::info!(list_id = item.list_id, code = %item.code, "expired code");
    }
    Ok(expired)
}

fn is_valid_code(code: &str) -> bool {
    code.len() == 8 && code.bytes().all(|byte| byte.is_ascii_alphanumeric())
}

#[cfg(test)]
mod tests {
    use super::{add_codes, get_active_items, is_valid_code};
    use crate::{db, services::list_service};
    use rusqlite::Connection;

    #[test]
    fn validates_codes_without_normalizing_case() {
        assert!(is_valid_code("A1b2C3d4"));
        assert!(is_valid_code("00001234"));
        assert!(!is_valid_code("ABC-1234"));
        assert!(!is_valid_code("abcdefg"));
        assert!(!is_valid_code("一二三四五六七八"));
    }

    #[test]
    fn adds_valid_codes_and_counts_duplicates_without_resetting_expiry() {
        let mut connection = Connection::open_in_memory().unwrap();
        db::migrations::run(&connection).unwrap();
        let list = list_service::create(&connection, "测试列表").unwrap();

        let first = add_codes(
            &mut connection,
            list.id,
            vec![
                "A1b2C3d4".into(),
                "A1b2C3d4".into(),
                "ABC-1234".into(),
                "a1b2c3d4".into(),
            ],
        )
        .unwrap();
        assert_eq!((first.added, first.duplicates, first.invalid), (2, 1, 1));
        let original_expiry = get_active_items(&connection, list.id).unwrap()[0].expires_at;

        let second = add_codes(&mut connection, list.id, vec!["A1b2C3d4".into()]).unwrap();
        assert_eq!((second.added, second.duplicates, second.invalid), (0, 1, 0));
        assert_eq!(
            get_active_items(&connection, list.id).unwrap()[0].expires_at,
            original_expiry
        );
    }
}
