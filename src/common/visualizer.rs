use unitn_market_2022::good::good_kind::GoodKind;
use serde::{Deserialize, Serialize};
use crate::common;
use reqwest_eventsource::{Event as ReqEvent, EventSource};
use futures::StreamExt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CustomEventKind {
    Bought,
    Sold,
    LockedBuy,
    LockedSell,
    Wait,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomEvent {
    pub kind: CustomEventKind,
    pub good_kind: GoodKind,
    pub quantity: f32,
    pub price: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogEvent {
    pub time: u32,
    pub event: CustomEvent,
    pub market: String,
    pub result: bool,
    pub error: Option<String>,
}

pub fn craft_log_event(time: u32, kind: CustomEventKind, good_kind: GoodKind, quantity: f32, price: f32, market: String, result: bool, error: Option<String>) -> LogEvent {
    let custom_ev = CustomEvent {
        kind,
        good_kind,
        quantity,
        price,
    };
    LogEvent {
        market,
        result,
        error,
        time,
        event: custom_ev,
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TraderGood {
    pub kind: GoodKind,
    pub quantity: f32,
}

pub fn wait_before_calling_api(milliseconds: u64) {
    std::thread::sleep(std::time::Duration::from_millis(milliseconds));
}

pub async fn get_trader_id() -> u8 {
    let client = reqwest::Client::new();
    let res = client.get("http://localhost:8000/delay").send().await;
    let mut id: u8 = 4;

    if let Err(res) = res {
        let trader_config = common::trader_config::get_trader_config();
        if trader_config.is_trader_SA() {
            id = 0;
        } else if trader_config.is_trader_AB() {
            id = 1;
        } else if trader_config.is_trader_TR() {
            id = 2;
        }
        return id;
    } else {
        let mut connection = EventSource::get("http://localhost:8000/traderToUse");
        loop {
            let next = connection.next().await;

            match next {
                Some(content) => match content {
                    Ok(ReqEvent::Message(message)) => {
                        id = message.data.parse::<u8>().unwrap();
                        break;
                    }
                    Err(err) => {
                        let trader_config = common::trader_config::get_trader_config();
                        if trader_config.is_trader_SA() {
                            id = 0;
                        } else if trader_config.is_trader_AB() {
                            id = 1;
                        } else if trader_config.is_trader_TR() {
                            id = 2;
                        }
                    }
                    _ => continue
                },
                None => {
                    let trader_config = common::trader_config::get_trader_config();
                    if trader_config.is_trader_SA() {
                        id = 0;
                    } else if trader_config.is_trader_AB() {
                        id = 1;
                    } else if trader_config.is_trader_TR() {
                        id = 2;
                    }
                }
            }
        }
        id
    }
}