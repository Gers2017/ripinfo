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
    pub city: String,
    pub country: String,
    pub loc: String,
    pub postal: Option<String>,
    pub timezone: Option<String>,
    pub region: Option<String>,
    pub hostname: Option<String>,
    pub privacy: Option<Privacy>,
    pub company: Option<Company>,
    pub carrier: Option<Carrier>,
    pub abuse: Option<Abuse>,
    pub domains: Option<Domains>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Company {
    pub name: Option<String>,
    pub domain: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Carrier {
    pub name: Option<String>,
    pub mcc: Option<String>,
    pub mnc: Option<String>,
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
    pub address: Option<String>,
    pub country: Option<String>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub network: Option<String>,
    pub phone: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Domains {
    pub ip: Option<String>,
    pub total: u16,
    pub domains: Vec<String>,
}
