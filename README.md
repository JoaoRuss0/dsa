This repository contains my solutions to some really interesting problems, pubished from time to time at the websites below.

- [Advent of Code](https://adventofcode.com/)
- [Everybody Codes](https://everybody.codes)
- [Codyssi](https://www.codyssi.com/)
- [Flip Flop](https://flipflop.slome.org/)
- [LeetCode](https://leetcode.com/)

Jetbrains File Template:

```velocity
#set($DAY_OF_PROBLEM = "")
#set($NAME_OF_PROBLEM = "")

#set($parent = "")
#set($grandparent = "")
#set($seg = "")
#foreach($seg in $DIR_PATH.split("/"))
#set($grandparent = $parent)
#set($parent = $seg)
#end

#set($colon                 = $TITLE.indexOf(":"))
#set($dayAt                 = $TITLE.indexOf("Day ") + 4)
#set($DAY_OF_PROBLEM        = $TITLE.substring($dayAt, $colon))
#set($nameAt                = $colon + 2)
#set($nameEnd               = $TITLE.lastIndexOf(" ---"))
#set($NAME_OF_PROBLEM       = $TITLE.substring($nameAt, $nameEnd))

pub fn run() {
println!("  ├─ Day ${DAY_OF_PROBLEM} - ${NAME_OF_PROBLEM}");

    let path = "input/$grandparent/$parent/D${DAY_OF_PROBLEM}.txt";
    let input = std::fs::read_to_string(path).unwrap();

    println!("  │  ├─ Part 1: {}", );
    //println!("  │  └─ Part 2: {}", );
}
```