use comfy_table::{
    Cell, CellAlignment, ColumnConstraint, ContentArrangement, InlineTable, Table, presets,
};
use pretty_assertions::assert_eq;

#[test]
fn simple_inline_table() {
    let mut table = Table::new();
    let mut inline = InlineTable::new();
    inline
        .set_header(vec!["Sub 1", "Sub 2"])
        .add_row(vec!["Alpha", "Beta"]);

    table
        .set_header(vec!["Col 1", "Col 2"])
        .add_row(vec!["Row 1, Col 1", "Row 1, Col 2"])
        .add_inline_table(inline)
        .add_row(vec!["Row 2, Col 1", "Row 2, Col 2"]);

    let expected = "+--------------+--------------+
| Col 1        | Col 2        |
+=============================+
| Row 1, Col 1 | Row 1, Col 2 |
|--------------+--------------|
+-------+------++
| Sub 1 | Sub 2 |
+===============+
| Alpha | Beta  |
+-------+------++
|--------------+--------------|
| Row 2, Col 1 | Row 2, Col 2 |
+--------------+--------------+";
    assert_eq!(expected, &table.to_string());
}

#[test]
fn multiple_inline_tables() {
    let mut table = Table::new();

    let mut inline1 = InlineTable::new();
    inline1
        .set_header(vec!["Sub A1", "Sub A2"])
        .add_row(vec!["Val A1", "Val A2"]);

    let mut inline2 = InlineTable::new();
    inline2
        .set_header(vec!["Sub B1", "Sub B2"])
        .add_row(vec!["Val B1", "Val B2"]);

    table
        .set_header(vec!["Col 1", "Col 2"])
        .add_row(vec!["First row 1", "First row 2"])
        .add_inline_table(inline1)
        .add_row(vec!["Second row 1", "Second row 2"])
        .add_inline_table(inline2);

    let expected = "+--------------+--------------+
| Col 1        | Col 2        |
+=============================+
| First row 1  | First row 2  |
|--------------+--------------|
+--------+-----+--+
| Sub A1 | Sub A2 |
+=================+
| Val A1 | Val A2 |
+--------+-----+--+
|--------------+--------------|
| Second row 1 | Second row 2 |
|--------------+--------------|
+--------+-----+--+
| Sub B1 | Sub B2 |
+=================+
| Val B1 | Val B2 |
+--------+--------+";
    assert_eq!(expected, &table.to_string());
}

#[test]
fn inline_table_before_first_row() {
    let mut table = Table::new();
    let mut inline = InlineTable::new();
    inline
        .set_header(vec!["Intro Key", "Intro Val"])
        .add_row(vec!["Version", "1.0.0"]);

    table
        .set_header(vec!["Col 1", "Col 2"])
        .add_inline_table(inline)
        .add_row(vec!["Row 1", "Data 1"]);

    let expected = "+-------+--------+
| Col 1 | Col 2  |
+================+
+-------+---+-----------+
| Intro Key | Intro Val |
+=======================+
| Version   | 1.0.0     |
+-------+---+-----------+
|-------+--------|
| Row 1 | Data 1 |
+-------+--------+";
    assert_eq!(expected, &table.to_string());
}

#[test]
fn inline_table_at_end() {
    let mut table = Table::new();
    let mut inline = InlineTable::new();
    inline
        .set_header(vec!["Footer Key", "Footer Val"])
        .add_row(vec!["Total", "100"]);

    table
        .set_header(vec!["Item", "Count"])
        .add_row(vec!["Apples", "40"])
        .add_row(vec!["Bananas", "60"])
        .add_inline_table(inline);

    let expected = "+---------+-------+
| Item    | Count |
+=================+
| Apples  | 40    |
|---------+-------|
| Bananas | 60    |
|---------+-------|
+---------+--+------------+
| Footer Key | Footer Val |
+=========================+
| Total      | 100        |
+------------+------------+";
    assert_eq!(expected, &table.to_string());
}

#[test]
fn inline_table_utf8_full() {
    let mut table = Table::new();
    table.load_style(presets::UTF8_FULL);

    let mut inline = InlineTable::new();
    inline
        .set_header(vec!["Sub 1", "Sub 2"])
        .add_row(vec!["UTF8 A", "UTF8 B"]);

    table
        .set_header(vec!["Main 1", "Main 2"])
        .add_row(vec!["Row 1", "Data 1"])
        .add_inline_table(inline)
        .add_row(vec!["Row 2", "Data 2"]);

    let expected = "┌────────┬────────┐
│ Main 1 ┆ Main 2 │
╞════════╪════════╡
│ Row 1  ┆ Data 1 │
├╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
╞════════╪════════╡
│ Sub 1  ┆ Sub 2  │
╞════════╪════════╡
│ UTF8 A ┆ UTF8 B │
╞════════╪════════╡
├╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ Row 2  ┆ Data 2 │
└────────┴────────┘";
    assert_eq!(expected, &table.to_string());
}

#[test]
fn inline_table_ascii_borders_only() {
    let mut table = Table::new();
    table.load_style(presets::ASCII_BORDERS_ONLY);

    let mut inline = InlineTable::new();
    inline
        .set_header(vec!["Sub 1", "Sub 2"])
        .add_row(vec!["Data A", "Data B"]);

    table
        .set_header(vec!["Head 1", "Head 2"])
        .add_row(vec!["R1C1", "R1C2"])
        .add_inline_table(inline)
        .add_row(vec!["R2C1", "R2C2"]);

    let expected = "+-----------------+
| Head 1   Head 2 |
+=================+
| R1C1     R1C2   |
|                 |
+=================+
| Sub 1    Sub 2  |
+=================+
| Data A   Data B |
+=================+
|                 |
| R2C1     R2C2   |
+-----------------+";
    assert_eq!(expected, &table.to_string());
}

