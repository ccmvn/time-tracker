use async_trait::async_trait;
use log::{error, info};
use sqlx::{Error, SqlitePool};

use crate::pages::index::{AbsenceEntry, TimeEntry, UserInfo};

#[async_trait]
pub trait Database {
    // User
    async fn add_user(&self, username: &str, email: &str) -> Result<i64, Error>;
    async fn get_user(&self, user_id: i64) -> Result<UserInfo, Error>;
    async fn get_all_users(&self) -> Result<Vec<UserInfo>, Error>;
    async fn delete_user(&self, user_id: i64) -> Result<i64, Error>;
    async fn update_password(&self, user_id: i64, new_password: &str) -> Result<i64, Error>;
    async fn get_password(&self, user_id: i64) -> Result<String, Error>;
    async fn update_email(&self, user_id: i64, new_email: &str) -> Result<i64, Error>;
    async fn get_email(&self, user_id: i64) -> Result<String, Error>;
    async fn update_authority(&self, user_id: i64, new_authority: &str) -> Result<i64, Error>;
    async fn check_login(&self, username: &str, password: &str) -> Result<Option<i64>, Error>;
    // Time Entry
    async fn add_time_entry(&self, user_id: i64, task: Option<&str>, spent_time: Option<i64>, date: Option<&str>) -> Result<i64, Error>;
    async fn get_time_entries(&self, user_id: i64) -> Result<Vec<TimeEntry>, Error>;
    async fn update_time_entry(&self, entry_id: i64, task: &str, spent_time: i64, date: &str) -> Result<i64, Error>;
    async fn delete_time_entry(&self, user_id: i64, entry_id: i64) -> Result<i64, Error>;
    // Absence Entry
    async fn add_absence_entry(&self, user_id: i64, absence_date: &str, reason: Option<&str>) -> Result<i64, Error>;
    async fn get_absence_entries(&self, user_id: i64) -> Result<Vec<AbsenceEntry>, Error>;
    async fn update_absence_entry(&self, entry_id: i64, absence_date: &str, reason: &str) -> Result<i64, Error>;
    async fn delete_absence_entry(&self, user_id: i64, entry_id: i64) -> Result<i64, Error>;
}

#[derive(Clone)]
pub struct DatabaseConnection {
    pub pool: SqlitePool,
}

#[async_trait]
impl Database for DatabaseConnection {
    // User
    async fn add_user(&self, username: &str, email: &str) -> Result<i64, Error> {
        match bcrypt::hash("abcdef123", bcrypt::DEFAULT_COST) {
            Ok(hashed_password) => {
                let rows_affected = sqlx::query!(
                    "INSERT INTO users (username, email, password) VALUES (?, ?, ?)",
                    username,
                    email,
                    hashed_password
                )
                    .execute(&self.pool)
                    .await?
                    .last_insert_rowid();

                Ok(rows_affected)
            }
            Err(_) => Err(Error::Configuration("Failed to hash password".into()))
        }
    }

