import puppeteer from 'puppeteer';
import prompt from 'prompt-async';
import http from 'http';

const TLD_LIST = {"aaa":"aaa","aarp":"aarp","abarth":"abarth","abb":"abb","abbott":"abbott","abbvie":"abbvie","abc":"abc","able":"able","abogado":"abogado","abudhabi":"abudhabi","ac":"ac","academy":"academy","accenture":"accenture","accountant":"accountant","accountants":"accountants","aco":"aco","active":"active","actor":"actor","ad":"ad","adac":"adac","ads":"ads","adult":"adult","ae":"ae","aeg":"aeg","aero":"aero","aetna":"aetna","af":"af","afamilycompany":"afamilycompany","afl":"afl","africa":"africa","ag":"ag","agakhan":"agakhan","agency":"agency","ai":"ai","aig":"aig","aigo":"aigo","airbus":"airbus","airforce":"airforce","airtel":"airtel","akdn":"akdn","al":"al","alfaromeo":"alfaromeo","alibaba":"alibaba","alipay":"alipay","allfinanz":"allfinanz","allstate":"allstate","ally":"ally","alsace":"alsace","alstom":"alstom","am":"am","americanexpress":"americanexpress","americanfamily":"americanfamily","amex":"amex","amfam":"amfam","amica":"amica","amsterdam":"amsterdam","analytics":"analytics","android":"android","anquan":"anquan","anz":"anz","ao":"ao","aol":"aol","apartments":"apartments","app":"app","apple":"apple","aq":"aq","aquarelle":"aquarelle","ar":"ar","arab":"arab","aramco":"aramco","archi":"archi","army":"army","arpa":"arpa","art":"art","arte":"arte","as":"as","asda":"asda","asia":"asia","associates":"associates","at":"at","athleta":"athleta","attorney":"attorney","au":"au","auction":"auction","audi":"audi","audible":"audible","audio":"audio","auspost":"auspost","author":"author","auto":"auto","autos":"autos","avianca":"avianca","aw":"aw","aws":"aws","ax":"ax","axa":"axa","az":"az","azure":"azure","ba":"ba","baby":"baby","baidu":"baidu","banamex":"banamex","bananarepublic":"bananarepublic","band":"band","bank":"bank","bar":"bar","barcelona":"barcelona","barclaycard":"barclaycard","barclays":"barclays","barefoot":"barefoot","bargains":"bargains","baseball":"baseball","basketball":"basketball","bauhaus":"bauhaus","bayern":"bayern","bb":"bb","bbc":"bbc","bbt":"bbt","bbva":"bbva","bcg":"bcg","bcn":"bcn","bd":"bd","be":"be","beats":"beats","beauty":"beauty","beer":"beer","bentley":"bentley","berlin":"berlin","best":"best","bestbuy":"bestbuy","bet":"bet","bf":"bf","bg":"bg","bh":"bh","bharti":"bharti","bi":"bi","bible":"bible","bid":"bid","bike":"bike","bing":"bing","bingo":"bingo","bio":"bio","biz":"biz","bj":"bj","black":"black","blackfriday":"blackfriday","blanco":"blanco","blockbuster":"blockbuster","blog":"blog","bloomberg":"bloomberg","blue":"blue","bm":"bm","bms":"bms","bmw":"bmw","bn":"bn","bnl":"bnl","bnpparibas":"bnpparibas","bo":"bo","boats":"boats","boehringer":"boehringer","bofa":"bofa","bom":"bom","bond":"bond","boo":"boo","book":"book","booking":"booking","bosch":"bosch","bostik":"bostik","boston":"boston","bot":"bot","boutique":"boutique","box":"box","br":"br","bradesco":"bradesco","bridgestone":"bridgestone","broadway":"broadway","broker":"broker","brother":"brother","brussels":"brussels","bs":"bs","bt":"bt","budapest":"budapest","bugatti":"bugatti","build":"build","builders":"builders","business":"business","buy":"buy","buzz":"buzz","bv":"bv","bw":"bw","by":"by","bz":"bz","bzh":"bzh","ca":"ca","cab":"cab","cafe":"cafe","cal":"cal","call":"call","calvinklein":"calvinklein","cam":"cam","camera":"camera","camp":"camp","cancerresearch":"cancerresearch","canon":"canon","capetown":"capetown","capital":"capital","capitalone":"capitalone","car":"car","caravan":"caravan","cards":"cards","care":"care","career":"career","careers":"careers","cars":"cars","cartier":"cartier","casa":"casa","case":"case","caseih":"caseih","cash":"cash","casino":"casino","cat":"cat","catering":"catering","catholic":"catholic","cba":"cba","cbn":"cbn","cbre":"cbre","cbs":"cbs","cc":"cc","cd":"cd","ceb":"ceb","center":"center","ceo":"ceo","cern":"cern","cf":"cf","cfa":"cfa","cfd":"cfd","cg":"cg","ch":"ch","chanel":"chanel","channel":"channel","chase":"chase","chat":"chat","cheap":"cheap","chintai":"chintai","christmas":"christmas","chrome":"chrome","chrysler":"chrysler","church":"church","ci":"ci","cipriani":"cipriani","circle":"circle","cisco":"cisco","citadel":"citadel","citi":"citi","citic":"citic","city":"city","cityeats":"cityeats","ck":"ck","cl":"cl","claims":"claims","cleaning":"cleaning","click":"click","clinic":"clinic","clinique":"clinique","clothing":"clothing","cloud":"cloud","club":"club","clubmed":"clubmed","cm":"cm","cn":"cn","co":"co","coach":"coach","codes":"codes","coffee":"coffee","college":"college","cologne":"cologne","com":"com","comcast":"comcast","commbank":"commbank","community":"community","company":"company","compare":"compare","computer":"computer","comsec":"comsec","condos":"condos","construction":"construction","consulting":"consulting","contact":"contact","contractors":"contractors","cooking":"cooking","cookingchannel":"cookingchannel","cool":"cool","coop":"coop","corsica":"corsica","country":"country","coupon":"coupon","coupons":"coupons","courses":"courses","cr":"cr","credit":"credit","creditcard":"creditcard","creditunion":"creditunion","cricket":"cricket","crown":"crown","crs":"crs","cruise":"cruise","cruises":"cruises","csc":"csc","cu":"cu","cuisinella":"cuisinella","cv":"cv","cw":"cw","cx":"cx","cy":"cy","cymru":"cymru","cyou":"cyou","cz":"cz","dabur":"dabur","dad":"dad","dance":"dance","data":"data","date":"date","dating":"dating","datsun":"datsun","day":"day","dclk":"dclk","dds":"dds","de":"de","deal":"deal","dealer":"dealer","deals":"deals","degree":"degree","delivery":"delivery","dell":"dell","deloitte":"deloitte","delta":"delta","democrat":"democrat","dental":"dental","dentist":"dentist","desi":"desi","design":"design","dev":"dev","dhl":"dhl","diamonds":"diamonds","diet":"diet","digital":"digital","direct":"direct","directory":"directory","discount":"discount","discover":"discover","dish":"dish","diy":"diy","dj":"dj","dk":"dk","dm":"dm","dnp":"dnp","do":"do","docs":"docs","doctor":"doctor","dodge":"dodge","dog":"dog","doha":"doha","domains":"domains","dot":"dot","download":"download","drive":"drive","dtv":"dtv","dubai":"dubai","duck":"duck","dunlop":"dunlop","duns":"duns","dupont":"dupont","durban":"durban","dvag":"dvag","dvr":"dvr","dz":"dz","earth":"earth","eat":"eat","ec":"ec","eco":"eco","edeka":"edeka","edu":"edu","education":"education","ee":"ee","eg":"eg","email":"email","emerck":"emerck","energy":"energy","engineer":"engineer","engineering":"engineering","enterprises":"enterprises","epost":"epost","epson":"epson","equipment":"equipment","er":"er","ericsson":"ericsson","erni":"erni","es":"es","esq":"esq","estate":"estate","esurance":"esurance","et":"et","etisalat":"etisalat","eu":"eu","eurovision":"eurovision","eus":"eus","events":"events","everbank":"everbank","exchange":"exchange","expert":"expert","exposed":"exposed","express":"express","extraspace":"extraspace","fage":"fage","fail":"fail","fairwinds":"fairwinds","faith":"faith","family":"family","fan":"fan","fans":"fans","farm":"farm","farmers":"farmers","fashion":"fashion","fast":"fast","fedex":"fedex","feedback":"feedback","ferrari":"ferrari","ferrero":"ferrero","fi":"fi","fiat":"fiat","fidelity":"fidelity","fido":"fido","film":"film","final":"final","finance":"finance","financial":"financial","fire":"fire","firestone":"firestone","firmdale":"firmdale","fish":"fish","fishing":"fishing","fit":"fit","fitness":"fitness","fj":"fj","fk":"fk","flickr":"flickr","flights":"flights","flir":"flir","florist":"florist","flowers":"flowers","fly":"fly","fm":"fm","fo":"fo","foo":"foo","food":"food","foodnetwork":"foodnetwork","football":"football","ford":"ford","forex":"forex","forsale":"forsale","forum":"forum","foundation":"foundation","fox":"fox","fr":"fr","free":"free","fresenius":"fresenius","frl":"frl","frogans":"frogans","frontdoor":"frontdoor","frontier":"frontier","ftr":"ftr","fujitsu":"fujitsu","fujixerox":"fujixerox","fun":"fun","fund":"fund","furniture":"furniture","futbol":"futbol","fyi":"fyi","ga":"ga","gal":"gal","gallery":"gallery","gallo":"gallo","gallup":"gallup","game":"game","games":"games","gap":"gap","garden":"garden","gb":"gb","gbiz":"gbiz","gd":"gd","gdn":"gdn","ge":"ge","gea":"gea","gent":"gent","genting":"genting","george":"george","gf":"gf","gg":"gg","ggee":"ggee","gh":"gh","gi":"gi","gift":"gift","gifts":"gifts","gives":"gives","giving":"giving","gl":"gl","glade":"glade","glass":"glass","gle":"gle","global":"global","globo":"globo","gm":"gm","gmail":"gmail","gmbh":"gmbh","gmo":"gmo","gmx":"gmx","gn":"gn","godaddy":"godaddy","gold":"gold","goldpoint":"goldpoint","golf":"golf","goo":"goo","goodhands":"goodhands","goodyear":"goodyear","goog":"goog","google":"google","gop":"gop","got":"got","gov":"gov","gp":"gp","gq":"gq","gr":"gr","grainger":"grainger","graphics":"graphics","gratis":"gratis","green":"green","gripe":"gripe","grocery":"grocery","group":"group","gs":"gs","gt":"gt","gu":"gu","guardian":"guardian","gucci":"gucci","guge":"guge","guide":"guide","guitars":"guitars","guru":"guru","gw":"gw","gy":"gy","hair":"hair","hamburg":"hamburg","hangout":"hangout","haus":"haus","hbo":"hbo","hdfc":"hdfc","hdfcbank":"hdfcbank","health":"health","healthcare":"healthcare","help":"help","helsinki":"helsinki","here":"here","hermes":"hermes","hgtv":"hgtv","hiphop":"hiphop","hisamitsu":"hisamitsu","hitachi":"hitachi","hiv":"hiv","hk":"hk","hkt":"hkt","hm":"hm","hn":"hn","hockey":"hockey","holdings":"holdings","holiday":"holiday","homedepot":"homedepot","homegoods":"homegoods","homes":"homes","homesense":"homesense","honda":"honda","honeywell":"honeywell","horse":"horse","hospital":"hospital","host":"host","hosting":"hosting","hot":"hot","hoteles":"hoteles","hotels":"hotels","hotmail":"hotmail","house":"house","how":"how","hr":"hr","hsbc":"hsbc","ht":"ht","hu":"hu","hughes":"hughes","hyatt":"hyatt","hyundai":"hyundai","ibm":"ibm","icbc":"icbc","ice":"ice","icu":"icu","id":"id","ie":"ie","ieee":"ieee","ifm":"ifm","ikano":"ikano","il":"il","im":"im","imamat":"imamat","imdb":"imdb","immo":"immo","immobilien":"immobilien","in":"in","industries":"industries","infiniti":"infiniti","info":"info","ing":"ing","ink":"ink","institute":"institute","insurance":"insurance","insure":"insure","int":"int","intel":"intel","international":"international","intuit":"intuit","investments":"investments","io":"io","ipiranga":"ipiranga","iq":"iq","ir":"ir","irish":"irish","is":"is","iselect":"iselect","ismaili":"ismaili","ist":"ist","istanbul":"istanbul","it":"it","itau":"itau","itv":"itv","iveco":"iveco","iwc":"iwc","jaguar":"jaguar","java":"java","jcb":"jcb","jcp":"jcp","je":"je","jeep":"jeep","jetzt":"jetzt","jewelry":"jewelry","jio":"jio","jlc":"jlc","jll":"jll","jm":"jm","jmp":"jmp","jnj":"jnj","jo":"jo","jobs":"jobs","joburg":"joburg","jot":"jot","joy":"joy","jp":"jp","jpmorgan":"jpmorgan","jprs":"jprs","juegos":"juegos","juniper":"juniper","kaufen":"kaufen","kddi":"kddi","ke":"ke","kerryhotels":"kerryhotels","kerrylogistics":"kerrylogistics","kerryproperties":"kerryproperties","kfh":"kfh","kg":"kg","kh":"kh","ki":"ki","kia":"kia","kim":"kim","kinder":"kinder","kindle":"kindle","kitchen":"kitchen","kiwi":"kiwi","km":"km","kn":"kn","koeln":"koeln","komatsu":"komatsu","kosher":"kosher","kp":"kp","kpmg":"kpmg","kpn":"kpn","kr":"kr","krd":"krd","kred":"kred","kuokgroup":"kuokgroup","kw":"kw","ky":"ky","kyoto":"kyoto","kz":"kz","la":"la","lacaixa":"lacaixa","ladbrokes":"ladbrokes","lamborghini":"lamborghini","lamer":"lamer","lancaster":"lancaster","lancia":"lancia","lancome":"lancome","land":"land","landrover":"landrover","lanxess":"lanxess","lasalle":"lasalle","lat":"lat","latino":"latino","latrobe":"latrobe","law":"law","lawyer":"lawyer","lb":"lb","lc":"lc","lds":"lds","lease":"lease","leclerc":"leclerc","lefrak":"lefrak","legal":"legal","lego":"lego","lexus":"lexus","lgbt":"lgbt","li":"li","liaison":"liaison","lidl":"lidl","life":"life","lifeinsurance":"lifeinsurance","lifestyle":"lifestyle","lighting":"lighting","like":"like","lilly":"lilly","limited":"limited","limo":"limo","lincoln":"lincoln","linde":"linde","link":"link","lipsy":"lipsy","live":"live","living":"living","lixil":"lixil","lk":"lk","llc":"llc","loan":"loan","loans":"loans","locker":"locker","locus":"locus","loft":"loft","lol":"lol","london":"london","lotte":"lotte","lotto":"lotto","love":"love","lpl":"lpl","lplfinancial":"lplfinancial","lr":"lr","ls":"ls","lt":"lt","ltd":"ltd","ltda":"ltda","lu":"lu","lundbeck":"lundbeck","lupin":"lupin","luxe":"luxe","luxury":"luxury","lv":"lv","ly":"ly","ma":"ma","macys":"macys","madrid":"madrid","maif":"maif","maison":"maison","makeup":"makeup","man":"man","management":"management","mango":"mango","map":"map","market":"market","marketing":"marketing","markets":"markets","marriott":"marriott","marshalls":"marshalls","maserati":"maserati","mattel":"mattel","mba":"mba","mc":"mc","mckinsey":"mckinsey","md":"md","me":"me","med":"med","media":"media","meet":"meet","melbourne":"melbourne","meme":"meme","memorial":"memorial","men":"men","menu":"menu","meo":"meo","merckmsd":"merckmsd","metlife":"metlife","mg":"mg","mh":"mh","miami":"miami","microsoft":"microsoft","mil":"mil","mini":"mini","mint":"mint","mit":"mit","mitsubishi":"mitsubishi","mk":"mk","ml":"ml","mlb":"mlb","mls":"mls","mm":"mm","mma":"mma","mn":"mn","mo":"mo","mobi":"mobi","mobile":"mobile","mobily":"mobily","moda":"moda","moe":"moe","moi":"moi","mom":"mom","monash":"monash","money":"money","monster":"monster","mopar":"mopar","mormon":"mormon","mortgage":"mortgage","moscow":"moscow","moto":"moto","motorcycles":"motorcycles","mov":"mov","movie":"movie","movistar":"movistar","mp":"mp","mq":"mq","mr":"mr","ms":"ms","msd":"msd","mt":"mt","mtn":"mtn","mtr":"mtr","mu":"mu","museum":"museum","mutual":"mutual","mv":"mv","mw":"mw","mx":"mx","my":"my","mz":"mz","na":"na","nab":"nab","nadex":"nadex","nagoya":"nagoya","name":"name","nationwide":"nationwide","natura":"natura","navy":"navy","nba":"nba","nc":"nc","ne":"ne","nec":"nec","net":"net","netbank":"netbank","netflix":"netflix","network":"network","neustar":"neustar","new":"new","newholland":"newholland","news":"news","next":"next","nextdirect":"nextdirect","nexus":"nexus","nf":"nf","nfl":"nfl","ng":"ng","ngo":"ngo","nhk":"nhk","ni":"ni","nico":"nico","nike":"nike","nikon":"nikon","ninja":"ninja","nissan":"nissan","nissay":"nissay","nl":"nl","no":"no","nokia":"nokia","northwesternmutual":"northwesternmutual","norton":"norton","now":"now","nowruz":"nowruz","nowtv":"nowtv","np":"np","nr":"nr","nra":"nra","nrw":"nrw","ntt":"ntt","nu":"nu","nyc":"nyc","nz":"nz","obi":"obi","observer":"observer","off":"off","office":"office","okinawa":"okinawa","olayan":"olayan","olayangroup":"olayangroup","oldnavy":"oldnavy","ollo":"ollo","om":"om","omega":"omega","one":"one","ong":"ong","onl":"onl","online":"online","onyourside":"onyourside","ooo":"ooo","open":"open","oracle":"oracle","orange":"orange","org":"org","organic":"organic","origins":"origins","osaka":"osaka","otsuka":"otsuka","ott":"ott","ovh":"ovh","pa":"pa","page":"page","panasonic":"panasonic","panerai":"panerai","paris":"paris","pars":"pars","partners":"partners","parts":"parts","party":"party","passagens":"passagens","pay":"pay","pccw":"pccw","pe":"pe","pet":"pet","pf":"pf","pfizer":"pfizer","pg":"pg","ph":"ph","pharmacy":"pharmacy","phd":"phd","philips":"philips","phone":"phone","photo":"photo","photography":"photography","photos":"photos","physio":"physio","piaget":"piaget","pics":"pics","pictet":"pictet","pictures":"pictures","pid":"pid","pin":"pin","ping":"ping","pink":"pink","pioneer":"pioneer","pizza":"pizza","pk":"pk","pl":"pl","place":"place","play":"play","playstation":"playstation","plumbing":"plumbing","plus":"plus","pm":"pm","pn":"pn","pnc":"pnc","pohl":"pohl","poker":"poker","politie":"politie","porn":"porn","post":"post","pr":"pr","pramerica":"pramerica","praxi":"praxi","press":"press","prime":"prime","pro":"pro","prod":"prod","productions":"productions","prof":"prof","progressive":"progressive","promo":"promo","properties":"properties","property":"property","protection":"protection","pru":"pru","prudential":"prudential","ps":"ps","pt":"pt","pub":"pub","pw":"pw","pwc":"pwc","py":"py","qa":"qa","qpon":"qpon","quebec":"quebec","quest":"quest","qvc":"qvc","racing":"racing","radio":"radio","raid":"raid","re":"re","read":"read","realestate":"realestate","realtor":"realtor","realty":"realty","recipes":"recipes","red":"red","redstone":"redstone","redumbrella":"redumbrella","rehab":"rehab","reise":"reise","reisen":"reisen","reit":"reit","reliance":"reliance","ren":"ren","rent":"rent","rentals":"rentals","repair":"repair","report":"report","republican":"republican","rest":"rest","restaurant":"restaurant","review":"review","reviews":"reviews","rexroth":"rexroth","rich":"rich","richardli":"richardli","ricoh":"ricoh","rightathome":"rightathome","ril":"ril","rio":"rio","rip":"rip","rmit":"rmit","ro":"ro","rocher":"rocher","rocks":"rocks","rodeo":"rodeo","rogers":"rogers","room":"room","rs":"rs","rsvp":"rsvp","ru":"ru","rugby":"rugby","ruhr":"ruhr","run":"run","rw":"rw","rwe":"rwe","ryukyu":"ryukyu","sa":"sa","saarland":"saarland","safe":"safe","safety":"safety","sakura":"sakura","sale":"sale","salon":"salon","samsclub":"samsclub","samsung":"samsung","sandvik":"sandvik","sandvikcoromant":"sandvikcoromant","sanofi":"sanofi","sap":"sap","sapo":"sapo","sarl":"sarl","sas":"sas","save":"save","saxo":"saxo","sb":"sb","sbi":"sbi","sbs":"sbs","sc":"sc","sca":"sca","scb":"scb","schaeffler":"schaeffler","schmidt":"schmidt","scholarships":"scholarships","school":"school","schule":"schule","schwarz":"schwarz","science":"science","scjohnson":"scjohnson","scor":"scor","scot":"scot","sd":"sd","se":"se","search":"search","seat":"seat","secure":"secure","security":"security","seek":"seek","select":"select","sener":"sener","services":"services","ses":"ses","seven":"seven","sew":"sew","sex":"sex","sexy":"sexy","sfr":"sfr","sg":"sg","sh":"sh","shangrila":"shangrila","sharp":"sharp","shaw":"shaw","shell":"shell","shia":"shia","shiksha":"shiksha","shoes":"shoes","shop":"shop","shopping":"shopping","shouji":"shouji","show":"show","showtime":"showtime","shriram":"shriram","si":"si","silk":"silk","sina":"sina","singles":"singles","site":"site","sj":"sj","sk":"sk","ski":"ski","skin":"skin","sky":"sky","skype":"skype","sl":"sl","sling":"sling","sm":"sm","smart":"smart","smile":"smile","sn":"sn","sncf":"sncf","so":"so","soccer":"soccer","social":"social","softbank":"softbank","software":"software","sohu":"sohu","solar":"solar","solutions":"solutions","song":"song","sony":"sony","soy":"soy","space":"space","spiegel":"spiegel","sport":"sport","spot":"spot","spreadbetting":"spreadbetting","sr":"sr","srl":"srl","srt":"srt","st":"st","stada":"stada","staples":"staples","star":"star","starhub":"starhub","statebank":"statebank","statefarm":"statefarm","statoil":"statoil","stc":"stc","stcgroup":"stcgroup","stockholm":"stockholm","storage":"storage","store":"store","stream":"stream","studio":"studio","study":"study","style":"style","su":"su","sucks":"sucks","supplies":"supplies","supply":"supply","support":"support","surf":"surf","surgery":"surgery","suzuki":"suzuki","sv":"sv","swatch":"swatch","swiftcover":"swiftcover","swiss":"swiss","sx":"sx","sy":"sy","sydney":"sydney","symantec":"symantec","systems":"systems","sz":"sz","tab":"tab","taipei":"taipei","talk":"talk","taobao":"taobao","target":"target","tatamotors":"tatamotors","tatar":"tatar","tattoo":"tattoo","tax":"tax","taxi":"taxi","tc":"tc","tci":"tci","td":"td","tdk":"tdk","team":"team","tech":"tech","technology":"technology","tel":"tel","telecity":"telecity","telefonica":"telefonica","temasek":"temasek","tennis":"tennis","teva":"teva","tf":"tf","tg":"tg","th":"th","thd":"thd","theater":"theater","theatre":"theatre","tiaa":"tiaa","tickets":"tickets","tienda":"tienda","tiffany":"tiffany","tips":"tips","tires":"tires","tirol":"tirol","tj":"tj","tjmaxx":"tjmaxx","tjx":"tjx","tk":"tk","tkmaxx":"tkmaxx","tl":"tl","tm":"tm","tmall":"tmall","tn":"tn","to":"to","today":"today","tokyo":"tokyo","tools":"tools","top":"top","toray":"toray","toshiba":"toshiba","total":"total","tours":"tours","town":"town","toyota":"toyota","toys":"toys","tr":"tr","trade":"trade","trading":"trading","training":"training","travel":"travel","travelchannel":"travelchannel","travelers":"travelers","travelersinsurance":"travelersinsurance","trust":"trust","trv":"trv","tt":"tt","tube":"tube","tui":"tui","tunes":"tunes","tushu":"tushu","tv":"tv","tvs":"tvs","tw":"tw","tz":"tz","ua":"ua","ubank":"ubank","ubs":"ubs","uconnect":"uconnect","ug":"ug","uk":"uk","unicom":"unicom","university":"university","uno":"uno","uol":"uol","ups":"ups","us":"us","uy":"uy","uz":"uz","va":"va","vacations":"vacations","vana":"vana","vanguard":"vanguard","vc":"vc","ve":"ve","vegas":"vegas","ventures":"ventures","verisign":"verisign","versicherung":"versicherung","vet":"vet","vg":"vg","vi":"vi","viajes":"viajes","video":"video","vig":"vig","viking":"viking","villas":"villas","vin":"vin","vip":"vip","virgin":"virgin","visa":"visa","vision":"vision","vista":"vista","vistaprint":"vistaprint","viva":"viva","vivo":"vivo","vlaanderen":"vlaanderen","vn":"vn","vodka":"vodka","volkswagen":"volkswagen","volvo":"volvo","vote":"vote","voting":"voting","voto":"voto","voyage":"voyage","vu":"vu","vuelos":"vuelos","wales":"wales","walmart":"walmart","walter":"walter","wang":"wang","wanggou":"wanggou","warman":"warman","watch":"watch","watches":"watches","weather":"weather","weatherchannel":"weatherchannel","webcam":"webcam","weber":"weber","website":"website","wed":"wed","wedding":"wedding","weibo":"weibo","weir":"weir","wf":"wf","whoswho":"whoswho","wien":"wien","wiki":"wiki","williamhill":"williamhill","win":"win","windows":"windows","wine":"wine","winners":"winners","wme":"wme","wolterskluwer":"wolterskluwer","woodside":"woodside","work":"work","works":"works","world":"world","wow":"wow","ws":"ws","wtc":"wtc","wtf":"wtf","xbox":"xbox","xerox":"xerox","xfinity":"xfinity","xihuan":"xihuan","xin":"xin","xn--11b4c3d":"\u0915\u0949\u092e","xn--1ck2e1b":"\u30bb\u30fc\u30eb","xn--1qqw23a":"\u4f5b\u5c71","xn--2scrj9c":"\u0cad\u0cbe\u0cb0\u0ca4","xn--30rr7y":"\u6148\u5584","xn--3bst00m":"\u96c6\u56e2","xn--3ds443g":"\u5728\u7ebf","xn--3e0b707e":"\ud55c\uad6d","xn--3hcrj9c":"\u0b2d\u0b3e\u0b30\u0b24","xn--3oq18vl8pn36a":"\u5927\u4f17\u6c7d\u8f66","xn--3pxu8k":"\u70b9\u770b","xn--42c2d9a":"\u0e04\u0e2d\u0e21","xn--45br5cyl":"\u09ad\u09be\u09f0\u09a4","xn--45brj9c":"\u09ad\u09be\u09b0\u09a4","xn--45q11c":"\u516b\u5366","xn--4gbrim":"\u0645\u0648\u0642\u0639","xn--54b7fta0cc":"\u09ac\u09be\u0982\u09b2\u09be","xn--55qw42g":"\u516c\u76ca","xn--55qx5d":"\u516c\u53f8","xn--5su34j936bgsg":"\u9999\u683c\u91cc\u62c9","xn--5tzm5g":"\u7f51\u7ad9","xn--6frz82g":"\u79fb\u52a8","xn--6qq986b3xl":"\u6211\u7231\u4f60","xn--80adxhks":"\u043c\u043e\u0441\u043a\u0432\u0430","xn--80ao21a":"\u049b\u0430\u0437","xn--80aqecdr1a":"\u043a\u0430\u0442\u043e\u043b\u0438\u043a","xn--80asehdb":"\u043e\u043d\u043b\u0430\u0439\u043d","xn--80aswg":"\u0441\u0430\u0439\u0442","xn--8y0a063a":"\u8054\u901a","xn--90a3ac":"\u0441\u0440\u0431","xn--90ae":"\u0431\u0433","xn--90ais":"\u0431\u0435\u043b","xn--9dbq2a":"\u05e7\u05d5\u05dd","xn--9et52u":"\u65f6\u5c1a","xn--9krt00a":"\u5fae\u535a","xn--b4w605ferd":"\u6de1\u9a6c\u9521","xn--bck1b9a5dre4c":"\u30d5\u30a1\u30c3\u30b7\u30e7\u30f3","xn--c1avg":"\u043e\u0440\u0433","xn--c2br7g":"\u0928\u0947\u091f","xn--cck2b3b":"\u30b9\u30c8\u30a2","xn--cg4bki":"\uc0bc\uc131","xn--clchc0ea0b2g2a9gcd":"\u0b9a\u0bbf\u0b99\u0bcd\u0b95\u0baa\u0bcd\u0baa\u0bc2\u0bb0\u0bcd","xn--czr694b":"\u5546\u6807","xn--czrs0t":"\u5546\u5e97","xn--czru2d":"\u5546\u57ce","xn--d1acj3b":"\u0434\u0435\u0442\u0438","xn--d1alf":"\u043c\u043a\u0434","xn--e1a4c":"\u0435\u044e","xn--eckvdtc9d":"\u30dd\u30a4\u30f3\u30c8","xn--efvy88h":"\u65b0\u95fb","xn--estv75g":"\u5de5\u884c","xn--fct429k":"\u5bb6\u96fb","xn--fhbei":"\u0643\u0648\u0645","xn--fiq228c5hs":"\u4e2d\u6587\u7f51","xn--fiq64b":"\u4e2d\u4fe1","xn--fiqs8s":"\u4e2d\u56fd","xn--fiqz9s":"\u4e2d\u570b","xn--fjq720a":"\u5a31\u4e50","xn--flw351e":"\u8c37\u6b4c","xn--fpcrj9c3d":"\u0c2d\u0c3e\u0c30\u0c24\u0c4d","xn--fzc2c9e2c":"\u0dbd\u0d82\u0d9a\u0dcf","xn--fzys8d69uvgm":"\u96fb\u8a0a\u76c8\u79d1","xn--g2xx48c":"\u8d2d\u7269","xn--gckr3f0f":"\u30af\u30e9\u30a6\u30c9","xn--gecrj9c":"\u0aad\u0abe\u0ab0\u0aa4","xn--gk3at1e":"\u901a\u8ca9","xn--h2breg3eve":"\u092d\u093e\u0930\u0924\u092e\u094d","xn--h2brj9c":"\u092d\u093e\u0930\u0924","xn--h2brj9c8c":"\u092d\u093e\u0930\u094b\u0924","xn--hxt814e":"\u7f51\u5e97","xn--i1b6b1a6a2e":"\u0938\u0902\u0917\u0920\u0928","xn--imr513n":"\u9910\u5385","xn--io0a7i":"\u7f51\u7edc","xn--j1aef":"\u043a\u043e\u043c","xn--j1amh":"\u0443\u043a\u0440","xn--j6w193g":"\u9999\u6e2f","xn--jlq61u9w7b":"\u8bfa\u57fa\u4e9a","xn--jvr189m":"\u98df\u54c1","xn--kcrx77d1x4a":"\u98de\u5229\u6d66","xn--kprw13d":"\u53f0\u6e7e","xn--kpry57d":"\u53f0\u7063","xn--kpu716f":"\u624b\u8868","xn--kput3i":"\u624b\u673a","xn--l1acc":"\u043c\u043e\u043d","xn--lgbbat1ad8j":"\u0627\u0644\u062c\u0632\u0627\u0626\u0631","xn--mgb9awbf":"\u0639\u0645\u0627\u0646","xn--mgba3a3ejt":"\u0627\u0631\u0627\u0645\u0643\u0648","xn--mgba3a4f16a":"\u0627\u06cc\u0631\u0627\u0646","xn--mgba7c0bbn0a":"\u0627\u0644\u0639\u0644\u064a\u0627\u0646","xn--mgbaakc7dvf":"\u0627\u062a\u0635\u0627\u0644\u0627\u062a","xn--mgbaam7a8h":"\u0627\u0645\u0627\u0631\u0627\u062a","xn--mgbab2bd":"\u0628\u0627\u0632\u0627\u0631","xn--mgbai9azgqp6j":"\u067e\u0627\u06a9\u0633\u062a\u0627\u0646","xn--mgbayh7gpa":"\u0627\u0644\u0627\u0631\u062f\u0646","xn--mgbb9fbpob":"\u0645\u0648\u0628\u0627\u064a\u0644\u064a","xn--mgbbh1a":"\u0628\u0627\u0631\u062a","xn--mgbbh1a71e":"\u0628\u06be\u0627\u0631\u062a","xn--mgbc0a9azcg":"\u0627\u0644\u0645\u063a\u0631\u0628","xn--mgbca7dzdo":"\u0627\u0628\u0648\u0638\u0628\u064a","xn--mgberp4a5d4ar":"\u0627\u0644\u0633\u0639\u0648\u062f\u064a\u0629","xn--mgbgu82a":"\u0680\u0627\u0631\u062a","xn--mgbi4ecexp":"\u0643\u0627\u062b\u0648\u0644\u064a\u0643","xn--mgbpl2fh":"\u0633\u0648\u062f\u0627\u0646","xn--mgbt3dhd":"\u0647\u0645\u0631\u0627\u0647","xn--mgbtx2b":"\u0639\u0631\u0627\u0642","xn--mgbx4cd0ab":"\u0645\u0644\u064a\u0633\u064a\u0627","xn--mix891f":"\u6fb3\u9580","xn--mk1bu44c":"\ub2f7\ucef4","xn--mxtq1m":"\u653f\u5e9c","xn--ngbc5azd":"\u0634\u0628\u0643\u0629","xn--ngbe9e0a":"\u0628\u064a\u062a\u0643","xn--ngbrx":"\u0639\u0631\u0628","xn--node":"\u10d2\u10d4","xn--nqv7f":"\u673a\u6784","xn--nqv7fs00ema":"\u7ec4\u7ec7\u673a\u6784","xn--nyqy26a":"\u5065\u5eb7","xn--o3cw4h":"\u0e44\u0e17\u0e22","xn--ogbpf8fl":"\u0633\u0648\u0631\u064a\u0629","xn--otu796d":"\u62db\u8058","xn--p1acf":"\u0440\u0443\u0441","xn--p1ai":"\u0440\u0444","xn--pbt977c":"\u73e0\u5b9d","xn--pgbs0dh":"\u062a\u0648\u0646\u0633","xn--pssy2u":"\u5927\u62ff","xn--q9jyb4c":"\u307f\u3093\u306a","xn--qcka1pmc":"\u30b0\u30fc\u30b0\u30eb","xn--qxam":"\u03b5\u03bb","xn--rhqv96g":"\u4e16\u754c","xn--rovu88b":"\u66f8\u7c4d","xn--rvc1e0am3e":"\u0d2d\u0d3e\u0d30\u0d24\u0d02","xn--s9brj9c":"\u0a2d\u0a3e\u0a30\u0a24","xn--ses554g":"\u7f51\u5740","xn--t60b56a":"\ub2f7\ub137","xn--tckwe":"\u30b3\u30e0","xn--tiq49xqyj":"\u5929\u4e3b\u6559","xn--unup4y":"\u6e38\u620f","xn--vermgensberater-ctb":"verm\u00f6gensberater","xn--vermgensberatung-pwb":"verm\u00f6gensberatung","xn--vhquv":"\u4f01\u4e1a","xn--vuq861b":"\u4fe1\u606f","xn--w4r85el8fhu5dnra":"\u5609\u91cc\u5927\u9152\u5e97","xn--w4rs40l":"\u5609\u91cc","xn--wgbh1c":"\u0645\u0635\u0631","xn--wgbl6a":"\u0642\u0637\u0631","xn--xhq521b":"\u5e7f\u4e1c","xn--xkc2al3hye2a":"\u0b87\u0bb2\u0b99\u0bcd\u0b95\u0bc8","xn--xkc2dl3a5ee0h":"\u0b87\u0ba8\u0bcd\u0ba4\u0bbf\u0baf\u0bbe","xn--y9a3aq":"\u0570\u0561\u0575","xn--yfro4i67o":"\u65b0\u52a0\u5761","xn--ygbi2ammx":"\u0641\u0644\u0633\u0637\u064a\u0646","xn--zfr164b":"\u653f\u52a1","xperia":"xperia","xxx":"xxx","xyz":"xyz","yachts":"yachts","yahoo":"yahoo","yamaxun":"yamaxun","yandex":"yandex","ye":"ye","yodobashi":"yodobashi","yoga":"yoga","yokohama":"yokohama","you":"you","youtube":"youtube","yt":"yt","yun":"yun","za":"za","zappos":"zappos","zara":"zara","zero":"zero","zip":"zip","zippo":"zippo","zm":"zm","zone":"zone","zuerich":"zuerich","zw":"zw"};
const EXCLUDED_DOMAINS = [ "www.w3.org", "amazonaws.com", "google.com", "twitter.com", "facebook.com", "efty.com", "linkedin.com", "joinmastodon.org", "hcaptcha.com", "github.com", "instagram.com", "www.pixiv.net", "reddit.com", "boards.4channel.org", "4chan.org" ];

