use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use super::schema::{parking_worker_setup, parking_space_information, parking_entrycar_list, parking_entrycar_out_list, parking_holiday_list, parking_payments_data, parking_entvhl_list, parking_payments_list, parking_prepayments_data, parking_prepayments_list, parking_vehicle_history, parking_refund_list, parking_registeredvehicle_history, parking_registeredvehicle_fee_setup, parking_registeredvehicle_setup, parking_fee_system_setup, parking_fee_system_coupon_info, parking_fee_system_discount_info, parking_payments_koces_data, parking_unpayments_data, parking_unpaymnets_info, parking_unpayments_list, parking_vehicle_image};
use chrono::NaiveDateTime;
use crate::schema::parking_entvhl_list::columns::serialnumber;

#[derive(Debug, Queryable, Serialize)]
pub struct ParkingWorkerSetup {
    pub wkidx : i32,
    pub wkid : String,
    pub wkpw: String,
    pub wkname: String,
    pub wktel: Option<String>,
    pub wkaddr: Option<String>,
    pub wklevel: String,
    pub wkstatus: i32,
    pub wkgroup: Option<String>,
    pub wkmemo: Option<String>,
    pub pkno: i32,
    pub wklys: Option<String>
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="parking_worker_setup"]
pub struct NewParkingWorkerSetup {
    pub wkid : String,
    pub wkpw: String,
    pub wkname: String,
    pub wktel: String,
    pub wkaddr: String,
    pub wklevel: String,
    pub wkstatus: i32,
    pub wkgroup:String,
    pub wkmemo: String,
    pub pkno: i32,
    pub wklys: String
}
    impl NewParkingWorkerSetup {
        pub fn new(wkid:String, wkpw:String, wkname:String, wktel:String, wkaddr:String, wklevel:String
        ,wkstatus:i32, wkgroup:String, wkmemo:String, pkno:i32, wklys:String) -> Self {
            NewParkingWorkerSetup {
                wkid,
                wkpw,
                wkname,
                wktel,
                wkaddr,
                wklevel,
                wkstatus,
                wkgroup,
                wkmemo,
                pkno,
                wklys
            }
        }
    }


