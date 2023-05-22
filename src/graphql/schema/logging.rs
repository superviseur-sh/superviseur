use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    sync::{Arc, Mutex},
    thread,
};

use async_graphql::{async_stream::stream, *};
use futures_util::Stream;
use tokio_stream::StreamExt;

use crate::{
    default_stdout,
    graphql::{schema::objects::subscriptions::TailLogStream, simple_broker::SimpleBroker},
    types::configuration::ConfigurationData,
    util::read_lines,
};

use super::objects::{
    log::Log,
    subscriptions::{self, LogStream},
};

#[derive(Default, Clone)]
pub struct LoggingQuery;

#[Object]
impl LoggingQuery {
    async fn tail(
        &self,
        ctx: &Context<'_>,
        id: ID,
        num_lines: Option<usize>,
        project_id: ID,
    ) -> Result<Log, Error> {
        let project_id = project_id.to_string();
        let config_map = ctx
            .data::<Arc<Mutex<HashMap<String, ConfigurationData>>>>()
            .unwrap();

        let config_map = config_map.lock().unwrap();

        if !config_map.contains_key(&project_id) {
            return Err(Error::new("Config file not found"));
        }

        let config = config_map.get(&project_id).unwrap();

        let (_, service) = config
            .services
            .iter()
            .find(|(_, s)| s.id == Some(id.to_string()))
            .ok_or_else(|| Error::new("Service not found"))?;
        let log_file = File::open(
            &service
                .stdout
                .clone()
                .unwrap_or(default_stdout!(config.project, service.name)),
        )
        .map_err(|e| Error::new(e.to_string()))?;

        let reader = BufReader::new(log_file);

        // Read the last `num_lines` lines of the file
        let mut lines: Vec<String> = reader
            .lines()
            .map(|line| line.unwrap())
            .collect::<Vec<String>>();

        let num_lines = num_lines.unwrap_or(10);
        let tail_lines = lines.split_off(lines.len().saturating_sub(num_lines));

        Ok(Log { lines: tail_lines })
    }

    async fn logs(&self, ctx: &Context<'_>, id: ID, project_id: ID) -> Result<Log, Error> {
        let project_id = project_id.to_string();
        let config_map = ctx
            .data::<Arc<Mutex<HashMap<String, ConfigurationData>>>>()
            .unwrap();

        let config_map = config_map.lock().unwrap();

        if !config_map.contains_key(&project_id) {
            return Err(Error::new("Config file not found"));
        }

        let config = config_map.get(&project_id).unwrap();

        let (_, service) = config
            .services
            .iter()
            .find(|(_, s)| s.id == Some(id.to_string()))
            .ok_or_else(|| Error::new("Service not found"))?;

        let lines = read_lines(
            &service
                .stdout
                .clone()
                .unwrap_or(default_stdout!(config.project, service.name)),
        )?;

        Ok(Log { lines })
    }
}

#[derive(Default, Clone)]
pub struct LoggingSubscription;

#[Subscription]
impl LoggingSubscription {
    async fn tail(
        &self,
        ctx: &Context<'_>,
        id: ID,
        project_id: ID,
    ) -> Result<impl Stream<Item = TailLogStream>, Error> {
        let project_id = project_id.to_string();
        let config_map = ctx
            .data::<Arc<Mutex<HashMap<String, ConfigurationData>>>>()
            .unwrap();

        let config_map = config_map.lock().unwrap();

        if !config_map.contains_key(&project_id) {
            return Err(Error::new("Config file not found"));
        }

        let config = config_map.get(&project_id).unwrap();

        let (_, service) = config
            .services
            .iter()
            .find(|(_, s)| s.id == Some(id.to_string()))
            .ok_or_else(|| Error::new("Service not found"))?;

        let log_file = File::open(
            &service
                .stdout
                .clone()
                .unwrap_or(default_stdout!(config.project, service.name)),
        )
        .map_err(|e| Error::new(e.to_string()))?;

        let reader = BufReader::new(log_file);

        // Read the last `num_lines` lines of the file
        let mut lines: Vec<String> = reader
            .lines()
            .map(|line| line.unwrap())
            .collect::<Vec<String>>();

        let num_lines = 10;
        let tail_lines = lines.split_off(lines.len().saturating_sub(num_lines));

        let cloned_id = id.clone();
        thread::spawn(move || {
            tail_lines.iter().for_each(|line| {
                let log = TailLogStream {
                    id: cloned_id.to_string(),
                    line: line.to_string(),
                };

                SimpleBroker::<TailLogStream>::publish(log);
            });
        });

        Ok(stream! {
            while let Some(log) = SimpleBroker::<TailLogStream>::subscribe().next().await {
                if ID(log.id.clone()) == id {
                    yield log;
                }
            }
        })
    }

    async fn logs(
        &self,
        ctx: &Context<'_>,
        id: ID,
        project_id: ID,
    ) -> Result<impl Stream<Item = subscriptions::LogStream>, Error> {
        let project_id = project_id.to_string();
        let config_map = ctx
            .data::<Arc<Mutex<HashMap<String, ConfigurationData>>>>()
            .unwrap();

        let config_map = config_map.lock().unwrap();

        if !config_map.contains_key(&project_id) {
            return Err(Error::new("Config file not found"));
        }

        let config = config_map.get(&project_id).unwrap();

        let (_, service) = config
            .services
            .iter()
            .find(|(_, s)| s.id == Some(id.to_string()))
            .ok_or_else(|| Error::new("Service not found"))?;

        let log_file = File::open(
            &service
                .stdout
                .clone()
                .unwrap_or(default_stdout!(config.project, service.name)),
        )
        .map_err(|e| Error::new(e.to_string()))?;

        let cloned_id = id.clone();
        thread::spawn(move || {
            let reader = BufReader::new(log_file);

            for line in reader.lines() {
                let line = line.unwrap_or_default();

                let log = subscriptions::LogStream {
                    id: cloned_id.to_string(),
                    line,
                };
                SimpleBroker::<LogStream>::publish(log);
            }
        });

        Ok(stream! {
            while let Some(log) = SimpleBroker::<LogStream>::subscribe().next().await {
                if ID(log.id.clone()) == id {
                    yield log;
                }
            }
        })
    }
}
