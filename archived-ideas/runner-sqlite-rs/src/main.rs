use std::{fs::File, io::Read};

type AppResult<T> = anyhow::Result<T>;

use anyhow::format_err;
use sqlite_rs::header::SqliteHeader;

fn main() -> AppResult<()> {
    App::run()?;
    Ok(())
}

struct App;

impl App {
    fn run() -> AppResult<()> {
        let mut f = File::open("flights.db")?;
        let mut sqlite_header_buffer: [u8; 100] = [0; 100];

        let _ = f.read(&mut sqlite_header_buffer)?;

        Self::hexdump(&sqlite_header_buffer)?;

        let sqlite_header = SqliteHeader::try_from(&sqlite_header_buffer)
            .map_err(|error| format_err!("{error}"))?;
        
        println!("{sqlite_header:?}");
        
        Self::print_sqlite_info(&sqlite_header)?;

        Ok(())
    }
    fn print_sqlite_info(sqlite_header: &SqliteHeader) -> AppResult<()> {
        const LABEL_WIDTH: usize = 21;

    let mut output = "".to_owned();

    output.push_str(&format!(
      "{label: <w$}{value}\n",
      w = LABEL_WIDTH,
      label = "database page size:",
      value = **sqlite_header.page_size()
    ));
    output.push_str(&format!(
      "{label: <w$}{value}\n",
      w = LABEL_WIDTH,
      label = "write format:",
      value =
        u8::from(sqlite_header.file_format_version_numbers().write_version())
    ));
    output.push_str(&format!(
      "{label: <w$}{value}\n",
      w = LABEL_WIDTH,
      label = "read format:",
      value =
        u8::from(sqlite_header.file_format_version_numbers().read_version())
    ));
    output.push_str(&format!(
      "{label: <w$}{value}\n",
      w = LABEL_WIDTH,
      label = "reserved bytes:",
      value = **sqlite_header.reserved_bytes_per_page()
    ));
    output.push_str(&format!(
      "{label: <w$}{value}\n",
      w = LABEL_WIDTH,
      label = "file change counter:",
      value = **sqlite_header.file_change_counter()
    ));

    output.push_str(&format!(
      "{label: <w$}{value}\n",
      w = LABEL_WIDTH,
      label = "database page count:",
      value = **sqlite_header.db_filesize_in_pages()
    ));

    output.push_str(&format!(
      "{label: <w$}{value}\n",
      w = LABEL_WIDTH,
      label = "freelist page count:",
      value = **sqlite_header.freelist_pages().total()
    ));

    output.push_str(&format!(
      "{label: <w$}{value}\n",
      w = LABEL_WIDTH,
      label = "schema cookie:",
      value = **sqlite_header.schema_cookie()
    ));

    output.push_str(&format!(
      "{label: <w$}{value}\n",
      w = LABEL_WIDTH,
      label = "schema format:",
      value = u32::from(sqlite_header.schema_format())
    ));

    output.push_str(&format!(
      "{label: <w$}{value}\n",
      w = LABEL_WIDTH,
      label = "default cache size:",
      value = **sqlite_header.suggested_cache_size()
    ));

    output.push_str(&format!(
      "{label: <w$}{value}\n",
      w = LABEL_WIDTH,
      label = "autovacuum top root:",
      value = **sqlite_header
        .incremental_vacuum_settings()
        .largest_root_btree_page()
    ));

    output.push_str(&format!(
      "{label: <w$}{value}\n",
      w = LABEL_WIDTH,
      label = "incremental vacuum:",
      value = u32::from(
        sqlite_header
          .incremental_vacuum_settings()
          .incremental_vacuum_mode()
      )
    ));

    output.push_str(&format!(
      "{label: <w$}{value}\n",
      w = LABEL_WIDTH,
      label = "text encoding:",
      value = sqlite_header.database_text_encoding()
    ));

    output.push_str(&format!(
      "{label: <w$}{value}\n",
      w = LABEL_WIDTH,
      label = "user version:",
      value = **sqlite_header.user_version()
    ));

    output.push_str(&format!(
      "{label: <w$}{value}\n",
      w = LABEL_WIDTH,
      label = "application id:",
      value = **sqlite_header.application_id()
    ));

    output.push_str(&format!(
      "{label: <w$}{value}\n",
      w = LABEL_WIDTH,
      label = "software version:",
      value = **sqlite_header.write_library_version()
    ));

    println!("{output}");
    Ok(())
    }

    fn hexdump(bytes: &[u8]) -> AppResult<()> {
        use hexyl::{BorderStyle, PrinterBuilder};
        use std::io;

        let stdout = io::stdout();
        let mut handle = stdout.lock();
        let mut printer = PrinterBuilder::new(&mut handle)
            .show_color(true)
            .show_char_panel(true)
            .show_position_panel(true)
            .with_border_style(BorderStyle::Unicode)
            .enable_squeezing(false)
            .num_panels(2)
            .group_size(1)
            .build();
        printer.print_all(&bytes[..])?;
        Ok(())
    }
}
