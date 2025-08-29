use std::fs::File;
use std::sync::Arc;

use arrow::array::{StringArray, Int64Array};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use parquet::arrow::ArrowWriter;
use parquet::file::properties::WriterProperties;

use crate::storage::types::ChatMessage;

pub fn export_to_parquet(messages: Vec<ChatMessage>, path: &str) -> anyhow::Result<()> {
    let schema = Arc::new(Schema::new(vec![
        Field::new("user", DataType::Utf8, false),
        Field::new("room", DataType::Utf8, false),
        Field::new("text", DataType::Utf8, false),
        Field::new("timestamp", DataType::Int64, false),
    ]));

    let users: Vec<&str> = messages.iter().map(|m| m.user.as_str()).collect();
    let rooms: Vec<&str> = messages.iter().map(|m| m.room.as_str()).collect();
    let texts: Vec<&str> = messages.iter().map(|m| m.text.as_str()).collect();
    let times: Vec<i64> = messages.iter().map(|m| m.timestamp).collect();

    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(StringArray::from(users)),
            Arc::new(StringArray::from(rooms)),
            Arc::new(StringArray::from(texts)),
            Arc::new(Int64Array::from(times)),
        ],
    )?;

    let file = File::create(path)?;
    let props = WriterProperties::builder().build();
    let mut writer = ArrowWriter::try_new(file, schema, Some(props))?;

    writer.write(&batch)?;
    writer.close()?;

    Ok(())
}
