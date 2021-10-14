-- Your SQL goes here

CREATE TABLE parking_entrycar_list
(
    `entcaridx`   int(11)  NOT NULL AUTO_INCREMENT,
    `vhidx`       int(11) NOT NULL primary key,
    `forcequtdiv` int(3)  NOT NULL DEFAULT '0',
    `insdadete`   timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
--     primary key (vhidx),
    UNIQUE (entcaridx)
--     unique (vhidx)
);

CREATE TABLE parking_worker_setup
(
    `wkidx`    int(11) NOT NULL AUTO_INCREMENT,
    `wkid`     varchar(20)  NOT NULL primary key,
    `wkpw`     varchar(100) NOT NULL,
    `wkname`   varchar(20)  NOT NULL,
    `wktel`    varchar(13) DEFAULT NUll,
    `wkaddr`   varchar(50) DEFAULT NULL,
    `wklevel`  varchar(1)   NOT NULL,
    `wkstatus` int(3) NOT NULL DEFAULT '0',
    `wkgroup`  varchar(20) DEFAULT NULL,
    `wkmemo`   varchar(70) DEFAULT NULL,
    `pkno`     int(3) NOT NULL,
    `wklys`    varchar(80) DEFAULT '',
--     primary key(wkid),
    UNIQUE (wkid),
    UNIQUE (wkidx),
    UNIQUE (pkno)
);


CREATE TABLE parking_space_information (
        `psiidx` int (10) NOT NULL AUTO_INCREMENT,
        `pkno` int (3) NOT NULL primary key,
        `maxspace` int (5) NOT NULL DEFAULT '0',
        `usedspace` int (5) NOT NULL DEFAULT '0',
        `emptyspace` int (5) NOT NULL DEFAULT '0',
        `overspace` int (6) NOT NULL DEFAULT '0',
        `psiregdate` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

--         primary key(pkno),
        UNIQUE (psiidx),
        UNIQUE (pkno)

        );

CREATE TABLE parking_entrycar_out_list(
          `entcaroutidx`   int(11)  NOT NULL AUTO_INCREMENT primary key,
          `forceoutdyte`   varchar(14) NOT NULL,
          `forceoutworker` varchar(40) NOT NULL,
          `forceoutdvn`    int(3) NOT NULL,
          `vhidx`          int(11) NOT NULL,
          `vhlnbr`         varchar(20) DEFAULT NULL,

--           primary key(entcaroutidx),
--           UNIQUE (entcaroutidx),
          UNIQUE (vhidx)
        );


CREATE TABLE parking_holiday_list
(
    `hlidx`  int(5)  NOT NULL AUTO_INCREMENT,
    `hydate` varchar(8)  NOT NULL primary key,
    `hyname` varchar(20) NOT NULL,
    `hyyear` varchar(4)  NOT NULL,

--     primary key(hydate),
    UNIQUE(hydate),
    UNIQUE(hlidx)
);

CREATE TABLE parking_payments_data
(
    `seq_evl` int(10) NOT NULL primary key,
    `vhlnbr`  varchar(20) NOT NULL,
    `entdyte` varchar(14) NOT NULL,
    `extdyte` varchar(14) NOT NULL,
    `pkgTe`   int(10) NOT NULL DEFAULT '0',
    `pkgfe`   int(10) NOT NULL DEFAULT '0',
    `dctfe`   int(10) NOT NULL DEFAULT '0',
    `pyot`    int(10) NOT NULL DEFAULT '0',
    `dctte`   varchar(20) DEFAULT NULL,
    `pytte`   int(3) NOT NULL DEFAULT '0',
    `imgph`   varchar(30) DEFAULT NULL,
    `pmcsidx` int(5) NOT NULL DEFAULT '0',

--     primary key(seq_evl),
    UNIQUE (vhlnbr)

);


