use crate::model::{CailbrateThermometer, ResetUptime, SetPower, Subsystem};
use juniper::FieldResult;
use kubos_service;
use log::info;

type Context = kubos_service::Context<Subsystem>;