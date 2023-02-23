use std::{env, str::FromStr};

use config::{Config, File, FileFormat};
use tonic::{transport::Server, Status, Response, Request};
use tracing::{metadata::LevelFilter, info};
use tracing_subscriber::{prelude::*, Layer, filter};
use voting::{VotingRequest, VotingResponse, GetVotesNumberRequest, GetVotesNumberResponse, voting_server::{Voting, VotingServer}};

mod custom_tracing_layer;
use custom_tracing_layer::CustomTracingLayer;

pub mod voting {
    tonic::include_proto!("voting");
}

#[derive(Debug, Default)]
pub struct VotingService {}

#[tonic::async_trait]
impl Voting for VotingService {
    async fn vote(&self, request: Request<VotingRequest>) -> Result<Response<VotingResponse>, Status> {
        info!("Received vote request...");
        let r = request.into_inner();
        match r.vote {
        0 => Ok(Response::new(voting::VotingResponse { confirmation: { 
            format!("Happy to confirm that you upvoted for {}", r.url)
        }})),
        1 => Ok(Response::new(voting::VotingResponse { confirmation: { 
            format!("Confirmation that you downvoted for {}", r.url)
        }})), 
        _ => Err(Status::new(tonic::Code::OutOfRange, "Invalid vote provided"))
        }
    }

    async fn get_votes_number(&self, request: Request<GetVotesNumberRequest>) -> Result<Response<GetVotesNumberResponse>, Status>  {
        let r = request.into_inner();
        info!("Received get votes request for: {}", r.url);
        // TODO: make some in-service counting, fake for now
        Ok(Response::new(voting::GetVotesNumberResponse { up: 5, down: 10}))
    }
}

/// Makes merged logging config. WIll panic if ENVROLE env variable is not set.
fn make_log_config() -> Config {
    let env_role = env::var("ENVROLE").unwrap();
    Config::builder()
        .add_source(File::new("env/logging", FileFormat::Yaml))
        .add_source(File::new(
            ("env/logging-".to_string() + &env_role).as_str(),
            FileFormat::Yaml,
        ))
        .build()
        .unwrap()
}

/// Makes merged main config. WIll panic if ENVROLE env variable is not set.
fn make_config() -> Config {
    let env_role = env::var("ENVROLE").unwrap();
    Config::builder()
        .add_source(File::new("env/service", FileFormat::Yaml))
        .add_source(File::new(
            ("env/service-".to_string() + &env_role).as_str(),
            FileFormat::Yaml,
        ))
        .build()
        .unwrap()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let log_config = make_log_config();
    // Enables spans and events with levels `INFO` and below:
    // let level_filter = LevelFilter::INFO;
    let level_str = log_config.get_string("level").unwrap();
    let level_filter: LevelFilter = match FromStr::from_str(&level_str) {
        Ok(filter) => filter,
        Err(error) => {
            panic!("Problem parsing log level: {error:?}, supplied level: {level_str}");
        }
    };
    // Set up how `tracing-subscriber` will deal with tracing data.
    tracing_subscriber::registry()
        .with(
            CustomTracingLayer
                .with_filter(level_filter)
                .with_filter(filter::filter_fn(|metadata| {
                    metadata.target().eq("service")
                })),
        )
        .init();

    info!(
        "Starting logging at level: {}, for env role: {}",
        &level_str,
        env::var("ENVROLE").unwrap()
    );
    let config = make_config();
    let reflection: bool = config.get("reflection").unwrap();
    info!(
        "Reflection configured as: {}",
        &reflection
    );
    // TODO: switch later back to IPv6?
    let port = config.get_int("port").unwrap();
    let address = format!("0.0.0.0:{port}").parse().unwrap();
    info!(
        "gRPC will serve on port: {}",
        &port
    );

    let voting_service = VotingService::default();

    if reflection {
        let reflection_servicer = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(tonic::include_file_descriptor_set!("protos_descriptor"))
        .build()
        .expect("failed to create gRPC reflection servicer");   
        
        // start the grpc server with reflection
        Server::builder()
            .add_service(VotingServer::new(voting_service))
            .add_service(reflection_servicer)
            // .serve_with_shutdown(address, signal())
            .serve(address)
            .await?;
    } else {
        // start the grpc server without reflection
        Server::builder()
            .add_service(VotingServer::new(voting_service))
            // .serve_with_shutdown(address, signal())
            .serve(address)
            .await?;
    }
    Ok(())
}
