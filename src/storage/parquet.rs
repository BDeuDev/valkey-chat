use std::{fs::File, sync::Arc};

use arrow::{array::{Int64Array, StringArray}, datatypes::{DataType, Field, Schema}, record_batch::RecordBatch};
use parquet::{arrow::{arrow_reader::ParquetRecordBatchReaderBuilder, ArrowWriter}, file::properties::WriterProperties};

use crate::models::chat_message::ChatMessage;

pub fn read(path: &str) -> anyhow::Result<Vec<ChatMessage>> {
    let file = File::open(path)?;

    let mut arrow_reader = ParquetRecordBatchReaderBuilder::try_new(file)?
        .with_batch_size(1024)
        .build()?;

    let mut messages = Vec::new();

    while let Some(batch) = arrow_reader.next() {
        let batch: RecordBatch = batch?;

        let user_col = batch
            .column(0)
            .as_any()
            .downcast_ref::<arrow::array::StringArray>()
            .unwrap();
        let room_col = batch
            .column(1)
            .as_any()
            .downcast_ref::<arrow::array::StringArray>()
            .unwrap();
        let text_col = batch
            .column(2)
            .as_any()
            .downcast_ref::<arrow::array::StringArray>()
            .unwrap();
        let ts_col = batch
            .column(3)
            .as_any()
            .downcast_ref::<arrow::array::Int64Array>()
            .unwrap();

        for i in 0..batch.num_rows() {
            messages.push(ChatMessage {
                user: user_col.value(i).to_string(),
                room: room_col.value(i).to_string(),
                text: text_col.value(i).to_string(),
                timestamp: ts_col.value(i),
            });
        }
    }

    Ok(messages)
}

pub fn export(messages: Vec<ChatMessage>, path: &str) -> anyhow::Result<()> {
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
