use std::iter::{repeat_n};

use comfy_table::{presets::{ASCII_BORDERS_ONLY, ASCII_BORDERS_ONLY_CONDENSED, ASCII_FULL_CONDENSED, ASCII_HORIZONTAL_ONLY, ASCII_MARKDOWN, ASCII_NO_BORDERS, NOTHING, UTF8_BORDERS_ONLY, UTF8_FULL, UTF8_FULL_CONDENSED, UTF8_HORIZONTAL_ONLY, UTF8_NO_BORDERS}, *};

const WIDTH: u16 = 70u16;

fn simple(style: TableStyle) {
    // 1. Define the innermost inline table first (Inside-Out Best Practice)
    let mut metrics_table = InlineTable::new();
    metrics_table
        .set_header(vec!["Component", "Latency", "Status"])
        .add_row(vec!["Cache Tier", "0.4 ms", "Healthy"])
        .add_row(vec!["Storage Tier", "2.1 ms", "Healthy"])
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
        .load_style(style)
        .set_content_arrangement(ContentArrangement::DynamicFullWidth)
        .set_width(WIDTH)
        .set_header(vec!["Cluster", "Environment"])
        .add_row(vec!["Production-Core", "Live"])
        .add_inline_table(service_section)
        .add_row(vec!["Staging-Core", "Idle"]);

    println!("{table}");
}

fn below_header(style: TableStyle) {
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
        .load_style(style)
        .set_content_arrangement(ContentArrangement::DynamicFullWidth)
        .set_width(WIDTH)
        .set_header(vec!["Cluster", "Environment"])
        .add_inline_table(service_section)
        .add_row(vec!["Production-Core", "Live"])
        .add_row(vec!["Staging-Core", "Idle"]);

    println!("{table}");
}

fn as_last_row(style: TableStyle) {
    // 1. Define the innermost inline table first (Inside-Out Best Practice)
    let mut metrics_table = InlineTable::new();
    metrics_table
        .set_header(vec!["Component", "Latency", "Status"])
        .add_row(vec!["Cache Tier", "0.4 ms", "Healthy"])
        .add_row(vec!["Storage Tier", "2.1 ms", "Healthy"])
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
        .load_style(style)
        .set_content_arrangement(ContentArrangement::DynamicFullWidth)
        .set_width(WIDTH)
        .set_header(vec!["Cluster", "Environment"])
        .add_row(vec!["Production-Core", "Live"])
        .add_row(vec!["Staging-Core", "Idle"])
        .add_inline_table(service_section);

    println!("{table}");
}

fn parent_no_header(style: TableStyle) {
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
        .load_style(style)
        .set_content_arrangement(ContentArrangement::DynamicFullWidth)
        .set_width(WIDTH)
        .add_row(vec!["Production-Core", "Live"])
        .add_inline_table(service_section)
        .add_row(vec!["Staging-Core", "Idle"]);

    println!("{table}");
}

fn parent_no_header_as_first_row(style: TableStyle) {
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
        .load_style(style)
        .set_content_arrangement(ContentArrangement::DynamicFullWidth)
        .set_width(WIDTH)
        .add_inline_table(service_section)
        .add_row(vec!["Production-Core", "Live"])
        .add_row(vec!["Staging-Core", "Idle"]);

    println!("{table}");
}

fn parent_no_header_last_row(style: TableStyle) {
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
        .load_style(style)
        .set_content_arrangement(ContentArrangement::DynamicFullWidth)
        .set_width(WIDTH)
        .add_row(vec!["Production-Core", "Live"])
        .add_row(vec!["Staging-Core", "Idle"])
        .add_inline_table(service_section);

    println!("{table}");
}

fn print_tables(style: TableStyle) {
    let seperator: String = repeat_n("=", WIDTH.into()).collect();
    println!("Simple:");
    println!("{:}\n", seperator);
    simple(style);

    println!("\n{:}", seperator);
    println!("Below header:");
    println!("{:}\n", seperator);
    below_header(style);

    println!("\n{:}", seperator);
    println!("As last row:");
    println!("{:}\n", seperator);
    as_last_row(style);

    println!("\n{:}", seperator);
    println!("Without parent header:");
    println!("{:}\n", seperator);
    parent_no_header(style);

    println!("\n{:}", seperator);
    println!("Without parent header as first row:");
    println!("{:}\n", seperator);
    parent_no_header_as_first_row(style);

    println!("\n{:}", seperator);
    println!("Without parent header as last row:");
    println!("{:}\n", seperator);
    parent_no_header_last_row(style);
}

fn main() {
    let seperator: String = repeat_n("=", WIDTH.into()).collect();
    println!("\n{:}", seperator);
    println!("ASCII_NO_BORDERS:");
    print_tables(ASCII_NO_BORDERS);
    println!("\n{:}", seperator);
    println!("ASCII_BORDERS_ONLY:");
    print_tables(ASCII_BORDERS_ONLY);
    println!("\n{:}", seperator);
    println!("ASCII_BORDERS_ONLY_CONDENSED:");
    print_tables(ASCII_BORDERS_ONLY_CONDENSED);
    println!("\n{:}", seperator);
    println!("ASCII_FULL_CONDENSED:");
    print_tables(ASCII_FULL_CONDENSED);
    println!("\n{:}", seperator);
    println!("ASCII_HORIZONTAL_ONLY:");
    print_tables(ASCII_HORIZONTAL_ONLY);
    println!("\n{:}", seperator);
    println!("ASCII_MARKDOWN:");
    print_tables(ASCII_MARKDOWN);
    println!("\n{:}", seperator);
    println!("UTF8_FULL:");
    print_tables(UTF8_FULL);
    println!("\n{:}", seperator);
    println!("UTF8_FULL_CONDENSED:");
    print_tables(UTF8_FULL_CONDENSED);
    println!("\n{:}", seperator);
    println!("UTF8_BORDERS_ONLY:");
    print_tables(UTF8_BORDERS_ONLY);
    println!("\n{:}", seperator);
    println!("UTF8_HORIZONTAL_ONLY:");
    print_tables(UTF8_HORIZONTAL_ONLY);
    println!("\n{:}", seperator);
    println!("UTF8_NO_BORDERS:");
    print_tables(UTF8_NO_BORDERS);
    println!("\n{:}", seperator);
    println!("NOTHING:");
    print_tables(NOTHING);
}