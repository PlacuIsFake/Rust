use std::sync::Arc;
use tokio_postgres::{Client, Error, NoTls, Row};

use crate::network_manager::handlers::{LoginReq, Response, SigninReq};

pub struct DataBase {
    client: Arc<Client>,
}

impl DataBase {
    pub async fn new() -> Result<Arc<Self>, Error> {
        let (client, connection) = tokio_postgres::connect("host=localhost user=postgres password=mysecretpassword dbname=postgres", NoTls).await?;

        tokio::spawn(async move {
            if let Err(err) = connection.await {
                println!("Error durring connection to the databse: {err}");
            }
        });

        client
            .execute(
                r"CREATE TABLE IF NOT EXISTS users (
                        id_user INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
                        username TEXT NOT NULL,
                        password TEXT NOT NULL
                        );",
                &[],
            )
            .await?;
        client
            .execute(
                r"CREATE TABLE IF NOT EXISTS messages (
                        id_message INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
                        id_sender INT NOT NULL REFERENCES users(id_user) ON DELETE CASCADE,
                        id_receiver INT NOT NULL REFERENCES users(id_user) ON DELETE CASCADE,
                        content TEXT NOT NULL,
                        date TIMESTAMP NOT NULL DEFAULT now(),
                        seen BOOL NOT NULL DEFAULT FALSE
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
                message: "The sender is not in the database".to_string()
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
                message: "The receiver is not in the database".to_string()
            };
            return Ok(resp);
        }

        let id_sender = match self
            .client
            .query("SELECT id_user FROM users WHERE username = $1;", &[&sender])
            .await?
            .first()
        {
            Some(r) => r.get(0),
            None => -1,
        };
        let id_receiver = match self
            .client
            .query(
                "SELECT id_user FROM users WHERE username = $1;",
                &[&receiver],
            )
            .await?
            .first()
        {
            Some(r) => r.get(0),
            None => -1,
        };
        if id_sender == -1 || id_receiver == -1 {
            let resp = Response{
                succes: false,
                message: "Eroare in timpul salvarii mesajului".to_string()
            };
            return Ok(resp);
        }
        self.client
            .execute(
                "INSERT INTO messages (content, id_sender, id_receiver) VALUES ($1, $2, $3);",
                &[&message, &id_sender, &id_receiver],
            )
            .await?;
        let resp = Response {
            succes: true,
            message: "Message saved".to_string()
        };
        Ok(resp)
    }
    pub async fn get_messages(
        &self,
        user1: &str,
        user2: &str,
        start: i32,
        finish: i32,
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
        let row = self.client.query(r"SELECT content FROM 
                            (SELECT content, ROWNUM rm FROM 
                            (SELECT * FROM messages m 
                            JOIN users u1 ON m.id_sender = u1.id_user 
                            JOIN users u2 ON m.id_receiver = u2.id_user 
                            WHERE (u1.username = $1 AND u2.username = $2) OR (u1.username = $1 AND u2.username = $2)
                            ORDER BY data ASC)) WHERE rm BETWEEN $3 AND $4;", &[&user1, &user2, &start, &finish]).await?;
        Ok(Some(row))
    }
}
