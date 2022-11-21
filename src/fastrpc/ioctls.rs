use std::fs::File;
use std::os::unix::io::AsRawFd;

use nix::{ioctl_none, ioctl_readwrite};
use nix::errno::Errno;

use crate::fastrpc::domains::FastRpcDomain;

#[repr(C)]
pub struct fastrpc_ioctl_capability {
    pub domain: u32,
    pub attribute_id: u32,
    pub capability: u32,
    pub(crate) reserved: [u32; 4],
}

// #define FASTRPC_IOCTL_INIT_ATTACH_SNS	_IO('R', 8)
const FASTRPC_INIT_ATTACH_SNS_TYPE: u8 = b'R';
const FASTRPC_INIT_ATTACH_SNS_NR: u8 = 8;
ioctl_none!(fastrpc_init_attach_sns, FASTRPC_INIT_ATTACH_SNS_TYPE, FASTRPC_INIT_ATTACH_SNS_NR);

// #define FASTRPC_IOCTL_GET_DSP_INFO	_IOWR('R', 13, struct fastrpc_ioctl_capability)
const FASTRPC_GET_DSP_INFO_TYPE: u8 = b'R';
const FASTRPC_GET_DSP_INFO_NR: u8 = 13;
ioctl_readwrite!(fastrpc_get_dsp_info, FASTRPC_GET_DSP_INFO_TYPE, FASTRPC_GET_DSP_INFO_NR,
    fastrpc_ioctl_capability);

pub fn init_and_attach_sns(domain: &FastRpcDomain) -> bool {
    let fastrpc_node = open_dev_node(&domain, false);
    let ret = match fastrpc_node {
        Some(node) => unsafe { fastrpc_init_attach_sns(node.as_raw_fd()) }
        _ => Err(Errno::ENOENT)
    };
    if ret.is_err() {
        println!("Could not attach to sensor protection domain: {}", ret.unwrap_err().desc());
        return false;
    }

    println!("Attached to sensor protection domain");
    return true;
}

pub fn is_domain_supported(domain: &FastRpcDomain) -> bool {
    let dsp_info_data = obtain_dsp_info_data(domain, 0);
    return dsp_info_data.capability == 1;
}

pub fn is_unsigned_protection_domain_supported(domain: &FastRpcDomain) -> bool {
    let dsp_info_data = obtain_dsp_info_data(domain, 1);
    return dsp_info_data.capability == 1;
}

fn obtain_dsp_info_data(domain: &FastRpcDomain, attribute_id: u32) -> fastrpc_ioctl_capability {
    let mut dsp_info_data = fastrpc_ioctl_capability {
        domain: domain.id,
        attribute_id,
        capability: 0,
        reserved: [0, 0, 0, 0],
    };

    let fastrpc_node = open_dev_node(&domain, false);
    let ret = match fastrpc_node {
        Some(node) => unsafe { fastrpc_get_dsp_info(node.as_raw_fd(), &mut dsp_info_data) }
        _ => Err(Errno::ENOENT)
    };
    if ret.is_err() {
        println!("Could not obtain DSP information: {}", ret.unwrap_err().desc());
        return dsp_info_data;
    }

    println!("Obtained DSP information: domain={}, attribute_id={}, capability={}",
             dsp_info_data.domain, dsp_info_data.attribute_id, dsp_info_data.capability);
    return dsp_info_data;
}

fn open_dev_node(domain: &FastRpcDomain, secure: bool) -> Option<File> {
    let fastrpc_node = if secure {
        domain.secure_node
    } else {
        domain.node
    };
    return match File::open(&fastrpc_node) {
        Err(why) => {
            println!("Could not open {}: {}", domain.node.display(), why.to_string());
            None
        }
        Ok(file) => Some(file),
    };
}
