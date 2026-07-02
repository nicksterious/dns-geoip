use trust_dns_resolver::error::ResolveResult;
use trust_dns_resolver::Resolver;
use trust_dns_resolver::{config::*, lookup::TxtLookup};

fn main() {
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
    let txt_response = resolver.txt_lookup("188.12.23.34.geo.ipns.cc.");
    display_txt(&txt_response);
}

fn display_txt(txt_response: &ResolveResult<TxtLookup>) {
    match txt_response {
        Err(_) => println!("No TXT Records."),
        Ok(txt_response) => {
            let mut i = 1;
            for record in txt_response.iter() {
                println!("TXT Record {}:", i);
                println!("{}", record.to_string());
                println!("");
                i = i + 1;
            }
        }
    }
}


// .... or something like this