CREATE TABLE parking_entvhl_list
(
    `seq_etyvhl` int(11) NOT NULL AUTO_INCREMENT,
    `vhlnbr`     varchar(20) NOT NULL,
    `entmcno`    int(3)  NOT NULL,
    `entdyte`    varchar(14) NOT NULL primary key,
    `smvhldvn`   int(1) NOT NULL DEFAULT '0',
    `rvhlnbr`    varchar(20) DEFAULT NULL,
    `vhidx`      int(10) NOT NULL,
    `imgpth`     varchar(30) DEFAULT NULL,
    `insdyte`    timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

--     primary key(entdyte),
    UNIQUE(vhlnbr),
    UNIQUE(seq_etyvhl),
    UNIQUE(vhidx)
);


CREATE TABLE parking_payments_list
(
    `pytidx`       int(11) NOT NULL AUTO_INCREMENT,
    `carnumber`    varchar(20) NOT NULL,
    `entdaytime`   varchar(14) NOT NULL,
    `outdaytime`   varchar(14) NOT NULL,
    `parkingtime`  int(11) NOT NULL DEFAULT '0',
    `parkingfee`   int(11) NOT NULL DEFAULT '0',
    `discountfee`  int(11) NOT NULL DEFAULT '0',
    `payout`       int(11) NOT NULL DEFAULT '0',
    `discounttype` varchar(20) NOT NULL,
    `paymenttype`  int(3) NOT NULL DEFAULT '0',
    `workerid`     varchar(20) NOT NULL,
    `pmcsidx`      int(5)  NOT NULL,
    `seq_evl`      int(10) NOT NULL primary key,
    `deistdyte`    varchar(14) DEFAULT NULL,
    `pytrctnbr`    varchar(12) DEFAULT NULL,
    `tsrdve`       varchar(14) DEFAULT NULL,

--     primary key(seq_evl),
    UNIQUE (pytidx)
);



CREATE TABLE parking_prepayments_data
(
    `seq_evl` int(10)  NOT NULL primary key,
    `vhlnbr`  varchar(20) NOT NULL,
    `entdyte` varchar(14) NOT NULL,
    `extdyte` varchar(14) NOT NULL,
    `pkgte`   int(10)  NOT NULL DEFAULT '0',
    `pkgfe`   int(10)  NOT NULL DEFAULT '0',
    `dctfe`   int(10)  NOT NULL DEFAULT '0',
    `pyot`    int(10)  NOT NULL DEFAULT '0',
    `dctte`   varchar(20) DEFAULT NULL,
    `pytte`   int(3)  NOT NULL DEFAULT '0',

--     primary key(seq_evl),
    unique(vhlnbr)

);


CREATE TABLE parking_prepayments_list
(
    `ppayidx`      int(10) NOT NULL AUTO_INCREMENT,
    `carnumber`    varchar(20) NOT NULL,
    `entdaytime`   varchar(14) NOT NULL,
    `outdaytime`   varchar(14) NOT NULL,
    `parkingtime`  int(11) NOT NULL DEFAULT '0',
    `parkingfee`   int(11) NOT NULL DEFAULT '0',
    `discountfee`  int(11) NOT NULL DEFAULT '0',
    `payout`       int(11) NOT NULL DEFAULT '0',
    `discounttype` varchar(20) NOT NULL,
    `paymenttype`  int(3) NOT NULL DEFAULT '0',
    `workerid`     varchar(20) NOT NULL,
    `pmcssidx`     int(5) NOT NULL,
    `seq_evl`      int(10) NOT NULL primary key,
    `pytrctnbr`    varchar(12) DEFAULT NULL,
    `ppytotdyte`   varchar(14) NOT NULL,
    `reoutdyte`    varchar(14) DEFAULT NULL,
    `ppcplcd`      int(3) NOT NULL DEFAULT '0',
    `insdyte`      timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

--     primary key(seq_evl),
    UNIQUE(ppayidx),
    UNIQUE(carnumber)

);



