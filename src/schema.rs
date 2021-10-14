table! {
    parking_entrycar_list (vhidx) {
        entcaridx -> Integer,
        vhidx -> Integer,
        forcequtdiv -> Integer,
        insdadete -> Nullable<Timestamp>,
    }
}

table! {
    parking_entrycar_out_list (entcaroutidx) {
        entcaroutidx -> Integer,
        forceoutdyte -> Varchar,
        forceoutworker -> Varchar,
        forceoutdvn -> Integer,
        vhidx -> Integer,
        vhlnbr -> Nullable<Varchar>,
    }
}

table! {
    parking_entvhl_list (entdyte) {
        seq_etyvhl -> Integer,
        serialnumber -> Varchar,
        vhlnbr -> Varchar,
        areano -> Nullable<Integer>,
        entmcno -> Integer,
        entdyte -> Varchar,
        smvhldvn -> Integer,
        rvhlnbr -> Nullable<Varchar>,
        vhidx -> Integer,
        imgpth -> Nullable<Varchar>,
        insdyte -> Nullable<Timestamp>,
    }
}

table! {
    parking_fee_system_setup (pkno) {
        pfssidx -> Integer,
        pkno -> Integer,
        pkregdete -> Varchar,
        pkruntime -> Varchar,
        pkdaydctime -> Varchar,
        pkholidaydctime -> Varchar,
        pksundaydctime -> Varchar,
        pksatdaydctime -> Varchar,
        calmethod -> Integer,
        scturntime -> Integer,
        scbasetime -> Integer,
        scbaseamount -> Integer,
        scadditionaltime -> Integer,
        scadditionalamount -> Integer,
        scsurchargetime -> Integer,
        ssurchargeamount -> Integer,
        scdaymaxamount -> Integer,
        scdaynightmaxamount -> Integer,
        scholidaymaxamount -> Integer,
        scsundaymaxamount -> Integer,
        scsatdaymaxamount -> Integer,
        bcturntime -> Integer,
        bcbasetime -> Integer,
        bcbaseamount -> Integer,
        bcadditionaltime -> Integer,
        bcadditionalamount -> Integer,
        bcsurchargetime -> Integer,
        bcsurchargeamount -> Integer,
        bcdaymaxamount -> Integer,
        bcdaynightmaxamount -> Integer,
        bcholidaymaxamount -> Integer,
        bcsundaymaxamount -> Integer,
        bcsatdaymaxamount -> Integer,
        extlccrnuedvn -> Varchar,
        zoexedvn -> Integer,
        zoexete -> Varchar,
        zoexeaodvn -> Integer,
        prepytactte -> Varchar,
        tnwnutactdvn -> Integer,
    }
}

table! {
    parking_holiday_list (hydate) {
        hlidx -> Integer,
        hydate -> Varchar,
        hyname -> Varchar,
        hyyear -> Varchar,
    }
}

table! {
    parking_payments_data (seq_evl) {
        seq_evl -> Integer,
        serialnumber -> Varchar,
        vhlnbr -> Varchar,
        entdyte -> Varchar,
        extdyte -> Varchar,
        pkgTe -> Integer,
        pkgfe -> Integer,
        dctfe -> Integer,
        pyot -> Integer,
        dctte -> Nullable<Varchar>,
        pytte -> Integer,
        imgph -> Nullable<Varchar>,
        pmcsidx -> Integer,
    }
}

table! {
    parking_payments_list (seq_evl) {
        pytidx -> Integer,
        serialnumber -> Varchar,
        carnumber -> Varchar,
        entdaytime -> Varchar,
        outdaytime -> Varchar,
        parkingtime -> Integer,
        parkingfee -> Integer,
        discountfee -> Integer,
        payout -> Integer,
        discounttype -> Varchar,
        paymenttype -> Integer,
        workerid -> Varchar,
        pmcsidx -> Integer,
        seq_evl -> Integer,
        deistdyte -> Nullable<Varchar>,
        pytrctnbr -> Nullable<Varchar>,
        tsrdve -> Nullable<Varchar>,
    }
}

