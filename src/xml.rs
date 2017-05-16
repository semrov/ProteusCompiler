use std::io::{Write,BufWriter};
use std::fs::File;
use std::io;
use std::env;

 /**
	 * Izpise objekt v XML obliki na izhodni tok.
	 * 
	 * @param xml
	 *            Izhodni tok, na katerega izpise ta objekt v XML obliki.
	 */
pub trait XMLable 
{
    fn to_xml(&self, xml : &mut Write);
}



pub struct ProteusXmlCreator {
    writer: BufWriter<File>,
    phase : String,
}

impl ProteusXmlCreator {
        /**
        * Odpre izhodno datoteko za izpis rezultatov posamezne faze prevajanja v
        * XML formatu.
        * 
        * Ime vhodne datoteke je enako imenu faze prevajanja s koncnico
        * <tt>.xml</tt>. V datoteko se izpise tudi glava XML dokumenta vkljucno z
        * (odprto) oznako glavnega elementa, ki je znova enak imenu faze.
        * 
        * Ce je v lupini nastavljena spremenljivka <tt>PROTEUSXSL</tt>, je njena
        * vrednost uporabljena kot direktorij, na katerem so shranjene pripadajoce
        * <tt>.xsl</tt> datoteke. V tem primeru se v glavo XML dokumenta izpise
        * tudi referenca na pripadajoco <tt>.xsl</tt> datoteko.
        * 
        * @param phase
        *            Ime faze prevajanja.
        * @return Odprta izhodna datoteka.
        */
    pub fn open(phase : String)   -> io::Result<ProteusXmlCreator>  //-> io::Result<BufWriter> 
    {
        let file = File::create(format!("{}.xml", phase))?;
        let writer = BufWriter::new(file);  

        let mut xml_creator = ProteusXmlCreator {phase, writer};

        writeln!(xml_creator.writer, "<?xml version=\"1.0\" encoding=\"ISO-8859-1\"?>")?;
        match env::var("PROTEUSXSL") 
        {
            // "<?xml-stylesheet type=\"text/xsl\" href=\"" + xslDir + "/" + phase + ".xsl\"?>"
            Ok(xsl_dir) => writeln!(xml_creator.writer, "<?xml-stylesheet type=\"text/xsl\" href=\"{}{}.xsl\"?>",xsl_dir,xml_creator.phase)?,
            Err(_) => println!("ProteusXmlCreator::open -> Error: Could not write!"),
        };
            
        writeln!(xml_creator.writer, "<{}>",xml_creator.phase)?;

        Ok(xml_creator)
    }
}


impl Drop for ProteusXmlCreator 
{
    fn drop(&mut self) 
    {
        writeln!(self.writer,"</{}>",self.phase).unwrap();
    }
}

impl Write for ProteusXmlCreator 
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> 
    {
        self.writer.write(buf)
    }
    fn flush(&mut self) -> io::Result<()> 
    {
        self.writer.flush()
    }
}
     