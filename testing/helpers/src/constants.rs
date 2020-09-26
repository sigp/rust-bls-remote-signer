// Legit BLS pairs.
pub const PUBLIC_KEY_1: &str = "b7354252aa5bce27ab9537fd0158515935f3c3861419e1b4b6c8219b5dbd15fcf907bddf275442f3e32f904f79807a2a";
pub const SECRET_KEY_1: &str = "68081afeb7ad3e8d469f87010804c3e8d53ef77d393059a55132637206cc59ec";

pub const PUBLIC_KEY_2: &str = "9324739760579527b4f8c34c5df42f9fd89f59fdbe8a01d58675769f60fec5da9b9c8d7a3203cf2217692e49e7b98d97";
pub const SECRET_KEY_2: &str = "45b5e876e5e57b23af3e86c37d708626cf1dcca6a650091bba2ddb3e0b7304ae";

pub const PUBLIC_KEY_3: &str = "8244ac66a8bffa0ce0af04d69ed7ed009951061259173a7c7ae1f25c049f0fcbbf2fad67b6d2b276a697315be755dac5";
pub const SECRET_KEY_3: &str = "1e52a4e54e89ccba813d5f902545749c356f6187341b4e765bf43ece401762f6";

// It is valid (from 0731e07e99a0b1c69f0de13ad65e5c374e72d0a997d43387ad70448485879ca1),
// But we are not uploading it.
pub const ABSENT_PUBLIC_KEY: &str = "827803e94e4b8d306735df9002465b310fabb39802341dc5c616a204e4e8dc7dbb6caa4733b5da54f8cdeec7788e7500";

// This is the public key of 0e5faaa97a63929cecb8597949ae148c0607f1b30bd057a7487efeb4c701fbf8.
pub const MISMATCHED_PUBLIC_KEY: &str = "83d40dfb1cbcf2a55c139faa3feec14bdae92dd485009ac8c5670d241f71c2ce064afa48dbaf091e16d0e4356038b948";

// This is the public key of 34e62afe7c4402009a46bf8af574f9d6701c2cf72b3868eeeb59dfa6e7ff6bcf.
pub const SUB_DIR_NAME: &str = "aadbe2d5c0316dd3c9a522029f332cde578730e61d759685d7ad3bf1166c5f0bf094c3abc105384506f052e2b7a1bae0";

// Silly files with long names (96 chars) to fill your BLS raw file directory.
pub const SILLY_FILE_NAME_1: &str =
    "IAmAdamPrinceofEterniaDefenderofthesecretsoftheCastleGrayskullThisisCringermyfearlessfriendFabul";
pub const SILLY_CONTENT_1: &str = "HemanandtheMastersoftheUniverse";

pub const SILLY_FILE_NAME_2: &str =
    "InthenearfutureDocTerrorandhiscyborgcompanionHackerunleashtheirforcestoconquerEarthOnlyoneforcec";
pub const SILLY_CONTENT_2: &str = "Centurions";

pub const SILLY_FILE_NAME_3: &str =
    "OurworldisinperilGaiathespiritoftheearthcannolongerstandtheterribledestructionplaguingourplanetS";
pub const SILLY_CONTENT_3: &str = "CaptainPlanet";

// Taken from some random string.
pub const SIGNING_ROOT: &str = "0xb6bb8f3765f93f4f1e7c7348479289c9261399a3c6906685e320071a1a13955c";

// Expected signature for the message 0xb6bb8f3765f93f4f1e7c7348479289c9261399a3c6906685e320071a1a13955c
// using 68081afeb7ad3e8d469f87010804c3e8d53ef77d393059a55132637206cc59ec as secret key
pub const EXPECTED_SIGNATURE_1: &str = "0xb5d0c01cef3b028e2c5f357c2d4b886f8e374d09dd660cd7dd14680d4f956778808b4d3b2ab743e890fc1a77ae62c3c90d613561b23c6adaeb5b0e288832304fddc08c7415080be73e556e8862a1b4d0f6aa8084e34a901544d5bb6aeed3a612";

// Expected signature for the message 0xb6bb8f3765f93f4f1e7c7348479289c9261399a3c6906685e320071a1a13955c
// using 45b5e876e5e57b23af3e86c37d708626cf1dcca6a650091bba2ddb3e0b7304ae as secret key
pub const EXPECTED_SIGNATURE_2: &str = "0xb6b63e3cecd0967d9f9b90e3ee113dfb21ecd3901dbc654ca69649ac5a0746758661306627f18bb6d7a6ea03ace069500ee79a28154c172dd71ffe4b711875e48b60466a90f3a4dcacdbc9b5f5434ad68c91e603fe1703324d83617f5270aead";

// Expected signature for the message 0xb6bb8f3765f93f4f1e7c7348479289c9261399a3c6906685e320071a1a13955c
// using 1e52a4e54e89ccba813d5f902545749c356f6187341b4e765bf43ece401762f6 as secret key
pub const EXPECTED_SIGNATURE_3: &str = "0x874f7d6d4174df1088ab40bd9a3c808554c55d6de1dffcacc7ef56c3ca22e20b52a23dd5bb6568a123b59df0bacef3de14d4c197a2fb2a5868a18c4b11f6d7957673d9a302bf6812b1d5df9b264504f682b43dfbcf4f9130cb5ebb9b8e3737de";
