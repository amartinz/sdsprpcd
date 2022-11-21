use crate::fastrpc::domains::*;
use crate::fastrpc::ioctls::*;

pub mod fastrpc;

fn main() {
    let adsp_domain = get_domain_for_type(FastRpcDomainType::ADSP);
    init_and_attach_sns(&adsp_domain);

    let is_adsp_supported = is_domain_supported(&adsp_domain);
    let is_adsp_unsigned_pd_supported = is_unsigned_protection_domain_supported(&adsp_domain);
    println!("ADSP: supported={}, unsigned_pd_supported={}",
             is_adsp_supported, is_adsp_unsigned_pd_supported);

    let cdsp_domain = get_domain_for_type(FastRpcDomainType::CDSP);
    let is_cdsp_supported = is_domain_supported(&cdsp_domain);
    let is_cdsp_unsigned_pd_supported = is_unsigned_protection_domain_supported(&cdsp_domain);
    println!("CDSP: supported={}, unsigned_pd_supported={}",
             is_cdsp_supported, is_cdsp_unsigned_pd_supported);

    let mdsp_domain = get_domain_for_type(FastRpcDomainType::MDSP);
    let is_mdsp_supported = is_domain_supported(&mdsp_domain);
    let is_mdsp_unsigned_pd_supported = is_unsigned_protection_domain_supported(&mdsp_domain);
    println!("MDSP: supported={}, unsigned_pd_supported={}",
             is_mdsp_supported, is_mdsp_unsigned_pd_supported);

    let sdsp_domain = get_domain_for_type(FastRpcDomainType::SDSP);
    let is_sdsp_supported = is_domain_supported(&sdsp_domain);
    let is_sdsp_unsigned_pd_supported = is_unsigned_protection_domain_supported(&sdsp_domain);
    println!("SDSP: supported={}, unsigned_pd_supported={}",
             is_sdsp_supported, is_sdsp_unsigned_pd_supported);
}
