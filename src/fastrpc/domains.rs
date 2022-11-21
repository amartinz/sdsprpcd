use std::path::Path;

pub struct FastRpcDomain {
    pub id: u32,
    pub node: &'static Path,
    pub secure_node: &'static Path,
}

pub enum FastRpcDomainType {
    ADSP,
    CDSP,
    MDSP,
    SDSP,
}

pub fn get_domain_for_type(domain_type: FastRpcDomainType) -> FastRpcDomain {
    return match domain_type {
        FastRpcDomainType::ADSP => FastRpcDomain {
            id: 0,
            node: Path::new("/dev/fastrpc-adsp"),
            secure_node: Path::new("/dev/fastrpc-adsp-secure"),
        },
        FastRpcDomainType::CDSP => FastRpcDomain {
            id: 3,
            node: Path::new("/dev/fastrpc-cdsp"),
            // CDSP gets the default ADSP node when requesting a secure domain
            secure_node: Path::new("/dev/fastrpc-adsp"),
        },
        FastRpcDomainType::MDSP => FastRpcDomain {
            id: 1,
            node: Path::new("/dev/fastrpc-mdsp"),
            // MDSP uses the ADSP secure node
            secure_node: Path::new("/dev/fastrpc-adsp-secure"),
        },
        FastRpcDomainType::SDSP => FastRpcDomain {
            id: 2,
            node: Path::new("/dev/fastrpc-sdsp"),
            // SDSP uses the ADSP secure node
            secure_node: Path::new("/dev/fastrpc-adsp-secure"),
        },
    };
}
