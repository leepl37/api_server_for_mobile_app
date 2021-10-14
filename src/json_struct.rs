use::serde::{Deserialize, Serialize};
use crate::models::{ParkingFeeSystemSetup, ParkingFeeSystemDiscountInfo, ParkingSpaceInformation, ParkingFeeSystemCouponInfo};

#[derive(Deserialize, Serialize)]
pub struct Json_Object_Sending_After_Login {
    pub result_code:i32,
    pub result_message:String,
    pub coupon_info:Vec<ParkingFeeSystemCouponInfo>,
    pub discount_info:Vec<ParkingFeeSystemDiscountInfo>,
    pub space_information:Vec<ParkingSpaceInformation>,
    pub setup:Vec<ParkingFeeSystemSetup>
}

#[derive(Deserialize, Serialize)]
pub struct Json_Object_Sending<T> {
    pub result_code:i32,
    pub result_message:String,
    pub count:usize,
    pub result: Option<T>
}


#[derive(Deserialize, Serialize)]
pub struct Json_Object_Sending_Post {
    pub result_code:i32,
    pub result_message:String,
    pub count:usize,
}

#[derive(Deserialize, Serialize)]
pub struct Json_Object_Err_Sending{
    pub result_code:i32,
    pub result_message:String
}


#[derive(Deserialize, Serialize)]
pub struct ApiCallOutCar {
    pub carnumber:String,
    pub serialnumber:String,
    pub extdyte:String,
    pub areano:i32,
    pub registdivision:i32,
    pub pkno:i32,
    pub imgph:String
}


#[derive(Deserialize, Serialize)]
pub struct ApiCallInCar {
    pub serialnumber: String,
    pub carnumber:String,
    pub entdyte:String,
    pub areano:i32,
    pub registdivision:i32,
    pub pkno:i32,
    pub imgph:String
}


#[derive(Deserialize, Serialize, Debug)]
pub struct ImgInfo {
    pub serialnumber: String,
    pub pkno:i32,
}



#[derive(Deserialize, Serialize)]
pub struct ApiLogin {
    pub id:String,
    pub pw:String
}