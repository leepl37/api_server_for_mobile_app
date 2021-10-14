#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest, delete};
#[macro_use]
extern crate diesel;
pub mod schema;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use diesel::{r2d2::ConnectionManager};
use std::fs::File;
type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;
use dotenv::dotenv;
use log::{error, info, warn, debug, trace};
use log4rs;
use actix_web::web::{Path, Query};
use models::{ParkingWorkerSetup, NewParkingWorkerSetup, ParkingEntrycarList, NewParkingEntrycarList, NewParkingSpaceInformation};
use crate::models::{ParkingSpaceInformation, NewParkingEntrycarOutList, ParkingEntrycarOutList, NewParkingHolidayList, ParkingHolidayList, NewParkingPaymentsData, ParkingPaymentsData, NewParkingEntvhlList, ParkingEntvhlList, NewParkingPaymentsList, ParkingPaymentsList, NewParkingPrePaymentsData, ParkingPrePaymentsData, NewParkingPrePaymentsList, ParkingPrePaymentsList, NewParkingVehicleHistory, ParkingVehicleHistory, NewParkingRefundList, ParkingRefundList, NewParkingRegisteredVehicleHistory, ParkingRegisteredVehicleHistory, NewParkingRegisteredVehicleFeeSetup, ParkingRegisteredVehicleFeeSetup, NewParkingRegisteredVehicleSetup, ParkingRegisteredVehicleSetup, NewParkingFeeSystemSetup, ParkingFeeSystemSetup, NewParkingFeeSystemCouponInfo, ParkingFeeSystemCouponInfo, ParkingFeeSystemDiscountInfo, ParkingPaymentsKocesData, ParkingUnpaymentsData, ParkingUnpaymentsInfo, ParkingUnpaymentsList, NewParkingFeeSystemDiscountInfo, NewParkingPaymentsKocesData, NewParkingUnpaymentsData, NewParkingUnpaymentsInfo, NewParkingUnpaymentsList, ParkingVehicleImage, NewParkingVehicleImage};
use crate::schema::parking_fee_system_setup::dsl::parking_fee_system_setup;
use crate::schema::parking_entrycar_list::dsl::parking_entrycar_list;
use crate::schema::parking_entrycar_out_list::dsl::parking_entrycar_out_list;
use crate::schema::parking_holiday_list::dsl::parking_holiday_list;
use crate::schema::parking_payments_data::dsl::parking_payments_data;
use crate::schema::parking_entvhl_list::dsl::parking_entvhl_list;
use crate::schema::parking_payments_list::dsl::parking_payments_list;
use crate::schema::parking_prepayments_data::dsl::parking_prepayments_data;
use crate::schema::parking_prepayments_list::dsl::parking_prepayments_list;
use crate::schema::parking_vehicle_history::dsl::parking_vehicle_history;
use crate::schema::parking_refund_list::dsl::parking_refund_list;
use crate::schema::parking_registeredvehicle_history::dsl::parking_registeredvehicle_history;
use crate::schema::parking_registeredvehicle_fee_setup::dsl::parking_registeredvehicle_fee_setup;
use crate::schema::parking_registeredvehicle_setup::dsl::parking_registeredvehicle_setup;
use crate::json_struct::{Json_Object_Sending_Post, Json_Object_Err_Sending, Json_Object_Sending_After_Login, ApiCallOutCar, ApiCallInCar, ApiLogin, ImgInfo};
use crate::json_function::{json_post_result, json_get_result, get_current_time, image_upload_file};
use crate::schema::parking_fee_system_coupon_info::dsl::parking_fee_system_coupon_info;
use crate::schema::parking_fee_system_discount_info::dsl::parking_fee_system_discount_info;
use crate::schema::parking_payments_koces_data::dsl::parking_payments_koces_data;
use crate::schema::parking_unpayments_data::dsl::parking_unpayments_data;
use crate::schema::parking_unpayments_list::dsl::parking_unpayments_list;
use crate::schema::parking_unpaymnets_info::dsl::parking_unpaymnets_info;
use crate::schema::parking_worker_setup::columns::wkid;
use crate::schema::parking_space_information::columns::pkno as space_pkno;
use crate::schema::parking_worker_setup::dsl::{parking_worker_setup, pkno, wkname};
use crate::schema::parking_space_information::dsl::parking_space_information;
use crate::schema::parking_fee_system_setup::columns::pkno as setup_pkno;
use crate::schema::parking_vehicle_history::columns::extdaytime;
use std::io::{Write};
use crate::schema::parking_vehicle_image::dsl::parking_vehicle_image;

