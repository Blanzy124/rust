use std::fmt::format;
use crate::inventory::yarn::dom_yarn_inv;
use dom_yarn_inv::Yarn;

use crate::inventory::yarn::inter_yarn_inv;

use reqwest::Error;
use serde::{Serialize, Deserialize};

pub async   fn add_yarn(j: String) -> String {


    let mut req: Result<Yarn, serde_json::Error>  = serde_json::from_str(&j);

    match req {
        Ok(yarn_) => {
            let mut yarn__: Result<Yarn, String>  = Yarn::new(yarn_.lot_number, yarn_.size, yarn_.color, yarn_.content, yarn_.location, yarn_.note);

            match yarn__ {
                Ok(yarn) => {
                    let int: Result<String, String> = inter_yarn_inv::yarn_add_MariaDB(yarn).await;
                    match int {
                        Ok(resp) => {
                            let respons = dom_yarn_inv::YarnRequestSucces{message: "perfect".to_string(), ok: true, };

                            let mut respj: String = serde_json::to_string(&respons).unwrap();

                            return respj;
                        }
                        Err(e) => {

                        }
                    }
                }
                Err(e) => {
                    let er: dom_yarn_inv::YarnRequestFail = dom_yarn_inv::YarnRequestFail{message: e, ok: false, error_codde: "1002".to_string()};
                    let erj: String = serde_json::to_string(&er)?;
                    erj
                }
            }

        }
        Err(e) => {

        }
    }


}