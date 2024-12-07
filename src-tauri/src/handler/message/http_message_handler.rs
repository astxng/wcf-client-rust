use async_trait::async_trait;

use crate::{
    handler::event_entity::{Event, EventHandler},
    service::global_service::GLOBAL,
};

use regex::Regex;
use serde_json::json;

/// 配置 http 回调地址后，将调用设置的url，
pub struct HttpMessageHandler {
    pub id: String,
}

#[async_trait]
impl EventHandler for HttpMessageHandler {
    async fn handle(&mut self, event: Event) {
        if let Event::ClientMessage(ref msg) = event {
            let global = GLOBAL.get().unwrap();
            let (cburl, msg_filter_regexp, wx_id, token) = {
                let config = global.wechat_config.read().unwrap();
                (config.cburl.clone(), config.msg_filter_regexp.clone(), config.wx_id.clone(), config.token.clone())
            };
            if cburl.is_empty() {
                log::info!("未配置回调地址，跳过处理");
                return;
            }
            if wx_id.is_empty() {
                log::info!("微信尚未登录，跳过处理");
                return;
            }
            if token.is_empty() {
                log::info!("平台尚未登录，跳过处理");
                return;
            }
            // 仅对文本消息做过滤，其他消息也默认转发，如好友消息，红包消息，链接消息等
            if msg.r#type == 1 {
                if let Some(ref regex_str) = msg_filter_regexp {
                    if let Ok(regex) = Regex::new(regex_str) {
                        if !regex.is_match(&msg.content) {
                            log::debug!("消息被过滤，内容: {:?}", &msg.content);
                            return;
                        }    
                    }
                } else {
                    log::debug!("未配置正则过滤，所有消息转发")
                }
            }

            for url in cburl {
                log::debug!("http服务 {} 回调地址为: {:?}", self.id, url.clone());
                if !url.starts_with("http") {
                    log::error!("http 转发消息失败，回调地址不合法");
                    continue;
                }
                let res = ureq::post(format!("{}/{}", &url, &wx_id).as_str())
                    .set("Authorization", format!("Bearer {}", token).as_str())
                    .send_json(json!(&msg));
                match res {
                    Ok(rsp) => {
                        if rsp.status() != 200 {
                            log::error!("转发消息失败，状态码: {}", rsp.status());
                        }
                        log::debug!("{}", rsp.into_string().unwrap());
                    }
                    Err(e) => {
                        log::error!("转发消息失败：{}", e);
                    }
                }
            }
        }
    }
}