table! {
    parking_prepayments_data (seq_evl) {
        seq_evl -> Integer,
        vhlnbr -> Varchar,
        entdyte -> Varchar,
        extdyte -> Varchar,
        pkgte -> Integer,
        pkgfe -> Integer,
        dctfe -> Integer,
        pyot -> Integer,
        dctte -> Nullable<Varchar>,
        pytte -> Integer,
    }
}

table! {
    parking_prepayments_list (seq_evl) {
        ppayidx -> Integer,
        carnumber -> Varchar,
        entdaytime -> Varchar,
        outdaytime -> Varchar,
        parkingtime -> Integer,
        parkingfee -> Integer,
        discountfee -> Integer,
        payout -> Integer,
        discounttype -> Varchar,
        paymenttype -> Integer,
        workerid -> Varchar,
        pmcssidx -> Integer,
        seq_evl -> Integer,
        pytrctnbr -> Nullable<Varchar>,
        ppytotdyte -> Varchar,
        reoutdyte -> Nullable<Varchar>,
        ppcplcd -> Integer,
        insdyte -> Nullable<Timestamp>,
    }
}

table! {
    parking_refund_list (pkno) {
        rlidx -> Integer,
        rfreason -> Varchar,
        pkno -> Integer,
        pytidx -> Integer,
    }
}

table! {
    parking_registeredvehicle_fee_setup (rvfidx) {
        rvfidx -> Integer,
        rvfdate -> Varchar,
        rvfstdate -> Varchar,
        rvfedDate -> Varchar,
        rvfstate -> Nullable<Integer>,
        rvftype -> Nullable<Integer>,
        rvfamount -> Nullable<Integer>,
        rvfregtype -> Nullable<Integer>,
        rvidx -> Integer,
        PkNo -> Integer,
    }
}

table! {
    parking_registeredvehicle_history (rvidx) {
        rvhtyidx -> Integer,
        rvidx -> Integer,
        rvnumber -> Varchar,
        username -> Nullable<Varchar>,
        tel -> Varchar,
        rvgroup -> Nullable<Varchar>,
        member -> Nullable<Varchar>,
        duty -> Nullable<Varchar>,
        rvname -> Nullable<Varchar>,
        rvdivision -> Integer,
        dcdivision -> Integer,
        registerdate -> Varchar,
        registerstate -> Integer,
        finalregtype -> Integer,
        startdate -> Varchar,
        enddate -> Varchar,
        memo -> Nullable<Varchar>,
        pkno -> Integer,
        usrdajbdete -> Timestamp,
    }
}

table! {
    parking_registeredvehicle_setup (rvnumber) {
        rvidx -> Integer,
        rvnumber -> Varchar,
        username -> Nullable<Varchar>,
        tel -> Varchar,
        rvgroup -> Nullable<Varchar>,
        member -> Nullable<Varchar>,
        duty -> Nullable<Varchar>,
        rvname -> Nullable<Varchar>,
        rvdivision -> Integer,
        dcdivision -> Integer,
        registerdate -> Varchar,
        registerstate -> Integer,
        finalregtype -> Integer,
        startdate -> Varchar,
        enddate -> Varchar,
        memo -> Nullable<Varchar>,
        pkno -> Integer,
        usrdajbdete -> Timestamp,
    }
}

table! {
    parking_space_information (pkno) {
        psiidx -> Integer,
        pkno -> Integer,
        maxspace -> Integer,
        usedspace -> Integer,
        emptyspace -> Integer,
        overspace -> Integer,
        psiregdate -> Nullable<Timestamp>,
    }
}

table! {
    parking_vehicle_history (pkno) {
        vhidx -> Integer,
        serialnumber -> Varchar,
        carnumber -> Varchar,
        areano -> Nullable<Integer>,
        entmcno -> Integer,
        entdaytime -> Varchar,
        extmcno -> Nullable<Integer>,
        extdaytime -> Varchar,
        registdivision -> Integer,
        smallcardivision -> Integer,
        rearcarnumber -> Nullable<Varchar>,
        insdaytime -> Varchar,
        pkno -> Integer,
        lvhidx -> Integer,
        tsrdve -> Nullable<Varchar>,
    }
}

