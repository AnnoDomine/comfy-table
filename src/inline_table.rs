use std::ops::{Deref, DerefMut};

use crate::{
    Table,
    utils::{
        arrangement::arrange_content,
        formatting::content_format::{BuildTableItem, format_content},
    },
};

#[derive(Debug, Clone)]
pub struct InlineTable {
    pub(crate) index_row_above: Option<usize>,
    table: Table,
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

    pub fn set_row_above_index(&mut self, index: Option<usize>) {
        self.index_row_above = index;
    }

    /// Reapply outer table to the inline table and place the inner table belowe the row when draw.
    ///
    /// This is necessary to retreive the styles from the outer table.
    pub fn build_inline_table(
        &self,
        outer_table: Table,
        index_row_above: Option<usize>,
    ) -> BuildTableItem {
        let mut inline_table = Self::new_from_table(outer_table, index_row_above);
        inline_table.table.columns = self.columns.clone();
        inline_table.table.header = self.header.clone();
        inline_table.table.rows = self.rows.clone();
        inline_table.table.inline_tables = self.inline_tables.clone();
        inline_table.style.top_border = self.style.header_separator;
        inline_table.style.bottom_border = self.style.header_separator;
        let display_info = arrange_content(&inline_table.table);
        BuildTableItem::Inline(format_content(&inline_table.table, &display_info))
    }
}
