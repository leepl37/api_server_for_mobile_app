use diesel::QueryResult;
use crate::json_struct::{Json_Object_Sending_Post, Json_Object_Sending};
use chrono::{Local};
use reqwest::blocking::{multipart, Response};
use dotenv::dotenv;
use actix_web::web::Json;
use reqwest::Error;
use std::fs;
use std::convert::TryFrom;

pub fn json_post_result(result: QueryResult<usize>) -> Json_Object_Sending_Post {
    return match result {
        Ok(result) => {
            let sending = Json_Object_Sending_Post {
                result_code: result as i32,
                result_message: "success".to_string(),
                count: 0
            };
            sending
        }
        Err(err) => {
            let sending = Json_Object_Sending_Post {
                result_code: 0,
                result_message: err.to_string(),
                count: 0
            };
            sending
        }
    };
}


pub fn json_get_result<T>(result : Vec<T>) -> Json_Object_Sending<Vec<T>> {
    let sending = Json_Object_Sending {
        result_code: 1,
        result_message: "success".to_string(),
        count: result.len(),
        result: Some(result)
    };
    sending
}

pub fn get_current_time() -> String {

    let now = Local::now();
    let date = now.date();
    let date_str = date.to_string();
    let string = date_str.replace('-', "");
    let mut date = String::from(&string[0..8]);
    let time = now.time();
    let time_str = time.to_string();
    let string2 = time_str.replace(':', "");
    let time = &string2[0..6];
    date.push_str(time);
    date
}


pub fn image_upload_file(path:&str) -> Result<Response, Box<dyn std::error::Error>> {
    let form= multipart::Form::new().file("image", path).unwrap();
    // let result1 = multipart::Part::file(file_path).unwrap().file_name(file_name).mime_str(&what_type_is).unwrap();
    // let form = form.part("assets", result1);
    let resp = reqwest::blocking::Client::new().post(get_server_url("/PostUploadImage"))
        .multipart(form)
        .send();
    match resp{
        Ok(res) => {
            let result = fs::remove_file(path);
            println!("파일이 삭제 되었습니다.");
            return Ok(res)
        }
        _ => {}
    };
    Ok(resp.unwrap())
}

pub fn get_server_url(str: &str) -> String  {
    dotenv().ok();
    let mut string = std::env::var("SERVER_URL").expect("SERVER_URL must be set");
    string.push_str(str);
    string
}