    async fn get_user(&self, id: i64) -> Result<UserInfo, Error> {
        let user = sqlx::query_as!(
            UserInfo,
            "SELECT id, username, email, authority FROM users WHERE id = ?",
            id
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }

    async fn get_all_users(&self) -> Result<Vec<UserInfo>, Error> {
        let users = sqlx::query_as!(
            UserInfo,
            "SELECT id, username, email, authority FROM users"
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(users)
    }

    async fn delete_user(&self, user_id: i64) -> Result<i64, Error> {
        let rows_affected = sqlx::query!(
            "DELETE FROM users WHERE id = ?",
            user_id
        )
            .execute(&self.pool)
            .await?
            .rows_affected();

        Ok(rows_affected.try_into().unwrap())
    }

    async fn update_password(&self, user_id: i64, new_password: &str) -> Result<i64, Error> {
        let hashed_password = bcrypt::hash(new_password, bcrypt::DEFAULT_COST)
            .map_err(|_| Error::Configuration("Failed to hash password".into()))?;

        let rows_affected = sqlx::query!(
            "UPDATE users SET password = ? WHERE id = ?",
            hashed_password,
            user_id
        )
            .execute(&self.pool)
            .await?
            .rows_affected();

        Ok(rows_affected.try_into().unwrap())
    }

    async fn get_password(&self, user_id: i64) -> Result<String, Error> {
        let row: (String, ) = sqlx::query_as("SELECT password FROM users WHERE id = ?")
            .bind(user_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(row.0)
    }

    async fn update_email(&self, user_id: i64, new_email: &str) -> Result<i64, Error> {
        let rows_affected = sqlx::query!(
            "UPDATE users SET email = ? WHERE id = ?",
            new_email,
            user_id
        )
            .execute(&self.pool)
            .await?
            .rows_affected();

        Ok(rows_affected.try_into().unwrap())
    }

    async fn get_email(&self, user_id: i64) -> Result<String, Error> {
        let row: (String, ) = sqlx::query_as("SELECT email FROM users WHERE id = ?")
            .bind(user_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(row.0)
    }

    async fn update_authority(&self, user_id: i64, new_authority: &str) -> Result<i64, Error> {
        let rows_affected = sqlx::query!(
            "UPDATE users SET authority = ? WHERE id = ?",
            new_authority,
            user_id
        )
            .execute(&self.pool)
            .await?
            .rows_affected();

        Ok(rows_affected.try_into().unwrap())
    }

    async fn check_login(&self, username: &str, password: &str) -> Result<Option<i64>, Error> {
        let result = sqlx::query!(
        "SELECT id, password FROM users WHERE username = ?",
        username
    )
            .fetch_optional(&self.pool)
            .await;

        match result {
            Ok(Some(row)) => {
                info!("Fetched password from database for username: {}", username);

                let hashed_password = row.password;
                let user_id = row.id;

                match bcrypt::verify(password, &hashed_password) {
                    Ok(true) => {
                        info!("Password verification successful for username: {}", username);
                        Ok(user_id)
                    }
                    _ => {
                        info!("Password verification failed for username: {}", username);
                        Ok(None)
                    }
                }
            }
            Ok(None) => {
                info!("No user found with username: {}", username);
                Ok(None)
            }
            Err(e) => {
                error!("Error fetching password from database for username: {}. Error: {:?}", username, e);
                Err(e)
            }
        }
    }

    // Time Entry
    async fn add_time_entry(&self, user_id: i64, task: Option<&str>, spent_time: Option<i64>, date: Option<&str>) -> Result<i64, Error> {
        let rows_affected = sqlx::query!(
            "INSERT INTO time_entries (user_id, task, spent_time, date) VALUES (?, ?, ?, ?)",
            user_id,
            task,
            spent_time,
            date
        )
            .execute(&self.pool)
            .await?
            .last_insert_rowid();

        Ok(rows_affected)
    }

    async fn get_time_entries(&self, user_id: i64) -> Result<Vec<TimeEntry>, Error> {
        let entries = sqlx::query_as!(
            TimeEntry,
            "SELECT id, task, spent_time, date FROM time_entries WHERE user_id = ?",
            user_id
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(entries)
    }

    async fn update_time_entry(&self, entry_id: i64, task: &str, spent_time: i64, date: &str) -> Result<i64, Error> {
        let rows_affected = sqlx::query(
            "UPDATE time_entries SET task = ?, spent_time = ?, date = ? WHERE id = ?"
        )
            .bind(task)
            .bind(spent_time)
            .bind(date)
            .bind(entry_id)
            .execute(&self.pool)
            .await?
            .rows_affected();

        Ok(rows_affected.try_into().unwrap())
    }

    async fn delete_time_entry(&self, user_id: i64, entry_id: i64) -> Result<i64, Error> {
        let rows_affected = sqlx::query(
            "DELETE FROM time_entries WHERE id = ? AND user_id = ?"
        )
            .bind(entry_id)
            .bind(user_id)
            .execute(&self.pool)
            .await?
            .rows_affected();

        Ok(rows_affected.try_into().unwrap())
    }

    // Absence Entry
    async fn add_absence_entry(&self, user_id: i64, absence_date: &str, reason: Option<&str>) -> Result<i64, Error> {
        let rows_affected = sqlx::query!(
            "INSERT INTO absence_entries (user_id, absence_date, reason) VALUES (?, ?, ?)",
            user_id,
            absence_date,
            reason
        )
            .execute(&self.pool)
            .await?
            .last_insert_rowid();

        Ok(rows_affected)
    }

    async fn get_absence_entries(&self, user_id: i64) -> Result<Vec<AbsenceEntry>, Error> {
        let entries = sqlx::query_as!(
            AbsenceEntry,
            "SELECT id, absence_date, reason FROM absence_entries WHERE user_id = ?",
            user_id
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(entries)
    }

    async fn update_absence_entry(&self, entry_id: i64, absence_date: &str, reason: &str) -> Result<i64, Error> {
        let rows_affected = sqlx::query!(
            "UPDATE absence_entries SET absence_date = ?, reason = ? WHERE id = ?",
            absence_date,
            reason,
            entry_id
        )
            .execute(&self.pool)
            .await?
            .rows_affected();

        Ok(rows_affected.try_into().unwrap())
    }

    async fn delete_absence_entry(&self, user_id: i64, entry_id: i64) -> Result<i64, Error> {
        let rows_affected = sqlx::query!(
            "DELETE FROM absence_entries WHERE id = ? AND user_id = ?",
            entry_id,
            user_id
        )
            .execute(&self.pool)
            .await?
            .rows_affected();

        Ok(rows_affected.try_into().unwrap())
    }
}
