use ark_bn254::{Bn254, Fq, Fq2, G1Affine, G2Affine, Fr}; use ark_groth16::VerifyingKey;
// For Bn254 curve types
use serde::Deserialize; // For deserializing JSON data
use std::str::FromStr; // For string parsing

use ark_groth16::{prepare_verifying_key,Groth16,Proof};

const  JSON_VERIFICATION_KEY : &str = r#"
{
    "protocol": "groth16",
    "curve": "bn128",
    "nPublic": 32,
    "vk_alpha_1": [
     "20491192805390485299153009773594534940189261866228447918068658471970481763042",
     "9383485363053290200918347156157836566562967994039712273449902621266178545958",
     "1"
    ],
    "vk_beta_2": [
     [
      "6375614351688725206403948262868962793625744043794305715222011528459656738731",
      "4252822878758300859123897981450591353533073413197771768651442665752259397132"
     ],
     [
      "10505242626370262277552901082094356697409835680220590971873171140371331206856",
      "21847035105528745403288232691147584728191162732299865338377159692350059136679"
     ],
     [
      "1",
      "0"
     ]
    ],
    "vk_gamma_2": [
     [
      "10857046999023057135944570762232829481370756359578518086990519993285655852781",
      "11559732032986387107991004021392285783925812861821192530917403151452391805634"
     ],
     [
      "8495653923123431417604973247489272438418190587263600148770280649306958101930",
      "4082367875863433681332203403145435568316851327593401208105741076214120093531"
     ],
     [
      "1",
      "0"
     ]
    ],
    "vk_delta_2": [
     [
      "7269673071694555329829623951275703149369200667383437560831236002024650510675",
      "10670447150038412094764638092464051265872841093076019518143860639642060635312"
     ],
     [
      "4082555195723360776359371512301327512060530133660179249127044411151126402638",
      "11125931132584371841236980826336991291058242123900717393264125463019472394383"
     ],
     [
      "1",
      "0"
     ]
    ],
    "vk_alphabeta_12": [
     [
      [
       "2029413683389138792403550203267699914886160938906632433982220835551125967885",
       "21072700047562757817161031222997517981543347628379360635925549008442030252106"
      ],
      [
       "5940354580057074848093997050200682056184807770593307860589430076672439820312",
       "12156638873931618554171829126792193045421052652279363021382169897324752428276"
      ],
      [
       "7898200236362823042373859371574133993780991612861777490112507062703164551277",
       "7074218545237549455313236346927434013100842096812539264420499035217050630853"
      ]
     ],
     [
      [
       "7077479683546002997211712695946002074877511277312570035766170199895071832130",
       "10093483419865920389913245021038182291233451549023025229112148274109565435465"
      ],
      [
       "4595479056700221319381530156280926371456704509942304414423590385166031118820",
       "19831328484489333784475432780421641293929726139240675179672856274388269393268"
      ],
      [
       "11934129596455521040620786944827826205713621633706285934057045369193958244500",
       "8037395052364110730298837004334506829870972346962140206007064471173334027475"
      ]
     ]
    ],
    "ic": [
     [
      "16028465895964049502490678558809417235683213451318411323451368790100438200187",
      "19389420058701719820409036630364176740133572318615644539431869869396355996406",
      "1"
     ],
     [
      "19056609601965810892203102323463525694563020624836160804068545364328568111170",
      "2778241474042390331385130789685611686278288226660364612713687611504300740720",
      "1"
     ],
     [
      "13824694941158746200639706412798507395606172423478149635951018085209930501741",
      "11459213799754435975330591227395707355942938396044057760289355697576417691401",
      "1"
     ],
     [
      "12798142727445632191499435787115246025917593250242452457353949034639371466240",
      "18053032594180740436957078064790983322851313022536981828227464396916029617028",
      "1"
     ],
     [
      "9701321412275694857423277786143413340697138464796638514567650330691599172837",
      "5731991725654477694203447904205147201404056149160031497890394914171175888861",
      "1"
     ],
     [
      "19384408339865943449158065948487163657017253685351911470860570806340584772901",
      "20435386114208608765117530617005566993249403003397513375227762988073318142384",
      "1"
     ],
     [
      "5378941748168826767390133573718436538323610708294919580583181702015620356855",
      "18243853058444074363786009243777291909697459016913701662220755348282517219058",
      "1"
     ],
     [
      "13259698299233430760604106385464357393251584309031071226620989708020906992392",
      "5118605763163255610117436600846474882771617767000697693016200615930077514878",
      "1"
     ],
     [
      "10305416518419393527760807641607398804404362842085377170352777562324757389761",
      "12849295076985258437250050819895769389041878260240548698672046713266867244846",
      "1"
     ],
     [
      "14062440981666185994645292384449522359998813844602571441767738590079634404634",
      "5435571536118484494914535244710321395542879944437315841119354784873979877414",
      "1"
     ],
     [
      "1484018865619577588407351138667317539650791060502035544229109364716566378266",
      "15365662599347326950686008355556733281956892676071074811700009775737774312079",
      "1"
     ],
     [
      "20102715513415929351526507077844755981215486253575516815776827951193280165455",
      "13586753207017434401570953683319762364630969732390623539114268665103516176537",
      "1"
     ],
     [
      "10808683349601519999722775101787678701108135273647177228898683165851324006500",
      "16051638336795927948554631893297577034865339319537483645096908635642972862524",
      "1"
     ],
     [
      "14951477908965694779740690702214839201507078378144637359705168109082183011098",
      "15944301634537346624644053523753266562876929658362022070690389968468678445735",
      "1"
     ],
     [
      "8046753144044434216214328848512745642170554075691866308797562119947441176663",
      "13138546747083577798353765216798348806072590639624095879792595751918903492953",
      "1"
     ],
     [
      "9269841744461896652796424871767881077636845862896062313742276610868410367737",
      "19204398179879155372339166209988729037117351201989323410307343855513763594307",
      "1"
     ],
     [
      "7716881099355517822414707905850738186665313868979335283902370142936811833912",
      "11528196729400489690891393487077421013615412514692179449654303318954736859528",
      "1"
     ],
     [
      "5046803005209293905563542730106947483747999603892732318129931942914976300307",
      "18359807028421021791710128712790840004739725878390496786442730043904799620775",
      "1"
     ],
     [
      "7981969658925373733933252815358815642177002459779989502729025804210070642798",
      "2201884520453602381757167226560048379234361443956248008351600855207748053326",
      "1"
     ],
     [
      "2423450431733424040986489560227207633662164512765576249322846031854185964390",
      "4932502478942742742955368234556279877956422961403016735993130979527534572419",
      "1"
     ],
     [
      "4136066568774249110767040217927273928858469176929854865116246381005179602442",
      "14910781990631332327444098879460029001183528084133380954744967860772990143548",
      "1"
     ],
     [
      "9306442077615686666514943566045531276825026791237942942397956627887332762174",
      "5453464967372952047053268732003513663945971520297600856947977773603507455690",
      "1"
     ],
     [
      "13668279378191339240686523311787157905702403786071724115024939263519696082875",
      "17015511864316463912130081097138575082002221925436737115301652345996774761994",
      "1"
     ],
     [
      "11903142172414592342924720977734235164487862927368805935476433263236657614748",
      "13172937375115616110879840408060964756231709365794854299281160685167078990257",
      "1"
     ],
     [
      "14851933212838495488006105249354081780150791704393111451754186978257300101615",
      "145540996971132661488497461982320053303230165053371395584307173765669751355",
      "1"
     ],
     [
      "13492140304688249757734378821089945640145592582925007128524509462657370652800",
      "9390414912955206153533825682501092416440637486304199868194459556400963016224",
      "1"
     ],
     [
      "3496623709796463612824269552762145851578913713796096780105508335992863532011",
      "20265982111402342744121770330588454353873018338721172861869003808234581001103",
      "1"
     ],
     [
      "11129642812738675393866741757689241407730910968746319967156400192984156982663",
      "7270648547566738015174758472940005793787550000213654707465902989385799417672",
      "1"
     ],
     [
      "5336645303583636244521189039268557324541570944747369508544831506071145394833",
      "19288570029059212549483359253619647241300370202409702983102892733256823930029",
      "1"
     ],
     [
      "16583272323448913824043736762589960508623287550020508587844159466977422327617",
      "7808423190665056416769605983838691330236789706587082822452001424004744554191",
      "1"
     ],
     [
      "18660408822191296886711375730505189874857257159388446851193059936069141343915",
      "7010768444553285503461992330222909022718986907742690725937070802247722257521",
      "1"
     ],
     [
      "18362076977628428780847502863112204181768349745828739102417796831338046366054",
      "21442620406111530168500515404464430908000908371506932655420247401742151555185",
      "1"
     ],
     [
      "5162240315828150614443552770241127056707917547754134341853842620334081875585",
      "15355077221254938058224342345720923978502202301865073779695296894696562882502",
      "1"
     ]
    ]
   }
