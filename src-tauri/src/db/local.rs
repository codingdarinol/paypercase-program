use rusqlite::{params, Connection, Result as SqliteResult};

use crate::models::*;

pub fn init_config_db(conn: &Connection) -> SqliteResult<()> {
    conn.execute_batch(
        "
        PRAGMA journal_mode=WAL;

        CREATE TABLE IF NOT EXISTS app_settings (
            key   TEXT PRIMARY KEY,
            value TEXT
        );

        CREATE TABLE IF NOT EXISTS configured_pttypes (
            id           INTEGER PRIMARY KEY AUTOINCREMENT,
            pttype       TEXT,
            name         TEXT,
            pcode        TEXT,
            hipdata_code TEXT,
            short_name   TEXT
        );

        CREATE TABLE IF NOT EXISTS configured_procedures (
            id         INTEGER PRIMARY KEY AUTOINCREMENT,
            icode      TEXT UNIQUE,
            name       TEXT,
            short_name TEXT
        );

        CREATE TABLE IF NOT EXISTS configured_drugs (
            id         INTEGER PRIMARY KEY AUTOINCREMENT,
            icode      TEXT UNIQUE,
            name       TEXT,
            short_name TEXT
        );

        CREATE TABLE IF NOT EXISTS configured_providers (
            id                     INTEGER PRIMARY KEY AUTOINCREMENT,
            health_med_provider_id INTEGER UNIQUE,
            full_name              TEXT,
            short_name             TEXT
        );

        CREATE TABLE IF NOT EXISTS payout_options (
            id     INTEGER PRIMARY KEY AUTOINCREMENT,
            amount REAL,
            label  TEXT
        );

        CREATE TABLE IF NOT EXISTS pending_export (
            id            INTEGER PRIMARY KEY AUTOINCREMENT,
            visit_date    DATE      NOT NULL,
            vn            TEXT      NOT NULL,
            hn            TEXT,
            cid           TEXT,
            first_name    TEXT,
            last_name     TEXT,
            gender        TEXT,
            age           INTEGER,
            rights        TEXT,
            symptoms      TEXT,
            procedure     TEXT,
            therapist     TEXT,
            total_revenue REAL,
            payout_amount REAL,
            created_at    TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            UNIQUE(visit_date, vn)
        );
    ",
    )?;

    let _ = conn.execute(
        "ALTER TABLE configured_providers ADD COLUMN short_name TEXT NOT NULL DEFAULT ''",
        [],
    );

    let default_config_str = include_str!("../../default_config.json");
    if let Ok(config) = serde_json::from_str::<serde_json::Value>(default_config_str) {
        seed_payout_options(conn, &config)?;
        seed_pttypes(conn, &config)?;
        seed_procedures(conn, &config)?;
        seed_drugs(conn, &config)?;
        seed_providers(conn, &config)?;
    } else {
        let count: i64 =
            conn.query_row("SELECT COUNT(*) FROM payout_options", [], |row| row.get(0))?;
        if count == 0 {
            conn.execute(
                "INSERT INTO payout_options (amount, label) VALUES (?1, ?2)",
                params![125.0f64, "Untuk PNS, Pemda, Bayar Sendiri"],
            )?;
            conn.execute(
                "INSERT INTO payout_options (amount, label) VALUES (?1, ?2)",
                params![20.0f64, "Untuk UHC, Jaminan Sosial"],
            )?;
        }
    }

    Ok(())
}

fn seed_payout_options(conn: &Connection, config: &serde_json::Value) -> SqliteResult<()> {
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM payout_options", [], |row| row.get(0))?;
    if count > 0 {
        return Ok(());
    }

    if let Some(arr) = config.get("payout_options").and_then(|v| v.as_array()) {
        for item in arr {
            if let (Some(amount), Some(label)) = (
                item.get("amount").and_then(|v| v.as_f64()),
                item.get("label").and_then(|v| v.as_str()),
            ) {
                conn.execute(
                    "INSERT INTO payout_options (amount, label) VALUES (?1, ?2)",
                    params![amount, label],
                )?;
            }
        }
    }

    Ok(())
}

