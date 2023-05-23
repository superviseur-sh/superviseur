use std::{
    collections::HashMap,
    net::SocketAddr,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::{
    api::superviseur::v1alpha1::{
        control_service_server::ControlServiceServer, core_service_server::CoreServiceServer,
        logging_service_server::LoggingServiceServer, project_service_server::ProjectServiceServer,
    },
    core::Core,
    {control::Control, logging::Logging, project::Project},
};
use anyhow::Error;
use owo_colors::OwoColorize;
use superviseur_core::{core::Superviseur, dependencies::DependencyGraph};
use superviseur_log::log::LogEngine;
use superviseur_provider::kv::kv::Provider;
use superviseur_types::{configuration::Service, process::Process, BANNER, UNIX_SOCKET_PATH};
use tokio::net::UnixListener;
use tokio_stream::wrappers::UnixListenerStream;
use tonic::transport::Server;

pub async fn exec(port: u16, serve: bool) -> Result<(), Error> {
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();
    println!("{}", BANNER.bright_purple());
    println!(
        "Listening on {}{} 🚀",
        "unix:".cyan(),
        UNIX_SOCKET_PATH.cyan()
    );
    if serve {
        println!("Listening on {} 🚀", addr.cyan());
    }

    let project_map = Arc::new(Mutex::new(HashMap::new()));
    let provider = Arc::new(Provider::default());
    let (cmd_tx, cmd_rx) = tokio::sync::mpsc::unbounded_channel();
    let (event_tx, events) = tokio::sync::mpsc::unbounded_channel();
    let processes = Arc::new(Mutex::new(vec![] as Vec<(Process, String)>));
    let cmd_rx = Arc::new(Mutex::new(cmd_rx));
    let service_graph = Arc::new(Mutex::new(vec![] as Vec<(DependencyGraph, String)>));
    let service_map = Arc::new(Mutex::new(vec![] as Vec<(HashMap<usize, Service>, String)>));
    let log_engine = Arc::new(Mutex::new(LogEngine::new()));
    let (superviseur_events_tx, superviseur_events_rx) = tokio::sync::mpsc::unbounded_channel();
    let superviseur_events_rx = Arc::new(tokio::sync::Mutex::new(superviseur_events_rx));

    let superviseur = Superviseur::new(
        cmd_rx,
        cmd_tx.clone(),
        event_tx.clone(),
        events,
        processes.clone(),
        provider.clone(),
        service_graph.clone(),
        service_map.clone(),
        log_engine.clone(),
        superviseur_events_tx.clone(),
    );

    let cloned_cmd_tx = cmd_tx.clone();
    let cloned_event_tx = event_tx.clone();
    let cloned_superviseur = superviseur.clone();
    let cloned_processes = processes.clone();
    let cloned_project_map = project_map.clone();
    let cloned_provider = Arc::clone(&provider);
    let cloned_log_engine = log_engine.clone();
    let cloned_superviseur_events_rx = superviseur_events_rx.clone();

    // create a one-shot channel to wait for the server to start
    let (tx, rx) = tokio::sync::oneshot::channel::<bool>();

    tokio::spawn(async move {
        let socket_path = PathBuf::from(UNIX_SOCKET_PATH);

        if socket_path.exists() {
            std::fs::remove_file(&socket_path).unwrap();
        }

        let listener = UnixListener::bind(&socket_path).unwrap();

        Server::builder()
            .accept_http1(true)
            .add_service(tonic_web::enable(LoggingServiceServer::new(Logging::new(
                cloned_superviseur.clone(),
                cloned_processes.clone(),
                cloned_provider.clone(),
                cloned_project_map.clone(),
                cloned_log_engine.clone(),
                cloned_superviseur_events_rx.clone(),
            ))))
            .add_service(tonic_web::enable(ControlServiceServer::new(Control::new(
                cloned_cmd_tx.clone(),
                cloned_event_tx.clone(),
                cloned_superviseur.clone(),
                cloned_processes.clone(),
                cloned_provider.clone(),
                cloned_project_map.clone(),
            ))))
            .add_service(tonic_web::enable(CoreServiceServer::new(Core::new(
                cloned_cmd_tx.clone(),
                cloned_event_tx,
                cloned_superviseur,
                cloned_processes.clone(),
                cloned_provider.clone(),
                cloned_project_map.clone(),
            ))))
            .add_service(tonic_web::enable(ProjectServiceServer::new(Project::new(
                cloned_cmd_tx,
                cloned_processes,
                cloned_provider,
                cloned_project_map,
            ))))
            .serve_with_incoming(UnixListenerStream::new(listener))
            .await
            .unwrap();
        tx.send(true).unwrap();
    });

    if serve {
        Server::builder()
            .accept_http1(true)
            .add_service(tonic_web::enable(LoggingServiceServer::new(Logging::new(
                superviseur.clone(),
                processes.clone(),
                provider.clone(),
                project_map.clone(),
                log_engine.clone(),
                superviseur_events_rx.clone(),
            ))))
            .add_service(tonic_web::enable(ControlServiceServer::new(Control::new(
                cmd_tx.clone(),
                event_tx.clone(),
                superviseur.clone(),
                processes.clone(),
                provider.clone(),
                project_map.clone(),
            ))))
            .add_service(tonic_web::enable(CoreServiceServer::new(Core::new(
                cmd_tx.clone(),
                event_tx,
                superviseur,
                processes.clone(),
                provider.clone(),
                project_map.clone(),
            ))))
            .add_service(tonic_web::enable(ProjectServiceServer::new(Project::new(
                cmd_tx,
                processes,
                provider,
                project_map,
            ))))
            .serve(addr)
            .await?;
    }

    // wait for the server to start
    rx.await.unwrap();

    Ok(())
}