#[test]
fn inline_table_alignment_and_multiline() {
    let mut table = Table::new();

    let mut inline = InlineTable::new();
    inline
        .set_header(vec![
            Cell::new("Left").set_alignment(CellAlignment::Left),
            Cell::new("Center").set_alignment(CellAlignment::Center),
            Cell::new("Right").set_alignment(CellAlignment::Right),
        ])
        .add_row(vec![
            Cell::new("A\nB").set_alignment(CellAlignment::Left),
            Cell::new("Mid").set_alignment(CellAlignment::Center),
            Cell::new("End\n123").set_alignment(CellAlignment::Right),
        ]);

    table
        .set_header(vec!["Col 1", "Col 2", "Col 3"])
        .add_row(vec!["Row 1", "Row 2", "Row 3"])
        .add_inline_table(inline)
        .add_row(vec!["Next 1", "Next 2", "Next 3"]);

    let expected = "+--------+--------+--------+
| Col 1  | Col 2  | Col 3  |
+==========================+
| Row 1  | Row 2  | Row 3  |
|--------+--------+--------|
+------+-+------+-+-----+
| Left | Center | Right |
+=======================+
| A    |   Mid  |   End |
| B    |        |   123 |
+------+-+------+-+-----+
|--------+--------+--------|
| Next 1 | Next 2 | Next 3 |
+--------+--------+--------+";
    assert_eq!(expected, &table.to_string());
}

#[test]
fn inline_table_constraints() {
    let mut table = Table::new();

    let mut inline = InlineTable::new();
    inline
        .set_header(vec!["Key", "Value"])
        .set_constraints(vec![
            ColumnConstraint::Absolute(comfy_table::Width::Fixed(10)),
            ColumnConstraint::Absolute(comfy_table::Width::Fixed(15)),
        ])
        .add_row(vec!["Short", "A longer description here"]);

    table
        .set_header(vec!["Title", "Detail"])
        .add_row(vec!["Main", "Overview"])
        .add_inline_table(inline);

    let output = table.to_string();
    assert!(output.contains("Short"));
    assert!(output.contains("A longer"));
}

#[test]
fn inline_table_deref_mut_methods() {
    let mut inline = InlineTable::new();
    assert!(inline.is_empty());
    assert_eq!(inline.row_count(), 0);

    inline.set_header(vec!["H1", "H2"]);
    inline.add_row(vec!["R1", "R2"]);
    inline.add_row(vec!["R3", "R4"]);

    assert!(!inline.is_empty());
    assert_eq!(inline.row_count(), 2);
    assert_eq!(inline.column_count(), 2);
}

#[test]
fn inline_table_more_columns() {
    let mut table = Table::new();
    let mut inline = InlineTable::new();
    inline
        .set_header(vec!["A", "B", "C", "D"])
        .add_row(vec!["1", "2", "3", "4"]);

    table
        .set_header(vec!["Left", "Right"])
        .add_row(vec!["Data L", "Data R"])
        .add_inline_table(inline)
        .add_row(vec!["End L", "End R"]);

    let expected = "+--------+--------+
| Left   | Right  |
+=================+
| Data L | Data R |
|--------+--------|
+---+---++--+---+
| A | B | C | D |
+===============+
| 1 | 2 | 3 | 4 |
+---+---++--+---+
|--------+--------|
| End L  | End R  |
+--------+--------+";
    assert_eq!(expected, &table.to_string());
}

#[test]
fn inline_table_fewer_columns() {
    let mut table = Table::new();
    let mut inline = InlineTable::new();
    inline
        .set_header(vec!["Single"])
        .add_row(vec!["Single Value"]);

    table
        .set_header(vec!["Col 1", "Col 2", "Col 3", "Col 4"])
        .add_row(vec!["A", "B", "C", "D"])
        .add_inline_table(inline)
        .add_row(vec!["E", "F", "G", "H"]);

    let expected = "+-------+-------+-------+-------+
| Col 1 | Col 2 | Col 3 | Col 4 |
+===============================+
| A     | B     | C     | D     |
|-------+-------+-------+-------|
+-------+------+
| Single       |
+==============+
| Single Value |
+-------+------+
|-------+-------+-------+-------|
| E     | F     | G     | H     |
+-------+-------+-------+-------+";
    assert_eq!(expected, &table.to_string());
}

#[test]
fn inline_in_inline() {
    let mut outer = Table::new();
    let mut mid = InlineTable::new();
    let mut inner = InlineTable::new();

    inner
        .set_header(vec!["Inn 1", "Inn 2"])
        .add_row(vec!["Deep 1", "Deep 2"]);

    mid.set_header(vec!["Mid 1", "Mid 2"])
        .add_row(vec!["Mid Data 1", "Mid Data 2"])
        .add_inline_table(inner);

    outer
        .set_header(vec!["Out 1", "Out 2"])
        .add_row(vec!["Outer 1", "Outer 2"])
        .add_inline_table(mid)
        .add_row(vec!["Final 1", "Final 2"]);

    let expected = "+---------+---------+
| Out 1   | Out 2   |
+===================+
| Outer 1 | Outer 2 |
|---------+---------|
+---------+--+------------+
| Mid 1      | Mid 2      |
+=========================+
| Mid Data 1 | Mid Data 2 |
|------------+------------|
+--------+---+----+
| Inn 1  | Inn 2  |
+=================+
| Deep 1 | Deep 2 |
+--------+---+----+
+---------+--+------------+
|---------+---------|
| Final 1 | Final 2 |
+---------+---------+";
    assert_eq!(expected, &outer.to_string());
}

#[test]
fn inline_table_dynamic() {
    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_width(40);

    let mut inline = InlineTable::new();
    inline
        .set_header(vec!["Sub Header 1", "Sub Header 2"])
        .add_row(vec!["A bit of longer text that should fit well", "Short"]);

    table
        .set_header(vec!["Column 1", "Column 2"])
        .add_row(vec![
            "Some long content here that will wrap if needed",
            "Small",
        ])
        .add_inline_table(inline)
        .add_row(vec!["Another row with text", "Another small"]);

    let expected = "+----------------------+---------------+
| Column 1             | Column 2      |
+======================================+
| Some long content    | Small         |
| here that will wrap  |               |
| if needed            |               |
|----------------------+---------------|
+----------------------++--------------+
| Sub Header 1          | Sub Header 2 |
+======================================+
| A bit of longer text  | Short        |
| that should fit well  |              |
+----------------------++--------------+
|----------------------+---------------|
| Another row with     | Another small |
| text                 |               |
+----------------------+---------------+";
    assert_eq!(expected, &table.to_string());
}

