use std::fs::File;
use std::sync::Arc;

use arrow::array::{Float64Array, UInt64Array, TimestampMillisecondArray};
use arrow::datatypes::{DataType, Field, Schema, TimeUnit};
use arrow::record_batch::RecordBatch;
use parquet::arrow::ArrowWriter;

use crate::features::window::WindowFeatures;

pub fn write_parquet(
    path: &str,
    windows: &[WindowFeatures],
) -> Result<(), Box<dyn std::error::Error>> {
    let schema = Arc::new(Schema::new(vec![
        Field::new(
            "window_start",
            DataType::Timestamp(TimeUnit::Millisecond, None),
            false,
        ),
        Field::new(
            "window_end",
            DataType::Timestamp(TimeUnit::Millisecond, None),
            false,
        ),
        Field::new("request_count", DataType::UInt64, false),
        Field::new("avg_latency", DataType::Float64, false),
    ]));
    
    let start = TimestampMillisecondArray::from(
        windows
            .iter()
            .map(|w| w.window_start.timestamp_millis())
            .collect::<Vec<i64>>(),
    );
    
    let end = TimestampMillisecondArray::from(
        windows
            .iter()
            .map(|w| w.window_end.timestamp_millis())
            .collect::<Vec<i64>>(),
    );
    
    let counts = UInt64Array::from(
        windows
            .iter()
            .map(|w| w.request_count as u64)
            .collect::<Vec<u64>>(),
    );
    
    let latency = Float64Array::from(
        windows
            .iter()
            .map(|w| w.avg_latency)
            .collect::<Vec<f64>>(),
    );
    
    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(start),
            Arc::new(end),
            Arc::new(counts),
            Arc::new(latency),
        ],
    )?;
    
    let file = File::create(path)?;
    let mut writer = ArrowWriter::try_new(file, schema, None)?;
    writer.write(&batch)?;
    writer.close()?;
    
    Ok(())
}