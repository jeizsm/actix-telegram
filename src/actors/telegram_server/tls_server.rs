#[cfg(feature = "tls")]
use actix_web::server::NativeTlsAcceptor;
#[cfg(feature = "alpn")]
use actix_web::server::OpensslAcceptor;
#[cfg(feature = "rust-tls")]
use actix_web::server::RustlsAcceptor;
#[cfg(feature = "tls")]
use native_tls::{Identity, Protocol, TlsAcceptor};
#[cfg(feature = "alpn")]
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
#[cfg(feature = "rust-tls")]
use rustls::{
    internal::pemfile::{certs, pkcs8_private_keys, rsa_private_keys},
    Certificate, NoClientAuth, PrivateKey, ServerConfig,
};
#[allow(unused_imports)]
use std::fs::{self, File};
#[allow(unused_imports)]
use std::io::BufReader;
use std::path::{Path, PathBuf};
use types::InputFile;

pub enum Key {
    #[cfg(feature = "rust-tls")]
    Rustls(RustlsKey),
    #[cfg(feature = "alpn")]
    Openssl(OpensslKey),
    #[cfg(feature = "tls")]
    NativeTls(NativeTlsKey),
}

#[cfg(feature = "rust-tls")]
pub enum KeyKind {
    RSA,
    PKCS8,
}

#[cfg(feature = "rust-tls")]
pub struct RustlsKey {
    key: PathBuf,
    kind: KeyKind,
}

#[cfg(feature = "alpn")]
pub struct OpensslKey {
    key: PathBuf,
}

#[cfg(feature = "tls")]
pub struct NativeTlsKey {
    password: String,
}

impl Key {
    #[cfg(feature = "rust-tls")]
    pub fn new<P: AsRef<Path>>(key: P, kind: KeyKind) -> Self {
        let key = key.as_ref().to_path_buf();
        if !key.exists() {
            panic!("Key file not found")
        }
        Key::Rustls(RustlsKey { key, kind })
    }

    #[cfg(feature = "openssl")]
    pub fn new<P: AsRef<Path>>(key: P) -> Self {
        let key = key.as_ref().to_path_buf();
        if !key.exists() {
            panic!("Key file not found")
        }
        Key::Openssl(OpensslKey { key })
    }

    #[cfg(feature = "tls")]
    pub fn new(password: String) -> Self {
        Key::NativeTls(NativeTlsKey { password })
    }

    #[cfg(feature = "rust-tls")]
    fn key(&self) -> PrivateKey {
        match self {
            Key::Rustls(RustlsKey { key, kind }) => {
                let key_file = &mut BufReader::new(File::open(key.as_path()).unwrap());
                let mut keys = match kind {
                    KeyKind::RSA => rsa_private_keys(key_file).unwrap(),
                    KeyKind::PKCS8 => pkcs8_private_keys(key_file).unwrap(),
                };
                keys.remove(0)
            }
        }
    }

    #[cfg(feature = "alpn")]
    fn key(&self) -> &Path {
        match self {
            Key::Openssl(OpensslKey { key }) => key.as_path(),
        }
    }

    #[cfg(feature = "tls")]
    fn key(&self) -> &str {
        match self {
            Key::NativeTls(NativeTlsKey { password }) => password,
        }
    }
}

pub enum Cert {
    #[cfg(feature = "rust-tls")]
    Rustls(RustlsCert),
    #[cfg(feature = "alpn")]
    Openssl(OpensslCert),
    #[cfg(feature = "tls")]
    NativeTls(NativeTlsCert),
}

#[cfg(feature = "rust-tls")]
pub struct RustlsCert {
    cert: PathBuf,
}

#[cfg(feature = "alpn")]
pub struct OpensslCert {
    cert: PathBuf,
}

#[cfg(feature = "tls")]
pub struct NativeTlsCert {
    cert_pem: PathBuf,
    cert_p12: PathBuf,
}

