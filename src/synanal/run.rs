use lexanal::LexicalAnalyzer;
use synanal::SyntaxAnalyzer;
use xml::ProteusXmlCreator;
use abstree::print_xml::AbsTreeXmlPrinter;
use abstree::{AbsTree};
use abstree::visitor::Visitor;
use std::str::FromStr;
use std;


pub fn run(program_name : String)
{
    std::env::set_var("PROTEUSXSL",std::env::current_dir().unwrap().join("xsl\\"));
    let mut abstree_xml_creator : ProteusXmlCreator = match ProteusXmlCreator::open(String::from_str("abstree").unwrap()) {
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

    let  mut syntax_analyzer = SyntaxAnalyzer::new(lexical_analyser);
    let abstree = syntax_analyzer.parse().map_err(|error| {
        println!("Error while opening file: {}", error);
        std::process::exit(-3);
    }).unwrap();

    let abstree : Box<AbsTree> = match abstree
    {
        Some(abstree) => abstree,
        None => 
        {
            println!("Error: Empty file!");
            std::process::exit(-2);
        },
    };

    let mut print_abs_xml : AbsTreeXmlPrinter = AbsTreeXmlPrinter::new(abstree_xml_creator);
    abstree.accept(&mut print_abs_xml);
}