fn seed_pttypes(conn: &Connection, config: &serde_json::Value) -> SqliteResult<()> {
    let count: i64 =
        conn.query_row("SELECT COUNT(*) FROM configured_pttypes", [], |row| row.get(0))?;
    if count > 0 {
        return Ok(());
    }

    if let Some(arr) = config.get("pttypes").and_then(|v| v.as_array()) {
        for item in arr {
            if let (Some(pttype), Some(name), Some(pcode), Some(hipdata_code), Some(short_name)) = (
                item.get("pttype").and_then(|v| v.as_str()),
                item.get("name").and_then(|v| v.as_str()),
                item.get("pcode").and_then(|v| v.as_str()),
                item.get("hipdata_code").and_then(|v| v.as_str()),
                item.get("short_name").and_then(|v| v.as_str()),
            ) {
                conn.execute(
                    "INSERT INTO configured_pttypes (pttype, name, pcode, hipdata_code, short_name) VALUES (?1, ?2, ?3, ?4, ?5)",
                    params![pttype, name, pcode, hipdata_code, short_name],
                )?;
            }
        }
    }

    Ok(())
}

fn seed_procedures(conn: &Connection, config: &serde_json::Value) -> SqliteResult<()> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM configured_procedures",
        [],
        |row| row.get(0),
    )?;
    if count > 0 {
        return Ok(());
    }

    if let Some(arr) = config.get("procedures").and_then(|v| v.as_array()) {
        for item in arr {
            if let (Some(icode), Some(name), Some(short_name)) = (
                item.get("icode").and_then(|v| v.as_str()),
                item.get("name").and_then(|v| v.as_str()),
                item.get("short_name").and_then(|v| v.as_str()),
            ) {
                conn.execute(
                    "INSERT INTO configured_procedures (icode, name, short_name) VALUES (?1, ?2, ?3)",
                    params![icode, name, short_name],
                )?;
            }
        }
    }

    Ok(())
}

fn seed_drugs(conn: &Connection, config: &serde_json::Value) -> SqliteResult<()> {
    let count: i64 =
        conn.query_row("SELECT COUNT(*) FROM configured_drugs", [], |row| row.get(0))?;
    if count > 0 {
        return Ok(());
    }

    if let Some(arr) = config.get("drugs").and_then(|v| v.as_array()) {
        for item in arr {
            if let (Some(icode), Some(name), Some(short_name)) = (
                item.get("icode").and_then(|v| v.as_str()),
                item.get("name").and_then(|v| v.as_str()),
                item.get("short_name").and_then(|v| v.as_str()),
            ) {
                conn.execute(
                    "INSERT INTO configured_drugs (icode, name, short_name) VALUES (?1, ?2, ?3)",
                    params![icode, name, short_name],
                )?;
            }
        }
    }

    Ok(())
}

fn seed_providers(conn: &Connection, config: &serde_json::Value) -> SqliteResult<()> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM configured_providers",
        [],
        |row| row.get(0),
    )?;
    if count > 0 {
        return Ok(());
    }

    if let Some(arr) = config.get("providers").and_then(|v| v.as_array()) {
        for item in arr {
            if let (Some(provider_id), Some(full_name), Some(short_name)) = (
                item.get("health_med_provider_id")
                    .and_then(|v| v.as_i64()),
                item.get("full_name").and_then(|v| v.as_str()),
                item.get("short_name").and_then(|v| v.as_str()),
            ) {
                conn.execute(
                    "INSERT INTO configured_providers (health_med_provider_id, full_name, short_name) VALUES (?1, ?2, ?3)",
                    params![provider_id, full_name, short_name],
                )?;
            }
        }
    }

    Ok(())
}

pub fn save_pttype(
    conn: &Connection,
    pttype: &str,
    name: &str,
    pcode: &str,
    hipdata_code: &str,
    short_name: &str,
) -> SqliteResult<()> {
    conn.execute(
        "INSERT INTO configured_pttypes (pttype, name, pcode, hipdata_code, short_name) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![pttype, name, pcode, hipdata_code, short_name],
    )?;
    Ok(())
}

pub fn get_all_pttypes(conn: &Connection) -> SqliteResult<Vec<PttypeConfig>> {
    let mut stmt = conn.prepare(
        "SELECT id, pttype, name, pcode, hipdata_code, short_name FROM configured_pttypes ORDER BY id",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(PttypeConfig {
            id: row.get(0)?,
            pttype: row.get::<_, String>(1).unwrap_or_default(),
            name: row.get::<_, String>(2).unwrap_or_default(),
            pcode: row.get::<_, String>(3).unwrap_or_default(),
            hipdata_code: row.get::<_, String>(4).unwrap_or_default(),
            short_name: row.get::<_, String>(5).unwrap_or_default(),
        })
    })?;
    rows.collect()
}

