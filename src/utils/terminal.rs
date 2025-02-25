// Wrapper for pretty-printing messages to the Terminal

use crate::api::ModSearchResult;
use colored::Colorize;
use dialoguer::Confirm;
use dialoguer::theme::ColorfulTheme;
use std::env;
use std::fmt::Display;

pub struct Terminal {
    colors_enabled: bool,
}

impl Terminal {
    pub fn new() -> Terminal {
        Terminal {
            colors_enabled: Terminal::colors_enabled(),
        }
    }

    fn colors_enabled() -> bool {
        env::var_os("NO_COLOR").is_none() && colored::control::SHOULD_COLORIZE.should_colorize()
    }

    pub fn print<T: ToString>(message: T) {
        println!("{}", message.to_string());
    }

    /// Prints a message to the terminal with a newline
    pub fn println<T: ToString>(message: T) {
        println!("{}", message.to_string());
    }

    /// Prints a message to the terminal with a newline
    pub fn print_error<T: ToString>(message: T) {
        eprintln!("{}", message.to_string());
    }

    /// Prints a message to the terminal with a newline
    pub fn print_error_and_exit<T: ToString>(message: T) {
        eprintln!("{}", message.to_string());
        std::process::exit(1);
    }

    pub fn confirm<T: ToString>(message: T) -> bool {
        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(message.to_string())
            .interact()
            .unwrap()
    }

    pub fn select<T: Display>(message: &str, options: &[T]) -> Option<usize> {
        dialoguer::FuzzySelect::with_theme(&ColorfulTheme::default())
            .with_prompt(message)
            .items(options)
            .default(0)
            .interact_opt()
            .unwrap_or(None)
    }

    pub fn input(message: &str) -> String {
        dialoguer::Input::with_theme(&ColorfulTheme::default())
            .with_prompt(message)
            .interact()
            .unwrap()
    }

    pub fn multi_select<T: Display>(message: &str, options: &[T]) -> Vec<usize> {
        dialoguer::MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt(message)
            .items(options)
            .interact()
            .unwrap()
    }

    fn format_mod_options(options: &[ModSearchResult]) -> Vec<String> {
        options
            .iter()
            .map(|m| {
                format!(
                    "{} by {} ({} downloads)",
                    m.name,
                    m.author,
                    m.downloads.unwrap_or(0)
                )
            })
            .collect()
    }

    pub fn print_table<T: ToString>(&self, columns: Vec<Columns<T>>) {
        if columns.is_empty() {
            return;
        }

        let column_widths: Vec<usize> = columns.iter().map(|col| col.max_width()).collect();

        // Print headers
        let header_row = columns
            .iter()
            .zip(&column_widths)
            .map(|(col, width)| format!("{:<width$}", col.header, width = width))
            .collect::<Vec<_>>()
            .join(" | ");

        let separator = column_widths
            .iter()
            .map(|width| "-".repeat(*width))
            .collect::<Vec<_>>()
            .join("-+-");

        if self.colors_enabled {
            println!("{}", header_row.bold());
            println!("{}", separator.dimmed());
        } else {
            println!("{}", header_row);
            println!("{}", separator);
        }

        // Print data rows
        let max_rows = columns.iter().map(|col| col.data.len()).max().unwrap_or(0);
        for row_idx in 0..max_rows {
            let row = columns
                .iter()
                .zip(&column_widths)
                .map(|(col, width)| {
                    format!(
                        "{:<width$}",
                        col.data
                            .get(row_idx)
                            .map(|val| val.to_string())
                            .unwrap_or_default(),
                        width = width
                    )
                })
                .collect::<Vec<_>>()
                .join(" | ");
            println!("{}", row);
        }
    }
}

pub struct Columns<T: ToString> {
    header: String,
    data: Vec<T>,
}

impl<T: ToString> Columns<T> {
    pub fn new(header: &str, data: Vec<T>) -> Columns<T> {
        Columns {
            header: header.to_string(),
            data,
        }
    }

    fn max_width(&self) -> usize {
        let header_width = self.header.len();
        let data_width = self
            .data
            .iter()
            .map(|val| val.to_string().len())
            .max()
            .unwrap_or(0);
        header_width.max(data_width)
    }
}