CREATE TABLE parking_vehicle_history
(
    `vhidx`            int(11) NOT NULL AUTO_INCREMENT,
    `carnumber`        varchar(20) NOT NULL,
    `entmcno`          int(3) NOT NULL,
    `entdaytime`       varchar(14) NOT NULL,
    `extmcno`          int(3) DEFAULT NULL,
    `extdaytime`       varchar(14) NOT NULL ,
    `registdivision`   int(1) NOT NULL DEFAULT '0',
    `smallcardivision` int(1) NOT NULL DEFAULT '0',
    `rearcarnumber`    varchar(20) DEFAULT NULL,
    `insdaytime`       varchar(14) NOT NULL,
    `pkno`             int(3) NOT NULL primary key,
    `lvhidx`           int(11) NOT NULL,
    `tsrdve`           varchar(14) DEFAULT NULL,

--     primary key(pkno),
    UNIQUE(vhidx)

);


CREATE TABLE parking_refund_list
(
    `rlidx`    int(11) NOT NULL AUTO_INCREMENT,
    `rfreason` varchar(50) NOT NULL,
    `pkno`     int(3) NOT NULL primary key,
    `pytidx`   int(11) NOT NULL,

--     primary key(pkno),
    unique(rlidx)

);


CREATE TABLE parking_registeredvehicle_history (
       `rvhtyidx` int(3)  NOT NULL AUTO_INCREMENT,
       `rvidx` int(3)  NOT NULL primary key,
       `rvnumber` varchar(20) NOT NULL,
       `username` varchar(20) DEFAULT NULL,
       `tel` varchar(14) NOT NULL DEFAULT '',
       `rvgroup` varchar(30) DEFAULT NULL,
       `member` varchar(20) DEFAULT NULL,
       `duty` varchar(20) DEFAULT NULL,
       `rvname` varchar(20) DEFAULT NULL,
       `rvdivision` int(1)  NOT NULL DEFAULT '1',
       `dcdivision` int(1)  NOT NULL DEFAULT '0',
       `registerdate` varchar(8) NOT NULL,
       `registerstate` int(1) NOT NULL DEFAULT '1',
       `finalregtype` int(3)  NOT NULL DEFAULT '0',
       `startdate` varchar(8) NOT NULL,
       `enddate` varchar(8) NOT NULL,
       `memo` varchar(50) DEFAULT NULL,
       `pkno` int(3) NOT NULL,
       `usrdajbdete` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

--         primary key(rvidx),
       unique (rvhtyidx),
       unique (rvnumber),
       unique (member)


);

