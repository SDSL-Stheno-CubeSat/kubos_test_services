use crate::model::{Subsystem, TxSuccess, RxReading, CalibrateThermometer};
use juniper::FieldResult;
use kubos_service;
use log::info;

type Context = kubos_service::Context<Subsystem>;

graphql_object!(Subsystem: Context as "Subsystem" |&self| {
    description: "Service subsystem"

    // Queries
    field power() -> FieldResult<bool> as "Status of power" {
        Ok(self.power()?)
    }

    field temperature() -> FieldResult<i32> as "Temperature value" {
        Ok(self.temperature()?)
    }
});

// Mutation returns

graphql_object!(TxSuccess: Context as "TxSuccess" |&self| {
    description: "Set UART transmission success return"

    field success() -> FieldResult<bool> as "Successful transmit" {
        Ok(self.success)
    }
});


/*
pub struct SetPower {
    pub power: bool,
}
*/
graphql_object!(SetPower: Context as "SetPower" |&self| {
    description: "Set Power return"

    field power() -> FieldResult<bool> as "Power setting" {
        Ok(self.power)
    }
});

graphql_object!(CalibrateThermometer: Context as "Calibrate Thermometer" |&self| {
    description: "Calibrate Thermometer Success"

    field success() -> FieldResult<bool> as "Calibrate Thermometer Success" {
        Ok(self.success)
    }
});

// Mutation returns Over

pub struct QueryRoot;

// Utilize Queries from above
graphql_object!(QueryRoot : Context as "Query" |&self| {
    field subsystem(&executor) -> FieldResult<&Subsystem> as "Subsystem query"
    {
        let num_queries = executor.context().get("num_queries");
        info!("Num queries {}", num_queries);
        let num = num_queries.parse::<i32>().unwrap_or(0) + 1;
        executor.context().set("num_queries", &format!("{}", num));
        Ok(executor.context().subsystem())
    }

    // Add extra "ping" Query for testing
    field ping() -> FieldResult<String>
    {
        Ok(String::from("pong"))
    }
});

pub struct MutationRoot;

graphql_object!(MutationRoot : Context as "Mutation" |&self| {

    // Mutation commands

    field set_power(&executor, power : bool) -> FieldResult<SetPower> 
        as "Set subsystem power state"
    {
        // .set_power from the model
        Ok(executor.context().subsystem().set_power(power)?)
    }

    field calibrate_thermometer(&executor) -> FieldResult<CalibrateThermometer>
        as "Calibrate subsystem thermometer"
    {
        Ok(executor.context().subsystem().calibrate_thermometer()?)
    }

    field commandRaw(&executor, data : String) -> FieldResult<TxSuccess>
        as "Send raw command to subsystem"
    {
        Ok(executor.context().subsystem().uart_tx(data))
    }

});