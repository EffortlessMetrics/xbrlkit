//! Cockpit envelope helpers.

use receipt_types::Receipt;

#[must_use]
pub fn to_sensor_report(sensor_id: &str, receipt: &Receipt) -> serde_json::Value {
    serde_json::json!({
      "kind": "sensor.report",
      "version": "v1",
      "subject": receipt.subject,
      "result": format!("{:?}", receipt.result).to_ascii_lowercase(),
      "sensor_id": sensor_id,
      "inner_receipt": receipt,
    })
}
