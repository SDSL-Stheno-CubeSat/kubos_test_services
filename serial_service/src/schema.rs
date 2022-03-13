use crate::model::{Subsystem, TxSuccess};
use juniper::FieldResult;
use kubos_service;
use log::info;

type Context = kubos_service::Context<Subsystem>;

// GraphQL model for Subsystem
graphql_object!(Subsystem: Context as "Subsystem" |&self| {
    description: "Service subsystem"

    field uart_rx() -> FieldResult<Vec<u8>> as "Reading of subsystem uart" {
        Ok(self.uart_rx()?)
    }
});



pub struct QueryRoot;

// Base GraphQL query model
graphql_object!(QueryRoot : Context as "Query" |&self| {
    field subsystem(&executor) -> FieldResult<&Subsystem>
        as "Subsystem query"
    {
        let num_queries = executor.context().get("num_queries");
        info!("Num queries {}", num_queries);
        let num = num_queries.parse::<i32>().unwrap_or(0) + 1;
        executor.context().set("num_queries", &format!("{}", num));
        Ok(executor.context().subsystem())
    }

    //field ping()
});



pub struct MutationRoot;

// Base GraphQL mutation model
graphql_object!(MutationRoot : Context as "Mutation" |&self| {

    // Each field represents functionality available
    // through the GraphQL mutations
    field uart_tx(&executor, data : String) -> FieldResult<TxSuccess>
        as "Transmit UART data"
    {
        Ok(executor.context().subsystem().uart_tx(data)?)
    }

});
