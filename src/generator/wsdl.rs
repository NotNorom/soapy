use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Wsdl {
    #[serde(rename = "{http://schemas.xmlsoap.org/wsdl/}definitions")]
    // xmlns:wsu="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-utility-1.0.xsd" xmlns:wsp="http://www.w3.org/ns/ws-policy" xmlns:wsp1_2="http://schemas.xmlsoap.org/ws/2004/09/policy" xmlns:wsam="http://www.w3.org/2007/05/addressing/metadata" xmlns:soap="http://schemas.xmlsoap.org/wsdl/soap/" xmlns:tns="http://soap.standard.webservice.ecotransit.ivembh.de/" xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns="http://schemas.xmlsoap.org/wsdl/"
    definitions: Definitions,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct Definitions {
    #[serde(rename = "$attr:targetNamespace")]
    target_namespace: String,
    #[serde(rename = "$attr:name")]
    name: String,

    #[serde(rename = "{http://schemas.xmlsoap.org/wsdl/}import")]
    import: Vec<Import>,
    #[serde(rename = "{http://schemas.xmlsoap.org/wsdl/}binding")]
    binding: Binding,
    #[serde(rename = "{http://schemas.xmlsoap.org/wsdl/}service")]
    service: Service,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct Import {
    #[serde(rename = "$attr:namespace")]
    namespace: String,
    #[serde(rename = "$attr:location")]
    location: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct Binding {
    #[serde(rename = "{http://schemas.xmlsoap.org/wsdl/soap/}soap:binding")]
    binding: SoapBinding,
    #[serde(rename = "{http://schemas.xmlsoap.org/wsdl/}operation")]
    operation: Vec<Operation>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct SoapBinding {
    #[serde(rename = "$attr:transport")]
    transport: String,
    #[serde(rename = "$attr:style")]
    style: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct Operation {
    #[serde(rename = "$attr:name")]
    name: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct Service {
    #[serde(rename = "$attr:name")]
    name: String,
    #[serde(rename = "{http://schemas.xmlsoap.org/wsdl/}port")]
    port: Port,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct Port {
    #[serde(rename = "{http://schemas.xmlsoap.org/wsdl/soap/}soap:address")]
    address: Address,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct Address {
    #[serde(rename = "$attr:location")]
    location: String,
}