#[test]
fn inline_table_dynamic_full_width() {
    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::DynamicFullWidth);
    table.set_width(50);

    let mut inline = InlineTable::new();
    inline
        .set_header(vec!["Detail 1", "Detail 2"])
        .add_row(vec!["Val 1", "Val 2"]);

    table
        .set_header(vec!["Header 1", "Header 2"])
        .add_row(vec!["Row 1", "Data 1"])
        .add_inline_table(inline)
        .add_row(vec!["Row 2", "Data 2"]);

    let expected = "+------------------------+-----------------------+
| Header 1               | Header 2              |
+================================================+
| Row 1                  | Data 1                |
|------------------------+-----------------------|
+------------------------=-----------------------+
| Detail 1               | Detail 2              |
+================================================+
| Val 1                  | Val 2                 |
+------------------------=-----------------------+
|------------------------+-----------------------|
| Row 2                  | Data 2                |
+------------------------+-----------------------+";
    assert_eq!(expected, &table.to_string());
}

#[test]
fn nested_different_columns_dynamic() {
    let mut outer = Table::new();
    outer.set_content_arrangement(ContentArrangement::Dynamic);
    outer.set_width(55);

    let mut mid = InlineTable::new();
    mid.set_header(vec!["M1", "M2", "M3", "M4"]).add_row(vec![
        "Short",
        "This is a longer field that wraps nicely in dynamic mode",
        "Mid",
        "Val",
    ]);

    let mut inner = InlineTable::new();
    inner.set_header(vec!["Single Wide Overview"]).add_row(vec![
        "A comprehensive summary text that demonstrates wrapping in single-column nested tables",
    ]);

    mid.add_inline_table(inner);

    outer
        .set_header(vec!["Main Category", "Overview"])
        .add_row(vec!["Engineering", "Active projects in Q4"])
        .add_inline_table(mid)
        .add_row(vec!["Operations", "Maintenance window"]);

    let expected = "+---------------+-----------------------+
| Main Category | Overview              |
+=======================================+
| Engineering   | Active projects in Q4 |
|---------------+-----------------------|
+-------+-------+----------------------+-----+-----+
| M1    | M2                           | M3  | M4  |
+==================================================+
| Short | This is a longer field that  | Mid | Val |
|       | wraps nicely in dynamic mode |     |     |
|-------+------------------------------+-----+-----|
+-------+------------------------------+-----+---+
| Single Wide Overview                           |
+================================================+
| A comprehensive summary text that demonstrates |
| wrapping in single-column nested tables        |
+-------+------------------------------+-----+---+
+-------+-------+----------------------+-----+-----+
|---------------+-----------------------|
| Operations    | Maintenance window    |
+---------------+-----------------------+";
    assert_eq!(expected, &outer.to_string());
}

#[test]
fn nested_different_columns_dynamic_full_width() {
    let mut outer = Table::new();
    outer.set_content_arrangement(ContentArrangement::DynamicFullWidth);
    outer.set_width(65);

    let mut mid = InlineTable::new();
    mid.set_header(vec!["Mid Key", "Mid Value"])
        .add_row(vec!["Cluster", "Production US-East"]);

    let mut inner = InlineTable::new();
    inner
        .set_header(vec!["Node", "CPU", "RAM", "Status"])
        .add_row(vec!["node-1", "85%", "16GB", "Healthy"])
        .add_row(vec!["node-2", "92%", "32GB", "Degraded"]);

    mid.add_inline_table(inner);

    outer
        .set_header(vec!["Service", "Environment", "Health"])
        .add_row(vec!["API Gateway", "Cloud", "OK"])
        .add_inline_table(mid)
        .add_row(vec!["Database", "On-Prem", "Warning"]);

    let expected = "+----------------------+----------------------+-----------------+
| Service              | Environment          | Health          |
+===============================================================+
| API Gateway          | Cloud                | OK              |
|----------------------+----------------------+-----------------|
+----------------------+---+------------------+-----------------+
| Mid Key                  | Mid Value                          |
+===============================================================+
| Cluster                  | Production US-East                 |
|--------------------------+------------------------------------|
+----------------+---------+---+--------------+-----------------+
| Node           | CPU         | RAM          | Status          |
+===============================================================+
| node-1         | 85%         | 16GB         | Healthy         |
|----------------+-------------+--------------+-----------------|
| node-2         | 92%         | 32GB         | Degraded        |
+----------------+---------+---+--------------+-----------------+
+----------------------+---+------------------+-----------------+
|----------------------+----------------------+-----------------|
| Database             | On-Prem              | Warning         |
+----------------------+----------------------+-----------------+";
    assert_eq!(expected, &outer.to_string());
}

#[test]
fn multiple_nested_tables_mixed_levels_dynamic() {
    let mut outer = Table::new();
    outer.set_content_arrangement(ContentArrangement::Dynamic);
    outer.set_width(60);

    // First inline hierarchy: 2 cols -> 4 cols
    let mut inline_level1 = InlineTable::new();
    inline_level1
        .set_header(vec!["Sub Dept", "Lead"])
        .add_row(vec!["Frontend", "Alice"]);

    let mut deep_inline = InlineTable::new();
    deep_inline
        .set_header(vec!["P1", "P2", "P3", "P4"])
        .add_row(vec!["UI", "UX", "Web", "App"]);

    inline_level1.add_inline_table(deep_inline);

    // Second inline hierarchy under next row: 5 columns
    let mut inline_level2 = InlineTable::new();
    inline_level2
        .set_header(vec!["A", "B", "C", "D", "E"])
        .add_row(vec!["1", "2", "3", "4", "5"]);

    outer
        .set_header(vec!["Division", "Location"])
        .add_row(vec!["Product", "Berlin"])
        .add_inline_table(inline_level1)
        .add_row(vec!["Sales", "Munich"])
        .add_inline_table(inline_level2)
        .add_row(vec!["HR", "Remote"]);

    let expected = "+----------+----------+
| Division | Location |
+=====================+
| Product  | Berlin   |
|----------+----------|
+----------=-------+
| Sub Dept | Lead  |
+==================+
| Frontend | Alice |
|----------+-------|
+----+----++----+-----+
| P1 | P2 | P3  | P4  |
+=====================+
| UI | UX | Web | App |
+----+----++----+-----+
+----------=-------+
|----------+----------|
| Sales    | Munich   |
|----------+----------|
+---+---+--++---+---+
| A | B | C | D | E |
+===================+
| 1 | 2 | 3 | 4 | 5 |
+---+---+--++---+---+
|----------+----------|
| HR       | Remote   |
+----------+----------+";
    assert_eq!(expected, &outer.to_string());
}