use actix_multipart::Multipart;
use futures_util::stream::StreamExt as _;
use futures_util::TryStreamExt;
use reqwest::blocking::Response;
use std::error::Error;



pub mod models;
mod test;
mod json_struct;
mod json_function;




// fn establish_connection() -> MysqlConnection {
//     dotenv().ok();
//     let database_url = std::env::var("DATABASE_URL")
//         .expect("DATABASE_URL must be set");
//     MysqlConnection::establish(&database_url)
//         .expect(&format!("Error connecting to {}", database_url))
// }

#[get("/apiCallOutCar")]
async fn api_call_out_car(info : Query<ApiCallOutCar>, pool : web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let car_info = parking_vehicle_history.filter(schema::parking_vehicle_history::columns::carnumber.eq(&info.carnumber))
        .filter(schema::parking_vehicle_history::columns::serialnumber.eq(&info.serialnumber))
        .first::<ParkingVehicleHistory>(&connection);
    let vhidx = car_info.unwrap().vhidx;
    let updated_vehicle_history = diesel::update(parking_vehicle_history).filter(schema::parking_vehicle_history::columns::vhidx.eq(vhidx))
        .set(extdaytime.eq(&info.extdyte)).execute(&connection);

    let result = diesel::delete(parking_entvhl_list
                                    .filter(schema::parking_entvhl_list::columns::vhidx.eq(vhidx)))
                                    .execute(&connection);
    let sending = json_post_result(result);
    HttpResponse::Ok().json(sending)
}


#[get("/apiCallIncar")]
async fn api_call_in_car(info : Query<ApiCallInCar>, pool: web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let history_info = NewParkingVehicleHistory {
        serialnumber: info.serialnumber.clone(),
        carnumber: info.carnumber.clone(),
        areano: Option::from(info.areano),
        entmcno: 0,
        entdaytime: info.entdyte.clone(),
        extmcno: 0,
        extdaytime: "".to_string(),
        registdivision: info.registdivision,
        smallcardivision: 0,
        rearcarnumber: "".to_string(),
        insdaytime: get_current_time(),
        pkno: info.pkno,
        lvhidx: 0,
        tsrdve: "".to_string()
    };
    let result = diesel::insert_into(parking_vehicle_history).values(history_info).execute(&connection);
    return match result {
        Ok(_) => {
            let get_inserted = parking_vehicle_history.filter(schema::parking_vehicle_history::columns::carnumber.eq(info.serialnumber.clone())).first::<ParkingVehicleHistory>(&connection);
            match get_inserted {
                Ok(get) => {
                    let entvhllist_info = NewParkingEntvhlList {
                        serialnumber: info.serialnumber.clone(),
                        vhlnbr: info.carnumber.clone(),
                        areano: Option::from(info.areano),
                        entmcno: 0,
                        entdyte: info.entdyte.clone(),
                        smvhldvn: 0,
                        rvhlnbr: "".to_string(),
                        vhidx: get.vhidx,
                        imgpth: info.imgph.clone()
                    };
                    let result1 = diesel::insert_into(parking_entvhl_list).values(entvhllist_info).execute(&connection);
                    let sending = json_post_result(result1);
                    HttpResponse::Ok().json(sending)
                }
                Err(_) => {
                    println!("mysql auto increment Error");
                    HttpResponse::Ok().body("Error Occurred at Mysql auto Increment plz check vhidx at history")
                }
            }
        }
        Err(_) => {
            let sending = Json_Object_Sending_Post {
                result_code: 1,
                result_message: "fail, 데이터 값이 올바르지 않습니다.".to_string(),
                count: 0
            };
            HttpResponse::Ok().json(sending)
        }
    }
}