const checkedDomains = [];
const checkQueue = [];
let found = 0;

const extractLinks = async (domain, browser) => {
    checkedDomains.unshift(domain);
    const page = await browser.newPage();
    try {
        await page.goto(`https://${domain}`);
        await page.waitForTimeout(2000);
        await autoScroll(page);
        const html = await page.evaluate(() => document.querySelector('*').outerHTML);
        const regex = /(([a-zA-Z0-9]+)(\.)([a-zA-Z0-9_]+)(\.)(([a-zA-Z]{2,})+))|(([a-zA-Z0-9]+)(\.)(([a-zA-Z]{2,})+))/g;
        const matches = html.match(regex);
        checkQueue.push(...Array.from(new Set(matches.filter((match) => {
            const spl = match.split('.');
            const tld = spl[spl.length - 1];
            const noSubDomain = spl.length == 2 ? match : spl.slice(1).join('.');
            return !!TLD_LIST[tld] && !EXCLUDED_DOMAINS.includes(match) && !EXCLUDED_DOMAINS.includes(noSubDomain) && !checkedDomains.includes(match) && !checkedDomains.includes(noSubDomain);
        }))));
        await page.close();
    } catch (e) {
        await page.close();
    }
};

async function autoScroll(page){
    await page.evaluate(async () => {
        await new Promise((resolve) => {
            var totalHeight = 0;
            var distance = 250;
            let counter = 0;
            var timer = setInterval(() => {
                var scrollHeight = document.body.scrollHeight;
                window.scrollBy(0, distance);
                totalHeight += distance;
                counter++;
                if(counter > 1500 || totalHeight >= scrollHeight - window.innerHeight){
                    clearInterval(timer);
                    resolve();
                }
            }, 100);
        });
    });
}

(async () => {
    prompt.start();
    const { StartingURL } = await prompt.get(["StartingURL"]);
    const browser = await puppeteer.launch({ headless: false });

    await extractLinks(StartingURL, browser);
    while (true) {
        const domain = checkQueue.pop();
        if (checkedDomains.includes(domain)) continue;
        try{
            let res = await fetch(`https://${domain}/api/v1/instance`);
            let meta = await res?.text();
            if (!meta) continue;
            let approval_required = JSON.parse(meta)?.approval_required;
            console.log(`${domain} - ${approval_required}`);
            //console.log(`http://127.0.0.1:8000/submit/${domain}`);
            http.get(`http://127.0.0.1:8000/submit/${domain}`, (res) => {
                let rawData = '';
                res.on('data', (chunk) => { rawData += chunk; });
                res.on('end', () => {
                    console.log(`status: ${rawData}`);
                    if (rawData == "added_unknown") {
                        found++;
                        console.log(`Found ${found} instances`);
                    }
                });
            }).on('error', (error) => {
                console.error(error)
            });
            //if (approval_required) {
            await extractLinks(domain, browser);
            //}
        } catch(e){
            //console.log(e);
        }
    }

    await browser.close();
})();