#[test]
fn nested_inline_tables_with_constraints_dynamic() {
    let mut outer = Table::new();
    outer.set_content_arrangement(ContentArrangement::Dynamic);
    outer.set_width(60);

    let mut mid = InlineTable::new();
    mid.set_header(vec!["Fixed Col", "Flexible Col"])
        .set_constraints(vec![
            ColumnConstraint::Absolute(comfy_table::Width::Fixed(15)),
            ColumnConstraint::LowerBoundary(comfy_table::Width::Fixed(20)),
        ])
        .add_row(vec![
            "Exactly 15",
            "Flexible with lower boundary constraint",
        ]);

    let mut inner = InlineTable::new();
    inner
        .set_header(vec!["Pct 40", "Pct 60"])
        .set_constraints(vec![
            ColumnConstraint::LowerBoundary(comfy_table::Width::Percentage(40)),
            ColumnConstraint::LowerBoundary(comfy_table::Width::Percentage(60)),
        ])
        .add_row(vec!["Portion 1", "Portion 2"]);

    mid.add_inline_table(inner);

    outer
        .set_header(vec!["Outer Left", "Outer Right"])
        .add_row(vec!["Start", "Begin test"])
        .add_inline_table(mid)
        .add_row(vec!["End", "Finish test"]);

    let expected = "+------------+-------------+
| Outer Left | Outer Right |
+==========================+
| Start      | Begin test  |
|------------+-------------|
+------------+--+-----------------------------------------+
| Fixed Col     | Flexible Col                            |
+=========================================================+
| Exactly 15    | Flexible with lower boundary constraint |
|---------------+-----------------------------------------|
+---------------+------+----------------------------------+
| Pct 40               | Pct 60                           |
+=========================================================+
| Portion 1            | Portion 2                        |
+---------------+------+----------------------------------+
+------------+--+-----------------------------------------+
|------------+-------------|
| End        | Finish test |
+------------+-------------+";
    assert_eq!(expected, &outer.to_string());
}

