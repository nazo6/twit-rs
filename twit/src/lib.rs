#![doc = include_str!("../../README.md")]

pub mod auth;
pub mod clients;
pub mod models;

pub type Query = Vec<(String, String)>;
