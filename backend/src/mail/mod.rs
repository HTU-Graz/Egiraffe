use minijinja::{Environment, context, path_loader};
use lettre::{message::{header::{self, ContentType}, Mailbox, MultiPart, SinglePart}, transport::smtp::{authentication::Credentials, AsyncSmtpTransportBuilder, Error}, Address, AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use crate::conf::CONF;
use crate::conf;
use once_cell::sync::OnceCell;
use std::sync::{Arc, Mutex};
use std::str::FromStr;
use std::fmt::format;
use justerror::Error;

pub static MAILER: OnceCell<Arc<Mutex<AsyncSmtpTransport<Tokio1Executor>>>> = OnceCell::new(); //Not sure if an arc is necessary here.
static ENV: OnceCell<Environment<'static>> = OnceCell::new();

#[Error]
pub enum MailError {
    EmailInvalid(Arc<str>),
    ServerError(),
}

//TODO: Better Error handling instead of unwrap!
pub fn init() {
    //TODO Initialize things only once!

    let mut e = Environment::new();
    e.set_loader(path_loader(&CONF.mail.templatepath));
    ENV.set(e.into()).expect("Error setting up Template Engine Minijinja!");
/*
    env.add_template("layout.html", include_str!(format!("../../{}/layout.html", $MAILTEMPLATE_DIR))).unwrap();
    env.add_template("activationmail.html", include_str!(format!("../../{}/activationmail.html", $MAILTEMPLATE_DIR))).unwrap();
    env.add_template("activationmail.txt", include_str!(format!("../../{}/activationmail.txt", $MAILTEMPLATE_DIR))).unwrap();
*/
    let creds = Credentials::new(CONF.mail.username.to_owned(), CONF.mail.password.to_owned());

    let transport: Result<AsyncSmtpTransportBuilder, Error>;
    if CONF.mail.encryption == conf::EncryptionType::StartTLS {
        transport = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&CONF.mail.hostname);
    } else if CONF.mail.encryption == conf::EncryptionType::SslTls {
        transport = AsyncSmtpTransport::<Tokio1Executor>::relay(&CONF.mail.hostname); //seems to be TLS
    } else {
        transport = Ok(AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&CONF.mail.hostname));
    }

    let mailer = transport
        .unwrap()
        .port(CONF.mail.port)
        .credentials(creds)
        .build();
    MAILER.set(Arc::new(Mutex::new(mailer))).expect("Error setting Mailer!");
    
}

pub async fn send_activation_mail(first_names: &str, last_name: &str, email: &str, token: &str) -> Result<(), MailError> {
    let subject = "Willkommen auf der EGIRAFFE"; //TODO: i18n

    //TODO: Is AutoEscape turned on?
    let vars = context! {
        first_names => first_names,
        last_name => last_name,
        subject => subject,
        activationToken => token,
        acitvationValidityPeriod => CONF.acitvationlinkvalidityperiod
    };

    let html = ENV.get().unwrap().get_template("activationmail.html").unwrap().render(vars.clone()).unwrap();
    println!("{}", html);
    let txt = ENV.get().unwrap().get_template("activationmail.txt").unwrap().render(vars).unwrap();
    println!("{}", txt);


    let email = Message::builder()
        .from(Mailbox::new(Some(CONF.mail.sendername.to_owned()), Address::from_str(&CONF.mail.senderemail).unwrap()))
        //TODO: Encode Malicious characters!
        .to(Mailbox::new(format!("{first_names} {last_name}").into(), Address::from_str(email).expect("Invalid Mail Address")))
        .subject(subject)
//        .header(ContentType::TEXT_MULTIPART)
        .multipart(
            MultiPart::alternative() // This is composed of two parts.
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_PLAIN)
                        .body(String::from(txt)),
                )
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_HTML)
                        .body(String::from(html)),
                ),
        )
        .expect("failed to build email");

    // Send the email
    let mailer = MAILER.get().unwrap().lock().unwrap();
    mailer.send(email).await.expect("Error sending Mail");

    Ok(())
}