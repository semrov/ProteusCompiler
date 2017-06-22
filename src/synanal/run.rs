use lexanal::LexicalAnalyzer;
use synanal::SyntaxAnalyzer;
use xml::ProteusXmlCreator;
use std::str::FromStr;
use std;


pub fn run(program_name : String)
{
    std::env::set_var("PROTEUSXSL",std::env::current_dir().unwrap().join("xsl\\"));
    let mut synanal_xml_creator : ProteusXmlCreator = match ProteusXmlCreator::open(String::from_str("synanal").unwrap()) {
        Ok(xml_creator) => xml_creator,
        Err(e) =>
        {
             println!("{}",e);
             std::process::exit(-2);
        },
    };
    let mut lexical_analyser = match LexicalAnalyzer::new(program_name) 
    {
        Ok(lexical_analyser) => lexical_analyser,
        Err(e) =>
        {
             println!("{}",e);
             std::process::exit(-1);
        }
    };

    let mut syntax_analyzer = SyntaxAnalyzer::new_with_xml_creator(lexical_analyser, synanal_xml_creator);
    syntax_analyzer.parse().map_err(|error| {
        println!("Error while opening file: {}", error);
        std::process::exit(-3);
    });

}