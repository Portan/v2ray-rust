use crate::config::{
    BlackHoleConfig, DirectConfig, Http2Config, ShadowsocksConfig, TlsConfig, TrojanConfig,
    VmessConfig, WebsocketConfig, SS_LOCAL_SHARED_CONTEXT,
};
use crate::proxy::blackhole::BlackHoleStreamBuilder;
use crate::proxy::direct::DirectStreamBuilder;
use crate::proxy::h2::Http2StreamBuilder;
use crate::proxy::shadowsocks::ShadowsocksBuilder;
use crate::proxy::tls::tls::TlsStreamBuilder;
use crate::proxy::trojan::TrojanStreamBuilder;
use crate::proxy::vmess::vmess_option::VmessOption;
use crate::proxy::vmess::VmessBuilder;
use crate::proxy::websocket::BinaryWsStreamBuilder;
use crate::proxy::{Address, ChainableStreamBuilder, ProtocolType};

pub trait ToChainableStreamBuilder: Sync + Send {
    fn to_chainable_stream_builder(&self, addr: Option<Address>)
        -> Box<dyn ChainableStreamBuilder>;
    fn tag(&self) -> &str;
    fn clone_box(&self) -> Box<dyn ToChainableStreamBuilder>;
    fn get_protocol_type(&self) -> ProtocolType;
    fn get_addr(&self) -> Option<Address> {
        None
    }
}
impl Clone for Box<dyn ToChainableStreamBuilder> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl ToChainableStreamBuilder for VmessConfig {
    fn to_chainable_stream_builder(
        &self,
        addr: Option<Address>,
    ) -> Box<dyn ChainableStreamBuilder> {
        Box::new(VmessBuilder {
            vmess_option: VmessOption {
                uuid: self.uuid,
                alter_id: 0,
                addr: addr.unwrap(),
                security_num: self.security_num,
                is_udp: false,
            },
        })
    }

    fn tag(&self) -> &str {
        self.tag.as_str()
    }

    fn clone_box(&self) -> Box<dyn ToChainableStreamBuilder> {
        Box::new(self.clone())
    }

    fn get_protocol_type(&self) -> ProtocolType {
        ProtocolType::VMESS
    }

    fn get_addr(&self) -> Option<Address> {
        Some(self.addr.clone())
    }
}

impl ToChainableStreamBuilder for TrojanConfig {
    fn to_chainable_stream_builder(
        &self,
        addr: Option<Address>,
    ) -> Box<dyn ChainableStreamBuilder> {
        Box::new(TrojanStreamBuilder::new(
            addr.unwrap(),
            self.password.as_bytes(),
            false,
        ))
    }
    fn tag(&self) -> &str {
        self.tag.as_str()
    }
    fn clone_box(&self) -> Box<dyn ToChainableStreamBuilder> {
        Box::new(self.clone())
    }

    fn get_protocol_type(&self) -> ProtocolType {
        ProtocolType::TROJAN
    }

    fn get_addr(&self) -> Option<Address> {
        Some(self.addr.clone())
    }
}
impl ToChainableStreamBuilder for TlsConfig {
    fn to_chainable_stream_builder(
        &self,
        _addr: Option<Address>,
    ) -> Box<dyn ChainableStreamBuilder> {
        Box::new(TlsStreamBuilder::new_from_config(
            self.sni.clone(),
            &self.cert_file,
            &self.key_file,
            self.verify_hostname,
            self.verify_sni,
        ))
    }
    fn tag(&self) -> &str {
        self.tag.as_str()
    }
    fn clone_box(&self) -> Box<dyn ToChainableStreamBuilder> {
        Box::new(self.clone())
    }

    fn get_protocol_type(&self) -> ProtocolType {
        ProtocolType::TLS
    }
}

impl ToChainableStreamBuilder for WebsocketConfig {
    fn to_chainable_stream_builder(
        &self,
        _addr: Option<Address>,
    ) -> Box<dyn ChainableStreamBuilder> {
        // we use early data config in uri query.
        if self.uri.max_early_data > 0 {
            Box::new(BinaryWsStreamBuilder::new_from_config(
                self.uri.uri.clone(),
                self.uri.max_early_data,
                self.uri.early_data_header_name.clone(),
                None,
                self.headers.clone(),
            ))
        } else if self.max_early_data > 0 && !self.early_data_header_name.is_empty() {
            Box::new(BinaryWsStreamBuilder::new_from_config(
                self.uri.uri.clone(),
                self.max_early_data,
                self.early_data_header_name.clone(),
                None,
                self.headers.clone(),
            ))
        } else {
            Box::new(BinaryWsStreamBuilder::new_from_config(
                self.uri.uri.clone(),
                0,
                String::new(),
                None,
                self.headers.clone(),
            ))
        }
    }
    fn tag(&self) -> &str {
        self.tag.as_str()
    }
    fn clone_box(&self) -> Box<dyn ToChainableStreamBuilder> {
        Box::new(self.clone())
    }

    fn get_protocol_type(&self) -> ProtocolType {
        ProtocolType::WS
    }
}
impl ToChainableStreamBuilder for BlackHoleConfig {
    fn to_chainable_stream_builder(
        &self,
        _addr: Option<Address>,
    ) -> Box<dyn ChainableStreamBuilder> {
        Box::new(BlackHoleStreamBuilder)
    }

    fn tag(&self) -> &str {
        self.tag.as_str()
    }

    fn clone_box(&self) -> Box<dyn ToChainableStreamBuilder> {
        Box::new(self.clone())
    }

    fn get_protocol_type(&self) -> ProtocolType {
        ProtocolType::BLACKHOLE
    }
}
impl ToChainableStreamBuilder for DirectConfig {
    fn to_chainable_stream_builder(
        &self,
        _addr: Option<Address>,
    ) -> Box<dyn ChainableStreamBuilder> {
        Box::new(DirectStreamBuilder)
    }

    fn tag(&self) -> &str {
        self.tag.as_str()
    }

    fn clone_box(&self) -> Box<dyn ToChainableStreamBuilder> {
        Box::new(self.clone())
    }

    fn get_protocol_type(&self) -> ProtocolType {
        ProtocolType::DIRECT
    }
}
impl ToChainableStreamBuilder for ShadowsocksConfig {
    fn to_chainable_stream_builder(
        &self,
        addr: Option<Address>,
    ) -> Box<dyn ChainableStreamBuilder> {
        Box::new(ShadowsocksBuilder::new_from_config(
            addr.unwrap(),
            self.password.as_str(),
            self.method,
            SS_LOCAL_SHARED_CONTEXT.clone(),
        ))
    }
    fn tag(&self) -> &str {
        self.tag.as_str()
    }
    fn clone_box(&self) -> Box<dyn ToChainableStreamBuilder> {
        Box::new(self.clone())
    }

    fn get_protocol_type(&self) -> ProtocolType {
        ProtocolType::SS
    }

    fn get_addr(&self) -> Option<Address> {
        Some(self.addr.clone())
    }
}

impl ToChainableStreamBuilder for Http2Config {
    fn to_chainable_stream_builder(
        &self,
        _addr: Option<Address>,
    ) -> Box<dyn ChainableStreamBuilder> {
        Box::new(Http2StreamBuilder::new(
            self.hosts.clone(),
            self.headers.clone(),
            self.method.clone(),
            self.path.clone(),
        ))
    }

    fn tag(&self) -> &str {
        self.tag.as_str()
    }

    fn clone_box(&self) -> Box<dyn ToChainableStreamBuilder> {
        Box::new(self.clone())
    }

    fn get_protocol_type(&self) -> ProtocolType {
        ProtocolType::H2
    }
}
