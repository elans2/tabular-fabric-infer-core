use arrow::array::StringArray;
use arrow::array::UInt64Array;
use arrow::record_batch::RecordBatch;

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tabular_fabric_cascade_infer::base::{GeneralInnerModelInfer, InferContext};
use tabular_fabric_cascade_infer::models::mistral::CandleMistralModelInfer;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let infer = CandleMistralModelInfer::new();

    let batch = RecordBatch::try_from_iter(vec![(
        "col",
        Arc::new(StringArray::from(vec![
            //"Here is a sample quick sort implementation in rust".to_string(),
            //"Here is a sample quick sort implementation in rust".to_string(),
            "<sentence>TAKE BACK RETURN</sentence>, split sentence, take first word".to_string(),
        ])) as _,
    )])
    .unwrap();

    let infer_context = InferContext::default();

    infer.load(HashMap::new());
    infer.infer(&batch, &infer_context, HashMap::new());

    Ok(())
}
