use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IpResponseBusiness {
    pub input: String,
    pub data: IpData,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IpData {
    pub ip: String,
    pub hostname: String,
    pub city: String,
    pub region: String,
    pub country: String,
    pub loc: String,
    pub postal: String,
    pub timezone: String,
    pub privacy: Option<Privacy>,
    pub company: Option<Company>,
    pub carrier: Option<Carrier>,
    pub abuse: Option<Abuse>,
    pub domains: Option<Domains>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Company {
    pub name: String,
    pub domain: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Carrier {
    pub name: String,
    pub mcc: String,
    pub mnc: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Privacy {
    pub vpn: bool,
    pub proxy: bool,
    pub tor: bool,
    pub relay: bool,
    pub hosting: bool,
    pub service: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Abuse {
    pub address: String,
    pub country: String,
    pub email: String,
    pub name: String,
    pub network: String,
    pub phone: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Domains {
    pub total: u16,
    pub domains: Vec<String>,
}
