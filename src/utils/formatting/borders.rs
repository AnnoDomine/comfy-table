use std::iter::repeat_n;

use crate::{
    table::Table,
    utils::{
        ColumnDisplayInfo, arrangement::arrange_content, formatting::content_format::BuildTableItem,
    },
};

pub(crate) fn draw_borders(
    table: &Table,
    rows: &[BuildTableItem],
    display_info: &[ColumnDisplayInfo],
    last_inline_table_of_higherst_level: bool,
    is_inline_table: bool,
    very_last_border: &mut Option<String>,
) -> Vec<String> {
    // We know how many lines there should be. Initialize the vector with the rough correct amount.
    // We might over allocate a bit, but that's better than under allocating.
    let mut lines = if let Some(capacity) = rows.first().map(|lines| lines.len()) {
        // Lines * 2 -> Lines + delimiters
        // + 5 -> header delimiters + header + bottom/top borders
        Vec::with_capacity(capacity * 2 + 5)
    } else {
        Vec::new()
    };

    if table.style.has_top_border() {
        lines.push(draw_top_border(table, display_info));
    }

    draw_rows(
        &mut lines,
        rows,
        table,
        display_info,
        last_inline_table_of_higherst_level,
        is_inline_table,
        very_last_border,
    );

    if !is_inline_table
        && let Some(border) = very_last_border
        && table.style.has_bottom_border()
    {
        lines.push(border.to_string());
    } else if table.style.has_bottom_border() {
        lines.push(draw_bottom_border(table, display_info));
    }

    lines
}

fn draw_top_border(table: &Table, display_info: &[ColumnDisplayInfo]) -> String {
    let left_corner = table.style.top_border.left.unwrap_or(' ');
    let top_border = table.style.top_border.fill.unwrap_or(' ');
    let intersection = table.style.top_border.junction.unwrap_or(' ');
    let right_corner = table.style.top_border.right.unwrap_or(' ');

    let mut line = String::new();
    // We only need the top left corner, if we need to draw a left border
    if should_draw_left_border(table) {
        line.push(left_corner);
    }

    // Build the top border line depending on the columns' width.
    // Also add the border intersections.
    let mut first = true;
    for info in display_info.iter() {
        // Only add something, if the column isn't hidden
        if !info.is_hidden {
            if !first {
                line.push(intersection);
            }
            line.extend(repeat_n(top_border, info.width().into()));
            first = false;
        }
    }

    // We only need the top right corner, if we need to draw a right border
    if should_draw_right_border(table) {
        line.push(right_corner);
    }

    line
}

fn draw_top_inline_border(
    table: &Table,
    display_info: &[ColumnDisplayInfo],
    connect_above_idx: &Vec<usize>,
) -> String {
    let left_corner = table.style.top_inline_border.left.unwrap_or(' ');
    let top_border = table.style.top_inline_border.fill.unwrap_or(' ');
    let intersection = table.style.top_inline_border.junction.unwrap_or(' ');
    let right_corner = table.style.top_inline_border.right.unwrap_or(' ');

    let connector = table.style.bottom_inline_border.junction.unwrap_or(' ');

    let intersection_connector = table.style.header_separator.junction.unwrap_or(' ');

    let mut line: Vec<char> = Vec::new();
    // We only need the top left corner, if we need to draw a left border
    if should_draw_left_border(table) {
        line.push(left_corner);
    }

    // Build the top border line depending on the columns' width.
    // Also add the border intersections.
    let mut first = true;
    for info in display_info.iter() {
        // Only add something, if the column isn't hidden
        if !info.is_hidden {
            if !first {
                line.push(intersection);
            }
            line.extend(repeat_n(top_border, info.width().into()));
            first = false;
        }
    }

    // We only need the top right corner, if we need to draw a right border
    if should_draw_right_border(table) {
        line.push(right_corner);
    }

    for connection in connect_above_idx {
        if connection <= &line.len() {
            if line[*connection] == intersection {
                line[*connection] = intersection_connector;
            } else {
                line[*connection] = connector;
            }
        }
    }

    line.into_iter().collect()
}