pub fn delete_pttype(conn: &Connection, id: i64) -> SqliteResult<()> {
    conn.execute("DELETE FROM configured_pttypes WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn save_procedure(
    conn: &Connection,
    icode: &str,
    name: &str,
    short_name: &str,
) -> SqliteResult<()> {
    conn.execute(
        "INSERT INTO configured_procedures (icode, name, short_name) VALUES (?1, ?2, ?3)
         ON CONFLICT(icode) DO UPDATE SET name = excluded.name, short_name = excluded.short_name",
        params![icode, name, short_name],
    )?;
    Ok(())
}

pub fn get_all_procedures(conn: &Connection) -> SqliteResult<Vec<ProcedureConfig>> {
    let mut stmt =
        conn.prepare("SELECT id, icode, name, short_name FROM configured_procedures ORDER BY id")?;
    let rows = stmt.query_map([], |row| {
        Ok(ProcedureConfig {
            id: row.get(0)?,
            icode: row.get::<_, String>(1).unwrap_or_default(),
            name: row.get::<_, String>(2).unwrap_or_default(),
            short_name: row.get::<_, String>(3).unwrap_or_default(),
        })
    })?;
    rows.collect()
}

pub fn delete_procedure(conn: &Connection, id: i64) -> SqliteResult<()> {
    conn.execute("DELETE FROM configured_procedures WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn save_drug(
    conn: &Connection,
    icode: &str,
    name: &str,
    short_name: &str,
) -> SqliteResult<()> {
    conn.execute(
        "INSERT INTO configured_drugs (icode, name, short_name) VALUES (?1, ?2, ?3)
         ON CONFLICT(icode) DO UPDATE SET name = excluded.name, short_name = excluded.short_name",
        params![icode, name, short_name],
    )?;
    Ok(())
}

pub fn get_all_drugs(conn: &Connection) -> SqliteResult<Vec<DrugConfig>> {
    let mut stmt =
        conn.prepare("SELECT id, icode, name, short_name FROM configured_drugs ORDER BY id")?;
    let rows = stmt.query_map([], |row| {
        Ok(DrugConfig {
            id: row.get(0)?,
            icode: row.get::<_, String>(1).unwrap_or_default(),
            name: row.get::<_, String>(2).unwrap_or_default(),
            short_name: row.get::<_, String>(3).unwrap_or_default(),
        })
    })?;
    rows.collect()
}

pub fn delete_drug(conn: &Connection, id: i64) -> SqliteResult<()> {
    conn.execute("DELETE FROM configured_drugs WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn save_provider(
    conn: &Connection,
    health_med_provider_id: i64,
    full_name: &str,
    short_name: &str,
) -> SqliteResult<()> {
    conn.execute(
        "INSERT INTO configured_providers (health_med_provider_id, full_name, short_name) VALUES (?1, ?2, ?3)
         ON CONFLICT(health_med_provider_id) DO UPDATE SET full_name = excluded.full_name, short_name = excluded.short_name",
        params![health_med_provider_id, full_name, short_name],
    )?;
    Ok(())
}

pub fn get_all_providers(conn: &Connection) -> SqliteResult<Vec<ProviderConfig>> {
    let mut stmt = conn.prepare(
        "SELECT id, health_med_provider_id, full_name, short_name FROM configured_providers ORDER BY id",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(ProviderConfig {
            id: row.get(0)?,
            health_med_provider_id: row.get(1)?,
            full_name: row.get::<_, String>(2).unwrap_or_default(),
            short_name: row.get::<_, String>(3).unwrap_or_default(),
        })
    })?;
    rows.collect()
}

pub fn delete_provider(conn: &Connection, id: i64) -> SqliteResult<()> {
    conn.execute("DELETE FROM configured_providers WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn get_payout_options(conn: &Connection) -> SqliteResult<Vec<PayoutOption>> {
    let mut stmt =
        conn.prepare("SELECT id, amount, label FROM payout_options ORDER BY amount DESC")?;
    let rows = stmt.query_map([], |row| {
        Ok(PayoutOption {
            id: row.get(0)?,
            amount: row.get(1)?,
            label: row.get::<_, String>(2).unwrap_or_default(),
        })
    })?;
    rows.collect()
}

pub fn add_payout_option(conn: &Connection, amount: f64, label: &str) -> SqliteResult<()> {
    conn.execute(
        "INSERT INTO payout_options (amount, label) VALUES (?1, ?2)",
        params![amount, label],
    )?;
    Ok(())
}

pub fn delete_payout_option(conn: &Connection, id: i64) -> SqliteResult<()> {
    conn.execute("DELETE FROM payout_options WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn upsert_pending_export(conn: &Connection, record: &UpsertPendingInput) -> SqliteResult<()> {
    conn.execute(
        "INSERT INTO pending_export (
            visit_date, vn, hn, cid, first_name, last_name, gender, age, rights, symptoms,
            procedure, therapist, total_revenue, payout_amount
        ) VALUES (
            ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14
        )
        ON CONFLICT(visit_date, vn) DO UPDATE SET
            hn = excluded.hn,
            cid = excluded.cid,
            first_name = excluded.first_name,
            last_name = excluded.last_name,
            gender = excluded.gender,
            age = excluded.age,
            rights = excluded.rights,
            symptoms = excluded.symptoms,
            procedure = excluded.procedure,
            therapist = excluded.therapist,
            total_revenue = excluded.total_revenue,
            payout_amount = excluded.payout_amount",
        params![
            record.visit_date,
            record.vn,
            record.hn,
            record.cid,
            record.first_name,
            record.last_name,
            record.gender,
            record.age,
            record.rights,
            record.symptoms,
            record.procedure,
            record.therapist,
            record.total_revenue,
            record.payout_amount,
        ],
    )?;
    Ok(())
}

pub fn get_pending_export(
    conn: &Connection,
    date_from: &str,
    date_to: &str,
) -> SqliteResult<Vec<PendingRow>> {
    let mut stmt = conn.prepare(
        "SELECT id, visit_date, vn, hn, cid, first_name, last_name, gender, age, rights, symptoms, procedure, therapist, total_revenue, payout_amount, created_at
         FROM pending_export
         WHERE visit_date BETWEEN ?1 AND ?2
         ORDER BY visit_date DESC, created_at DESC, id DESC",
    )?;
    let rows = stmt.query_map(params![date_from, date_to], |row| {
        Ok(PendingRow {
            id: row.get(0)?,
            visit_date: row.get::<_, String>(1).unwrap_or_default(),
            vn: row.get::<_, String>(2).unwrap_or_default(),
            hn: row.get::<_, String>(3).unwrap_or_default(),
            cid: row.get::<_, String>(4).unwrap_or_default(),
            first_name: row.get::<_, String>(5).unwrap_or_default(),
            last_name: row.get::<_, String>(6).unwrap_or_default(),
            gender: row.get::<_, String>(7).unwrap_or_default(),
            age: row.get(8).ok(),
            rights: row.get::<_, String>(9).unwrap_or_default(),
            symptoms: row.get::<_, String>(10).unwrap_or_default(),
            procedure: row.get::<_, String>(11).unwrap_or_default(),
            therapist: row.get::<_, String>(12).unwrap_or_default(),
            total_revenue: row.get::<_, f64>(13).unwrap_or_default(),
            payout_amount: row.get::<_, f64>(14).unwrap_or_default(),
            created_at: row.get::<_, String>(15).unwrap_or_default(),
        })
    })?;
    rows.collect()
}

pub fn delete_pending_by_id(conn: &Connection, id: i64) -> SqliteResult<()> {
    conn.execute("DELETE FROM pending_export WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn delete_pending_by_range(
    conn: &Connection,
    date_from: &str,
    date_to: &str,
) -> SqliteResult<usize> {
    conn.execute(
        "DELETE FROM pending_export WHERE visit_date BETWEEN ?1 AND ?2",
        params![date_from, date_to],
    )
}

pub fn get_monthly_stats(conn: &Connection) -> SqliteResult<Vec<MonthlyStats>> {
    let mut stmt = conn.prepare(
        "SELECT
            strftime('%Y-%m', visit_date) AS month,
            COUNT(*) AS count,
            COALESCE(SUM(total_revenue), 0) AS total_revenue,
            COALESCE(SUM(payout_amount), 0) AS total_payout
         FROM pending_export
         GROUP BY strftime('%Y-%m', visit_date)
         ORDER BY month DESC",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(MonthlyStats {
            month: row.get::<_, String>(0).unwrap_or_default(),
            count: row.get(1)?,
            total_revenue: row.get::<_, f64>(2).unwrap_or_default(),
            total_payout: row.get::<_, f64>(3).unwrap_or_default(),
        })
    })?;
    rows.collect()
}

pub fn preview_delete_range(
    conn: &Connection,
    date_from: &str,
    date_to: &str,
) -> SqliteResult<Vec<PendingRow>> {
    get_pending_export(conn, date_from, date_to)
}