#[test]
fn test_nested_mega_table() {
    // Service 1: Ingress
    let mut ingress_config = InlineTable::new();
    ingress_config.set_header(vec![
        "Global Configuration Attribute",
        "Deployment Parameter / Value Specification",
    ]);
    ingress_config.add_row(vec![
        "ingress.controller.runtime",
        "Envoy Gateway Proxy v1.31.2-distroless (eBPF accelerated kernel socket dispatch)",
    ]);
    ingress_config.add_row(vec![
        "ssl.tls.certificate.authority",
        "Let's Encrypt Authority X2 (Auto-Renewal: Active, Expiry: 2026-12-15 00:00:00 UTC)",
    ]);
    ingress_config.add_row(vec![
        "traffic.routing.algorithm",
        "Round-Robin with Weighted Dynamic Latency Feedback & Outlier Ejection (AnomalyThreshold: 3)",
    ]);
    ingress_config.add_row(vec![
        "ddos.mitigation.layer",
        "Cloudflare Magic Transit + BGP Anycast + Inline RateLimiting (150,000 req/sec sustained burst)",
    ]);

    let mut ingress_nodes = InlineTable::new();
    ingress_nodes.set_header(vec![
        "Node Instance Identifier",
        "IP Address / Subnet / CIDR",
        "CPU Cores & Arch Allocation",
        "Memory Footprint (Alloc / Max Capacity)",
    ]);
    ingress_nodes.add_row(vec![
        "edge-ingress-node-01",
        "10.142.10.101/24 [VPC-Prod-Tier-1]",
        "32 vCPU (AMD EPYC 9654 Genoa @ 2.4GHz)",
        "48.2 GB / 128 GB [ECC Reg. DDR5 SDRAM]",
    ]);
    ingress_nodes.add_row(vec![
        "edge-ingress-node-02",
        "10.142.10.102/24 [VPC-Prod-Tier-1]",
        "32 vCPU (AMD EPYC 9654 Genoa @ 2.4GHz)",
        "52.8 GB / 128 GB [ECC Reg. DDR5 SDRAM]",
    ]);
    ingress_nodes.add_row(vec![
        "edge-ingress-node-03",
        "10.142.10.103/24 [VPC-Prod-Tier-1]",
        "32 vCPU (AMD EPYC 9654 Genoa @ 2.4GHz)",
        "98.4 GB / 128 GB [HIGH RAM SATURATION]",
    ]);
    ingress_nodes.add_row(vec![
        "edge-ingress-node-04",
        "10.142.10.104/24 [VPC-Prod-Tier-1]",
        "32 vCPU (AMD EPYC 9654 Genoa @ 2.4GHz)",
        "12.0 GB / 128 GB [DRAINING / STANDBY]",
    ]);

    let mut ingress_workloads = InlineTable::new();
    ingress_workloads.set_header(vec![
        "Container Workload ID",
        "Protocol & Transport",
        "Port Bindings Listener",
        "Thread Pool Load (Active / Max)",
        "Lifecycle Health Status Check",
    ]);
    ingress_workloads.add_row(vec![
        "c-ingress-proxy-01a",
        "HTTP/2, HTTP/3, QUIC",
        "TCP:443, UDP:443",
        " 840 / 2048 workers ( 41.0%)",
        "PASS [TLS 1.3 Handshake: 1.2ms]",
    ]);
    ingress_workloads.add_row(vec![
        "c-ingress-proxy-01b",
        "HTTP/2, gRPC-Web",
        "TCP:8443, TCP:9001",
        "1420 / 2048 workers ( 69.3%)",
        "PASS [Upstream Keep-Alive OK]",
    ]);
    ingress_workloads.add_row(vec![
        "c-ingress-ratelimit-01",
        "gRPC (Internal IPC)",
        "TCP:8081",
        " 210 / 1024 workers ( 20.5%)",
        "PASS [Token Bucket Sync Optimal]",
    ]);
    ingress_workloads.add_row(vec![
        "c-ingress-telemetry-01",
        "OpenTelemetry-OTLP",
        "TCP:4317, TCP:4318",
        " 980 / 1024 workers ( 95.7%)",
        "WARN [Buffer Saturation Threshold]",
    ]);

    // Service 2: Database
    let mut db_config = InlineTable::new();
    db_config.set_header(vec![
        "Global Configuration Attribute",
        "Deployment Parameter / Value Specification",
    ]);
    db_config.add_row(vec![
        "database.engine.version",
        "PostgreSQL 16.4 Enterprise Cluster Edition (Citus Horizontal Distributed Scaling v12.1)",
    ]);
    db_config.add_row(vec![
        "consensus.raft.quorum",
        "Multi-Paxos Consensus Engine (3 DC Quorum Leader: fra-core-db-01, Heartbeat: 50ms)",
    ]);
    db_config.add_row(vec![
        "storage.tier.configuration",
        "8x 3.84TB Samsung PM1733 NVMe (RAID 10 Hardware Controller, Direct I/O Async Write-Through)",
    ]);

    let mut db_shards = InlineTable::new();
    db_shards.set_header(vec![
        "Partition Shard Name",
        "Host Node Identifier",
        "Port Listener Socket",
        "Read/Write Role Assignment",
        "Replication Lag & Sync Verification",
    ]);
    db_shards.add_row(vec![
        "shard-cust-acc-001-a",
        "srv-fra-db-node-01",
        "pgsql://0.0.0.0:5432",
        "Primary Leader [Read/Write]",
        "0 ms [Zero-Data-Loss Synchronous]",
    ]);
    db_shards.add_row(vec![
        "shard-cust-acc-001-b",
        "srv-fra-db-node-02",
        "pgsql://0.0.0.0:5432",
        "Standby Follower [Read-Only]",
        "4 ms [Async Streaming WAL Apply]",
    ]);
    db_shards.add_row(vec![
        "shard-cust-acc-001-c",
        "srv-ams-db-node-03",
        "pgsql://0.0.0.0:5432",
        "Disaster Recovery [Offsite]",
        "48 ms [Cross-DC WAN Interconnect]",
    ]);
    db_shards.add_row(vec![
        "shard-orders-tx-002-a",
        "srv-fra-db-node-01",
        "pgsql://0.0.0.0:5433",
        "Primary Leader [Read/Write]",
        "0 ms [Zero-Data-Loss Synchronous]",
    ]);
    db_shards.add_row(vec![
        "shard-orders-tx-002-b",
        "srv-fra-db-node-02",
        "pgsql://0.0.0.0:5433",
        "Standby Follower [Read-Only]",
        "8 ms [Async Streaming WAL Apply]",
    ]);
    db_shards.add_row(vec![
        "shard-orders-tx-002-c",
        "srv-ams-db-node-03",
        "pgsql://0.0.0.0:5433",
        "Out-of-Sync Replica [STALLED]",
        "CRITICAL: 14,250 ms [Connection Drop]",
    ]);

    let mut db_metrics = InlineTable::new();
    db_metrics.set_header(vec![
        "Metric Telemetry",
        "Time Sampling Window",
        "Avg Throughput Rate",
        "Latency p95 / p99 Peak",
        "Cache Hit Ratio Rate",
        "Diagnostic Alert Status",
    ]);
    db_metrics.add_row(vec![
        "DB Read Query Rate",
        "Last 15 min rolling",
        "148,290 queries/sec",
        "0.84 ms / 2.15 ms",
        "99.42% (Buffer Cache)",
        "OK / NOMINAL PERFORMANCE",
    ]);
    db_metrics.add_row(vec![
        "DB Write Trans Rate",
        "Last 15 min rolling",
        " 38,410 commits/sec",
        "4.12 ms / 9.80 ms",
        "WAL Flush: 1.10 ms",
        "OK / NOMINAL PERFORMANCE",
    ]);
    db_metrics.add_row(vec![
        "Lock Wait Content.",
        "Last 15 min rolling",
        "    142 blocks/sec",
        "45.20 ms / 180.50 ms",
        "Deadlocks detected: 0",
        "WARN / ELEVATED ROW-LOCKS",
    ]);
    db_metrics.add_row(vec![
        "Replication Stream",
        "Last 15 min rolling",
        "  1.2 GB/sec network",
        "WAN RTT: 18.40 ms",
        "Packet Loss: 0.08%",
        "ERR / SHARD-002-C RETRY",
    ]);

    // Service 3: Kafka
    let mut kafka_config = InlineTable::new();
    kafka_config.set_header(vec![
        "Global Configuration Attribute",
        "Deployment Parameter / Value Specification",
    ]);
    kafka_config.add_row(vec![
        "broker.cluster.id",
        "kafka-prod-eventstream-cl-99 (Apache Kafka 3.7.0 KRaft mode without ZooKeeper)",
    ]);
    kafka_config.add_row(vec![
        "default.retention.policy",
        "168 Hours (7 Days) / Segment Size: 1024 MB / LZ4 High Compression Ratio: 3.4x",
    ]);
    kafka_config.add_row(vec![
        "security.inter.broker.protocol",
        "SASL_SSL / SCRAM-SHA-512 Mutual Authentication (ACL Strict Enforcement Enabled)",
    ]);

    let mut kafka_nodes = InlineTable::new();
    kafka_nodes.set_header(vec![
        "Node Instance Identifier",
        "IP Address / Subnet / CIDR",
        "CPU Cores & Arch Allocation",
        "Memory Footprint (Alloc / Max Capacity)",
    ]);
    kafka_nodes.add_row(vec![
        "gcp-kafka-broker-01",
        "10.200.4.11/20 [GCP-VPC-Core]",
        "16 vCPU (Intel Xeon Platinum 8481C)",
        "54.0 GB / 64.0 GB [JVM Heap: 32.0 GB]",
    ]);
    kafka_nodes.add_row(vec![
        "gcp-kafka-broker-02",
        "10.200.4.12/20 [GCP-VPC-Core]",
        "16 vCPU (Intel Xeon Platinum 8481C)",
        "53.2 GB / 64.0 GB [JVM Heap: 32.0 GB]",
    ]);
    kafka_nodes.add_row(vec![
        "gcp-kafka-broker-03",
        "10.200.4.13/20 [GCP-VPC-Core]",
        "16 vCPU (Intel Xeon Platinum 8481C)",
        "55.1 GB / 64.0 GB [JVM Heap: 32.0 GB]",
    ]);

    let mut kafka_schedule = InlineTable::new();
    kafka_schedule.set_header(vec![
        "Scheduled Window",
        "Assigned Action Item",
        "Target Cluster Resource Identifier",
        "Remediation Operator Lead Team",
    ]);
    kafka_schedule.add_row(vec![
        "2026-10-02 22:00:00 UTC",
        "Node Reseat & NVMe Hot-Swap",
        "srv-ams-db-node-03 [shard-orders-02c]",
        "SRE On-Call Primary Team (Frankfurt)",
    ]);
    kafka_schedule.add_row(vec![
        "2026-10-03 01:30:00 UTC",
        "Drain Pods & Rolling Node Reboot",
        "edge-ingress-node-03 [VPC-Tier-1]",
        "Platform Automation Controller (Bot)",
    ]);

    let mut outer = Table::new();
    outer.set_content_arrangement(ContentArrangement::DynamicFullWidth);
    outer.set_width(160);

    outer.set_header(vec![
        "ENTERPRISE DISTRIBUTED SYSTEM TOPOLOGY & DEPLOYMENT RUNBOOK MATRIX [VERSION 4.2.0-RELEASE]",
    ]);

    // Banner before first row
    let mut groupes = InlineTable::new();
    groupes.set_header(vec![
        "Primary Core Service Group",
        "Deployment Target & Cloud Availability Zones",
        "Operational Status / Health SLA",
    ]);

    ingress_nodes.add_inline_table(ingress_workloads);
    ingress_config.add_inline_table(ingress_nodes);

    groupes.add_row(vec![
        "Global-API-Edge-Ingress",
        "AWS eu-central-1 (Multi-AZ: eu-central-1a, 1b, 1c)",
        "ACTIVE / HEALTHY (SLA: 99.995%)",
    ]);
    groupes.add_inline_table(ingress_config);

    db_shards.add_inline_table(db_metrics);
    db_config.add_inline_table(db_shards);

    groupes.add_row(vec![
        "Distributed-Database-Persistence-Grid",
        "On-Premises Bare-Metal Equinix DC-Frankfurt & Amsterdam",
        "DEGRADED / ATTENTION REQUIRED",
    ]);
    groupes.add_inline_table(db_config);

    kafka_nodes.add_inline_table(kafka_schedule);
    kafka_config.add_inline_table(kafka_nodes);

    groupes.add_row(vec![
        "Event-Streaming-Kafka-Bus",
        "Google Cloud Platform europe-west3 (Frankfurt)",
        "OPERATIONAL / HEALTHY",
    ]);
    groupes.add_inline_table(kafka_config);
    outer.add_inline_table(groupes);
    outer.add_row(vec![
        "AUDIT LOG SUMMARY: Automated cluster reconciliation scheduled for 2026-10-02T22:00:00Z. Failover target ready on srv-fra-db-node-02.",
    ]);

    let expected = "+--------------------------------------------------------------------------------------------------------------------------------------------------------------+
| ENTERPRISE DISTRIBUTED SYSTEM TOPOLOGY & DEPLOYMENT RUNBOOK MATRIX [VERSION 4.2.0-RELEASE]                                                                   |
+==============================================================================================================================================================+
+------------------------------------------------+------------------------------------------------------------------+------------------------------------------+
| Primary Core Service Group                     | Deployment Target & Cloud Availability Zones                     | Operational Status / Health SLA          |
+==============================================================================================================================================================+
| Global-API-Edge-Ingress                        | AWS eu-central-1 (Multi-AZ: eu-central-1a, 1b, 1c)               | ACTIVE / HEALTHY (SLA: 99.995%)          |
|------------------------------------------------+------------------------------------------------------------------+------------------------------------------|
+-----------------------------------------------++------------------------------------------------------------------+------------------------------------------+
| Global Configuration Attribute                | Deployment Parameter / Value Specification                                                                   |
+==============================================================================================================================================================+
| ingress.controller.runtime                    | Envoy Gateway Proxy v1.31.2-distroless (eBPF accelerated kernel socket dispatch)                             |
|-----------------------------------------------+--------------------------------------------------------------------------------------------------------------|
| ssl.tls.certificate.authority                 | Let's Encrypt Authority X2 (Auto-Renewal: Active, Expiry: 2026-12-15 00:00:00 UTC)                           |
|-----------------------------------------------+--------------------------------------------------------------------------------------------------------------|
| traffic.routing.algorithm                     | Round-Robin with Weighted Dynamic Latency Feedback & Outlier Ejection (AnomalyThreshold: 3)                  |
|-----------------------------------------------+--------------------------------------------------------------------------------------------------------------|
| ddos.mitigation.layer                         | Cloudflare Magic Transit + BGP Anycast + Inline RateLimiting (150,000 req/sec sustained burst)               |
|-----------------------------------------------+--------------------------------------------------------------------------------------------------------------|
+-----------------------------+-----------------+---------------------+-------------------------------------------+--------------------------------------------+
| Node Instance Identifier    | IP Address / Subnet / CIDR            | CPU Cores & Arch Allocation               | Memory Footprint (Alloc / Max Capacity)    |
+==============================================================================================================================================================+
| edge-ingress-node-01        | 10.142.10.101/24 [VPC-Prod-Tier-1]    | 32 vCPU (AMD EPYC 9654 Genoa @ 2.4GHz)    | 48.2 GB / 128 GB [ECC Reg. DDR5 SDRAM]     |
|-----------------------------+---------------------------------------+-------------------------------------------+--------------------------------------------|
| edge-ingress-node-02        | 10.142.10.102/24 [VPC-Prod-Tier-1]    | 32 vCPU (AMD EPYC 9654 Genoa @ 2.4GHz)    | 52.8 GB / 128 GB [ECC Reg. DDR5 SDRAM]     |
|-----------------------------+---------------------------------------+-------------------------------------------+--------------------------------------------|
| edge-ingress-node-03        | 10.142.10.103/24 [VPC-Prod-Tier-1]    | 32 vCPU (AMD EPYC 9654 Genoa @ 2.4GHz)    | 98.4 GB / 128 GB [HIGH RAM SATURATION]     |
|-----------------------------+---------------------------------------+-------------------------------------------+--------------------------------------------|
| edge-ingress-node-04        | 10.142.10.104/24 [VPC-Prod-Tier-1]    | 32 vCPU (AMD EPYC 9654 Genoa @ 2.4GHz)    | 12.0 GB / 128 GB [DRAINING / STANDBY]      |
|-----------------------------+---------------------------------------+-------------------------------------------+--------------------------------------------|
+---------------------------+-+-----------------------+---------------+-----------+-------------------------------+----+---------------------------------------+
| Container Workload ID     | Protocol & Transport    | Port Bindings Listener    | Thread Pool Load (Active / Max)    | Lifecycle Health Status Check         |
+==============================================================================================================================================================+
| c-ingress-proxy-01a       | HTTP/2, HTTP/3, QUIC    | TCP:443, UDP:443          |  840 / 2048 workers ( 41.0%)       | PASS [TLS 1.3 Handshake: 1.2ms]       |
|---------------------------+-------------------------+---------------------------+------------------------------------+---------------------------------------|
| c-ingress-proxy-01b       | HTTP/2, gRPC-Web        | TCP:8443, TCP:9001        | 1420 / 2048 workers ( 69.3%)       | PASS [Upstream Keep-Alive OK]         |
|---------------------------+-------------------------+---------------------------+------------------------------------+---------------------------------------|
| c-ingress-ratelimit-01    | gRPC (Internal IPC)     | TCP:8081                  |  210 / 1024 workers ( 20.5%)       | PASS [Token Bucket Sync Optimal]      |
|---------------------------+-------------------------+---------------------------+------------------------------------+---------------------------------------|
| c-ingress-telemetry-01    | OpenTelemetry-OTLP      | TCP:4317, TCP:4318        |  980 / 1024 workers ( 95.7%)       | WARN [Buffer Saturation Threshold]    |
+---------------------------+-+-----------------------+---------------+-----------+-------------------------------+----+---------------------------------------+
+-----------------------------+-----------------+---------------------+-------------------------------------------+--------------------------------------------+
+-----------------------------------------------++------------------------------------------------------------------+------------------------------------------+
|------------------------------------------------+------------------------------------------------------------------+------------------------------------------|
| Distributed-Database-Persistence-Grid          | On-Premises Bare-Metal Equinix DC-Frankfurt & Amsterdam          | DEGRADED / ATTENTION REQUIRED            |
|------------------------------------------------+------------------------------------------------------------------+------------------------------------------|
+------------------------------------------------=------------------------------------------------------------------+------------------------------------------+
| Global Configuration Attribute                 | Deployment Parameter / Value Specification                                                                  |
+==============================================================================================================================================================+
| database.engine.version                        | PostgreSQL 16.4 Enterprise Cluster Edition (Citus Horizontal Distributed Scaling v12.1)                     |
|------------------------------------------------+-------------------------------------------------------------------------------------------------------------|
| consensus.raft.quorum                          | Multi-Paxos Consensus Engine (3 DC Quorum Leader: fra-core-db-01, Heartbeat: 50ms)                          |
|------------------------------------------------+-------------------------------------------------------------------------------------------------------------|
| storage.tier.configuration                     | 8x 3.84TB Samsung PM1733 NVMe (RAID 10 Hardware Controller, Direct I/O Async Write-Through)                 |
|------------------------------------------------+-------------------------------------------------------------------------------------------------------------|
+---------------------------+--------------------+-----+-------------------------+----------------------------------+------------------------------------------+
| Partition Shard Name      | Host Node Identifier     | Port Listener Socket    | Read/Write Role Assignment       | Replication Lag & Sync Verification      |
+==============================================================================================================================================================+
| shard-cust-acc-001-a      | srv-fra-db-node-01       | pgsql://0.0.0.0:5432    | Primary Leader [Read/Write]      | 0 ms [Zero-Data-Loss Synchronous]        |
|---------------------------+--------------------------+-------------------------+----------------------------------+------------------------------------------|
| shard-cust-acc-001-b      | srv-fra-db-node-02       | pgsql://0.0.0.0:5432    | Standby Follower [Read-Only]     | 4 ms [Async Streaming WAL Apply]         |
|---------------------------+--------------------------+-------------------------+----------------------------------+------------------------------------------|
| shard-cust-acc-001-c      | srv-ams-db-node-03       | pgsql://0.0.0.0:5432    | Disaster Recovery [Offsite]      | 48 ms [Cross-DC WAN Interconnect]        |
|---------------------------+--------------------------+-------------------------+----------------------------------+------------------------------------------|
| shard-orders-tx-002-a     | srv-fra-db-node-01       | pgsql://0.0.0.0:5433    | Primary Leader [Read/Write]      | 0 ms [Zero-Data-Loss Synchronous]        |
|---------------------------+--------------------------+-------------------------+----------------------------------+------------------------------------------|
| shard-orders-tx-002-b     | srv-fra-db-node-02       | pgsql://0.0.0.0:5433    | Standby Follower [Read-Only]     | 8 ms [Async Streaming WAL Apply]         |
|---------------------------+--------------------------+-------------------------+----------------------------------+------------------------------------------|
| shard-orders-tx-002-c     | srv-ams-db-node-03       | pgsql://0.0.0.0:5433    | Out-of-Sync Replica [STALLED]    | CRITICAL: 14,250 ms [Connection Drop]    |
|---------------------------+--------------------------+-------------------------+----------------------------------+------------------------------------------|
+------------------------+--+----------------------+---+--------------------+----+---------------------+------------+------------+-----------------------------+
| Metric Telemetry       | Time Sampling Window    | Avg Throughput Rate    | Latency p95 / p99 Peak   | Cache Hit Ratio Rate    | Diagnostic Alert Status     |
+==============================================================================================================================================================+
| DB Read Query Rate     | Last 15 min rolling     | 148,290 queries/sec    | 0.84 ms / 2.15 ms        | 99.42% (Buffer Cache)   | OK / NOMINAL PERFORMANCE    |
|------------------------+-------------------------+------------------------+--------------------------+-------------------------+-----------------------------|
| DB Write Trans Rate    | Last 15 min rolling     |  38,410 commits/sec    | 4.12 ms / 9.80 ms        | WAL Flush: 1.10 ms      | OK / NOMINAL PERFORMANCE    |
|------------------------+-------------------------+------------------------+--------------------------+-------------------------+-----------------------------|
| Lock Wait Content.     | Last 15 min rolling     |     142 blocks/sec     | 45.20 ms / 180.50 ms     | Deadlocks detected: 0   | WARN / ELEVATED ROW-LOCKS   |
|------------------------+-------------------------+------------------------+--------------------------+-------------------------+-----------------------------|
| Replication Stream     | Last 15 min rolling     |   1.2 GB/sec network   | WAN RTT: 18.40 ms        | Packet Loss: 0.08%      | ERR / SHARD-002-C RETRY     |
+------------------------+--+----------------------+---+--------------------+----+---------------------+------------+------------+-----------------------------+
+---------------------------+--------------------+-----+-------------------------+----------------------------------+------------------------------------------+
+------------------------------------------------=------------------------------------------------------------------+------------------------------------------+
|------------------------------------------------+------------------------------------------------------------------+------------------------------------------|
| Event-Streaming-Kafka-Bus                      | Google Cloud Platform europe-west3 (Frankfurt)                   | OPERATIONAL / HEALTHY                    |
|------------------------------------------------+------------------------------------------------------------------+------------------------------------------|
+------------------------------------------------+-----+------------------------------------------------------------+------------------------------------------+
| Global Configuration Attribute                       | Deployment Parameter / Value Specification                                                            |
+==============================================================================================================================================================+
| broker.cluster.id                                    | kafka-prod-eventstream-cl-99 (Apache Kafka 3.7.0 KRaft mode without ZooKeeper)                        |
|------------------------------------------------------+-------------------------------------------------------------------------------------------------------|
| default.retention.policy                             | 168 Hours (7 Days) / Segment Size: 1024 MB / LZ4 High Compression Ratio: 3.4x                         |
|------------------------------------------------------+-------------------------------------------------------------------------------------------------------|
| security.inter.broker.protocol                       | SASL_SSL / SCRAM-SHA-512 Mutual Authentication (ACL Strict Enforcement Enabled)                       |
|------------------------------------------------------+-------------------------------------------------------------------------------------------------------|
+-------------------------------+----------------------+-------------+------------------------------------------+----------------------------------------------+
| Node Instance Identifier      | IP Address / Subnet / CIDR         | CPU Cores & Arch Allocation              | Memory Footprint (Alloc / Max Capacity)      |
+==============================================================================================================================================================+
| gcp-kafka-broker-01           | 10.200.4.11/20 [GCP-VPC-Core]      | 16 vCPU (Intel Xeon Platinum 8481C)      | 54.0 GB / 64.0 GB [JVM Heap: 32.0 GB]        |
|-------------------------------+------------------------------------+------------------------------------------+----------------------------------------------|
| gcp-kafka-broker-02           | 10.200.4.12/20 [GCP-VPC-Core]      | 16 vCPU (Intel Xeon Platinum 8481C)      | 53.2 GB / 64.0 GB [JVM Heap: 32.0 GB]        |
|-------------------------------+------------------------------------+------------------------------------------+----------------------------------------------|
| gcp-kafka-broker-03           | 10.200.4.13/20 [GCP-VPC-Core]      | 16 vCPU (Intel Xeon Platinum 8481C)      | 55.1 GB / 64.0 GB [JVM Heap: 32.0 GB]        |
|-------------------------------+------------------------------------+------------------------------------------+----------------------------------------------|
+------------------------------++------------------------------------+-+----------------------------------------+---+------------------------------------------+
| Scheduled Window             | Assigned Action Item                  | Target Cluster Resource Identifier         | Remediation Operator Lead Team           |
+==============================================================================================================================================================+
| 2026-10-02 22:00:00 UTC      | Node Reseat & NVMe Hot-Swap           | srv-ams-db-node-03 [shard-orders-02c]      | SRE On-Call Primary Team (Frankfurt)     |
|------------------------------+---------------------------------------+--------------------------------------------+------------------------------------------|
| 2026-10-03 01:30:00 UTC      | Drain Pods & Rolling Node Reboot      | edge-ingress-node-03 [VPC-Tier-1]          | Platform Automation Controller (Bot)     |
+------------------------------++------------------------------------+-+----------------------------------------+---+------------------------------------------+
+-------------------------------+----------------------+-------------+------------------------------------------+----------------------------------------------+
+------------------------------------------------+-----+------------------------------------------------------------+------------------------------------------+
+------------------------------------------------+------------------------------------------------------------------+------------------------------------------+
|--------------------------------------------------------------------------------------------------------------------------------------------------------------|
| AUDIT LOG SUMMARY: Automated cluster reconciliation scheduled for 2026-10-02T22:00:00Z. Failover target ready on srv-fra-db-node-02.                         |
+--------------------------------------------------------------------------------------------------------------------------------------------------------------+";
    for line in outer.lines() {
        assert_eq!(line.chars().count(), 160);
    }

    assert_eq!(expected, &outer.to_string());
}
