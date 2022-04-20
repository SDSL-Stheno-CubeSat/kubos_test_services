use crate::model::{Image, Resolution, Subsystem, Camera};
use juniper::FieldResult;
use kubos_service;
use log::info;

type Context = kubos_service::Context<Subsystem>;

// GraphQL model for Resolution
// Can eliminate the resolution field to be an object if x and y fields are not
// required seperately

graphql_object!(Resolution: Context as "Resolution" |&self| {
    description: "Resolution struct"

    field x() -> FieldResult<i32> as "Pixels of resolution in x direction" {
        Ok(self.x as i32)
    }

    field y() -> FieldResult<i32> as "Pixels of resolution in y direction" {
        Ok(self.y as i32)
    }
});

// GraphQL model for Camera

graphql_object!(Camera: Context as "Camera" |&self| {
    description: "Camera struct"

    field started() -> FieldResult<bool> as "Camera started flag" {
        Ok(self.started)
    }

    field resolution() -> FieldResult<&Resolution> as "Current resolution of the camera" {
        Ok(&self.resolution)
    }

});

// GraphQL model for Image

graphql_object!(Image: Context as "Image" |&self| {
    description: "Image struct"

    field resolution() -> FieldResult<&Resolution> as "Resolution of image" {
        Ok(&self.resolution)
    }

    field bytes() -> FieldResult<&Vec<i32>> as "Image byte stream" {
        Ok(&self.bytes)
    }

});

// GraphQL model for Subsystem

graphql_object!(Subsystem: Context as "Subsystem" |&self| {
    description: "Camera Service subsystem"

    field capture() -> FieldResult<Image> as "Image capture query field" {
        Ok(self.capture()?)
    }

});

// Base GraphQL query model
pub struct QueryRoot;

graphql_object!(QueryRoot : Context as "Query" |&self| {

    field subsystem(&executor) -> FieldResult<&Subsystem> as "Subsystem query"
    {
        let num_queries = executor.context().get("num_queries");
        info!("Num queries {}", num_queries);
        let num = num_queries.parse::<i32>().unwrap_or(0) + 1;
        executor.context().set("num_queries", &format!("{}", num));
        Ok(executor.context().subsystem())
    }

});

pub struct MutationRoot;

// Base GraphQL mutation model
graphql_object!(MutationRoot : Context as "Mutation" |&self| {

    field start_camera(&executor) -> FieldResult<bool>
        as "Start the camera"
    {
        Ok(executor.context().subsystem().start_camera()?)
    }

    field stop_camera(&executor) -> FieldResult<bool>
        as "Stop the camera"
    {
        Ok(executor.context().subsystem().stop_camera()?)
    }

    // TODO: Figure why can't get resolution as a object in graphql query arguments
    field set_resolution(&executor, x: i32, y: i32) -> FieldResult<bool>
        as "Set resolution"
    {
        Ok(executor.context().subsystem().set_resolution(x, y)?)
    }

});