use std::{fs::File, str::FromStr};
use std::fs;
use std::io::prelude::*;
use reqwest::Proxy;
use serde_derive::{Deserialize, Serialize};
use vpngen_lib::get_config;
use std::path::PathBuf;

use clap::Parser;


use ss_to_json_outline;
use amcofixer_lib;

/*fn print_format(config: &ConfigJson, template_text: &String) -> String {
    let text_to_return = template_text
        .replace("[!outline_config]", &config.outline_config.access_key)
        .replace("[!amnezia_config]", &config.amnz_ovc_config.file_content)
        .replace("[!amnezia_path]", &config.amnz_ovc_config.file_name)
        .replace("[!wireguard_config]", &config.wireguard_config.file_content)
        .replace("[!wireguard_path]", &config.wireguard_config.file_name);
    text_to_return
}   */

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    template: Option<PathBuf>, //Test option.
    #[arg(short, long, value_name = "FILE")]
    config_dir: Option<PathBuf>,
    #[arg(short, long)]
    shadowsocks: bool,
    #[arg(short, long)]
    amnezia: bool,
    #[arg(short='f', long)]
    amnezia_fixer: bool,
    #[arg(short, long)]
    wireguard: bool,
    #[arg(short='g', long)]
    vpn_gen_config: bool,
    #[arg(short='l', long)]
    vless: bool,
    #[arg(long)]
    all: bool,
    /// Usage example: vpngen_lib --socks5 127.0.0.1:9052
    #[arg(long)]
    socks5: Option<String>
}

fn save_to_file(file_name: &String, file_content: &String) -> std::io::Result<()> {
    let mut file = File::create(file_name.as_str())?;
    file.write_all(file_content.as_bytes())?;
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct TemplateText{
    outline: String,
    shadowsocks: String,
    amnezia: String,
    amnezia_fixer: String,
    wireguard: String,
    vless: String,
    vpngen: String
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    let template_text = match  cli.template {
        Some(template_path) => {
            let data = fs::read_to_string(template_path)?;
            serde_json::from_str(&data)?
        }
        None => TemplateText{
            outline: "Outline: [!outline_config]".to_string(),
            shadowsocks: "ShadowSocks: [!shadowsocks]".to_string(),
            amnezia: "Amnezia: [!amnezia_path]".to_string(),
            amnezia_fixer: "Fixed amnezia file: [!amnezia_path]".to_string(),
            wireguard: "Wireguard: [!wireguard_path]".to_string(),
            vless: "Vless: [!vless]".to_string(),
            vpngen: "VpnGen: [!vpngen]".to_string()
        }
    };

    let config_dir= match  cli.config_dir {
        Some(template_path) => template_path,
        None => PathBuf::from_str("./vpn_config").unwrap()   
    }.to_str().unwrap().to_string();

    let client = match cli.socks5 {
        Some(data) => reqwest::Client::builder().proxy(Proxy::http(data)?).build()?,
        None => reqwest::Client::new()
    };
    
    
    let config = get_config(&client).await?;

    let mut printable_text: String = format!("### {}\n\n", config.user_name);

    printable_text.push_str(template_text.outline.replace("[!outline_config]", &config.outline_config.access_key).as_str());
    if cli.shadowsocks || cli.all{
        printable_text.push('\n');
        printable_text.push('\n');
        printable_text.push_str(&template_text.shadowsocks.replace("[!shadowsocks]", &format!("{}.json", &config.user_name).as_str()));
        let shadowsocks_config = serde_json::to_string(&ss_to_json_outline::ss_to_json(config.outline_config.access_key).unwrap()).unwrap();
        save_to_file(&format!("{}/{}.json", config_dir, config.user_name), &shadowsocks_config)?;
    }
    
    if cli.amnezia || cli.all{
        printable_text.push('\n');
        printable_text.push('\n');
        printable_text.push_str(template_text.amnezia.replace("[!amnezia_path]", &config.amnz_ovc_config.file_name).as_str());
        save_to_file(&format!("{}/{}", config_dir, config.amnz_ovc_config.file_name), &config.amnz_ovc_config.file_content)?;
    }
    if cli.amnezia_fixer || cli.all{
        printable_text.push('\n');
        printable_text.push('\n');
        printable_text.push_str(&template_text.amnezia_fixer.replace("[!amnezia_path]", &format!("fixed_{}", &config.amnz_ovc_config.file_name).as_str()));
        save_to_file(&format!("{}/fixed_{}", config_dir, config.amnz_ovc_config.file_name), &amcofixer_lib::fixer(&config.amnz_ovc_config.file_content))?;
    }
    if cli.wireguard || cli.all{
        printable_text.push('\n');
        printable_text.push('\n');
        printable_text.push_str(template_text.wireguard.replace("[!wireguard_path]", &config.wireguard_config.file_name).as_str());
        save_to_file(&format!("{}/{}", config_dir, config.wireguard_config.file_name), &config.wireguard_config.file_content)?;
    }
    if cli.vpn_gen_config || cli.all {
        printable_text.push_str(
            format!(
                "\n\n{}",
                template_text.vpngen.replace("[!vpngen]", &config.v_p_n_gen_config)
            ).as_str()
        );

    }
    if cli.vless || cli.all {
        printable_text.push_str(
            format!(
                "\n\n{}",
                template_text.vless.replace("[!vless]", &config.proto0_config.access_key)
            ).as_str()
        );

    }
    println!("{}", printable_text);
    Ok(())
}