"#;

#[derive(Deserialize)]
struct JsonVerifyingKey {
    vk_alpha_1: Vec<String>,
    vk_beta_2: Vec<Vec<String>>,
    vk_gamma_2: Vec<Vec<String>>,
    vk_delta_2: Vec<Vec<String>>,
    ic: Vec<Vec<String>>,
}


#[derive(Deserialize)]
struct JsonProof {
    pi_a: Vec<String>,
    pi_b: Vec<Vec<String>>,
    pi_c: Vec<String>,
    // protocol and curve fields are not needed for the conversion
}

#[ic_cdk::update]
fn main(json_proof:String,public_input_json:String) -> bool {
    
    let json_vk: JsonVerifyingKey = serde_json::from_str(JSON_VERIFICATION_KEY).expect("JSON was not well-formatted");

    let vk = convert_to_verifying_key(json_vk);

    let vkPrepared = prepare_verifying_key(&vk);

    let proof_data: JsonProof = serde_json::from_str(json_proof.as_str()).expect("Proof JSON was not well-formatted");
    

    let pi_a = parse_g1(&proof_data.pi_a);
    let pi_b = parse_g2(&proof_data.pi_b);
    let pi_c = parse_g1(&proof_data.pi_c);


    let final_proof = Proof {
        a:pi_a,
        b: pi_b,
        c:pi_c
    };
    
    let parsed: Vec<String> = serde_json::from_str(&public_input_json).expect("Error in array json");

    let public_input = convert_to_public_input(parsed);


    
    // let verify = Groth16<Bn254>::verify_proof(vkPrepared, proof, public_inputs)
    // let bool = Groth16::verify_proof(pvk, proof, public_inputs);

     match Groth16::<Bn254>::verify_proof(&vkPrepared, &final_proof, &public_input) {
        Ok(valid) => {
          ic_cdk::println!("Proof is valid");
          ic_cdk::println!("{}", valid);
          return valid
        },
        Err(_) => 
        {
          ic_cdk::println!("Proof is invalid.");
          return false
        }
        
    };

    // Now `vk` is your VerifyingKey::<Bn254>
}

