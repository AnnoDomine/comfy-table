use comfy_table::{presets::UTF8_FULL, *};

fn main() {
    // 1. Define the innermost inline table first (Inside-Out Best Practice)
    let mut metrics_table = InlineTable::new();
    metrics_table
        .set_header(vec!["Component", "Latency", "Status"])
        .add_row(vec!["Cache Tier", "0.4 ms", "Healthy"])
        .add_row(vec!["Storage Tier", "2.1 ms", "Healthy"]);

    // 2. Define the intermediate section and nest the inner table
    let mut service_section = InlineTable::new();
    service_section
        .set_header(vec!["Service", "Region"])
        .add_row(vec!["Authentication API", "eu-central-1"])
        .add_inline_table(metrics_table);

    // 3. Assemble the outer table
    let mut table = Table::new();
    table
        .load_style(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::DynamicFullWidth)
        .set_width(70)
        .set_header(vec!["Cluster", "Environment"])
        .add_row(vec!["Production-Core", "Live"])
        .add_inline_table(service_section)
        .add_row(vec!["Staging-Core", "Idle"]);

    println!("{table}");
}
