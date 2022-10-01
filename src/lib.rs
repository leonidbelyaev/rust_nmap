//! A module for nmap xml

#[allow(dead_code)]
extern crate serde;
extern crate serde_xml_rs;
use std::fs;

use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[allow(non_camel_case_types)]
pub struct nmap_run {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(deserialize_with = "empty_str_to_none_i32")]
    pub start: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startstr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xmloutputversion: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaninfo: Option<scaninfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbose: Option<verbose>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debugging: Option<debugging>,
    //TODO: taskbegin taskend taskprogress
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub taskbegin: Option<Vec<task_info>>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub taskend: Option<Vec<task_info>>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub taskprogress: Option<Vec<task_progress_info>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prescript: Option<prescript>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // #[serde(rename = "prescript>script")]
    // pub prescripts: Option<Vec<script>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postscript: Option<postscript>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // #[serde(rename = "postscript>script")]
    // pub postscripts: Option<Vec<script>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<Vec<host>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<Vec<target>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runstats: Option<runstats>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[allow(non_camel_case_types)]
pub struct prescript {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<Vec<script>>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[allow(non_camel_case_types)]
pub struct postscript {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<Vec<script>>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[allow(non_camel_case_types)]
pub struct scaninfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(deserialize_with = "empty_str_to_none_u32")]
    pub numservices: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanflags: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[allow(non_camel_case_types)]
pub struct verbose {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(deserialize_with = "empty_str_to_none_u32")]
    pub level: Option<u32>,
}
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[allow(non_camel_case_types)]
pub struct debugging {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(deserialize_with = "empty_str_to_none_u32")]
    pub level: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[allow(non_camel_case_types)]
pub struct task_info {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(deserialize_with = "empty_str_to_none_i32")]
    pub time: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extrainfo: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[allow(non_camel_case_types)]
pub struct task_progress_info {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(deserialize_with = "empty_str_to_none_i32")]
    pub time: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(deserialize_with = "empty_str_to_none_f32")]
    pub percent: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(deserialize_with = "empty_str_to_none_u32")]
    pub remaining: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(deserialize_with = "empty_str_to_none_u32")]
    pub etc: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[allow(non_camel_case_types)]
pub struct script {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<Vec<table>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elem: Option<Vec<element>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[allow(non_camel_case_types)]
pub struct table {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elem: Option<Vec<element>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<Vec<table>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[allow(non_camel_case_types)]
pub struct element {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "$value")]
    pub value: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[allow(non_camel_case_types)]
pub struct host {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(deserialize_with = "empty_str_to_none_i32")]
    pub starttime: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(deserialize_with = "empty_str_to_none_i32")]
    pub endtime: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<status>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Vec<address>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostnames: Option<hostnames>,
    // #[serde(rename = "hostnames>hostname")]
    // pub hostnames: Option<Vec<hostname>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smurf: Option<Vec<smurf>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<ports2>,
    // #[serde(rename = "ports>port")]
    // pub ports: Option<Vec<port>>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // #[serde(rename = "ports>extraports")]
    // pub extraports: Option<Vec<extraports>>,
    // #[serde(rename = "ports")]
    // pub ports2: Option<Vec<port>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<os>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance: Option<distance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uptime: Option<uptime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipidsequence: Option<ipidsequence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcptssequence: Option<tcptssequence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostscript: Option<hostscript>,
    // #[serde(rename = "hostscript>script")]
    // pub hostscripts: Option<script>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<trace>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub times: Option<times>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[allow(non_camel_case_types)]
