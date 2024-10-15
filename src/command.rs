use crate::cli::Command;
use crate::response::ApiResponse;
use anyhow::Result;
use reqwest::Client;

pub struct GameClient {
    port: u16,
    client: Client,
}

impl GameClient {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            client: Client::new(),
        }
    }

    pub async fn play(&self, command: Command) -> Result<()> {
        match command {
            Command::Deal => self.deal().await?,
            Command::Hit { token } => self.hit(token).await?,
            Command::Stay { token } => self.stay(token).await?,
            Command::Stats => self.stats().await?,
            Command::History { start_date } => self.history(start_date).await?,
            Command::Delete { sure, token } => self.delete_history(sure, token).await?,
        }

        Ok(())
    }

    fn build_url(&self, endpoint: impl AsRef<str>) -> String {
        format!("http://0.0.0.0:{}/{}", self.port, endpoint.as_ref())
    }

    async fn deal(&self) -> Result<()> {
        let resp: ApiResponse = self
            .client
            .post(self.build_url("deal"))
            .send()
            .await?
            .json()
            .await?;

        println!("{resp}");
        Ok(())
    }

    async fn hit(&self, token: Option<String>) -> Result<()> {
        let endpoint = match token {
            Some(tkn) => format!("hit?token={tkn}"),
            None => "hit".to_string(),
        };

        let resp: ApiResponse = self
            .client
            .post(self.build_url(endpoint))
            .send()
            .await?
            .json()
            .await?;

        println!("{resp}");
        Ok(())
    }

    async fn stay(&self, token: Option<String>) -> Result<()> {
        let endpoint = match token {
            Some(tkn) => format!("stay?token={tkn}"),
            None => "stay".to_string(),
        };

        let resp: ApiResponse = self
            .client
            .post(self.build_url(endpoint))
            .send()
            .await?
            .json()
            .await?;

        println!("{resp}");
        Ok(())
    }

    async fn stats(&self) -> Result<()> {
        let resp: ApiResponse = self
            .client
            .get(self.build_url("stats"))
            .send()
            .await?
            .json()
            .await?;

        println!("{resp}");

        Ok(())
    }

    async fn history(&self, start_date: Option<String>) -> Result<()> {
        let endpoint = match start_date {
            Some(sd) => format!("history?start={sd}"),
            None => "history".to_string(),
        };

        let resp: ApiResponse = self
            .client
            .get(self.build_url(endpoint))
            .send()
            .await?
            .json()
            .await?;
        println!("{resp}");

        Ok(())
    }

    async fn delete_history(&self, sure: bool, token: Option<String>) -> Result<()> {
        if !sure {
            println!("Sure flag not set, not deleting history");
        }

        let url = match token {
            Some(tkn) => self.build_url(format!("delete/{tkn}?sure=true")),
            None => self.build_url("delete?sure=true"),
        };

        let resp: ApiResponse = self.client.delete(url).send().await?.json().await?;
        println!("{resp}");

        Ok(())
    }
}
