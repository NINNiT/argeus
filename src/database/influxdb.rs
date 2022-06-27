use influx_db_client::{point, Client, Error, Point, Points, Precision, Value};

use crate::endpoint::MonitoringEndpoint;

pub async fn generate_point(monitoring_endpoint: &MonitoringEndpoint) -> Result<Point, Error> {
    let point = point!("metric_data")
        .add_tag("url", monitoring_endpoint.url().await)
        .add_field(
            "status",
            Value::Integer(monitoring_endpoint.status().await.unwrap().as_u16().into()),
        )
        .add_field(
            "response_time",
            Value::Float(
                monitoring_endpoint
                    .response_time()
                    .await
                    .unwrap()
                    .as_secs_f64(),
            ),
        );

    Ok(point)
}

pub async fn send_points(points: Points) -> Result<(), Error> {
    let client = Client::default().set_authentication("root", "root");
    client
        .write_points(points, Some(Precision::Seconds), None)
        .await?;
    Ok(())
}