impl Cert {
    #[cfg(feature = "rust-tls")]
    pub fn new<P: AsRef<Path>>(cert: P) -> Self {
        let cert = cert.as_ref().to_path_buf();
        if !cert.exists() {
            panic!("Certificate file not found")
        }
        Cert::Rustls(RustlsCert { cert })
    }

    #[cfg(feature = "openssl")]
    pub fn new<P: AsRef<Path>>(cert: P) -> Self {
        let cert = cert.as_ref().to_path_buf();
        if !cert.exists() {
            panic!("Certificate file not found")
        }
        Cert::Openssl(OpensslCert { cert })
    }

    #[cfg(feature = "tls")]
    pub fn new<P: AsRef<Path>, P1: AsRef<Path>>(cert_pem: P, cert_p12: P1) -> Self {
        let cert_pem = cert_pem.as_ref().to_path_buf();
        let cert_p12 = cert_p12.as_ref().to_path_buf();
        if !(cert_pem.exists() && cert_p12.exists()) {
            panic!("Certificate files not found")
        }
        Cert::NativeTls(NativeTlsCert { cert_pem, cert_p12 })
    }

    #[cfg(feature = "rust-tls")]
    fn cert(&self) -> Vec<Certificate> {
        match self {
            Cert::Rustls(RustlsCert { cert }) => {
                let cert_file = &mut BufReader::new(File::open(cert.as_path()).unwrap());
                certs(cert_file).unwrap()
            }
        }
    }

    #[cfg(feature = "alpn")]
    fn cert(&self) -> &Path {
        match self {
            Cert::Openssl(OpensslCert { cert }) => cert.as_path(),
        }
    }

    #[cfg(feature = "tls")]
    fn cert(&self) -> Vec<u8> {
        match self {
            Cert::NativeTls(NativeTlsCert { cert_p12, .. }) => {
                fs::read(cert_p12.as_path()).unwrap()
            }
        }
    }

    #[cfg(feature = "rust-tls")]
    pub(super) fn input_file(&self) -> InputFile {
        match self {
            Cert::Rustls(RustlsCert { cert }) => InputFile::Disk {
                path: cert.to_string_lossy().to_string(),
            },
        }
    }

    #[cfg(feature = "alpn")]
    pub(super) fn input_file(&self) -> InputFile {
        match self {
            Cert::Openssl(OpensslCert { cert }) => InputFile::Disk {
                path: cert.to_string_lossy().to_string(),
            },
        }
    }

    #[cfg(feature = "tls")]
    pub(super) fn input_file(&self) -> InputFile {
        match self {
            Cert::NativeTls(NativeTlsCert { cert_pem, .. }) => InputFile::Disk {
                path: cert_pem.to_string_lossy().to_string(),
            },
        }
    }
}

pub struct CertAndKey {
    pub(super) cert: Cert,
    key: Key,
}

impl CertAndKey {
    pub fn new(cert: Cert, key: Key) -> Self {
        Self { cert, key }
    }

    #[cfg(feature = "rust-tls")]
    pub(super) fn get_acceptor(&self) -> RustlsAcceptor {
        let mut config = ServerConfig::new(NoClientAuth::new());
        config
            .set_single_cert(self.cert.cert(), self.key.key())
            .unwrap();
        RustlsAcceptor::new(config)
    }

    #[cfg(feature = "alpn")]
    pub(super) fn get_acceptor(&self) -> OpensslAcceptor {
        let mut builder = SslAcceptor::mozilla_modern(SslMethod::tls()).unwrap();
        builder
            .set_private_key_file(self.key.key(), SslFiletype::PEM)
            .unwrap();
        builder.set_certificate_chain_file(self.cert.cert()).unwrap();
        OpensslAcceptor::new(builder).unwrap()
    }

    #[cfg(feature = "tls")]
    pub(super) fn get_acceptor(&self) -> NativeTlsAcceptor {
        let identity = Identity::from_pkcs12(&self.cert.cert(), self.key.key()).unwrap();
        let acceptor = TlsAcceptor::builder(identity)
            .min_protocol_version(Some(Protocol::Tlsv12))
            .build();
        NativeTlsAcceptor::new(acceptor.unwrap())
    }
}