table! {
    parking_worker_setup (wkid) {
        wkidx -> Integer,
        wkid -> Varchar,
        wkpw -> Varchar,
        wkname -> Varchar,
        wktel -> Nullable<Varchar>,
        wkaddr -> Nullable<Varchar>,
        wklevel -> Varchar,
        wkstatus -> Integer,
        wkgroup -> Nullable<Varchar>,
        wkmemo -> Nullable<Varchar>,
        pkno -> Integer,
        wklys -> Nullable<Varchar>,
    }
}
//
table! {
    parking_fee_system_coupon_info (cpidx) {
        	cpidx -> Integer,
        	cpname -> Varchar,
        	cpamount -> Varchar,
        	cptime -> Varchar,
        	cpdetail -> Varchar,
    }
}

table! {
    parking_fee_system_discount_info (pfsdcidx) {
        	pfsdcidx -> Integer,
        	dcname -> Varchar,
        	dcamount -> Varchar,
        	dctime -> Varchar,
        	dcrate -> Varchar,
        	dctype -> Varchar,
    }
}
table! {
 parking_payments_koces_data (pfpykdidx) {
            pfpykdidx -> Integer,
        	serialnumber	-> Varchar,
        	carnumber	-> Varchar,
        	payout	-> Varchar,
        	splpc	-> Varchar,
        	vat	-> Varchar,
        	confmno	-> Varchar,
        	confmde	-> Varchar,
        	confmtime	-> Varchar,
        	cardno	-> Varchar,
        	issuecmpnynm	-> Varchar,
        	buyingcmpnynm	-> Varchar,
    }
}
table! {
 parking_unpayments_data (seq_evl){
        	seq_evl -> Integer,
        	serialnumber -> Varchar,
        	vhlnbr -> Varchar,
        	entdyte -> Varchar,
        	extdyte -> Varchar,
        	pkgte -> Integer,
        	pkgfe -> Integer,
        	dctfe -> Integer,
        	pyot -> Integer,
        	dctte -> Varchar,
        	pytte -> Integer,
        	imgph -> Varchar,
        	pmcsidx -> Integer,
        }
    }

table! {
 parking_unpaymnets_info (upyidx) {
        	upyidx -> Integer,
        	unpayname -> Varchar,
        }
    }


table! {
 parking_unpayments_list (pytidx){
        	pytidx -> Integer,
        	serialnumber -> Varchar,
        	carnumber -> Varchar,
        	entdaytime -> Varchar,
        	outdaytime -> Varchar,
        	parkingtime -> Integer,
        	parkingfee -> Integer,
        	discountfee -> Integer,
        	payout -> Integer,
        	discounttype -> Varchar,
        	paymenttype -> Integer,
        	workerid -> Varchar,
        	seq_evl -> Integer,
        	pytrctnbr -> Varchar,
        	tsrdve -> Varchar,
        }
    }

table! {
    parking_vehicle_image (viidx) {
        viidx -> Integer,
        serialnumber -> Varchar,
        pkno -> Integer,
        imgpth -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    parking_vehicle_image,
    parking_entrycar_list,
    parking_entrycar_out_list,
    parking_entvhl_list,
    parking_fee_system_setup,
    parking_holiday_list,
    parking_payments_data,
    parking_payments_list,
    parking_prepayments_data,
    parking_prepayments_list,
    parking_refund_list,
    parking_registeredvehicle_fee_setup,
    parking_registeredvehicle_history,
    parking_registeredvehicle_setup,
    parking_space_information,
    parking_vehicle_history,
    parking_worker_setup,
    parking_fee_system_coupon_info,
    parking_fee_system_discount_info,
    parking_payments_koces_data,
    parking_unpayments_data,
    parking_unpaymnets_info,
    parking_unpayments_list,

);
