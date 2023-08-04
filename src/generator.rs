#[allow(unused)]
mod wsdl;
#[allow(unused)]
mod xsd;


pub struct Operation  {
    
}


#[cfg(test)]
mod test_generator {
    use xml_serde::from_str;

    use super::{wsdl::Wsdl, xsd::Xsd};

    #[test]
    fn test_wsdl() {
        tracing_subscriber::fmt::init();
        let wsdl_str = include_str!("../assets/ETWStandard.wsdl");

        let wsdl_struct = from_str::<Wsdl>(wsdl_str);
        println!("{wsdl_struct:#?}")
    }
    #[test]
    fn test_xsd() {
        tracing_subscriber::fmt::init();
        let xsd_str = include_str!("../assets/ETWStandard_2.xsd");
        let result = from_str::<Xsd>(xsd_str);
        println!("{result:#?}")
    }
}
