/*Unfortunately as I write this I have had issues getting this scrapper to run due to one of the openssl crates
I'm not sure exactly what the issue is, but I'm close to the deadline and unable to complete this portion of my project.  Though the main focus is the Alexa skill I decided to include it as it was some work I have completed and had
working at one point.
*/
extern crate reqwest;
extern crate regex;
use regex::Regex;



//Player struct
struct Bplayer {

	pts: String,
	gp: String,
	reb: String,
	ast: String,
	fgp: String,
	three: String,
	ftp: String,

}	

//Scraper
fn scrape() -> Result<(), reqwest::Error> {
	let urlLillard = "https://www.basketball-reference.com/players/l/lillada01/gamelog/2019";
	//Converts webpage to plain text
	let htmlLillard = reqwest::get(urlLillard)?.text()?;
	
	//Regular expressions for finding specific data
	let regp = Regex::new("<h4 class=\"poptip\" data-tip=\"Games\">G</h4><p>(.*?)</p>").unwrap();
	let repts = Regex::new("<h4 class=\"poptip\" data-tip=\"Points\">PTS</h4><p>(.*?)</p>").unwrap();
	let rereb = Regex::new("<h4 class=\"poptip\" data-tip=\"Total Rebounds\">TRB</h4><p>(.*?)</p>").unwrap();
	let reast = Regex::new("<h4 class=\"poptip\" data-tip=\"Assists\">AST</h4><p>(.*?)</p>").unwrap();
	let refgp = Regex::new("<h4 class=\"poptip\" data-tip=\"Field Goal Percentage\">FG%</h4><p>(.*?)</p>").unwrap();
	let re3pt = Regex::new("<h4 class=\"poptip\" data-tip=\"3-Point Field Goal Percentage\">FG3%</h4><p>(.*?)</p>").unwrap();
	let refth = Regex::new("<h4 class=\"poptip\" data-tip=\"Free Throw Percentage\">FT%</h4><p>(.*?)</p>").unwrap();

	//Applies regexes to the plain text tile then collects the data and places it into strings
	let mut gps:String = regp.captures_iter(&htmlLillard).map(|x| { x[1].to_string()}).collect();
	let mut points:String = repts.captures_iter(&htmlLillard).map(|x| { x[1].to_string()}).collect();
	let mut rebs:String = rereb.captures_iter(&htmlLillard).map(|x| { x[1].to_string()}).collect();
	let mut asts:String = reast.captures_iter(&htmlLillard).map(|x| { x[1].to_string()}).collect();
	let mut fgper:String = refgp.captures_iter(&htmlLillard).map(|x| { x[1].to_string()}).collect();
	let mut threept:String = re3pt.captures_iter(&htmlLillard).map(|x| { x[1].to_string()}).collect();
	let mut freeth:String = refth.captures_iter(&htmlLillard).map(|x| { x[1].to_string()}).collect();

	//Places the data in the struct
	let Lillard:Bplayer = Bplayer { pts:points, gp:gps, reb:rebs, ast:asts, fgp:fgper, three:threept, ftp:freeth };
	println!("Player Name: Damian Lillard, Games Played: {:?}, Points: {:?}, Rebounds: {:?}, Assists: {:?}, FG% {:?}, 3% {:?}, FreeThrow% {:?}", 
	Lillard.gp, Lillard.pts, Lillard.reb, Lillard.ast, Lillard.fgp, Lillard.three, Lillard.ftp);

	Ok(())
}

fn main() {
	scrape().unwrap();

}
