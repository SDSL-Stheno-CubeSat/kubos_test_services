use crate::model::{CalibrateThermometer, ResetUptime, SetPower, Subsystem};
use juniper::FieldResult;
use kubos_service;
use log::info;

type Context = kubos_service::Context<Subsystem>;

// GraphQL model for Subsystem
graphql_object!(Subsystem: Context as "Subsystem" |&self| {
    description: "Service subsystem"

    field power() -> FieldResult<bool> as "Power state of subsystem" {
        Ok(self.power()?)
    }

    field uptime() -> FieldResult<i32> as "Uptime of subsystem" {
        Ok(self.uptime()?)
    }

    field temperature() -> FieldResult<i32> as "Temperature of subsystem" {
        Ok(self.temperature()?)
    }
});

// GraphQL model for CalibrateThermometer return
graphql_object!(CalibrateThermometer: Context as "CalibrateThermometer" |&self| {
    description: "Calibrating thermometer return"

    field temperature() -> FieldResult<i32> as "Temp of subsystem" {
        Ok(self.temperature)
    }
});

// GraphQL model for ResetUptime return
graphql_object!(ResetUptime: Context as "ResetUptime" |&self| {
    description: "Reset uptime return"

    field uptime() -> FieldResult<i32> as "Uptime of subsystem" {
        Ok(self.uptime)
    }
});

// GraphQL model for SetPower return
graphql_object!(SetPower: Context as "SetPower" |&self| {
    description: "Enable Power Return"

    field power() -> FieldResult<bool> as "Power state of subsystem" {
        Ok(self.power)
    }
});

pub struct QueryRoot;

// Base GraphQL query model
graphql_object!(QueryRoot : Context as "Query" |&self| {
    field subsystem(&executor) -> FieldResult<&Subsystem> as "Subsystem query"
    {
        let num_queries = executor.context().get("num_queries");
        info!("Num queries {}", num_queries);
        let num = num_queries.parse::<i32>().unwrap_or(0) + 1;
        executor.context().set("num_queries", &format!("{}", num));
        Ok(executor.context().subsystem())
    }

    field ping() -> FieldResult<String>
    {
        Ok(String::from("pong"))
    }
});

pub struct MutationRoot;

// Base GraphQL mutation model
graphql_object!(MutationRoot : Context as "Mutation" |&self| {

    // Each field represents functionality available
    // through the GraphQL mutations
    field set_power(&executor, power : bool) -> FieldResult<SetPower>
        as "Set subsystem power state"
    {
        Ok(executor.context().subsystem().set_power(power)?)
    }

    field reset_uptime(&executor) -> FieldResult<ResetUptime>
        as "Resets uptime counter of subsystem"
    {
        Ok(executor.context().subsystem().reset_uptime()?)
    }

    field calibrate_thermometer(&executor) -> FieldResult<CalibrateThermometer>
        as "Calibrate thermometer"
    {
        Ok(executor.context().subsystem().calibrate_thermometer()?)
    }

});