use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr};

//Sadly the variables with underscores can't be dynamically changed using environment variables - therefore I just concationate everything
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub mail: MailConfig,
    pub webserver: WebServerConfig,

    pub baseurl: String,
    pub acitvationlinkvalidityperiod: i8, //in days

    #[cfg(feature = "import")]
    pub import: ImportConfig,
}

#[cfg(feature = "import")]
#[derive(Debug, Deserialize, Serialize)]
pub struct ImportConfig {
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub debugdefaultentries: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MailConfig {
    pub activated: bool, //Are Mails sent in general?
    pub hostname: String,
    pub username: String,
    pub password: String,
    pub port: u16,
    pub encryption: EncryptionType,
    pub senderemail: String,
    pub sendername: String,
    pub templatepath: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum EncryptionType {
    None,
    StartTLS,
    SslTls,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WebServerConfig {
    // Make sure to build the frontend first!
    pub staticdir: String,
    pub indexfile: String,
    pub ip: IpAddr,
    pub port: u16,
}

pub static CONF: Lazy<Config> = Lazy::new(|| Config::load());

impl Config {
    pub fn load() -> Self {
        let defaults: Config;
        //Production defaults
        #[cfg(feature = "prod")]
        {
            defaults = Config {
                database: DatabaseConfig {
                    url: "".into(),
                    debugdefaultentries: false,
                },
                mail: MailConfig {
                    activated: true,
                    hostname: "".into(),
                    username: "".into(),
                    password: "".into(),
                    port: 587,
                    encryption: EncryptionType::StartTLS,
                    senderemail: "".into(),
                    sendername: "".into(),
                    templatepath: "../frontend/mailtemplates/".into(),
                },
                webserver: WebServerConfig {
                    // Make sure to build the frontend first!
                    staticdir: "../frontend/dist".into(),
                    indexfile: "../frontend/dist/index.html".into(),
                    ip: std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 42)), //TODO: Should that be changed for prod?
                    port: 42002,
                },

                baseurl: "https://egiraffe.at".into(),
                acitvationlinkvalidityperiod: 3,
            };
        }

        //overwrite them with debug defaults if needed
        #[cfg(not(feature = "prod"))]
        {
            defaults = Config {
                database: DatabaseConfig {
                    url: "postgresql://egiraffe:hunter2@localhost:5432/egiraffe?sslmode=disable"
                        .into(),
                    debugdefaultentries: true,
                },
                mail: MailConfig {
                    //I will use Mailpit for Mail related testing
                    activated: false,
                    hostname: "localhost".into(),
                    username: "dummyuser".into(),
                    password: "dummypassword".into(),
                    port: 1025,
                    encryption: EncryptionType::None,
                    senderemail: "debugdummy@localhost".into(),
                    sendername: "Egiraffe Debug Sendername".into(),
                    templatepath: "../frontend/mailtemplates/".into(),
                },
                webserver: WebServerConfig {
                    // Make sure to build the frontend first!
                    staticdir: "../frontend/dist".into(),
                    indexfile: "../frontend/dist/index.html".into(),
                    ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 42)),
                    port: 42002,
                },
                baseurl: "http://localhost:42002".into(),
                acitvationlinkvalidityperiod: 3,
                #[cfg(feature = "import")]
                import: ImportConfig {
                    url: "mysql://root:hunter2@localhost:3306/eg_old".into(),
                },
            };
        }

        let config: Config = Figment::from(Serialized::defaults(defaults))
            .merge(Toml::file("eg_config.toml"))
            .merge(Env::prefixed("EGIRAFFE_").split("_"))
            .extract()
            .unwrap();

        // println!("config: {:#?}", config);

        // Validate some default values
        Config::validate(&config);

        config
    }

    pub fn validate(s: &Config) {
        //TODO: Validate  exists!

        #[cfg(not(feature = "prod"))]
        {
            //No Validations (yet)
        }

        #[cfg(feature = "prod")]
        {
            assert!(s.database.debugdefaultentries == false);
            assert!(s.mail.encryption != EncryptionType::None); //if desired, remove the check
            assert!(s.mail.activated == true);

            assert!(s.database.url != "");
            assert!(s.mail.hostname != "");
            assert!(s.mail.username != "");
            assert!(s.mail.password != "");
            assert!(s.mail.senderemail != "");
            assert!(s.mail.sendername != "");
        }
    }
}