fn draw_bottom_inline_border(
    table: &Table,
    display_info: &[ColumnDisplayInfo],
    connect_below_idx: &Vec<usize>,
) -> String {
    let left_corner = table.style.bottom_inline_border.left.unwrap_or(' ');
    let bottom_border = table.style.bottom_inline_border.fill.unwrap_or(' ');
    let intersection = table.style.bottom_inline_border.junction.unwrap_or(' ');
    let right_corner = table.style.bottom_inline_border.right.unwrap_or(' ');

    let connector = table.style.top_inline_border.junction.unwrap_or(' ');

    let intersection_connector = table.style.header_separator.junction.unwrap_or(' ');

    let mut line = Vec::new();
    // We only need the bottom left corner, if we need to draw a left border
    if should_draw_left_border(table) {
        line.push(left_corner);
    }

    // Add the bottom border lines depending on column width
    // Also add the border intersections.
    let mut first = true;
    for info in display_info.iter() {
        // Only add something, if the column isn't hidden
        if !info.is_hidden {
            if !first {
                line.push(intersection);
            }
            line.extend(repeat_n(bottom_border, info.width().into()));
            first = false;
        }
    }

    // We only need the bottom right corner, if we need to draw a right border
    if should_draw_right_border(table) {
        line.push(right_corner);
    }

    for connection in connect_below_idx {
        if connection <= &line.len() {
            if line[*connection] == intersection {
                line[*connection] = intersection_connector;
            } else {
                line[*connection] = connector;
            }
        }
    }

    line.into_iter().collect()
}

fn draw_rows(
    lines: &mut Vec<String>,
    rows: &[BuildTableItem],
    table: &Table,
    display_info: &[ColumnDisplayInfo],
    last_inline_table_of_higherst_level: bool,
    is_inline_table: bool,
    very_last_border: &mut Option<String>,
) {
    let draw_left_border = should_draw_left_border(table);
    let draw_right_border = should_draw_right_border(table);
    let draw_vertical_lines = should_draw_vertical_lines(table);

    // Iterate over all rows
    let mut row_iter = rows.iter().enumerate().peekable();
    while let Some((row_index, item)) = row_iter.next() {
        match item {
            BuildTableItem::Inline(inline) => {
                if row_index == 0 && table.header.is_none() && table.style.has_top_border() {
                    lines.pop();
                }
                // Apply arrangement, style and width from the parent table to the inner table.
                let style = table.style();
                let width = table.width();
                let arrangement = table.content_arrangement();
                let mut inline_table = inline.table.table.clone();
                inline_table.load_style(style);
                inline_table.set_content_arrangement(arrangement);
                if let Some(w) = width {
                    inline_table.set_width(w);
                }

                // Crate the display information
                let arranged_content = arrange_content(&inline_table);

                // Identify as last item of highest level and parent is highest level
                let is_last_item = row_iter.peek().is_none();
                let last_item_and_inline_table_of_higherst_level_check =
                    (!is_inline_table || last_inline_table_of_higherst_level) && is_last_item;
                    
                if table.style.has_bottom_border()
                    && last_item_and_inline_table_of_higherst_level_check
                {
                    *very_last_border = Some(draw_bottom_border(&inline_table, &arranged_content))
                }

                // Retreive inner table as a string vector.
                let mut draw_inner_table = draw_borders(
                    &inline_table,
                    &inline.items,
                    &arranged_content,
                    last_item_and_inline_table_of_higherst_level_check,
                    true,
                    very_last_border,
                );

                // Start manipulate borders so the inner table fitts to the parent

                // Manipulate the top border of the inline table if the line before is a row
                let mut connect_above_idx: Vec<usize> = Vec::new();
                if let Some(junction) = table.style.top_inline_border.junction {
                    // Find out where the junctions of the line above placed.
                    let line = draw_top_inline_border(table, display_info, &Vec::new());
                    let line_len = line.chars().count();
                    connect_above_idx = line
                        .chars()
                        .enumerate()
                        .filter(|(idx, c)| {
                            table.style.has_vertical_lines()
                                && *idx != 0
                                && *idx != line_len - 1
                                && *c == junction
                        })
                        .map(|(idx, _)| idx)
                        .collect();
                };

                // Redraw top and bottom border of inner table

                // Only redraw the top inline table border if it have one.
                if inline_table.style.has_top_inline_border() && row_index != 0 {
                    let inline_header_top_seperator = draw_top_inline_border(
                        &inline_table,
                        &arranged_content,
                        &connect_above_idx,
                    );
                    if table.style.has_top_border() {
                        draw_inner_table[0] = inline_header_top_seperator;
                    } else {
                        draw_inner_table.insert(0, inline_header_top_seperator);
                    }
                }

                if inline_table.style.has_bottom_inline_border() {
                    let inline_bootom_seperator = draw_bottom_inline_border(
                        &inline_table,
                        &arranged_content,
                        &connect_above_idx,
                    );
                    if table.style.has_bottom_border() {
                        draw_inner_table.pop();
                    }

                    // Only redraw the bottom inline table border if it have one.
                    if !last_item_and_inline_table_of_higherst_level_check {
                        draw_inner_table.push(inline_bootom_seperator);
                        // To complete the connections, we draw a row seperator if the parent have one and we are not in the last row
                        if table.style.has_row_separator()
                            && table.style.has_bottom_border()
                            && row_iter.peek().is_some()
                        {
                            draw_inner_table.push(draw_horizontal_lines(
                                table,
                                display_info,
                                false,
                            ));
                        }
                    }
                }

                // Append the inner table to the draw vector.
                lines.append(&mut draw_inner_table);
            }
            BuildTableItem::Row(row) => {
                // Styling depends on whether we're currently in the header or not.
                let style = if row_index == 0 && table.header.is_some() {
                    table.style.header_lines
                } else {
                    table.style.content_lines
                };
                let left_border = style.left.unwrap_or(' ');
                let vertical_lines = style.junction.unwrap_or(' ');
                let right_border = style.right.unwrap_or(' ');

                // Concatenate the line parts and insert the vertical borders if needed
                for line_parts in row.iter() {
                    let mut line = String::new();
                    if draw_left_border {
                        line.push(left_border);
                    }

                    let mut part_iter = line_parts.iter().peekable();
                    while let Some(part) = part_iter.next() {
                        line += part;
                        if part_iter.peek().is_none() && draw_right_border {
                            line.push(right_border);
                        } else if part_iter.peek().is_some() && draw_vertical_lines {
                            line.push(vertical_lines);
                        }
                    }

                    lines.push(line);
                }

                // Draw the horizontal header line if desired, otherwise continue to the next iteration
                if row_index == 0 && table.header.is_some() {
                    if table.style.has_header_separator() {
                        lines.push(draw_horizontal_lines(table, display_info, true));
                    }
                    continue;
                }

                // Draw a horizontal line, if we desired and if we aren't in the last row of the table.
                if row_iter.peek().is_some() && table.style.has_row_separator() {
                    lines.push(draw_horizontal_lines(table, display_info, false));
                }
            }
        }
    }
}

