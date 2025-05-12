use std::fmt::Display;

use thiserror::Error;

#[derive(Debug, Display, Error)]
pub enum CreateChatError {

}

#[derive(Debug, Display, Error)]
pub enum GetChatError {

}

#[derive(Debug, Display, Error)]
pub enum UpdateChatError {

}

#[derive(Debug, Display, Error)]
pub enum GetPictureError {

}