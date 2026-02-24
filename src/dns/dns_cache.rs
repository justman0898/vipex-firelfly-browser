use redis::AsyncCommands;
use std::net::SocketAddr;
use std::time::Duration;
use redis::{Client, Commands, RedisResult};
use crate::exceptions::custom_error::AppError;

pub struct DnsCache {
    client: Client,
    ttl: Duration,
}

impl DnsCache {
    pub fn new() -> Result<Self, AppError> {
        let redis_url = std::env::var("REDIS_URL")?;
        let dns_ttl_secs = std::env::var("DNS_TTL_SECS")?;

        let redis_client: Client = redis::Client::open(redis_url)?;
        let ttl: Duration = {
            Duration::from_secs(dns_ttl_secs.parse::<u64>()?)
        };

        Ok(
            Self{
            client: redis_client,
            ttl,
            }
        )
    }

    pub async fn get(&self, key: &str) -> Option<Vec<SocketAddr>> {
        let mut connection = self.client.get_async_connection()
            .await
            .ok()?;

        let value: Option<String> = connection.get(key)
            .await
            .ok()?;

        if let Some(json) = value {
            serde_json::from_str(&json).ok()
        }else {
            None
        }
    }

    pub async fn insert(&self, key: &str, addrs: &Vec<SocketAddr>) {
        if let Ok(mut connection) = self.client.get_async_connection().await {
            if let Ok(json) = serde_json::to_string(&addrs) {
                let _: RedisResult<()> = connection.set_ex(key, json, self.ttl.as_secs())
                .await;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::dotenv;
    use std::env;

    #[tokio::test]
    async fn test_dns_cache() {
        dotenv().ok();
        let cache = DnsCache::new().unwrap();
        let addrs: Vec<SocketAddr> = ["74.6.231.20:43", "74.6.143.25:43", "98.137.11.163:43", "98.137.11.164:43", "74.6.143.26:43", "74.6.231.21:43"]
            .iter()
            .map(|socket_addr| socket_addr.parse().unwrap())
        .collect();

        cache.insert("yahoo.com", &addrs).await;

        let value = cache.get("yahoo.com").await.unwrap();
        eprintln!("{:?}", value);
        assert_eq!(value, addrs);




    }
}