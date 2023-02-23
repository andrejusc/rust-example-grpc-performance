use chrono::prelude::{DateTime, Utc};
use std::{collections::BTreeMap, time::SystemTime};
use tracing_subscriber::Layer;

fn iso8601(st: &std::time::SystemTime) -> String {
    let dt: DateTime<Utc> = (*st).into();
    // formats like "2023-01-27T20:28:50.004726747Z"
    format!("{dt:?}")
}

pub struct CustomTracingLayer;

impl<S> Layer<S> for CustomTracingLayer
where
    S: tracing::Subscriber,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        // println!("Got event!");
        // println!("  level={:?}", event.metadata().level());
        // println!("  target={:?}", event.metadata().target());
        // println!("  name={:?}", event.metadata().name());
        // for field in event.fields() {
        //     println!("  field={}", field.name());
        // }
        // let mut visitor = PrintlnVisitor;
        // event.record(&mut visitor);

        // Convert the values into a JSON object
        let mut fields = BTreeMap::new();
        let mut visitor = JsonVisitor(&mut fields);
        event.record(&mut visitor);

        // Output the event in JSON
        let output = serde_json::json!({
          "timestamp": iso8601(&SystemTime::now()),
          "level": event.metadata().level().as_str(),
          "target": event.metadata().target(),
          "name": event.metadata().name(),
          "fields": fields,
        });
        // println!("{}", serde_json::to_string_pretty(&output).unwrap());
        println!("{}", serde_json::to_string(&output).unwrap());
    }
}

struct PrintlnVisitor;

impl tracing::field::Visit for PrintlnVisitor {
    fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
        println!("  field={} value={}", field.name(), value)
    }

    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        println!("  field={} value={}", field.name(), value)
    }

    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        println!("  field={} value={}", field.name(), value)
    }

    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        println!("  field={} value={}", field.name(), value)
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        println!("  field={} value={}", field.name(), value)
    }

    fn record_error(
        &mut self,
        field: &tracing::field::Field,
        value: &(dyn std::error::Error + 'static),
    ) {
        println!("  field={} value={}", field.name(), value)
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        println!("  field={} value={:?}", field.name(), value)
    }
}

struct JsonVisitor<'a>(&'a mut BTreeMap<String, serde_json::Value>);

impl<'a> tracing::field::Visit for JsonVisitor<'a> {
    fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        self.0
            .insert(field.name().to_string(), serde_json::json!(value));
    }

    fn record_error(
        &mut self,
        field: &tracing::field::Field,
        value: &(dyn std::error::Error + 'static),
    ) {
        self.0.insert(
            field.name().to_string(),
            serde_json::json!(value.to_string()),
        );
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        self.0.insert(
            field.name().to_string(),
            serde_json::json!(format!("{value:?}")),
        );
    }
}