#[get("/apiLogin")]
async fn api_login(info : Query<ApiLogin>, pool: web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let admin = parking_worker_setup.filter(wkid.eq(info.id.clone())).first::<ParkingWorkerSetup>(&connection);
    return match admin {
        Ok(u) => {
            match u.wkpw == info.pw {
                true => {
                    //pk 없는 객체 먼저 선별하기.
                    let coupon_info: Vec<ParkingFeeSystemCouponInfo> = parking_fee_system_coupon_info.get_results(&connection).unwrap();
                    let discount_info: Vec<ParkingFeeSystemDiscountInfo> = parking_fee_system_discount_info.get_results(&connection).unwrap();
                    let space_information = parking_space_information.filter(space_pkno.eq(u.pkno)).first::<ParkingSpaceInformation>(&connection);
                    let setup = parking_fee_system_setup.filter(setup_pkno.eq(u.pkno)).first::<ParkingFeeSystemSetup>(&connection);
                    let sending = Json_Object_Sending_After_Login {
                        result_code: 0,
                        result_message: "성공".to_string(),
                        coupon_info,
                        discount_info,
                        space_information: vec![space_information.unwrap()],
                        setup: vec![setup.unwrap()]
                    };
                    HttpResponse::Ok().json(sending)
                }
                false => {
                    println!("비밀번호가 일치하지 않습니다.");
                    let sending = Json_Object_Err_Sending { result_code: 0, result_message: "계정을 확인하세요.".to_string() };
                    HttpResponse::Ok().json(sending)
                }
            }
        },
        Err(e) => {
            println!("{:?}", e);
            let sending = Json_Object_Err_Sending { result_code: 0, result_message: "계정을 확인하세요.".to_string() };
            HttpResponse::Ok().json(sending)
        }
    };
}
#[post("/PostParkingUnpaymentsList")]
async fn post_parking_unpayments_list(info:Query<NewParkingUnpaymentsList>, pool: web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let result = diesel::insert_into(parking_unpayments_list).values(info.into_inner())
        .execute(&connection);
    let sending = json_post_result(result);
    HttpResponse::Ok().json(sending)
}



#[get("/GetParkingUnpaymentsList")]
async fn get_parking_unpayments_list(pool: web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let vec:Vec<ParkingUnpaymentsList> = parking_unpayments_list.get_results(&connection).unwrap();
    let sending = json_get_result(vec);
    HttpResponse::Ok().json(sending)
}


#[post("/PostParkingUnpaymentsInfo")]
async fn post_parking_unpayments_info(info : Query<NewParkingUnpaymentsInfo>, pool:web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let result = diesel::insert_into(parking_unpaymnets_info).values(info.into_inner())
        .execute(&connection);
    let sending = json_post_result(result);
    HttpResponse::Ok().json(sending)
}

#[get("/GetParkingUnpaymentsInfo")]
async fn get_parking_unpayments_info(pool:web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let vec:Vec<ParkingUnpaymentsInfo> = parking_unpaymnets_info.get_results(&connection).unwrap();
    let sending = json_get_result(vec);
    HttpResponse::Ok().json(sending)
}


#[post("/PostParkingUnpaymentsData")]
async fn post_parking_unpayments_data(info : Query<NewParkingUnpaymentsData>, pool:web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let result = diesel::insert_into(parking_unpayments_data).values(info.into_inner())
        .execute(&connection);
    let sending = json_post_result(result);
    HttpResponse::Ok().json(sending)
}


#[get("/GetParkingUnpaymentsData")]
async fn get_parking_unpayments_data(pool:web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let vec:Vec<ParkingUnpaymentsData> = parking_unpayments_data.get_results(&connection).unwrap();
    let sending = json_get_result(vec);
    HttpResponse::Ok().json(sending)
}


#[post("/PostParkingPaymentsKocesData")]
async fn post_parking_payments_koces_data(info : Query<NewParkingPaymentsKocesData>, pool:web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let result = diesel::insert_into(parking_payments_koces_data).values(info.into_inner())
        .execute(&connection);
    let sending = json_post_result(result);
    HttpResponse::Ok().json(sending)
}

#[get("/GetParkingPaymentsKocesData")]
async fn get_parking_payments_koces_data(pool:web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let vec:Vec<ParkingPaymentsKocesData> = parking_payments_koces_data.get_results(&connection).unwrap();
    let sending = json_get_result(vec);
    HttpResponse::Ok().json(sending)
}

