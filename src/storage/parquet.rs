use std::{fs::File, sync::Arc};
use actix_web::web::Bytes;
use arrow::{array::{Int64Array, StringArray}, datatypes::{DataType, Field, Schema}, record_batch::RecordBatch};
use parquet::{arrow::{arrow_reader::ParquetRecordBatchReaderBuilder, ArrowWriter}, file::properties::WriterProperties};

use crate::models::chat_message::Message;

pub fn read_from_bytes(bytes: Bytes) -> anyhow::Result<Vec<Message>> {
    let mut arrow_reader = ParquetRecordBatchReaderBuilder::try_new(bytes)?
        .with_batch_size(1024)
        .build()?;

    let mut messages = Vec::new();

    while let Some(batch) = arrow_reader.next() {
        let batch: RecordBatch = batch?;

        let user_col = batch
            .column(0)
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap();
        let room_col = batch
            .column(1)
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap();
        let text_col = batch
            .column(2)
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap();
        let ts_col = batch
            .column(3)
            .as_any()
            .downcast_ref::<Int64Array>()
            .unwrap();

        for i in 0..batch.num_rows() {
            messages.push(Message {
                user: user_col.value(i).to_string(),
                room: room_col.value(i).to_string(),
                text: text_col.value(i).to_string(),
                timestamp: ts_col.value(i),
            });
        }
    }

    Ok(messages)
}

pub fn write_file(messages: Vec<Message>, path: &str) -> Result<(),parquet::errors::ParquetError>{
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::fs;

    #[test]
    fn test_write_and_read_parquet() {
        let messages = vec![
            Message {
                user: "alice".to_string(),
                room: "general".to_string(),
                text: "hello".to_string(),
                timestamp: 123,
            },
            Message {
                user: "bob".to_string(),
                room: "general".to_string(),
                text: "hi".to_string(),
                timestamp: 456,
            },
        ];

        let tmpfile = NamedTempFile::new().unwrap();
        let path = tmpfile.path().to_str().unwrap();

        write_file(messages.clone(), path).unwrap();

        let raw = fs::read(path).unwrap();
        let bytes = Bytes::from(raw);

        let result = read_from_bytes(bytes).unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].user, "alice");
        assert_eq!(result[0].text, "hello");
        assert_eq!(result[1].user, "bob");
        assert_eq!(result[1].text, "hi");
    }
}
