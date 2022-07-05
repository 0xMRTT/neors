use clap::Parser;
extern crate os_release;

use os_release::OsRelease;
use std::io;
use std::collections::HashMap;
#[derive(Debug)]
enum DistrosLogo {
    ArchLinux,
    ArchLinuxARM,
    Debian,
    Tux, // Default
    Fedora,
    Gentoo,
    Kali,
    KUbuntu,
    LinuxMint,
    Manjaro,
    OpenSuse,
    Ubuntu,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The name of the distro to display the logo for.
    #[clap(short, long, value_parser)]
    distro_logo: Option<String>,

}

fn auto_determine_distro() -> DistrosLogo {
    let release = OsRelease::new().unwrap();
    let distro = release.id;
    match distro.as_str() {
        "arch" => DistrosLogo::ArchLinux,
        "arch-arm" => DistrosLogo::ArchLinuxARM,
        "debian" => DistrosLogo::Debian,
        "tux" => DistrosLogo::Tux,
        "fedora" => DistrosLogo::Fedora,
        "gentoo" => DistrosLogo::Gentoo,
        "kali" => DistrosLogo::Kali,
        "ubuntu" => DistrosLogo::KUbuntu,
        "linuxmint" => DistrosLogo::LinuxMint,
        "manjaro" => DistrosLogo::Manjaro,
        "opensuse" => DistrosLogo::OpenSuse,
        "ubuntu" => DistrosLogo::Ubuntu,
        _ => DistrosLogo::Tux,
    }
    
}

fn get_distro_version() -> String {
    let release = OsRelease::new().unwrap();
    release.version
}

fn get_distro_name() -> String {
    let release = OsRelease::new().unwrap();
    release.name
}

fn get_distro_pretty_name() -> String {
    let release = OsRelease::new().unwrap();
    release.pretty_name
}

fn make_data() -> HashMap<String, String> {
    let mut data = HashMap::new();
    data.insert("distro".to_string(), format!("{:?}", auto_determine_distro()));
    data.insert("distro.version".to_string(), get_distro_version());
    data.insert("distro.name".to_string(), get_distro_name());
    data.insert("distro.pretty_name".to_string(), get_distro_pretty_name());
}

fn main() {
    let args = Args::parse();

    println!("Distro : {:?}", auto_determine_distro());
    println!("Version: {}", get_distro_version());
    println!("Name   : {}", get_distro_name());
    println!("Pretty : {}", get_distro_pretty_name());

}
