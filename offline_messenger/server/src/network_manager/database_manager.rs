use std::sync::Arc;
use tokio_postgres::{Client, Error, NoTls, Row};

use crate::network_manager::handlers::{LoginReq, Response, SigninReq};

pub struct DataBase {
    client: Arc<Client>,
}

impl DataBase {
    pub async fn new() -> Result<Arc<Self>, Error> {
        let (client, connection) = tokio_postgres::connect(
            "host=localhost user=postgres password=mysecretpassword dbname=postgres",
            NoTls,
        )
        .await?;

        tokio::spawn(async move {
            if let Err(err) = connection.await {
                println!("Error durring connection to the databse: {err}");
            }
        });

        client
            .execute(
                r"CREATE TABLE IF NOT EXISTS users (
                        username TEXT PRIMARY KEY,
                        password TEXT NOT NULL
                        );",
                &[],
            )
            .await?;
        client
            .execute(
                r"CREATE TABLE IF NOT EXISTS messages (
                        id_message INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
                        sender TEXT NOT NULL REFERENCES users(username) ON DELETE CASCADE,
                        receiver TEXT NOT NULL REFERENCES users(username) ON DELETE CASCADE,
                        content TEXT NOT NULL,
                        date TIMESTAMP NOT NULL DEFAULT now(),
                        responding_to_msg TEXT,
                        responding_to_user TEXT
                        );",
                &[],
            )
            .await?;
        Ok(Arc::new(Self {
            client: Arc::new(client),
        }))
    }
    pub async fn signin(&self, user_info: SigninReq) -> Result<Response, Error> {
        let exists: bool = self
            .client
            .query_one(
                "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1);",
                &[&user_info.username],
            )
            .await?
            .get(0);
        if exists {
            let resp = Response {
                succes: false,
                message: "Username taken!".to_string(),
            };
            return Ok(resp);
        }
        println!("Hmm...");
        self.client
            .execute(
                "INSERT INTO users (username, password) VALUES ($1, $2);",
                &[&user_info.username, &user_info.password],
            )
            .await?;
        let resp = Response {
            succes: true,
            message: "Signed in with succes!".to_string(),
        };
        Ok(resp)
    }
    pub async fn login(&self, user_info: LoginReq) -> Result<Response, Error> {
        let exists: bool = self
            .client
            .query_one(
                "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1 AND password = $2);",
                &[&user_info.username, &user_info.password],
            )
            .await?
            .get(0);
        if !exists {
            let resp = Response {
                succes: false,
                message: "Invalid username and/or password.".to_string(),
            };
            return Ok(resp);
        }
        let resp = Response {
            succes: true,
            message: "Logged in with succes!".to_string(),
        };
        Ok(resp)
    }
    pub async fn send_message(
        &self,
        sender: &str,
        receiver: &str,
        message: &str,
    ) -> Result<Response, Error> {
        let exists: bool = self
            .client
            .query_one(
                "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1);",
                &[&sender],
            )
            .await?
            .get(0);
        if !exists {
            let resp = Response {
                succes: false,
                message: "The sender is not in the database".to_string(),
            };
            return Ok(resp);
        }
        let exists: bool = self
            .client
            .query_one(
                "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1);",
                &[&receiver],
            )
            .await?
            .get(0);
        if !exists {
            let resp = Response {
                succes: false,
                message: "The receiver is not in the database".to_string(),
            };
            return Ok(resp);
        }

        self.client
            .execute(
                "INSERT INTO messages (content, sender, receiver) VALUES ($1, $2, $3);",
                &[&message, &sender, &receiver],
            )
            .await?;
        let resp = Response {
            succes: true,
            message: "Message saved".to_string(),
        };
        Ok(resp)
    }
    pub async fn send_message_with_resp(
        &self,
        sender: &str,
        receiver: &str,
        message: &str,
        resp_msg: &str,
        resp_usr: &str,
    ) -> Result<Response, Error> {
        let exists: bool = self
            .client
            .query_one(
                "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1);",
                &[&sender],
            )
            .await?
            .get(0);
        if !exists {
            let resp = Response {
                succes: false,
                message: "The sender is not in the database".to_string(),
            };
            return Ok(resp);
        }
        let exists: bool = self
            .client
            .query_one(
                "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1);",
                &[&receiver],
            )
            .await?
            .get(0);
        if !exists {
            let resp = Response {
                succes: false,
                message: "The receiver is not in the database".to_string(),
            };
            return Ok(resp);
        }

        self.client
            .execute(
                "INSERT INTO messages (content, sender, receiver, responding_to_msg, responding_to_user) VALUES ($1, $2, $3, $4, $5);",
                &[&message, &sender, &receiver, &resp_msg, &resp_usr],
            )
            .await?;
        let resp = Response {
            succes: true,
            message: "Message saved".to_string(),
        };
        Ok(resp)
    }
    pub async fn get_messages(
        &self,
        user1: &str,
        user2: &str,
        offset: i64,
    ) -> Result<Option<Vec<Row>>, Error> {
        let exists: bool = self
            .client
            .query_one(
                "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1);",
                &[&user1],
            )
            .await?
            .get(0);
        if !exists {
            return Ok(None);
        }
        let exists: bool = self
            .client
            .query_one(
                "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1);",
                &[&user2],
            )
            .await?
            .get(0);
        if !exists {
            return Ok(None);
        }
        let row = self.client.query(r"SELECT content, sender, responding_to_msg, responding_to_user FROM 
                            messages m JOIN users u1 ON m.sender = u1.username JOIN users u2 ON m.receiver = u2.username 
                            WHERE (u1.username = $1 AND u2.username = $2) OR (u1.username = $2 AND u2.username = $1)
                            ORDER BY date ASC LIMIT 50 OFFSET $3;", &[&user1, &user2, &offset]).await?;
        Ok(Some(row))
    }

    pub async fn get_user_list(&self, user: &str) -> Result<Option<Vec<Row>>, Error> {
        let row = self
            .client
            .query(
                r"SELECT username FROM users WHERE username != $1 ORDER BY username ASC;",
                &[&user],
            )
            .await?;
        Ok(Some(row))
    }
}