#[post("/PostParkingFeeSystemDiscountInfo")]
async fn post_parking_fee_system_discount_info(info : Query<NewParkingFeeSystemDiscountInfo>, pool:web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let result = diesel::insert_into(parking_fee_system_discount_info).values(info.into_inner())
        .execute(&connection);
    let sending = json_post_result(result);
    HttpResponse::Ok().json(sending)
}

#[get("/GetParkingFeeSystemDiscountInfo")]
async fn get_parking_fee_system_discount_info(pool:web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let vec:Vec<ParkingFeeSystemDiscountInfo> = parking_fee_system_discount_info.get_results(&connection).unwrap();
    let sending = json_get_result(vec);
    HttpResponse::Ok().json(sending)
}

#[post("/PostParkingFeeSystemCouponInfo")]
async fn post_parking_fee_system_coupon_info(info : Query<NewParkingFeeSystemCouponInfo>, pool:web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let result = diesel::insert_into(parking_fee_system_coupon_info).values(info.into_inner())
        .execute(&connection);
    let sending = json_post_result(result);

    HttpResponse::Ok().json(sending)
}

#[get("/GetParkingFeeSystemCouponInfo")]
async fn get_parking_fee_system_coupon_info(pool:web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let vec:Vec<ParkingFeeSystemCouponInfo> = parking_fee_system_coupon_info.get_results(&connection).unwrap();
    let sending = json_get_result(vec);
    HttpResponse::Ok().json(sending)
}

#[post("/PostParkingFeeSystemSetup")]
async fn post_parking_fee_system_setup(info : Query<NewParkingFeeSystemSetup>, pool:web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let result = diesel::insert_into(parking_fee_system_setup).values(info.into_inner())
        .execute(&connection);
    let sending = json_post_result(result);

    HttpResponse::Ok().json(sending)
}

#[get("GetParkingFeeSystemSetup")]
async fn get_parking_fee_system_setup(pool:web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let vec:Vec<ParkingFeeSystemSetup> = parking_fee_system_setup.get_results(&connection).unwrap();
    let sending = json_get_result(vec);
    HttpResponse::Ok().json(sending)
}

#[post("/PostParkingRegisteredVehicleSetup")]
async fn post_parking_registered_vehicle_setup(info:Query<NewParkingRegisteredVehicleSetup>, pool:web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let result = diesel::insert_into(parking_registeredvehicle_setup).values(info.into_inner())
        .execute(&connection);
    let sending = json_post_result(result);

    HttpResponse::Ok().json(sending)
}

#[get("/GetParkingRegisteredVehicleSetup")]
async fn get_parking_registered_vehicle_setup(pool:web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let vec:Vec<ParkingRegisteredVehicleSetup> = parking_registeredvehicle_setup.get_results(&connection).unwrap();
    let sending = json_get_result(vec);
    HttpResponse::Ok().json(sending)
}

#[post("/PostParkingRegisteredVehicleFeeSetup")]
async fn post_parking_registered_vehicle_fee_setup(info : Query<NewParkingRegisteredVehicleFeeSetup>, pool:web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let result = diesel::insert_into(parking_registeredvehicle_fee_setup).values(info.into_inner())
        .execute(&connection);
    let sending = json_post_result(result);
    HttpResponse::Ok().json(sending)
}

#[get("/GetParkingRegisteredVehicleFeeSetup")]
async fn get_parking_registered_vehicle_fee_setup(pool:web::Data<Pool>) -> impl Responder{
    let connection = pool.get().unwrap();
    let vec:Vec<ParkingRegisteredVehicleFeeSetup> = parking_registeredvehicle_fee_setup.get_results(&connection).unwrap();
    let sending = json_get_result(vec);
    HttpResponse::Ok().json(sending)
}

#[post("/PostParkingRegisteredVehicleHistory")]
async fn post_parking_registered_vehicle_history(info : Query<NewParkingRegisteredVehicleHistory>, pool:web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let result = diesel::insert_into(parking_registeredvehicle_history).values(info.into_inner())
        .execute(&connection);

    let sending = json_post_result(result);

    HttpResponse::Ok().json(sending)
}

