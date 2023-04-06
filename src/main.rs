use crate::file_header::page_type::PageType;
use crate::page::base_page::BasePage;
use crate::page::index_page::page::IndexPage;
use crate::page::sdi_blob_page::SdiBlobPage;
use crate::page::PageEnums;
use crate::tablespace::table::read_table_info;
use crate::tablespace::TableSpace;

mod file_header;
mod file_trailer;
mod page;
mod tablespace;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    author = "Lu Beilin",
    version,
    about = "一个.ibd文件解析工具，帮助学习InnoDB"
)]
pub struct BaseArgs {
    /// .ibd文件路径
    ibd_page: String,
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// 查看所有页
    List,
    /// 查看具体页
    Page {
        /// 页号
        page_num: u32,
    },
}

fn cmd() {
    let args = BaseArgs::parse();
    let tablespace = TableSpace::new(args.ibd_page).unwrap();
    match args.command {
        Commands::List => {
            let page = tablespace.fsp_page().unwrap();
            println!("size:{}", page.fsp_header.size());
            for page_num in 0..page.fsp_header.size() {
                let page = tablespace.page(page_num).unwrap();
                println!("page_num: {} , page_type: {:?}", page_num, page.page_type());
            }
        }
        Commands::Page { page_num } => {
            let page = tablespace.page(page_num).unwrap();
            println!("{}", page);
        }
    }
}

fn main() {
    cmd()
}