#[derive(Debug,Queryable, Serialize)]
pub struct ParkingEntrycarList {
    pub entcaridx : i32,
    pub vhidx : i32,
    pub forcequtdiv: i32,
    pub insdadete: Option<NaiveDateTime>
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="parking_entrycar_list"]
pub struct NewParkingEntrycarList {
    pub vhidx : i32,
    pub forcequtdiv: i32
    // pub insdadete: NaiveDateTime
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct ParkingSpaceInformation {
    pub psiidx: i32,
    pub pkno: i32,
    pub maxspace: i32,
    pub usedspace: i32,
    pub emptyspace: i32,
    pub overspace: i32,
    pub psiregdate: Option<NaiveDateTime>
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="parking_space_information"]
pub struct NewParkingSpaceInformation {
    pub pkno: i32,
    pub maxspace: i32,
    pub usedspace: i32,
    pub emptyspace: i32,
    pub overspace: i32
}

#[derive(Debug, Serialize, Queryable)]
pub struct ParkingEntrycarOutList {
    pub entcaroutidx: i32,
    pub forceoutdyte: String,
    pub forceoutworker: String,
    pub forceoutdvn: i32,
    pub vhidx: i32,
    pub vhlnbr: Option<String>
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="parking_entrycar_out_list"]
pub struct NewParkingEntrycarOutList {
    pub forceoutdyte: String,
    pub forceoutworker: String,
    pub forceoutdvn: i32,
    pub vhidx: i32,
    pub vhlnbr: String
}

#[derive(Debug, Serialize, Queryable)]
pub struct ParkingHolidayList {
    pub hLidx: i32,
    pub hydate: String,
    pub hyname: String,
    pub hyyear: String
}

#[derive(Debug, Insertable, Deserialize)]
#[table_name="parking_holiday_list"]
pub struct NewParkingHolidayList {
    pub(crate) hydate: String,
    pub(crate) hyname: String,
    pub(crate) hyyear: String
}

#[derive(Debug, Serialize, Queryable)]
pub struct ParkingPaymentsData {
    pub seq_evl: i32,
    pub serialnumber: String,
    pub vhlnbr: String,
    pub entdyte: String,
    pub extdyte: String,
    pub pkgTe: i32,
    pub pkgfe:i32,
    pub dctfe:i32,
    pub pyot:i32,
    pub dctte:Option<String>,
    pub pytte:i32,
    pub imgph:Option<String>,
    pub pmcsidx:i32
}
#[derive(Debug, Deserialize, Insertable)]
#[table_name="parking_payments_data"]
pub struct NewParkingPaymentsData {
    pub seq_evl: i32,
    pub serialnumber: String,
    pub vhlnbr: String,
    pub entdyte: String,
    pub extdyte: String,
    pub pkgTe: i32,
    pub pkgfe:i32,
    pub dctfe:i32,
    pub pyot:i32,
    pub dctte:String,
    pub pytte:i32,
    pub imgph:String,
    pub pmcsidx:i32
}

#[derive(Debug, Queryable, Serialize)]
pub struct ParkingEntvhlList {
    pub seq_etyvhl: i32,
    pub serialnumber: String,
    pub vhlnbr: String,
    pub areano: Option<i32>,
    pub entmcno: i32,
    pub entdyte: String,
    pub smvhldvn:i32,
    pub rvhlnbr:Option<String>,
    pub vhidx:i32,
    pub imgpth:Option<String>,
    pub insdyte:Option<NaiveDateTime>
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="parking_entvhl_list"]
pub struct NewParkingEntvhlList {
    pub serialnumber: String,
    pub vhlnbr: String,
    pub areano: Option<i32>,
    pub entmcno: i32,
    pub entdyte: String,
    pub smvhldvn:i32,
    pub rvhlnbr:String,
    pub vhidx:i32,
    pub imgpth:String
}

#[derive(Debug, Queryable, Serialize)]
pub struct ParkingPaymentsList {
    pub pytidx: i32,
    pub serialnumber: String,
    pub carnumber: String,
    pub entdaytime: String,
    pub outdaytime: String,
    pub parkingtime: i32,
    pub parkingfee: i32,
    pub discountfee: i32,
    pub payout: i32,
    pub discounttype:String,
    pub paymenttype:i32,
    pub workerid:String,
    pub pmcsidx:i32,
    pub seq_evl:i32,
    pub deistdyte:Option<String>,
    pub pytrctnbr:Option<String>,
    pub tsrdve:Option<String>
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="parking_payments_list"]
pub struct NewParkingPaymentsList {
    pub serialnumber: String,
    pub carnumber: String,
    pub entdaytime: String,
    pub outdaytime: String,
    pub parkingtime: i32,
    pub parkingfee: i32,
    pub discountfee: i32,
    pub payout: i32,
    pub discounttype:String,
    pub paymenttype:i32,
    pub workerid:String,
    pub pmcsidx:i32,
    pub seq_evl:i32,
    pub deistdyte:String,
    pub pytrctnbr:String,
    pub tsrdve:String
}

#[derive(Queryable, Serialize)]
pub struct ParkingPrePaymentsData {
    pub seq_evl: i32,
    pub vhlnbr: String,
    pub entdyte: String,
    pub extdyte: String,
    pub pkgte:i32,
    pub pkgfe:i32,
    pub dctfe:i32,
    pub pyot:i32,
    pub dctte:Option<String>,
    pub pytte: i32
}

#[derive(Deserialize, Insertable, Debug)]
#[table_name="parking_prepayments_data"]
pub struct NewParkingPrePaymentsData {
    pub seq_evl: i32,
    pub vhlnbr: String,
    pub entdyte: String,
    pub extdyte: String,
    pub pkgte:i32,
    pub pkgfe:i32,
    pub dctfe:i32,
    pub pyot:i32,
    pub dctte:String,
    pub pytte: i32
}

#[derive(Queryable, Debug, Serialize)]
pub struct ParkingPrePaymentsList {
    pub ppayidx: i32,
    pub carnumber: String,
    pub entdaytime: String,
    pub outdaytime: String,
    pub parkingtime:i32,
    pub parkingfee:i32,
    pub discountfee:i32,
    pub payout:i32,
    pub discounttype:String,
    pub paymenttype:i32,
    pub workerid:String,
    pub pmcssidx:i32,
    pub seq_evl:i32,
    pub pytrctnbr:Option<String>,
    pub ppytotdyte:String,
    pub reoutdyte:Option<String>,
    pub ppcplcd:i32,
    pub insdyte:Option<NaiveDateTime>
}

#[derive(Deserialize, Debug, Insertable)]
#[table_name="parking_prepayments_list"]
pub struct NewParkingPrePaymentsList {
    pub carnumber: String,
    pub entdaytime: String,
    pub outdaytime: String,
    pub parkingtime:i32,
    pub parkingfee:i32,
    pub discountfee:i32,
    pub payout:i32,
    pub discounttype:String,
    pub paymenttype:i32,
    pub workerid:String,
    pub pmcssidx:i32,
    pub seq_evl:i32,
    pub pytrctnbr:String,
    pub ppytotdyte:String,
    pub reoutdyte:String,
    pub ppcplcd:i32
}

#[derive(Debug, Serialize, Queryable)]
pub struct ParkingVehicleHistory {
    pub vhidx:i32,
    pub serialnumber:String,
    pub carnumber: String,
    pub areano: Option<i32>,
    pub entmcno:i32,
    pub entdaytime:String,
    pub extmcno:Option<i32>,
    pub extdaytime:String,
    pub registdivision:i32,
    pub smallcardivision:i32,
    pub rearcarnumber:Option<String>,
    pub insdaytime:String,
    pub pkno:i32,
    pub lvhidx:i32,
    pub tsrdve:Option<String>,
}

#[derive(Deserialize, Insertable, Debug)]
#[table_name="parking_vehicle_history"]
pub struct NewParkingVehicleHistory { // entdyte
    pub serialnumber:String, //
    pub carnumber: String, //
    pub areano: Option<i32>, //
    pub entmcno:i32,
    pub entdaytime:String,
    pub extmcno:i32,
    pub extdaytime:String,
    pub registdivision:i32, //
    pub smallcardivision:i32,
    pub rearcarnumber:String,
    pub insdaytime:String,
    pub pkno:i32,//
    pub lvhidx:i32,
    pub tsrdve:String,
}

#[derive(Serialize, Queryable, Debug)]
pub struct ParkingRefundList {
    pub rlidx:i32,
    pub rfreason:String,
    pub pkno:i32,
    pub pytidx:i32
}

#[derive(Deserialize, Debug, Insertable)]
#[table_name="parking_refund_list"]
pub struct NewParkingRefundList {
    pub rfreason:String,
    pub pkno:i32,
    pub pytidx:i32
}

#[derive(Serialize, Queryable, Debug)]
pub struct ParkingRegisteredVehicleHistory {
    pub rvhtyidx:i32,
    pub rvidx:i32,
    pub rvnumber:String,
    pub username:Option<String>,
    pub tel:String,
    pub rvgroup:Option<String>,
    pub member:Option<String>,
    pub duty:Option<String>,
    pub rvname:Option<String>,
    pub rvdivision:i32,
    pub dcdivision:i32,
    pub registerdate:String,
    pub registerstate:i32,
    pub finalregtype:i32,
    pub startdate:String,
    pub enddate:String,
    pub memo:Option<String>,
    pub pkno:i32,
    pub usrdajbdete:NaiveDateTime
}

#[derive(Deserialize, Insertable, Debug)]
#[table_name="parking_registeredvehicle_history"]
pub struct NewParkingRegisteredVehicleHistory {
    pub rvidx:i32,
    pub rvnumber:String,
    pub username:String,
    pub tel:String,
    pub rvgroup:String,
    pub member:String,
    pub duty:String,
    pub rvname:String,
    pub rvdivision:i32,
    pub dcdivision:i32,
    pub registerdate:String,
    pub registerstate:i32,
    pub finalregtype:i32,
    pub startdate:String,
    pub enddate:String,
    pub memo:String,
    pub pkno:i32
}

#[derive(Serialize, Queryable, Debug)]
pub struct ParkingRegisteredVehicleFeeSetup {
    pub rvfidx: i32,
    pub rvfdate: String,
    pub rvfstdate: String,
    pub rvfedDate: String,
    pub rvfstate:Option<i32>,
    pub rvftype:Option<i32>,
    pub rvfamount:Option<i32>,
    pub rvfregtype:Option<i32>,
    pub rvidx:i32,
    pub PkNo:i32
}

#[derive(Deserialize, Insertable, Debug)]
#[table_name="parking_registeredvehicle_fee_setup"]
pub struct NewParkingRegisteredVehicleFeeSetup {
    pub rvfdate: String,
    pub rvfstdate: String,
    pub rvfedDate: String,
    pub rvfstate:i32,
    pub rvftype:i32,
    pub rvfamount:i32,
    pub rvfregtype:i32,
    pub rvidx:i32,
    pub PkNo:i32
}


#[derive(Serialize, Debug, Queryable)]
pub struct ParkingRegisteredVehicleSetup {
    pub rvidx:i32,
    pub rvnumber: String,
    pub username: Option<String>,
    pub tel:String,
    pub rvgroup:Option<String>,
    pub member:Option<String>,
    pub duty:Option<String>,
    pub rvname:Option<String>,
    pub rvdivision:i32,
    pub dcdivision:i32,
    pub registerdate:String,
    pub registerstate:i32,
    pub finalregtype:i32,
    pub startdate:String,
    pub enddate:String,
    pub memo:Option<String>,
    pub pkno:i32,
    pub usrdajbdete:NaiveDateTime
}

#[derive(Deserialize, Insertable, Debug)]
#[table_name="parking_registeredvehicle_setup"]
pub struct NewParkingRegisteredVehicleSetup {
    pub rvnumber: String,
    pub username: String,
    pub tel:String,
    pub rvgroup:String,
    pub member:String,
    pub duty:String,
    pub rvname:String,
    pub rvdivision:i32,
    pub dcdivision:i32,
    pub registerdate:String,
    pub registerstate:i32,
    pub finalregtype:i32,
    pub startdate:String,
    pub enddate:String,
    pub memo:String,
    pub pkno:i32
}

#[derive(Serialize, Queryable, Debug, Deserialize)]
pub struct ParkingFeeSystemSetup {
    pub pfssidx:i32,
    pub pkno:i32,
    pub pkregdete:String,
    pub pkruntime:String,
    pub pkdaydctime:String,
    pub pkholidaydctime:String,
    pub pksundaydctime:String,
    pub pksatdaydctime:String,
    pub calmethod:i32,
    pub scturntime:i32,
    pub scbasetime:i32,
    pub scbaseamount:i32,
    pub scadditionaltime:i32,
    pub scadditionalamount:i32,
    pub scsurchargetime:i32,
    pub ssurchargeamount:i32,
    pub scdaymaxamount:i32,
    pub scdaynightmaxamount:i32,
    pub scholidaymaxamount:i32,
    pub scsundaymaxamount:i32,
    pub scsatdaymaxamount:i32,
    pub bcturntime:i32,
    pub bcbasetime:i32,
    pub bcbaseamount:i32,
    pub bcadditionaltime:i32,
    pub bcadditionalamount:i32,
    pub bcsurchargetime:i32,
    pub bcsurchargeamount:i32,
    pub bcdaymaxamount:i32,
    pub bcdaynightmaxamount:i32,
    pub bcholidaymaxamount:i32,
    pub bcsundaymaxamount:i32,
    pub bcsatdaymaxamount:i32,
    pub extlccrnuedvn:String,
    pub zoexedvn:i32,
    pub zoexete:String,
    pub zoexeaodvn:i32,
    pub prepytactte:String,
    pub tnwnutactdvn:i32
}

#[derive(Deserialize, Insertable, Debug)]
#[table_name="parking_fee_system_setup"]
pub struct NewParkingFeeSystemSetup {
    pub pkno:i32,
    pub pkregdete:String,
    pub pkruntime:String,
    pub pkdaydctime:String,
    pub pkholidaydctime:String,
    pub pksundaydctime:String,
    pub pksatdaydctime:String,
    pub calmethod:i32,
    pub scturntime:i32,
    pub scbasetime:i32,
    pub scbaseamount:i32,
    pub scadditionaltime:i32,
    pub scadditionalamount:i32,
    pub scsurchargetime:i32,
    pub ssurchargeamount:i32,
    pub scdaymaxamount:i32,
    pub scdaynightmaxamount:i32,
    pub scholidaymaxamount:i32,
    pub scsundaymaxamount:i32,
    pub scsatdaymaxamount:i32,
    pub bcturntime:i32,
    pub bcbasetime:i32,
    pub bcbaseamount:i32,
    pub bcadditionaltime:i32,
    pub bcadditionalamount:i32,
    pub bcsurchargetime:i32,
    pub bcsurchargeamount:i32,
    pub bcdaymaxamount:i32,
    pub bcdaynightmaxamount:i32,
    pub bcholidaymaxamount:i32,
    pub bcsundaymaxamount:i32,
    pub bcsatdaymaxamount:i32,
    pub extlccrnuedvn:String,
    pub zoexedvn:i32,
    pub zoexete:String,
    pub zoexeaodvn:i32,
    pub prepytactte:String,
    pub tnwnutactdvn:i32
}


#[derive(Serialize, Debug, Queryable, Deserialize)]
pub struct ParkingFeeSystemCouponInfo{
    pub cpidx:i32,
    pub cpname:String,
    pub cpamount:String,
    pub cptime:String,
    pub cpdetail:String
}

#[derive(Deserialize, Insertable, Debug)]
#[table_name="parking_fee_system_coupon_info"]
pub struct NewParkingFeeSystemCouponInfo {
    pub cpname:String,
    pub cpamount:String,
    pub cptime:String,
    pub cpdetail:String
}

#[derive(Serialize, Debug, Queryable, Deserialize)]
pub struct  ParkingFeeSystemDiscountInfo {
    pub pfsdcidx: i32,
    pub dcname: String,
    pub dcamount: String,
    pub dctime: String,
    pub dcrate: String,
    pub dctype: String
}

#[derive(Deserialize, Insertable, Debug)]
#[table_name="parking_fee_system_discount_info"]
pub struct  NewParkingFeeSystemDiscountInfo {
    pub dcname: String,
    pub dcamount: String,
    pub dctime: String,
    pub dcrate: String,
    pub dctype: String
}


#[derive(Serialize, Debug, Queryable)]
pub struct  ParkingPaymentsKocesData {
    pub pfpykdidx:i32,
    pub serialnumber: String,
    pub carnumber: String,
    pub payout: String,
    pub splpc: String,
    pub vat: String,
    pub confmno: String,
    pub confmde: String,
    pub confmtime: String,
    pub cardno: String,
    pub issuecmpnynm: String,
    pub buyingcmpnynm: String
}

#[derive(Deserialize, Insertable, Debug)]
#[table_name="parking_payments_koces_data"]
pub struct  NewParkingPaymentsKocesData {
    pub serialnumber: String,
    pub carnumber: String,
    pub payout: String,
    pub splpc: String,
    pub vat: String,
    pub confmno: String,
    pub confmde: String,
    pub confmtime: String,
    pub cardno: String,
    pub issuecmpnynm: String,
    pub buyingcmpnynm: String
}

#[derive(Serialize, Debug, Queryable)]
pub struct  ParkingUnpaymentsData {
    pub seq_evl: i32,
    pub serialnumber: String,
    pub vhlnbr: String,
    pub entdyte: String,
    pub extdyte: String,
    pub pkgte: i32,
    pub pkgfe: i32,
    pub dctfe: i32,
    pub pyot: i32,
    pub dctte: String,
    pub pytte: i32,
    pub imgph: String,
    pub pmcsidx: i32
}
#[derive(Deserialize, Insertable, Debug)]
#[table_name="parking_unpayments_data"]
pub struct  NewParkingUnpaymentsData {
    pub serialnumber: String,
    pub vhlnbr: String,
    pub entdyte: String,
    pub extdyte: String,
    pub pkgte: i32,
    pub pkgfe: i32,
    pub dctfe: i32,
    pub pyot: i32,
    pub dctte: String,
    pub pytte: i32,
    pub imgph: String,
    pub pmcsidx: i32
}


#[derive(Serialize, Debug, Queryable)]
pub struct  ParkingUnpaymentsInfo {
    pub upyidx: i32,
    pub unpayname: String
}

#[derive(Deserialize, Insertable, Debug)]
#[table_name="parking_unpaymnets_info"]
pub struct  NewParkingUnpaymentsInfo {
    pub unpayname: String
}


#[derive(Serialize, Debug, Queryable)]
pub struct  ParkingUnpaymentsList {
    pub pytidx: i32,
    pub serialnumber: String,
    pub carnumber: String,
    pub entdaytime: String,
    pub outdaytime: String,
    pub parkingtime: i32,
    pub parkingfee: i32,
    pub discountfee: i32,
    pub payout: i32,
    pub discounttype: String,
    pub paymenttype: i32,
    pub workerid: String,
    pub seq_evl: i32,
    pub pytrctnbr: String,
    pub tsrdve: String
}

#[derive(Deserialize, Insertable, Debug)]
#[table_name="parking_unpayments_list"]
pub struct  NewParkingUnpaymentsList {
    pub serialnumber: String,
    pub carnumber: String,
    pub entdaytime: String,
    pub outdaytime: String,
    pub parkingtime: i32,
    pub parkingfee: i32,
    pub discountfee: i32,
    pub payout: i32,
    pub discounttype: String,
    pub paymenttype: i32,
    pub workerid: String,
    pub seq_evl: i32,
    pub pytrctnbr: String,
    pub tsrdve: String
}

#[derive(Serialize, Debug, Queryable)]
pub struct ParkingVehicleImage{
    pub viidx:i32,
    pub serialnumber:String,
    pub pkno:i32,
    pub imgpth:String
}

#[derive(Deserialize, Debug, Insertable)]
#[table_name="parking_vehicle_image"]
pub struct NewParkingVehicleImage{
    pub serialnumber:String,
    pub pkno:i32,
    pub imgpth:String
}

impl NewParkingVehicleImage {
    pub fn new(image_serialnumber:String, pkno:i32, imgpth:String) -> NewParkingVehicleImage {
        NewParkingVehicleImage{
            serialnumber: image_serialnumber,
            pkno,
            imgpth
        }
    }
}


