use async_graphql::Schema;
use schema::{Mutation, Query, Subscription};

pub mod macros;
pub mod schema;
pub mod simple_broker;

pub type SuperviseurSchema = Schema<Query, Mutation, Subscription>;
