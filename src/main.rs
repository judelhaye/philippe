extern crate irc;
extern crate rand;

use std::default::Default;
use std::convert::AsRef;

use irc::client::prelude::*;
use rand::{thread_rng, sample};

fn eightball() -> String {
    let mut rng = rand::thread_rng();
    let answers: Vec<&str> = vec![
        "Essaye plus tard.",
        "Essaye encore.",
        "Pas d'avis.",
        "C'est ton destin.",
        "Le sort en est jeté.",
        "Une chance sur deux.",
        "Repose ta question.",
        "D'après moi, oui.",
        "C'est certain.",
        "Oui, Absolument !",
        "Tu peux compter la dessus !",
        "Sans aucun doute.",
        "Très probable.",
        "Oui",
        "C'est bien parti",
        "C'est non.",
        "Peu probable.",
        "Faut pas réver.",
        "N'y comptes pas.",
        "Impossible.",
        "Et la marmotte, elle mets le chocolat dans le papier d'alu ...",
        "Réfléchis-y longuement ..."
    ];
    let sample = sample(&mut rng, answers, 1);
    sample[0].to_string()
}

fn sender(msg: Option<String>) -> String {
	let line = msg.unwrap();
    let prefix: Vec<&str> = line.split("!").collect();
	prefix[0].to_string()
}

fn main() {
    let config = Config {
        nickname: Some(format!("De4dBot")),
        alt_nicks: Some(vec![format!("rusty")]),
        server: Some(format!("irc.yousserver.tld")),
        channels: Some(vec![format!("#sandbox")]),
        .. Default::default()
    };
    let mut awaymode = false;
    let server = IrcServer::from_config(config).unwrap();
    server.identify().unwrap();
    for message in server.iter() {
        let message = message.unwrap();
        println!("{}", message.into_string());
        match &message.command[..] {
            "PRIVMSG" => {
                if let Some(msg) = message.suffix {
                    let nick = sender(message.prefix);
                    let mut chan = String::new();
                    match message.args[0].as_ref() {
                        "De4dBot" => chan = nick.clone(),
                        _ => chan = message.args[0].clone(),
                    }
                    /*
                     * Check if message is for deadbot
                     *  and act if necessary
                     */
                    if msg.contains("De4dBot")
                        || message.args[0] == "De4dBot" {
                        // Panier is for quitting the chan
                        if msg.contains("panier"){
                            let _ = server.send_quit("Bye bye");
                        } else if msg.contains("mode")
                                  && nick.to_lowercase() == "jdelhaye".to_string() {
                            // only khrogos can change modes
                            if msg.contains("away"){
                                awaymode = !awaymode;
                                let reply = "away mode is ".to_string()
                                            + &awaymode.to_string();
                                server.send_privmsg(&chan,
                                                    &reply)
                                                    .unwrap()
                            }
                        } else if msg.contains("avis"){
                            // Laungh a magic eightball
                            server.send_privmsg(&chan,
                                                &eightball())
                                                .unwrap();
                        }

                    }

                    /*
                     * analyse messages that are NOT for the bot
                     */
                    else {
                        if awaymode && msg.contains("your_nickname") {
                            let reply = "<your_nickname> is no available ATM.";
                            server.send_privmsg(&chan,
                                                &reply)
                                                .unwrap();
                        }
                    }
                }
            },
            "JOIN" => {
                let nick = sender(message.prefix);
                let chan = message.suffix.unwrap();
                let mut reply = String::new();
                match &nick as &str {
                    "De4dBot" => {
                        reply = format!("Bonjour {} ! comment ils vont aujourd'hui ? ", chan);
                    },
                    "your_nickname" => {
                        reply = format!("Your welcome message");
                    },
                    _ => {
                        reply = format!(" Boujour, {} !", nick);
                    }
                }
                server.send_privmsg(&chan, &reply).unwrap();;
            } ,
            "PART" =>  {
                server.send_privmsg(&message.args[0], "Reviens nous vite !").unwrap();
            }
            _ => { continue; }
        }
    }
}
