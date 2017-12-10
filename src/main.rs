extern crate regex;
#[macro_use]
extern crate lazy_static;

use std::env;

mod solve_1_1;
mod solve_1_2;
mod solve_2_1;
mod solve_2_2;
mod solve_3_1;
mod solve_3_2;
mod solve_4_1;
mod solve_4_2;
mod solve_5_1;
mod solve_5_2;
mod solve_6_1;
mod solve_6_2;
mod solve_7_1;
mod solve_7_2;
mod solve_8_1;
mod solve_9_1;

fn day1() {
    let input = "68376334795224855827459835293967497295464175589881588256882344699473595413912688278647235862566123233983921662578792917453912795352746426512649965615919588512125567186837411371179875287621488759761429629174886972298349197722423458299323141529413191327622485249495864168181327197661454464926326248274999448373741839963155646828842752761293142356422964355349521987483211496361289666375779728345952231649453711684539164893151811849653331845998998597991146881361717234517911759893792348815818755262456378627116779495435596139617246571678531183335956244163871445674244765586446362529159854137535962117184875192273872222899887357292312978286182636232921252574738118347521187637829623831872437381979223955675634257889137823684924127338433248519515211796732599314921611399736571277222546332369461136277417419794865524123989722492356536832313937597437717873787593849468836733642529378547151146397532997237439387663769334722979172954835154486382983716698212694357398153392926255272961384626131829678171219569288685597141132355322788254163923888378155573948753185423158997877718687642446457446643422536541238979761725496426292359382168535641216124211741896552562128941824172241913873537828976172738276983915232241451589421911121567228899853934667954786256223614621554618294467191255153395256524786159758429643756586457639177183891162214163549688595416893383194995824534247841414247526268212761954913719452114876764745799982792594753759626334319631191917894368116738893548797661111899664138398354818931135486984944719992393148681724116616741428937687985152658296679845474766477741553632712968679175356452987459761126437216758171182395219393289199148996813762849991484678429793578629331215796996751484375784895561682156658579887518746862371751372692472765217374791324656745291574784495299477362964676351148183676897122366838656342745944945275263617729359831466565694983217252594237828187612857523344265418227883219383138893873384775659548637662867572687198263688597865118173921615178165442133987362382721444844952715592955744739873677838847693982379696776";
    println!("day 1");
    println!("  part 1: {}", solve_1_1::solve(input));
    println!("  part 2: {}", solve_1_2::solve(input));
}

fn day2() {
    let input = "493    458     321     120     49      432     433     92      54      452     41      461     388     409     263     58
                 961    98      518     188     958     114     1044    881     948     590     972     398     115     116     451     492
                 76     783     709     489     617     72      824     452     748     737     691     90      94      77      84      756
                 204    217     90      335     220     127     302     205     242     202     259     110     118     111     200     112
                 249    679     4015    106     3358    1642    228     4559    307     193     4407    3984    3546    2635    3858    924
                 1151   1060    2002    168     3635    3515    3158    141     4009    3725    996     142     3672    153     134     1438
                 95     600     1171    1896    174     1852    1616    928     79      1308    2016    88      80      1559    1183    107
                 187    567     432     553     69      38      131     166     93      132     498     153     441     451     172     575
                 216    599     480     208     224     240     349     593     516     450     385     188     482     461     635     220
                 788    1263    1119    1391    1464    179     1200    621     1304    55      700     1275    226     57      43      51
                 1571   58      1331    1253    60      1496    1261    1298    1500    1303    201     73      1023    582     69      339
                 80     438     467     512     381     74      259     73      88      448     386     509     346     61      447     435
                 215    679     117     645     137     426     195     619     268     223     792     200     720     260     303     603
                 631    481     185     135     665     641     492     408     164     132     478     188     444     378     633     516
                 1165   1119    194     280     223     1181    267     898     1108    124     618     1135    817     997     129     227
                 404    1757    358     2293    2626    87      613     95      1658    147     75      930     2394    2349    86      385";
    println!("day 2");
    println!("  part 1: {}", solve_2_1::solve(input));
    println!("  part 2: {}", solve_2_2::solve(input));
}

fn day3() {
    let input = 265149;
    println!("day 3");
    println!("  part 1: {}", solve_3_1::solve(input));
    println!("  part 2: {}", solve_3_2::solve(input));
}

fn day4() {
    println!("day 4");
    println!("  part 1: {}", solve_4_1::solve("inputs/input_4_1.txt"));
    println!("  part 2: {}", solve_4_2::solve("inputs/input_4_1.txt"));
}

fn day5() {
    println!("day 5");
    println!("  part 1: {}", solve_5_1::solve("inputs/input_5.txt"));
    println!("  part 2: {}", solve_5_2::solve("inputs/input_5.txt"));
}

fn day6() {
    println!("day 6");
    println!("  part 1: {}", solve_6_1::solve("inputs/input_6.txt"));
    println!("  part 2: {}", solve_6_2::solve("inputs/input_6.txt"));
}

fn day7() {
    println!("day 7");
    println!("  part 1: {}", solve_7_1::solve(include_str!("../inputs/input_7.txt")));
    println!("  part 2: {}", solve_7_2::solve(include_str!("../inputs/input_7.txt")));
}

fn day8() {
    println!("day 8");
    let (part1, part2) = solve_8_1::solve();
    println!("  part 1: {}", part1);
    println!("  part 2: {}", part2);
}

fn day9() {
    println!("day 9");
    let (p1, p2) = solve_9_1::solve();
    println!("  part 1: {}", p1);
    println!("  part 2: {}", p2);
}


fn main() {
    let day = env::args().nth(1).unwrap_or_else(|| String::from("1"));

    match day.as_ref() {
        "1" => day1(),
        "2" => day2(),
        "3" => day3(),
        "4" => day4(),
        "5" => day5(),
        "6" => day6(),
        "7" => day7(),
        "8" => day8(),
        "9" => day9(),
        _ => panic!()
    }
}