fn convert_to_public_input(json_array: Vec<String>) -> Vec<Fr> {
  json_array
      .into_iter()
      .map(|s| Fr::from_str(&s).expect("Failed to parse"))
      .collect()
}



fn convert_to_verifying_key(json_vk: JsonVerifyingKey) -> VerifyingKey<Bn254> {
    let alpha_g1 = parse_g1(&json_vk.vk_alpha_1);
    let beta_g2 = parse_g2(&json_vk.vk_beta_2);
    let gamma_g2 = parse_g2(&json_vk.vk_gamma_2);
    let delta_g2 = parse_g2(&json_vk.vk_delta_2);
    let gamma_abc_g1 = json_vk
        .ic
        .iter()
        .map(|point| parse_g1(point))
        .collect();
    
    VerifyingKey {
        alpha_g1,
        beta_g2,
        gamma_g2,
        delta_g2,
        gamma_abc_g1,        
    }
}

fn parse_g1(coords: &[String]) -> G1Affine {
    let x = Fq::from_str(&coords[0]).unwrap();
    let y = Fq::from_str(&coords[1]).unwrap();
    G1Affine::new(x, y) // Updated to two arguments
}

fn parse_g2(coords: &[Vec<String>]) -> G2Affine {
    let x = Fq2::new(
        Fq::from_str(&coords[0][0]).unwrap(),
        Fq::from_str(&coords[0][1]).unwrap(),
    );
    let y = Fq2::new(
        Fq::from_str(&coords[1][0]).unwrap(),
        Fq::from_str(&coords[1][1]).unwrap(),
    );
    G2Affine::new(x, y) // Updated to two arguments
}