#[get("/GetParkingRegisteredVehicleHistory")]
async fn get_parking_registered_vehicle_history(pool:web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let vec:Vec<ParkingRegisteredVehicleHistory> = parking_registeredvehicle_history.get_results(&connection).unwrap();

    let sending = json_get_result(vec);
    HttpResponse::Ok().json(sending)
}

#[post("/PostParkingRefundList")]
async fn post_parking_refund_list(info :web::Query<NewParkingRefundList>, pool:web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let result = diesel::insert_into(parking_refund_list).values(info.into_inner())
        .execute(&connection);
    let sending = json_post_result(result);
    HttpResponse::Ok().json(sending)
}

#[get("/GetParkingRefundList")]
async fn get_parking_refund_list(pool:web::Data<Pool> ) -> impl Responder {
    let connection = pool.get().unwrap();
    let vec:Vec<ParkingRefundList> = parking_refund_list.get_results(&connection).unwrap();
    let sending = json_get_result(vec);
    HttpResponse::Ok().json(sending)
}


#[post("/PostParkingVehicleHistory")]
async fn post_parking_vehicle_history(info : Query<NewParkingVehicleHistory>, pool:web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let result = diesel::insert_into(parking_vehicle_history).values(info.into_inner())
        .execute(&connection);
    let sending = json_post_result(result);
    HttpResponse::Ok().json(sending)
}

#[get("/GetParkingVehicleHistory")]
async fn get_parking_vehicle_history(pool:web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let vec:Vec<ParkingVehicleHistory> = parking_vehicle_history.get_results(&connection).unwrap();
    let sending = json_get_result(vec);
    HttpResponse::Ok().json(sending)
}

#[post("/PostParkingPrePaymentsList")]
async fn post_parking_prepayments_list(info : Query<NewParkingPrePaymentsList>, pool:web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let result = diesel::insert_into(parking_prepayments_list).values(info.into_inner())
        .execute(&connection);
    let sending = json_post_result(result);
    HttpResponse::Ok().json(sending)
}

#[get("/GetParkingPrePaymentsList")]
async fn get_parking_prepayments_list(pool:web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let vec:Vec<ParkingPrePaymentsList> = parking_prepayments_list.get_results(&connection).unwrap();
    let sending = json_get_result(vec);
    HttpResponse::Ok().json(sending)
}

#[post("/PostParkingPrePaymentsData")]
async fn post_parking_prepayments_data(info : Query<NewParkingPrePaymentsData>, pool:web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let result = diesel::insert_into(parking_prepayments_data).values(info.into_inner())
        .execute(&connection);
    let sending = json_post_result(result);
    HttpResponse::Ok().json(sending)
}
#[get("/GetParkingPrePaymentsData")]
async fn get_parking_prepayments_data(pool:web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let vec:Vec<ParkingPrePaymentsData> = parking_prepayments_data.get_results(&connection).unwrap();
    let sending = json_get_result(vec);
    HttpResponse::Ok().json(sending)
}

#[post("/PostParkingPaymentsList")]
async fn post_parking_payments_list(info : Query<NewParkingPaymentsList>, pool:web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let result = diesel::insert_into(parking_payments_list).values(info.into_inner())
        .execute(&connection);
    let sending = json_post_result(result);
    HttpResponse::Ok().json(sending)
}
#[get("/GetParkingPaymentsList")]
async fn get_parking_payments_list(pool:web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let vec:Vec<ParkingPaymentsList> = parking_payments_list.get_results(&connection).unwrap();
    let sending = json_get_result(vec);
    HttpResponse::Ok().json(sending)
}


#[post("/PostParkingEntvhlList")]
async fn post_parking_entvhl_list(info : Query<NewParkingEntvhlList>, pool:web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let result = diesel::insert_into(parking_entvhl_list).values(info.into_inner())
        .execute(&connection);
    let sending = json_post_result(result);
    HttpResponse::Ok().json(sending)
}
#[get("/GetParkingEntvhlList")]
async fn get_parking_entvhl_list(pool:web::Data<Pool>) -> impl Responder{
    let connection = pool.get().unwrap();
    let vec:Vec<ParkingEntvhlList> = parking_entvhl_list.get_results(&connection).unwrap();
    let sending = json_get_result(vec);
    HttpResponse::Ok().json(sending)
}


