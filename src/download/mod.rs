mod mc_dl_addr;

use self::mc_dl_addr::*;
use crate::file_system::*;

pub enum McResDlAddr {
    Official,
    BangBang93,
    McBBS,
}

impl McResDlAddr {
    pub fn select_dl_addr(dl_addr: Self) -> &'static str {
        match dl_addr {
            Self::Official => OFFICIAL,
            Self::BangBang93 => BANGBANG93,
            Self::McBBS => MCBBS,
        }
    }
}

pub fn download_mc_version_manifest(dl_addr: McResDlAddr) {
    todo!()
}