pub struct hostscript {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<script>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[allow(non_camel_case_types)]
pub struct hostnames {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<Vec<hostname>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[allow(non_camel_case_types)]
pub struct ports2 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<Vec<port>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extraports: Option<Vec<extraports>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[allow(non_camel_case_types)]
pub struct times {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srtt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rttvar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[allow(non_camel_case_types)]
pub struct trace {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proto: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(deserialize_with = "empty_str_to_none_u32")]
    pub port: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hop")]
    pub hops: Option<Vec<hop>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[allow(non_camel_case_types)]
pub struct hop {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(deserialize_with = "empty_str_to_none_f32")]
    pub ttl: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(deserialize_with = "empty_str_to_none_f32")]
    pub rtt: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipaddr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[allow(non_camel_case_types)]
pub struct tcptssequence {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[allow(non_camel_case_types)]
pub struct ipidsequence {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[allow(non_camel_case_types)]
pub struct uptime {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(deserialize_with = "empty_str_to_none_u32")]
    pub seconds: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastboot: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[allow(non_camel_case_types)]
pub struct distance {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(deserialize_with = "empty_str_to_none_i32")]
    pub value: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[allow(non_camel_case_types)]
pub struct os {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portused: Option<Vec<portused>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub osmatch: Option<Vec<osmatch>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub osfingerprint: Option<Vec<osfingerprint>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[allow(non_camel_case_types)]
pub struct portused {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proto: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(deserialize_with = "empty_str_to_none_u32")]
    pub portid: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[allow(non_camel_case_types)]
pub struct osmatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(deserialize_with = "empty_str_to_none_u32")]
    pub accuracy: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(deserialize_with = "empty_str_to_none_u32")]
    pub line: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub osclass: Option<Vec<osclass>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[allow(non_camel_case_types)]
pub struct osclass {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub osgen: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(deserialize_with = "empty_str_to_none_u32")]
    pub accuracy: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub osfamily: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cpe")]
    pub cpe: Option<String>,
    // pub cpes: Option<Vec<cpe>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[allow(non_camel_case_types)]
pub struct osfingerprint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[allow(non_camel_case_types)]
pub struct extraports {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(deserialize_with = "empty_str_to_none_u32")]
    pub count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extrareasons: Option<Vec<reason>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[allow(non_camel_case_types)]
pub struct reason {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(deserialize_with = "empty_str_to_none_u32")]
    pub count: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[allow(non_camel_case_types)]
pub struct status {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(deserialize_with = "empty_str_to_none_f32")]
    pub reason_ttl: Option<f32>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[allow(non_camel_case_types)]
pub struct address {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addrtype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[allow(non_camel_case_types)]
pub struct hostname {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[allow(non_camel_case_types)]
pub struct smurf {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[allow(non_camel_case_types)]
pub struct port {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(deserialize_with = "empty_str_to_none_u32")]
    pub portid: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<state>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<owner>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<service>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<Vec<script>>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[allow(non_camel_case_types)]
pub struct state {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(deserialize_with = "empty_str_to_none_f32")]
    pub reason_ttl: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_ip: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[allow(non_camel_case_types)]
pub struct owner {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[allow(non_camel_case_types)]
pub struct service {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(deserialize_with = "empty_str_to_none_u32")]
    pub conf: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extrainfo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tunnel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proto: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rpcnum: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lowver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hiver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ostype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devicetype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servicefp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cpe")]
    pub cpe: Option<Vec<cpe>>,
    // pub cpes: Option<Vec<cpe>>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[allow(non_camel_case_types)]
pub struct cpe {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "$value")]
    value: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[allow(non_camel_case_types)]
pub struct target {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specification: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[allow(non_camel_case_types)]
pub struct runstats {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished: Option<finished>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hoststats: Option<hoststats>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[allow(non_camel_case_types)]
pub struct finished {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(deserialize_with = "empty_str_to_none_i32")]
    pub time: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(deserialize_with = "empty_str_to_none_f32")]
    pub elapsed: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errormsg: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[allow(non_camel_case_types)]
pub struct hoststats {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(deserialize_with = "empty_str_to_none_i32")]
    pub up: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(deserialize_with = "empty_str_to_none_i32")]
    pub down: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(deserialize_with = "empty_str_to_none_i32")]
    pub total: Option<i32>,
}

type BoxResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// # Examples
///
/// Basic usage:
///
/// ```no_run
/// use rust_nmap;
///
/// let result = rust_nmap::parse_nmap_xml_file("/xxx/nmap_result.xml");
/// println!("{:?}", result.unwrap());
/// ```
pub fn parse_nmap_xml_file(filename: &str) -> BoxResult<nmap_run> {
    let xml_info = match fs::read_to_string(filename) {
        Ok(xml_info) => xml_info,
        Err(err) => return Err(Box::new(err) as Box<dyn std::error::Error>),
    };
    let nmap_run_info = match serde_xml_rs::from_str(&xml_info) {
        Ok(nmap_run_info) => nmap_run_info,
        Err(err) => return Err(Box::new(err) as Box<dyn std::error::Error>),
    };
    Ok(nmap_run_info)
}

pub fn parse_nmap_xml_bytes(bytes: &'static [u8]) -> BoxResult<nmap_run> {
    let xml_info = match std::str::from_utf8(bytes) {
        Ok(xml_info) => xml_info,
        Err(err) => return Err(Box::new(err) as Box<dyn std::error::Error>),
    };
    let nmap_run_info = match serde_xml_rs::from_str(&xml_info) {
        Ok(nmap_run_info) => nmap_run_info,
        Err(err) => return Err(Box::new(err) as Box<dyn std::error::Error>),
    };
    Ok(nmap_run_info)
}

pub fn parse_nmap_xml_str(xml_info: &str) -> BoxResult<nmap_run> {
    let nmap_run_info = match serde_xml_rs::from_str(&xml_info) {
        Ok(nmap_run_info) => nmap_run_info,
        Err(err) => return Err(Box::new(err) as Box<dyn std::error::Error>),
    };
    Ok(nmap_run_info)
}

fn empty_str_to_none_u32<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;
    let parsed = buf.as_str().parse::<u32>();
    match parsed {
        Err(e) => Ok(None),
        Ok(int) => Ok(Some(int)),
    }
}

fn empty_str_to_none_f32<'de, D>(deserializer: D) -> Result<Option<f32>, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;
    let parsed = buf.as_str().parse::<f32>();
    match parsed {
        Err(e) => Ok(None),
        Ok(int) => Ok(Some(int)),
    }
}

fn empty_str_to_none_i32<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;
    let parsed = buf.as_str().parse::<i32>();
    match parsed {
        Err(e) => Ok(None),
        Ok(int) => Ok(Some(int)),
    }
}

#[test]
fn empty_int_as_none() {
    let input = r#"
<nmaprun start="1649957085" profile_name="" xmloutputversion="1.04" scanner="nmap" version="7.92" startstr="Thu Apr 14 13:24:45 2022" args="nmap -T4 -A -v --unique -iL neu_as">
  <verbose level="1"></verbose>
  <debugging level="0"></debugging>
  <host comment="">
    <status state="down"></status>
    <address addrtype="ipv4" vendor="" addr="44.44.126.0"></address>
    <hostnames></hostnames>
    <ports></ports>
    <os></os>
    <uptime lastboot="" seconds=""></uptime>
  </host>
  <runstats>
    <finished timestr="Fri Apr 15 05:21:02 2022" time="1650014462"></finished>
    <hosts down="129882" total="131840" up="1958"></hosts>
  </runstats>
</nmaprun>
"#;

    let result = parse_nmap_xml_str(input).unwrap();

    assert_eq!(
        result
            .host
            .unwrap()
            .into_iter()
            .next()
            .unwrap()
            .uptime
            .unwrap()
            .seconds,
        None
    )
}