#[post("/PostParkingPaymentsData")]
async fn post_parking_payments_data(info : Query<NewParkingPaymentsData>, pool:web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let result = diesel::insert_into(parking_payments_data).values(info.into_inner())
        .execute(&connection);
    let sending = json_post_result(result);
    HttpResponse::Ok().json(sending)
}
#[get("/GetParkingPaymentsData")]
async fn get_parking_payments_data(pool:web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let vec:Vec<ParkingPaymentsData> = parking_payments_data.get_results(&connection).unwrap();
    let sending = json_get_result(vec);
    HttpResponse::Ok().json(sending)
}

#[post("/PostParkingHolydayList")]
async fn post_parking_holiday_list(info : Query<NewParkingHolidayList>, pool:web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let result = diesel::insert_into(parking_holiday_list).values(info.into_inner())
        .execute(&connection);
    let sending = json_post_result(result);
    HttpResponse::Ok().json(sending)
}
#[get("/GetParkingHolydayList")]
async fn get_parking_holiday_list(pool:web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let vec:Vec<ParkingHolidayList> = parking_holiday_list.get_results(&connection).unwrap();
    let sending = json_get_result(vec);
    HttpResponse::Ok().json(sending)
}

#[post("/PostParkingEntryCarOutList")]
async fn post_parking_entrycar_out_list(info : Query<NewParkingEntrycarOutList>, pool:web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let result = diesel::insert_into(parking_entrycar_out_list).values(info.into_inner())
        .execute(&connection);
    let sending = json_post_result(result);
    HttpResponse::Ok().json(sending)
}

#[get("/GetParkingEntryCarOutList")]
async fn get_parking_entrycar_out_list(pool:web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let vec:Vec<ParkingEntrycarOutList> = parking_entrycar_out_list.get_results(&connection).unwrap();
    let sending = json_get_result(vec);
    HttpResponse::Ok().json(sending)
}


#[post("/PostParkingSpaceInformation")]
async fn post_parking_space_information(info : Query<NewParkingSpaceInformation>, pool:web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let result = diesel::insert_into(parking_space_information).values(info.into_inner())
        .execute(&connection);
    let sending = json_post_result(result);
    HttpResponse::Ok().json(sending)
}

#[get("/GetParkingSpaceInformation")]
async fn get_parking_space_information(pool:web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let vec:Vec<ParkingSpaceInformation> = parking_space_information.get_results(&connection).unwrap();
    let sending = json_get_result(vec);
    HttpResponse::Ok().json(sending)

}

#[post("/PostParkingEntrycarList")]
async fn post_parking_entry_car_list(info : Query<NewParkingEntrycarList>, pool:web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let result = diesel::insert_into(parking_entrycar_list).values(info.into_inner())
        .execute(&connection);
    let sending = json_post_result(result);
    HttpResponse::Ok().json(sending)
}

#[get("/GetParkingEntrycarList")]
async fn get_parking_entrycar_list(pool:web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let vec:Vec<ParkingEntrycarList> = parking_entrycar_list.get_results(&connection).unwrap();
    let sending = json_get_result(vec);
    HttpResponse::Ok().json(sending)
}

#[delete("/DeleteParkingWorkerSetup")]
async fn delete_parking_worker_setup(pool:web::Data<Pool>) -> impl Responder {
    // let result = diesel::delete(parking_worker_setup.filter(pkno.eq(1))).execute(&connection);

    HttpResponse::Ok().json("Deleted")
}

#[get("/UpdateParkingWorkerSetup")]
async fn update_parking_worker_setup(pool:web::Data<Pool>) ->impl Responder {
    let connection = pool.get().unwrap();
    let result = diesel::update(parking_worker_setup.filter(pkno.eq(10)))
        .set(wkname.eq("Jena"))
        .execute(&connection);
    println!("{:?}", result.unwrap());
    // 0 이면 없는 거  1이면 있는 거
    HttpResponse::Ok().json("Updated")
}
//
// #[test]
// fn test_copy_file(){
//     use std::fs::File;
//     let mut file = File::open("gwanho_haskell.png").unwrap();
//     let mut buffer = Vec::new();
//     let result = file.read_to_end(&mut buffer).unwrap();
//     let mut result1 = File::create("./tmp/test.png").unwrap();
//     result1.write_all(&buffer);
//     assert_eq!(1,1)
//     // file
//     // println!("{:?}", buffer);
// }





