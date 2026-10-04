//! Inline table support for embedding nested tables beneath rows of a parent table.
//!
//! An [`InlineTable`] acts as a nested table rendered directly below a specific row in the outer [`Table`].
//! It automatically inherits the parent table's border styles and column separators,
//! while maintaining its own column structure, headers, rows, and constraints.
//!
//! Because `InlineTable` implements [`Deref`] and [`DerefMut`] targeting [`Table`], you can use
//! all familiar table configuration methods ([`Table::set_header`], [`Table::add_row`], etc.) directly
//! on an `InlineTable`.
//!
//! # Best Practice: Inside-Out Composition
//!
//! When working with inline tables—especially when creating multi-level nested tables—the
//! recommended approach is to define tables **inside-out** (bottom-up):
//!
//! 1. Define and populate the innermost [`InlineTable`] instances with their columns, headers, and rows.
//! 2. Combine them into higher-level inline tables or sections using [`Table::add_inline_table`].
//! 3. Finally, assemble the outer [`Table`], adding rows and attaching the prepared inline tables.
//!
//! This modular pattern keeps table logic clean, prevents scope confusion, and ensures clear data flow.
//!
//! # Example
//!
//! ```
//! use comfy_table::{InlineTable, Table};
//!
//! // 1. Define the innermost nested table first
//! let mut inner_table = InlineTable::new();
//! inner_table
//!     .set_header(vec!["Component", "Latency", "Status"])
//!     .add_row(vec!["Cache Tier", "0.4 ms", "Healthy"])
//!     .add_row(vec!["Storage Tier", "2.1 ms", "Healthy"]);
//!
//! // 2. Define the intermediate section and attach the inner table
//! let mut service_section = InlineTable::new();
//! service_section
//!     .set_header(vec!["Service", "Region"])
//!     .add_row(vec!["Authentication API", "eu-central-1"])
//!     .add_inline_table(inner_table);
//!
//! // 3. Assemble the outer table
//! let mut outer = Table::new();
//! outer
//!     .set_header(vec!["Cluster", "Environment"])
//!     .add_row(vec!["Production-Core", "Live"])
//!     .add_inline_table(service_section)
//!     .add_row(vec!["Staging-Core", "Idle"]);
//!
//! println!("{outer}");
//! ```

use std::ops::{Deref, DerefMut};

use crate::{
    Table,
    utils::{
        arrangement::arrange_content,
        formatting::content_format::{BuildInlineTableItem, BuildTableItem, format_content},
    },
};

/// A nested table designed to be embedded inline directly below a row within a parent [`Table`].
///
/// `InlineTable` wraps an inner [`Table`] and tracks its placement relative to the rows of the
/// parent table. It implements [`Deref`] and [`DerefMut`] targeting [`Table`], allowing full access
/// to all table configuration methods (such as [`Table::set_header`], [`Table::add_row`],
/// [`Table::set_constraints`], etc.).
///
/// When added to a parent table via [`Table::add_inline_table`], it is rendered below the most recently
/// added row and inherits the outer table's styling for consistent appearance.
///
/// # Best Practice: Inside-Out Construction
///
/// It is recommended to construct tables **inside-out** (bottom-up):
/// 1. Define and populate the innermost [`InlineTable`] instances first.
/// 2. Nest them into intermediate inline tables via [`Table::add_inline_table`].
/// 3. Finally, assemble the outer [`Table`].
///
/// # Example
///
/// ```
/// use comfy_table::{InlineTable, Table};
///
/// let mut inline = InlineTable::new();
/// inline.set_header(vec!["Sub", "Entry"]);
/// inline.add_row(vec!["Nested", "Value"]);
///
/// let mut table = Table::new();
/// table.add_row(vec!["Main", "Row"]);
/// table.add_inline_table(inline);
/// ```
#[derive(Debug, Clone)]
pub struct InlineTable {
    pub(crate) index_row_above: Option<usize>,
    pub table: Table,
}

struct InlineTableFromArgs {
    index_row_above: Option<usize>,
    table: Table,
}

impl From<InlineTableFromArgs> for InlineTable {
    fn from(args: InlineTableFromArgs) -> Self {
        let mut core = args.table;
        core.columns = Vec::new();
        core.header = None;
        core.rows = Vec::new();
        core.inline_tables = Vec::new();
        Self {
            index_row_above: args.index_row_above,
            table: core,
        }
    }
}

impl Deref for InlineTable {
    type Target = Table;
    fn deref(&self) -> &Self::Target {
        &self.table
    }
}

impl DerefMut for InlineTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.table
    }
}

impl Default for InlineTable {
    fn default() -> Self {
        Self::new()
    }
}

impl InlineTable {
    /// Create a new, empty [`InlineTable`].
    ///
    /// By default, the table has no headers, rows, or predefined row association.
    ///
    /// ```
    /// use comfy_table::InlineTable;
    ///
    /// let inline = InlineTable::new();
    /// assert!(inline.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            table: Table::new(),
            index_row_above: None,
        }
    }

    fn new_from_table(table: Table, index_row_above: Option<usize>) -> Self {
        Self::from(InlineTableFromArgs {
            table,
            index_row_above,
        })
    }

    /// Set the 0-based index of the parent table row directly above this inline table.
    ///
    /// When using [`Table::add_inline_table`], this index is automatically populated based on
    /// the most recently added row. You can manually adjust this index if you need custom
    /// row positioning, or pass `None` to display the inline table above the first row.
    pub fn set_row_above_index(&mut self, index: Option<usize>) {
        self.index_row_above = index;
    }

    /// Formats the inline table into table items using the styles and arrangement of the parent table.
    ///
    /// This reapplies the outer table's styling (such as border and separator characters) to ensure
    /// visual consistency across the entire table output.
    pub fn build_inline_table(
        &self,
        outer_table: &Table,
        index_row_above: Option<usize>,
    ) -> BuildTableItem {
        let mut inline_table = Self::new_from_table(outer_table.clone(), index_row_above);
        inline_table.table.columns = self.columns.clone();
        inline_table.table.header = self.header.clone();
        inline_table.table.rows = self.rows.clone();
        inline_table.table.inline_tables = self.inline_tables.clone();
        let display_info = arrange_content(&inline_table);
        BuildTableItem::Inline(BuildInlineTableItem {
            items: format_content(&inline_table.table, &display_info),
            table: self.clone(),
        })
    }
}
