use std;
use lexanal::LexicalAnalyzer;
use xml::ProteusXmlCreator;
use std::str::FromStr;
use xml::XMLable;

pub fn run(program_name : String)
{
    std::env::set_var("PROTEUSXSL",std::env::current_dir().unwrap().join("xsl\\"));
    let mut lexical_analyser = match LexicalAnalyzer::new(program_name) 
    {
        Ok(lexical_analyser) => lexical_analyser,
        Err(e) =>
        {
             println!("{}",e);
             std::process::exit(-1);
        }
    };

    let mut lexanal_xml_creator : ProteusXmlCreator = match ProteusXmlCreator::open(String::from_str("lexanal").unwrap()) {
        Ok(xml_creator) => xml_creator,
        Err(e) =>
        {
             println!("{}",e);
             std::process::exit(-2);
        },
    };
    loop 
    {
       match lexical_analyser.get_next_symbol() 
       {
           Ok(Some(symbol)) => symbol.to_xml(&mut lexanal_xml_creator),
           Ok(None) => break,
           Err(e) => 
           {
               println!("An error occured while lexical analysis: {}",e);
           }
       }
    }
}