#[post("/PostParkingVehicleImage")]
async fn post_parking_vehicle_image(info:Query<ImgInfo>,mut payload: Multipart, pool:web::Data<Pool>) -> impl Responder {
    let mut img_serialnumber = &info.serialnumber;
    let mut img_pk:i32 = info.pkno;
    let mut img_path = String::from("");
    while let Ok(Some(mut field)) = payload.try_next().await {
        match field.content_disposition().unwrap().get_filename() {
            None => {
                //dll file이름 하면 정보 나옴(linux)
                //러스트는 한번에 하면 freed while still in use Error 발생.
                // println!("name: {:?}", field.content_disposition().unwrap().get_name().unwrap());
                // let contentdispostion = field.content_disposition().unwrap();
                // let name = contentdispostion.get_name().unwrap();
                // let data = std::str::from_utf8(&&*field.next().await.unwrap().unwrap()).unwrap();
                // let result_of_bytes = field.next().await.unwrap();
                // let bytes = result_of_bytes.unwrap();
                // let str_from_byte = std::str::from_utf8(bytes.as_ref()).unwrap();
                // if name == "serialnumber"{
                //     img_serialnumber = str_from_byte.parse().unwrap();
                // }
                // if name == "pk" {
                //     img_pk = str_from_byte.parse().unwrap();
                // }
                // println!("이름: {}", name);
                // println!("데이터: {:?}", str_from_byte);
            }
            Some(f) => {
                let file_name = f;
                // println!("file_name: {:?}", file_name);
                let path = format!("./images/{}", file_name);
                let result3 = File::create(&path);

                let mut file = match result3{
                    Ok(f) => f,
                    Err(e) => {
                        let sending = Json_Object_Sending_Post {
                            result_code: 1,
                            result_message: "fail, 파일이 존재하지 않습니다..".to_string(),
                            count: 0
                        };
                        return HttpResponse::Ok().json(sending)
                    }
                };
                // 에러 처리 객체 만들어서 구현하기.

                let buffer = field.next().await.unwrap().unwrap();
                // println!("buffer: {:?}", buffer);
                let result = file.write_all(&buffer);
        //     f = web::block(move || f.write_all(&data).map(|_| f)).await.unwrap(); 차이점은?
                match result {
                    Ok(_) => { println!("파일이 생성되었습니다.");
                    let result2 = image_upload_file(&path);
                    match result2 {
                        Ok(_) => { println!("업로드 성공 ");}
                        Err(_) => { println!("업로드 실패");}
                    }
                    img_path = path.clone()
                    }
                    Err(err) => { println!("에러 발생: {}", err);}
                }
            }
        }
    }
    let newImageInfo = NewParkingVehicleImage::new(img_serialnumber.clone(), img_pk, img_path.clone());
    let connection = pool.get().unwrap();
    let result1 = diesel::insert_into(parking_vehicle_image).values(newImageInfo).execute(&connection);
    let sending = json_post_result(result1);
    HttpResponse::Ok().json(sending)
}

#[get("/GetParkingVehicleImage")]
async fn get_parking_vehicle_image(pool:web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let vec:Vec<ParkingVehicleImage> = parking_vehicle_image.get_results(&connection).unwrap();

    let sending = json_get_result(vec);
    HttpResponse::Ok().json(sending)
}

#[post("/PostParkingWorkerSetup")]
async fn post_parking_worker_setup(info : Query<NewParkingWorkerSetup>, pool:web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let result = diesel::insert_into(parking_worker_setup).values(info.into_inner())
        .execute(&connection);
    let sending = json_post_result(result);
    HttpResponse::Ok().json(sending)
}

#[get("/GetParkingWorkerSetup")]
async fn get_parking_worker_setup(pool:web::Data<Pool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let vec:Vec<ParkingWorkerSetup> = parking_worker_setup.get_results(&connection).unwrap();
    let sending = json_get_result(vec);
    HttpResponse::Ok().json(sending)
}