// The horizontal line that separates between rows.
fn draw_horizontal_lines(
    table: &Table,
    display_info: &[ColumnDisplayInfo],
    header: bool,
) -> String {
    // Styling depends on whether we're currently on the header line or not.
    let separator = if header {
        table.style.header_separator
    } else {
        table.style.row_separator
    };
    let left_intersection = separator.left.unwrap_or(' ');
    let horizontal_lines = separator.fill.unwrap_or(' ');
    let middle_intersection = separator.junction.unwrap_or(' ');
    let right_intersection = separator.right.unwrap_or(' ');

    let mut line = String::new();
    // We only need the bottom left corner, if we need to draw a left border
    if should_draw_left_border(table) {
        line.push(left_intersection);
    }

    let draw_vertical_lines = should_draw_vertical_lines(table);

    // Append the middle lines depending on the columns' widths.
    // Also add the middle intersections.
    let mut first = true;
    for info in display_info.iter() {
        // Only add something, if the column isn't hidden
        if !info.is_hidden {
            if !first && draw_vertical_lines {
                line.push(middle_intersection);
            }
            line.extend(repeat_n(horizontal_lines, info.width().into()));
            first = false;
        }
    }

    // We only need the bottom right corner, if we need to draw a right border
    if should_draw_right_border(table) {
        line.push(right_intersection);
    }

    line
}

fn draw_bottom_border(table: &Table, display_info: &[ColumnDisplayInfo]) -> String {
    let left_corner = table.style.bottom_border.left.unwrap_or(' ');
    let bottom_border = table.style.bottom_border.fill.unwrap_or(' ');
    let middle_intersection = table.style.bottom_border.junction.unwrap_or(' ');
    let right_corner = table.style.bottom_border.right.unwrap_or(' ');

    let mut line = String::new();
    // We only need the bottom left corner, if we need to draw a left border
    if should_draw_left_border(table) {
        line.push(left_corner);
    }

    // Add the bottom border lines depending on column width
    // Also add the border intersections.
    let mut first = true;
    for info in display_info.iter() {
        // Only add something, if the column isn't hidden
        if !info.is_hidden {
            if !first {
                line.push(middle_intersection);
            }
            line.extend(repeat_n(bottom_border, info.width().into()));
            first = false;
        }
    }

    // We only need the bottom right corner, if we need to draw a right border
    if should_draw_right_border(table) {
        line.push(right_corner);
    }

    line
}

pub fn should_draw_left_border(table: &Table) -> bool {
    table.style.has_left_border()
}

pub fn should_draw_right_border(table: &Table) -> bool {
    table.style.has_right_border()
}

pub fn should_draw_vertical_lines(table: &Table) -> bool {
    table.style.has_vertical_lines()
}