CREATE TABLE parking_fee_system_setup (
      `pfssidx` int(11) NOT NULL AUTO_INCREMENT ,
      `pkno` int(3) NOT NULL primary key,
      `pkregdete` varchar(14) NOT NULL,
      `pkdaydctime` varchar(8) NOT NULL DEFAULT '00000000',
      `pkholidaydctime` varchar(8) NOT NULL DEFAULT '00000000',
      `pksundaydctime` varchar(8) NOT NULL DEFAULT '00000000',
      `pksatdaydctime` varchar(8) NOT NULL DEFAULT '00000000',
      `calmethod` int(3) NOT NULL DEFAULT '0',
      `scturntime` int(3) NOT NULL DEFAULT '0',
      `scbasetime` int(5)  NOT NULL DEFAULT '0',
      `scbaseamount` int(5) NOT NULL DEFAULT '0',
      `scadditionaltime` int(5) NOT NULL DEFAULT '0',
      `scadditionalamount` int(5) NOT NULL DEFAULT '0',
      `scsurchargetime` int(5)  NOT NULL DEFAULT '0',
      `ssurchargeamount`int(5)  NOT NULL DEFAULT '0',
      `scdaymaxamount` int(10)  NOT NULL DEFAULT '0',
      `scdaynightmaxamount` int(10)  NOT NULL DEFAULT '0',
      `scholidaymaxamount` int(10) NOT NULL DEFAULT '0',
      `scsundaymaxamount` int(10) NOT NULL DEFAULT '0',
      `scsatdaymaxamount` int(10) NOT NULL DEFAULT '0',
      `bcturntime` int(3)  NOT NULL DEFAULT '0',
      `bcbasetime` int(5)    NOT NULL DEFAULT '0',
      `bcbaseamount` int(5)  NOT NULL DEFAULT '0',
      `bcadditionaltime` int(5)  NOT NULL DEFAULT '0',
      `bcadditionalamount` int(5)  NOT NULL DEFAULT '0',
      `bcsurchargetime` int(5)  NOT NULL DEFAULT '0',
      `bcsurchargeamount` int(5)  NOT NULL DEFAULT '0',
      `bcdaymaxamount` int(10)  NOT NULL DEFAULT '0',
      `bcdaynightmaxamount` int(10)  NOT NULL DEFAULT '0',
      `bcholidaymaxamount` int(10)  NOT NULL DEFAULT '0',
      `bcsundaymaxamount` int(10)  NOT NULL DEFAULT '0',
      `bcsatdaymaxamount` int(10)  NOT NULL DEFAULT '0',
      `extlccrnuedvn` varchar(10) NOT NULL DEFAULT '',
      `zoexedvn` int(3)  NOT NULL DEFAULT '0',
      `zoexete` varchar(8) NOT NULL DEFAULT '',
      `zoexeaodvn` int(3)  NOT NULL DEFAULT '0',
      `prepytactte` varchar(8) NOT NULL DEFAULT '',
      `tnwnutactdvn` int(3) NOT NULL DEFAULT '0',

--       primary key (pkno),
      UNIQUE(pfssidx),
      unique(pkno)
);



CREATE TABLE parking_registeredvehicle_fee_setup
(
    `rvfidx`     int(10) NOT NULL AUTO_INCREMENT PRIMARY KEY,
    `rvfdate`    varchar(8) NOT NULL,
    `rvfstdate`  varchar(8) NOT NULL,
    `rvfedDate`  varchar(8) NOT NULL,
    `rvfstate`   int(4) DEFAULT NULL,
    `rvftype`    int(4) DEFAULT NULL,
    `rvfamount`  int(11) DEFAULT NULL,
    `rvfregtype` int(3)  DEFAULT NULL,
    `rvidx`      int(10) NOT NULL,
    `PkNo`       int(3) NOT NULL,

    unique (rvfidx),
    unique (rvidx)

);


CREATE TABLE parking_registeredvehicle_setup
(
    `rvidx`         int(3) NOT NULL AUTO_INCREMENT,
    `rvnumber`      varchar(20) NOT NULL PRIMARY KEY,
    `username`      varchar(20)          DEFAULT NULL,
    `tel`           varchar(14) NOT NULL DEFAULT '',
    `rvgroup`       varchar(30)          DEFAULT NULL,
    `member`        varchar(20)          DEFAULT NULL,
    `duty`          varchar(20)          DEFAULT NULL,
    `rvname`        varchar(20)          DEFAULT NULL,
    `rvdivision`    int(1) NOT NULL DEFAULT '1',
    `dcdivision`    int(1) NOT NULL DEFAULT '0',
    `registerdate`  varchar(8)  NOT NULL,
    `registerstate` int(1)  NOT NULL DEFAULT '1',
    `finalregtype`  int(3)  NOT NULL DEFAULT '0',
    `startdate`     varchar(8)  NOT NULL,
    `enddate`       varchar(8)  NOT NULL,
    `memo`          varchar(50)          DEFAULT NULL,
    `pkno`          int(3) NOT NULL,
    `usrdajbdete`   timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

    unique (rvidx),
    unique (rvnumber),
    unique (member)
);

