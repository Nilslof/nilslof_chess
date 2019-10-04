use std::thread;
use crate::{DataHandler};
use std::sync::{Mutex, Arc};
use regex::Regex;
use std::io::Read;

pub(crate) struct NetworkHandler {
    client: reqwest::Client,
    target_address: Arc<Mutex<Option<String>>>, //opponent ipv4 adress
    local_color: Arc<Mutex<Option<nilslof_chess_engine::Colour>>>, //my playing colour

    //number of times white has won, number of times black has won
    draw_requested: Arc<Mutex<(bool, bool)>>,
    //you, the guy she tells you not to worry about/your opponent
    rematch_requested: Arc<Mutex<(bool, bool)>>, //you, the guy she tells you not to worry about/your opponent
}

impl NetworkHandler {
    pub(crate) fn get_target_address(&self) -> Result<String, ()> {
        if let Some(addr) = &*self.target_address.lock().unwrap() {
            return Ok(addr.clone());
        }
        Err(())
    }

    pub(crate) fn set_target_address(&mut self, address: String) {
        *self.target_address.lock().unwrap() = Some(address);
    }

    pub(crate) fn get_local_player_color(&self) -> Option<nilslof_chess_engine::Colour> {
        self.local_color.lock().unwrap().clone()
    }

    pub(crate) fn set_local_color(&mut self, color: nilslof_chess_engine::Colour) {
        *self.local_color.lock().unwrap() = Some(color);
    }

    pub(crate) fn new() -> Self {
        let mut out = NetworkHandler {
            client: reqwest::ClientBuilder::new().timeout(Some(core::time::Duration::from_secs(5))).build().unwrap(),
            target_address: Arc::new(Mutex::new(None)),
            local_color: Arc::new(Mutex::new(None)),
            draw_requested: Arc::new(Mutex::new((false, false))),
            rematch_requested: Arc::new(Mutex::new((false, false))),
        };

        out.listen();

        out
    }

    fn listen(&mut self) {
        let local_color_ref = self.local_color.clone();
        let request_draw_ref = self.draw_requested.clone();
        let request_rematch_ref = self.rematch_requested.clone();
        let address = self.target_address.clone();

        thread::spawn( move || {
            rouille::Server::new("0.0.0.0:7878", move |request| {

                let mut response = rouille::Response::text("");

                let regex_for_start_square = Regex::new("\"start_square\"(\\s)*:(\\s)*\"[a-h][1-8]\"").unwrap();
                let regex_for_end_square = Regex::new("\"end_square\"(\\s)*:(\\s)*\"[a-h][1-8]\"").unwrap();
                let regex_for_promotion = Regex::new("\"promotes_to\"(\\s)*:(\\s)*\"[QRBN]\"").unwrap();

                let regex_for_square_extraction = Regex::new("[a-h][1-8]").unwrap();
                let regex_for_promotion_extraction = Regex::new("[QRBN]").unwrap();

                let mut request_text = "".to_string();

                request.data().unwrap().read_to_string(&mut request_text);

                println!("{}", request_text);


                match request.method() {
                    "GET" => {}
                    "POST" => {
                        let url = request.url();
                        let remote_addr = format!("{}:7878", request.remote_addr().ip());
                        let mut response_code = 0;

                        println!("{}", url);

                        if url == "/start-game" {
                            if local_color_ref.lock().unwrap().is_none() {
                                if request_text.contains("white") {
                                    *address.lock().unwrap() = Some(remote_addr.to_string());
                                    *local_color_ref.lock().unwrap() = Some(nilslof_chess_engine::Colour::Black);
                                    response_code = 200;
                                } else if request_text.contains("black") {
                                    *address.lock().unwrap() = Some(remote_addr.to_string());
                                    *local_color_ref.lock().unwrap() = Some(nilslof_chess_engine::Colour::White);
                                    response_code = 200;
                                } else {
                                    response_code = 400;
                                }
                            }
                        } else if url == "/move" {
                            if regex_for_start_square.is_match(request_text.as_ref())
                                && regex_for_end_square.is_match(request_text.as_ref()) {
                                let start_square = regex_for_square_extraction.captures(
                                    regex_for_start_square.captures(request_text.as_ref()).unwrap()
                                        .get(0).unwrap().as_str()).unwrap().get(0).unwrap().as_str();
                                let end_square = regex_for_square_extraction.captures(
                                    regex_for_end_square.captures(request_text.as_ref()).unwrap()
                                        .get(0).unwrap().as_str()).unwrap().get(0).unwrap().as_str();

                                response_code = 200;

                            }
                        } else if url == "/request-draw" {
                            request_draw_ref.lock().unwrap().1 = true;
                            //response_body = format!("{0}\"draw_accepted\":{2}{1}", "{", "}", request_draw_ref.lock().unwrap().0);
                            response_code = 501;
                        } else if url == "/request-rematch" {
                            request_rematch_ref.lock().unwrap().1 = true;
                            //response_body = format!("{0}\"draw_accepted\":{2}{1}", "{", "}", request_rematch_ref.lock().unwrap().0);
                            response_code = 501;
                        }

                        response = response.with_status_code(response_code);
                    }
                    _ => {
                        response = response.with_status_code(405);
                    }
                }

                response
            }).unwrap().pool_size(1).run();
        });
    }

    pub(crate) fn send(&self, url: &str, message: String) -> Result<u16, ()> {
        let full_url = format!("{}", self.target_address.lock().unwrap().as_ref().unwrap());

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::HOST, reqwest::header::HeaderValue::from_bytes(full_url.as_ref()).unwrap());
        headers.insert(reqwest::header::CONTENT_TYPE, reqwest::header::HeaderValue::from_bytes(b"text/json").unwrap());
        let mut var = self.client.post(format!("http://{}{}", full_url, url).as_str()).headers(headers).body(message).send();

        match var {
            Ok(mut response) => {
                return Ok(response.status().as_u16());
            }
            Err(e) => {
                return Err(());
            }
        }
    }
}