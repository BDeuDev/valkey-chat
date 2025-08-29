use std::fs::File;

use arrow::record_batch::RecordBatch;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

use crate::storage::types::ChatMessage;

pub fn read_parquet_history(path: &str) -> anyhow::Result<Vec<ChatMessage>> {
    // Abrimos el archivo directamente como `File`
    let file = File::open(path)?;

    // Creamos el builder directamente desde File
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