#[get("/")]
async fn hello(_pool:web::Data<Pool>) -> impl Responder {
    // let connection = establish_connection();
    // let x = parking_worker_setup.filter(wkidx.eq(1)).first::<ParkingWorkerSetup>(&connection);
    // log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    // log::error!("{}", "Error_test");

    HttpResponse::Ok().json("Hello world!")
    // HttpResponse::Ok().body("Hello world body!")
}

#[post("/hello/{id}")]
async fn hellotest(Path(id):Path<i32>) -> impl Responder{
    HttpResponse::Ok().json(id)
}

//Match information
#[get("hello/{v1}/{v2}")]
async fn hellov(req: HttpRequest) -> impl Responder {
    let x:i32 = req.match_info().get("v1").unwrap().parse().unwrap();
    let y:i32 = req.match_info().get("v2").unwrap().parse().unwrap();
    let (v3, v4): (u8, u8) = req.match_info().load().unwrap();
    println!("{},{}", v3,v4);
    HttpResponse::Ok().json( x + y )
}
//Path information extractor
#[get("/path_extractor/{name}/{id}")]
async fn path_extractor(info: Path<(String, i32)>) -> impl Responder {
    let string = info.into_inner();
    HttpResponse::Ok().json(string)
}

/*
연락처 받는 페이지 ->
if 연락처가 있다 -> 승인결과 if( 승인이 됐으면  qr코드 else 현재 요청 중입니다.

 */




#[actix_web ::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::fs::create_dir_all("./images").unwrap();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create mysql pool.");

    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    HttpServer::new(move || {
        // let tera = Tera::new("templates/**/*").unwrap();
        App::new()
            // .wrap()
            // .data(tera)
            .data(pool.clone())
            .service(actix_files::Files::new("/assets", ".").show_files_listing())//assets/images/파일이름
            .service(post_parking_vehicle_image)
            .service(get_parking_vehicle_image)
            .service(path_extractor)
            .service(hello)
            .service(hellotest)
            .service(api_login)
            .service(api_call_in_car)
            .service(api_call_out_car)
            .service(get_parking_unpayments_list)
            .service(post_parking_unpayments_list)
            .service(get_parking_unpayments_info)
            .service(post_parking_unpayments_info)
            .service(get_parking_unpayments_data)
            .service(post_parking_unpayments_data)
            .service(get_parking_payments_koces_data)
            .service(post_parking_payments_koces_data)
            .service(get_parking_fee_system_discount_info)
            .service(post_parking_fee_system_discount_info)
            .service(get_parking_fee_system_coupon_info)
            .service(post_parking_fee_system_coupon_info)
            .service(get_parking_fee_system_setup)
            .service(post_parking_fee_system_setup)
            .service(post_parking_registered_vehicle_setup)
            .service(get_parking_registered_vehicle_setup)
            .service(post_parking_registered_vehicle_fee_setup)
            .service(get_parking_registered_vehicle_fee_setup)
            .service(get_parking_registered_vehicle_history)
            .service(post_parking_registered_vehicle_history)
            .service(post_parking_refund_list)
            .service(get_parking_refund_list)
            .service(get_parking_vehicle_history)
            .service(post_parking_vehicle_history)
            .service(post_parking_prepayments_list)
            .service(get_parking_prepayments_list)
            .service(get_parking_prepayments_data)
            .service(post_parking_prepayments_data)
            .service(get_parking_payments_list)
            .service(post_parking_payments_list)
            .service(get_parking_entvhl_list)
            .service(post_parking_entvhl_list)
            .service(get_parking_payments_data)
            .service(post_parking_payments_data)
            .service(get_parking_holiday_list)
            .service(post_parking_holiday_list)
            .service(get_parking_entrycar_out_list)
            .service(post_parking_entrycar_out_list)
            .service(get_parking_space_information)
            .service(post_parking_space_information)
            .service(get_parking_entrycar_list)
            .service(post_parking_entry_car_list)
            .service(update_parking_worker_setup)
            .service(delete_parking_worker_setup)
            .service(get_parking_worker_setup)
            .service(post_parking_worker_setup)

        })
        .bind("0.0.0.0:9595")?
        // .bind("127.0.0.1:8000")?
        .run()
        .await
}




